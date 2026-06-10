use core::ptr;
use core::hint::spin_loop;

use crate::pins::{
    PC1_LCD_CS, PC5_SPI1_SCK, PC6_SPI1_MOSI, PD2_TIM1_CH1_BL, PD3_LCD_DC, PD4_LCD_RST,
    gpio_bshr_reset, gpio_bshr_set,
};
use crate::regs::*;
use crate::rcc::system_init_48mhz_hse;

/// CPU core clock after [`platform_init`] (Hz).
pub const DELAY_ASSUMED_CPU_HZ: u32 = 48_000_000;

const APPROX_CYCLES_PER_BUSY_ITER: u32 = 5;
const BUSY_LOOPS_PER_MS: u32 = DELAY_ASSUMED_CPU_HZ / 1000 / APPROX_CYCLES_PER_BUSY_ITER;

#[inline(always)]
fn delay_busy_loops(mut n: u32) {
    while n > 0 {
        spin_loop();
        n -= 1;
    }
}

pub fn delay_us(us: u32) {
    let per_us = (BUSY_LOOPS_PER_MS / 1000).max(1);
    delay_busy_loops(per_us.saturating_mul(us));
}

pub fn delay_ms(ms: u32) {
    for _ in 0..ms {
        delay_busy_loops(BUSY_LOOPS_PER_MS);
    }
}

#[inline(always)]
unsafe fn gpioc_bshr(v: u32) {
    ptr::write_volatile((GPIOC_BASE + GPIO_BSHR_OFFSET) as *mut u32, v);
}

#[inline(always)]
unsafe fn gpiod_bshr(v: u32) {
    ptr::write_volatile((GPIOD_BASE + GPIO_BSHR_OFFSET) as *mut u32, v);
}

#[inline(always)]
pub fn cs_low() {
    unsafe { gpioc_bshr(gpio_bshr_reset(PC1_LCD_CS)) }
}

#[inline(always)]
pub fn cs_high() {
    unsafe { gpioc_bshr(gpio_bshr_set(PC1_LCD_CS)) }
}

#[inline(always)]
pub fn dc_cmd() {
    unsafe { gpiod_bshr(gpio_bshr_reset(PD3_LCD_DC)) }
}

#[inline(always)]
pub fn dc_data() {
    unsafe { gpiod_bshr(gpio_bshr_set(PD3_LCD_DC)) }
}

#[inline(always)]
pub fn rst_low() {
    unsafe { gpiod_bshr(gpio_bshr_reset(PD4_LCD_RST)) }
}

#[inline(always)]
pub fn rst_high() {
    unsafe { gpiod_bshr(gpio_bshr_set(PD4_LCD_RST)) }
}

/// Backlight PWM period (ATRLR). Duty range: `0..=BACKLIGHT_PWM_MAX`.
pub const BACKLIGHT_PWM_MAX: u16 = 1023;

static mut BACKLIGHT_PWM_READY: bool = false;

/// Backlight full on via TIM1 CH1 PWM on **PD2**.
pub fn backlight_on() {
    backlight_set_duty(BACKLIGHT_PWM_MAX);
}

/// Set backlight brightness. `duty`: `0` = off, `BACKLIGHT_PWM_MAX` = full on.
pub fn backlight_set_duty(duty: u16) {
    unsafe {
        if !BACKLIGHT_PWM_READY {
            tim1_backlight_init();
            BACKLIGHT_PWM_READY = true;
        }
        let ch1 = (TIM1_BASE + TIM_CH1CVR) as *mut u32;
        ptr::write_volatile(ch1, u32::from(duty.min(BACKLIGHT_PWM_MAX)));
    }
}

