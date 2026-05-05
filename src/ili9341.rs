use crate::hw::*;

pub const TILE_W: u16 = 40;
pub const TILE_H: u16 = 40;

pub struct Ili9341;

impl Ili9341 {
    pub fn new() -> Self { Self }

    pub fn init(&self) {
        rst_low();
        delay(50_000);
        rst_high();
        delay(200_000);

        self.cmd(0x01);
        delay(200_000);

        self.cmd(0x3A); self.data(0x55);
        self.cmd(0x36); self.data(0x48);

        self.cmd(0x11);
        delay(200_000);

        self.cmd(0x35); self.data(0x00);
        self.cmd(0x29);
    }

    #[inline(always)]
    fn cmd(&self, c: u8) {
        dc_cmd(); cs_low(); spi_write(c); cs_high();
    }

    #[inline(always)]
    fn data(&self, d: u8) {
        dc_data(); cs_low(); spi_write(d); cs_high();
    }

    fn data16(&self, v: u16) {
        self.data((v >> 8) as u8);
        self.data(v as u8);
    }

    pub fn set_window(&self, x0:u16,y0:u16,x1:u16,y1:u16){
        self.cmd(0x2A); self.data16(x0); self.data16(x1);
        self.cmd(0x2B); self.data16(y0); self.data16(y1);
        self.cmd(0x2C);
    }

    pub fn push_tile(&self, tx:u16,ty:u16,buf:&[u16]){
        let x=tx*TILE_W; let y=ty*TILE_H;
        self.set_window(x,y,x+TILE_W-1,y+TILE_H-1);

        dc_data(); cs_low();
        for &p in buf{
            spi_write((p>>8) as u8);
            spi_write(p as u8);
        }
        cs_high();
    }
}

pub struct Tiles { pub dirty:[bool;48] }

impl Tiles {
    pub const fn new()->Self{Self{dirty:[true;48]}}

    pub fn next(&self)->Option<(u16,u16)>{
        for i in 0..48 {
            if self.dirty[i] {
                return Some((i as u16 % 6, i as u16 / 6));
            }
        }
        None
    }

    pub fn clear(&mut self,x:u16,y:u16){
        self.dirty[(y*6+x) as usize]=false;
    }
}
