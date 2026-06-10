use crate::ili9341::Ili9341;

const FONT_W: u8 = 6;
const FONT_H: u8 = 8;
const GLYPH_DMA_CAP: usize = 512;

fn glyph(_c: u8) -> [u8; 6] {
    [0xFF, 0x81, 0xBD, 0xA5, 0x81, 0xFF]
}

static mut GLYPH_BUF: [u16; GLYPH_DMA_CAP] = [0; GLYPH_DMA_CAP];

fn rasterize_glyph(ch: u8, fg: u16, bg: u16, out: &mut [u16]) -> usize {
    let g = glyph(ch);
    let mut i = 0usize;
    for byte in g {
        for bit in 0..8 {
            let on = (byte >> (7 - bit)) & 1;
            out[i] = if on == 1 { fg } else { bg };
            i += 1;
        }
    }
    i
}

pub fn draw_char(lcd: &Ili9341, ch: u8, x: u16, y: u16, fg: u16, bg: u16) {
    let pixels = (FONT_W as u16) * (FONT_H as u16);
    let buf = unsafe { &mut GLYPH_BUF[..pixels as usize] };
    rasterize_glyph(ch, fg, bg, buf);
    lcd.push_pixels(
        x,
        y,
        x + FONT_W as u16 - 1,
        y + FONT_H as u16 - 1,
        buf,
    );
}

pub fn draw_text(lcd: &Ili9341, s: &str, x: u16, y: u16, fg: u16, bg: u16) {
    let mut x = x;
    for c in s.bytes() {
        draw_char(lcd, c, x, y, fg, bg);
        x += FONT_W as u16;
    }
}