unsafe fn tim1_backlight_init() {
    let rcc = RCC_BASE as *mut u32;
    ptr::write_volatile(
        rcc.add(reg_idx(RCC_APB2PCENR)),
        ptr::read_volatile(rcc.add(reg_idx(RCC_APB2PCENR)))
            | RCC_APB2_IOPDEN
            | RCC_APB2_TIM1EN,
    );

    // PD2 → TIM1_CH1, AF push-pull 50 MHz.
    gpio_pin_cfgr(GPIOD_BASE, PD2_TIM1_CH1_BL, GPIO_MODE_OUT_50MHZ, GPIO_CNF_AF_PP);

    let tim = TIM1_BASE;
    ptr::write_volatile((tim + TIM_PSC) as *mut u16, 0);
    ptr::write_volatile((tim + TIM_ATRLR) as *mut u16, BACKLIGHT_PWM_MAX);

    let swevgr = (tim + TIM_SWEVGR) as *mut u8;
    ptr::write_volatile(swevgr, TIM_UG);

    let chctlr1 = (tim + TIM_CHCTLR1) as *mut u16;
    ptr::write_volatile(
        chctlr1,
        ptr::read_volatile(chctlr1) & !(TIM_OC1M_1 | TIM_OC1M_2),
    );
    ptr::write_volatile(
        chctlr1,
        ptr::read_volatile(chctlr1) | TIM_OC1M_1 | TIM_OC1M_2 | TIM_OC1PE,
    );

    let ccer = (tim + TIM_CCER) as *mut u16;
    ptr::write_volatile(ccer, ptr::read_volatile(ccer) | TIM_CC1E);

    let bdtr = (tim + TIM_BDTR) as *mut u16;
    ptr::write_volatile(bdtr, ptr::read_volatile(bdtr) | TIM_MOE);

    let ctlr1 = (tim + TIM_CTLR1) as *mut u16;
    ptr::write_volatile(ctlr1, ptr::read_volatile(ctlr1) | TIM_CEN | TIM_ARPE);
}

unsafe fn gpio_pin_cfgr(base: u32, pin: u8, mode: u32, cnf: u32) {
    let cfg = (base + GPIO_CFGR_OFFSET) as *mut u32;
    let shift = u32::from(pin) * 4;
    let mask = 0xF << shift;
    let val = (mode | (cnf << 2)) << shift;
    ptr::write_volatile(cfg, (ptr::read_volatile(cfg) & !mask) | val);
}

unsafe fn lcd_gpio_init() {
    let rcc = RCC_BASE as *mut u32;
    ptr::write_volatile(
        rcc.add(reg_idx(RCC_APB2PCENR)),
        ptr::read_volatile(rcc.add(reg_idx(RCC_APB2PCENR))) | RCC_APB2_IOPCEN | RCC_APB2_IOPDEN,
    );

    let out = GPIO_MODE_OUT_50MHZ;
    gpio_pin_cfgr(GPIOC_BASE, PC1_LCD_CS, out, 0);
    gpio_pin_cfgr(GPIOC_BASE, PC5_SPI1_SCK, out, GPIO_CNF_AF_PP);
    gpio_pin_cfgr(GPIOC_BASE, PC6_SPI1_MOSI, out, GPIO_CNF_AF_PP);
    gpio_pin_cfgr(GPIOD_BASE, PD3_LCD_DC, out, 0);
    gpio_pin_cfgr(GPIOD_BASE, PD4_LCD_RST, out, 0);

    cs_high();
    dc_data();
    rst_high();
}

unsafe fn spi_data_8b() {
    let ctlr1 = (SPI1_BASE + SPI_CTLR1) as *mut u16;
    ptr::write_volatile(ctlr1, ptr::read_volatile(ctlr1) & !SPI_CTLR1_DFF);
}

unsafe fn spi_data_16b() {
    let ctlr1 = (SPI1_BASE + SPI_CTLR1) as *mut u16;
    ptr::write_volatile(ctlr1, ptr::read_volatile(ctlr1) | SPI_CTLR1_DFF);
}

