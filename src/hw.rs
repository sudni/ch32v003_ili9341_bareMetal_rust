use core::ptr;

const SPI1_BASE: u32 = 0x40013000;
const GPIOC_BSHR: *mut u32 = 0x40011010 as *mut u32;
const GPIOD_BSHR: *mut u32 = 0x40011410 as *mut u32;

#[inline(always)]
pub fn spi_write(b: u8) {
    unsafe {
        let sr = (SPI1_BASE + 0x08) as *mut u16;
        let dr = (SPI1_BASE + 0x0C) as *mut u8;

        while ptr::read_volatile(sr) & (1 << 1) == 0 {}
        ptr::write_volatile(dr, b);
        while ptr::read_volatile(sr) & (1 << 0) == 0 {}
        let _ = ptr::read_volatile(dr);
    }
}

#[inline(always)]
pub fn cs_low() { unsafe { *GPIOC_BSHR = 1 << (1 + 16); } }
#[inline(always)]
pub fn cs_high() { unsafe { *GPIOC_BSHR = 1 << 1; } }
#[inline(always)]
pub fn dc_cmd() { unsafe { *GPIOD_BSHR = 1 << (3 + 16); } }
#[inline(always)]
pub fn dc_data() { unsafe { *GPIOD_BSHR = 1 << 3; } }
#[inline(always)]
pub fn rst_low() { unsafe { *GPIOD_BSHR = 1 << (4 + 16); } }
#[inline(always)]
pub fn rst_high() { unsafe { *GPIOD_BSHR = 1 << 4; } }

pub fn delay(mut n: u32) {
    while n > 0 {
        unsafe { core::ptr::read_volatile(&0); }
        n -= 1;
    }
}
