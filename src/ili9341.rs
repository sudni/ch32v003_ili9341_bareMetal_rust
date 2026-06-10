use crate::hw::*;

pub const WIDTH: u16 = 240;
pub const HEIGHT: u16 = 320;

pub const TILE_W: u16 = 40;
pub const TILE_H: u16 = 40;

const TILES_X: u16 = WIDTH / TILE_W;
const TILES_Y: u16 = HEIGHT / TILE_H;
const TILE_COUNT: usize = (TILES_X * TILES_Y) as usize;

/// ILI9341 command bytes (C reference driver + ILITEK datasheet).
mod cmd {
    pub const SWRESET: u8 = 0x01;
    pub const SLEEP_OUT: u8 = 0x11;
    pub const GAMMA_SET: u8 = 0x26;
    pub const DISPLAY_ON: u8 = 0x29;
    pub const COLUMN_ADDR: u8 = 0x2A;
    pub const PAGE_ADDR: u8 = 0x2B;
    pub const GRAM: u8 = 0x2C;
    pub const WRITE_CONTINUE: u8 = 0x3C;
    pub const TE_ON: u8 = 0x35;
    pub const MAC: u8 = 0x36;
    pub const PIXEL_FORMAT: u8 = 0x3A;
    pub const POWERA: u8 = 0xCB;
    pub const POWERB: u8 = 0xCF;
    pub const DTCA: u8 = 0xE8;
    pub const DTCB: u8 = 0xEA;
    pub const POWER_SEQ: u8 = 0xED;
    pub const PRC: u8 = 0xF7;
    pub const POWER1: u8 = 0xC0;
    pub const POWER2: u8 = 0xC1;
    pub const VCOM1: u8 = 0xC5;
    pub const VCOM2: u8 = 0xC7;
    pub const FRC: u8 = 0xB1;
    pub const DFC: u8 = 0xB6;
    pub const ENABLE_3G: u8 = 0xF2;
    pub const PGAMMA: u8 = 0xE0;
    pub const NGAMMA: u8 = 0xE1;
}

/// Post-reset / power-on register block from Adamir Hamulic C driver.
static INIT_TABLE: &[(u8, &[u8])] = &[
    (cmd::POWERA, &[0x39, 0x2C, 0x00, 0x34, 0x02]),
    (cmd::POWERB, &[0x00, 0xC1, 0x30]),
    (cmd::DTCA, &[0x85, 0x00, 0x78]),
    (cmd::DTCB, &[0x00, 0x00]),
    (cmd::POWER_SEQ, &[0x64, 0x03, 0x12, 0x81]),
    (cmd::PRC, &[0x20]),
    (cmd::POWER1, &[0x23]),
    (cmd::POWER2, &[0x10]),
    (cmd::VCOM1, &[0x3E, 0x28]),
    (cmd::VCOM2, &[0x86]),
    (cmd::MAC, &[0x48]),
    (cmd::PIXEL_FORMAT, &[0x55]),
    (cmd::FRC, &[0x00, 0x13]),
    (cmd::DFC, &[0x08, 0x82, 0x27]),
    (cmd::ENABLE_3G, &[0x00]),
    (cmd::COLUMN_ADDR, &[0x00, 0x00, 0x00, 0xEF]),
    (cmd::PAGE_ADDR, &[0x00, 0x00, 0x01, 0x3F]),
    (cmd::GAMMA_SET, &[0x01]),
    (
        cmd::PGAMMA,
        &[
            0x0F, 0x31, 0x2B, 0x0C, 0x0E, 0x08, 0x4E, 0xF1, 0x37, 0x07, 0x10, 0x03, 0x0E, 0x09,
            0x00,
        ],
    ),
    (
        cmd::NGAMMA,
        &[
            0x00, 0x0E, 0x14, 0x03, 0x11, 0x07, 0x31, 0xC1, 0x48, 0x08, 0x0F, 0x0C, 0x31, 0x36,
            0x0F,
        ],
    ),
];

pub struct Ili9341;

impl Ili9341 {
    pub fn new() -> Self {
        Self
    }

