use crate::ili9341::Ili9341;

/* 6x8 minimal ASCII subset (dummy pattern) */
const FONT_W: u8 = 6;
const FONT_H: u8 = 8;

fn glyph(_c:u8)->[u8;6]{
    [0xFF,0x81,0xBD,0xA5,0x81,0xFF]
}

pub fn draw_char(lcd:&Ili9341,ch:u8,x:u16,y:u16,fg:u16,bg:u16){
    let g=glyph(ch);

    lcd.set_window(x,y,x+FONT_W as u16-1,y+FONT_H as u16-1);

    for byte in g {
        for i in 0..8 {
            let bit = (byte >> (7-i)) & 1;
            let col = if bit==1 {fg} else {bg};

            crate::hw::dc_data();
            crate::hw::cs_low();
            crate::hw::spi_write((col>>8) as u8);
            crate::hw::spi_write(col as u8);
            crate::hw::cs_high();
        }
    }
}

pub fn draw_text(lcd:&Ili9341,s:&str,x:u16,y:u16,fg:u16,bg:u16){
    let mut x=x;
    for c in s.bytes(){
        draw_char(lcd,c,x,y,fg,bg);
        x+=FONT_W as u16;
    }
}