unsafe fn spi_init() {
    let rcc = RCC_BASE as *mut u32;
    ptr::write_volatile(
        rcc.add(reg_idx(RCC_APB2PCENR)),
        ptr::read_volatile(rcc.add(reg_idx(RCC_APB2PCENR))) | RCC_APB2_IOPCEN | RCC_APB2_SPI1EN,
    );
    ptr::write_volatile(
        rcc.add(reg_idx(RCC_AHBPCENR)),
        ptr::read_volatile(rcc.add(reg_idx(RCC_AHBPCENR))) | RCC_AHB_DMA1EN,
    );

    let ctlr1 = (SPI1_BASE + SPI_CTLR1) as *mut u16;
    ptr::write_volatile(
        ctlr1,
        SPI_CTLR1_SSI | SPI_CTLR1_SSM | SPI_CTLR1_MSTR,
    );

    let ctlr2 = (SPI1_BASE + SPI_CTLR2) as *mut u16;
    ptr::write_volatile(ctlr2, ptr::read_volatile(ctlr2) | SPI_CTLR2_TXDMAEN);

    let ch = DMA1_CH3_BASE;
    let cfg = (ch + DMA_CFGR) as *mut u32;
    ptr::write_volatile(
        cfg,
        u32::from(DMA_CFGR_PL_0 | DMA_CFGR_PL_1 | DMA_CFGR_PSIZE_0 | DMA_CFGR_MSIZE_0 | DMA_CFGR_DIR),
    );
    ptr::write_volatile(
        (ch + DMA_PADDR) as *mut u32,
        SPI1_BASE + SPI_DATAR,
    );

    ptr::write_volatile(ctlr1, ptr::read_volatile(ctlr1) | SPI_CTLR1_SPE);
    spi_data_8b();
}

/// Clock (48 MHz HSE+PLL), LCD GPIO, SPI1 @ fPCLK/2, DMA1 ch3 → SPI TX.
pub fn platform_init() {
    system_init_48mhz_hse();
    unsafe {
        lcd_gpio_init();
        spi_init();
    }
}

pub fn spi_write(b: u8) {
    unsafe {
        spi_data_8b();
        let sr = (SPI1_BASE + SPI_STATR) as *mut u16;
        let dr = (SPI1_BASE + SPI_DATAR) as *mut u16;
        while ptr::read_volatile(sr) & SPI_STATR_TXE == 0 {}
        ptr::write_volatile(dr, u16::from(b));
        while ptr::read_volatile(sr) & SPI_STATR_BSY != 0 {}
    }
}

pub fn spi_write16(v: u16) {
    unsafe {
        spi_data_16b();
        let sr = (SPI1_BASE + SPI_STATR) as *mut u16;
        let dr = (SPI1_BASE + SPI_DATAR) as *mut u16;
        while ptr::read_volatile(sr) & SPI_STATR_TXE == 0 {}
        ptr::write_volatile(dr, v);
        while ptr::read_volatile(sr) & SPI_STATR_BSY != 0 {}
    }
}

unsafe fn dma_mem_inc(on: bool) {
    let cfg = (DMA1_CH3_BASE + DMA_CFGR) as *mut u32;
    if on {
        ptr::write_volatile(cfg, ptr::read_volatile(cfg) | u32::from(DMA_CFGR_MINC));
    } else {
        ptr::write_volatile(cfg, ptr::read_volatile(cfg) & !u32::from(DMA_CFGR_MINC));
    }
}

unsafe fn dma_wait_ch3() {
    let intfr = (DMA1_BASE + DMA1_INTFR) as *mut u32;
    while ptr::read_volatile(intfr) & DMA_TCIF3 == 0 {}
    ptr::write_volatile((DMA1_BASE + DMA1_INTFCR) as *mut u32, DMA_CGIF3);
}

/// DMA 16-bit SPI burst; `mem_inc` false repeats one halfword (solid fill).
pub fn spi_dma_pixels(data: *const u16, count: u16, mem_inc: bool) {
    if count == 0 {
        return;
    }
    unsafe {
        spi_data_16b();
        dma_mem_inc(mem_inc);

        let ch = DMA1_CH3_BASE;
        let cfg = (ch + DMA_CFGR) as *mut u32;
        ptr::write_volatile(cfg, ptr::read_volatile(cfg) & !u32::from(DMA_CFGR_EN));
        ptr::write_volatile((ch + DMA_MADDR) as *mut u32, data as u32);
        ptr::write_volatile((ch + DMA_CNTR) as *mut u32, u32::from(count));
        ptr::write_volatile(cfg, ptr::read_volatile(cfg) | u32::from(DMA_CFGR_EN));

        dma_wait_ch3();
    }
}