    pub fn init(&self) {
        platform_init();

        rst_low();
        delay_us(10);
        rst_high();
        delay_us(10);

        self.cmd(cmd::SWRESET);
        delay_ms(100);

        for &(c, params) in INIT_TABLE {
            self.cmd(c);
            for &p in params {
                self.data(p);
            }
        }

        self.cmd(cmd::SLEEP_OUT);
        delay_ms(10);
        self.cmd(cmd::DISPLAY_ON);
        delay_ms(10);
        self.cmd(cmd::GRAM);

        self.cmd(cmd::TE_ON);
        self.data(0x00);

        backlight_on();
    }

    #[inline(always)]
    fn cmd(&self, c: u8) {
        dc_cmd();
        cs_low();
        spi_write(c);
        cs_high();
    }

    #[inline(always)]
    fn data(&self, d: u8) {
        dc_data();
        cs_low();
        spi_write(d);
        cs_high();
    }

    fn data16(&self, v: u16) {
        dc_data();
        cs_low();
        spi_write16(v);
        cs_high();
    }

    pub fn set_window(&self, x0: u16, y0: u16, x1: u16, y1: u16) {
        self.cmd(cmd::COLUMN_ADDR);
        self.data16(x0);
        self.data16(x1);
        self.cmd(cmd::PAGE_ADDR);
        self.data16(y0);
        self.data16(y1);
    }

    /// Solid RGB565 rectangle via DMA (MINC off — one color repeated).
    pub fn fill_rectangle(&self, x0: u16, y0: u16, x1: u16, y1: u16, color: u16) {
        let w = x1.saturating_sub(x0) + 1;
        let h = y1.saturating_sub(y0) + 1;
        let pixels = u32::from(w) * u32::from(h);
        if pixels == 0 {
            return;
        }

        self.set_window(x0, y0, x1, y1);
        self.cmd(cmd::GRAM);
        self.cmd(cmd::WRITE_CONTINUE);

        dc_data();
        cs_low();

        let mut left = pixels;
        while left > 0 {
            let chunk = left.min(u32::from(u16::MAX)) as u16;
            spi_dma_pixels(&color, chunk, false);
            left -= u32::from(chunk);
        }

        cs_high();
    }

    /// Push one solid-color tile (no RAM tile buffer — matches C `fill_rectangle` pattern).
    pub fn push_solid_tile(&self, tx: u16, ty: u16, color: u16) {
        let x0 = tx * TILE_W;
        let y0 = ty * TILE_H;
        self.fill_rectangle(
            x0,
            y0,
            x0 + TILE_W - 1,
            y0 + TILE_H - 1,
            color,
        );
    }

    /// RGB565 buffer via DMA (MINC on).
    pub fn push_pixels(&self, x0: u16, y0: u16, x1: u16, y1: u16, buf: &[u16]) {
        let expected = (x1 as u32 + 1 - x0 as u32) * (y1 as u32 + 1 - y0 as u32);
        if buf.len() as u32 != expected {
            return;
        }

        self.set_window(x0, y0, x1, y1);
        self.cmd(cmd::GRAM);
        self.cmd(cmd::WRITE_CONTINUE);

        dc_data();
        cs_low();
        spi_dma_pixels(buf.as_ptr(), buf.len() as u16, true);
        cs_high();
    }
}

pub struct Tiles {
    pub dirty: [bool; TILE_COUNT],
}

impl Tiles {
    pub const fn new() -> Self {
        Self { dirty: [true; TILE_COUNT] }
    }

    pub fn next(&self) -> Option<(u16, u16)> {
        for i in 0..TILE_COUNT {
            if self.dirty[i] {
                return Some((i as u16 % TILES_X, i as u16 / TILES_X));
            }
        }
        None
    }

    pub fn clear(&mut self, x: u16, y: u16) {
        self.dirty[(y * TILES_X + x) as usize] = false;
    }

    pub fn mark_all(&mut self) {
        for d in self.dirty.iter_mut() {
            *d = true;
        }
    }
}
