//! Bitmap text rendering (Tilen Majerle font format, SPI DMA).

mod data;

use crate::ili9341::Ili9341;

use data::{DATA_11X18, DATA_16X26, DATA_7X10};

/// Max glyph pixels across bundled fonts (16×26 = 416).
const GLYPH_DMA_CAP: usize = 512;

static mut GLYPH_BUF: [u16; GLYPH_DMA_CAP] = [0; GLYPH_DMA_CAP];

/// Printable ASCII from space (`0x20`) through `~` (`0x7E`).
const GLYPH_COUNT: usize = 95;

/// Font descriptor (same layout as C `TM_FontDef_t`).
pub struct Font {
    pub width: u8,
    pub height: u8,
    data: &'static [u16],
}

impl Font {
    pub const fn new(width: u8, height: u8, data: &'static [u16]) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub const fn glyph_pixels(&self) -> usize {
        self.width as usize * self.height as usize
    }

    fn glyph_rows(&self, ch: u8) -> Option<&[u16]> {
        let idx = ch.checked_sub(b' ')? as usize;
        if idx >= GLYPH_COUNT {
            return None;
        }
        let start = idx * self.height as usize;
        let end = start + self.height as usize;
        self.data.get(start..end)
    }
}

pub const FONT_7X10: Font = Font::new(7, 10, &DATA_7X10);
pub const FONT_11X18: Font = Font::new(11, 18, &DATA_11X18);
pub const FONT_16X26: Font = Font::new(16, 26, &DATA_16X26);

fn rasterize_glyph(font: &Font, ch: u8, fg: u16, bg: u16, out: &mut [u16]) -> usize {
    let rows = font.glyph_rows(ch).unwrap_or_else(|| {
        font.glyph_rows(b' ').unwrap_or(&[])
    });

    let w = font.width as usize;
    let mut i = 0usize;
    for &row in rows {
        for col in 0..w {
            let on = (row << col) & 0x8000 != 0;
            out[i] = if on { fg } else { bg };
            i += 1;
        }
    }
    i
}

/// Width in pixels of `s` at `font` (no trailing space trim).
pub fn text_width(font: &Font, s: &str) -> u16 {
    font.width as u16 * s.len() as u16
}

pub fn draw_char(
    lcd: &Ili9341,
    font: &Font,
    ch: u8,
    x: u16,
    y: u16,
    fg: u16,
    bg: u16,
) {
    let pixels = font.glyph_pixels();
    if pixels > GLYPH_DMA_CAP {
        return;
    }

    let buf = unsafe { &mut GLYPH_BUF[..pixels] };
    rasterize_glyph(font, ch, fg, bg, buf);
    lcd.push_pixels(
        x,
        y,
        x + font.width as u16 - 1,
        y + font.height as u16 - 1,
        buf,
    );
}

pub fn draw_text(
    lcd: &Ili9341,
    font: &Font,
    s: &str,
    mut x: u16,
    y: u16,
    fg: u16,
    bg: u16,
) {
    for c in s.bytes() {
        draw_char(lcd, font, c, x, y, fg, bg);
        x += font.width as u16;
    }
}
