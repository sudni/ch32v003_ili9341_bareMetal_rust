//! RCC: 48 MHz from 24 MHz HSE × PLL×2 (same as C reference `SetSysClockTo_48MHz_HSE`).

use core::ptr;

use crate::regs::*;

const HSE_STARTUP_RETRIES: u32 = 0x0500;

pub fn system_init_48mhz_hse() {
    unsafe {
        let rcc = RCC_BASE as *mut u32;
        let afio = AFIO_BASE as *mut u32;
        let flash = FLASH_R_BASE as *mut u32;

        ptr::write_volatile(rcc.add(reg_idx(RCC_APB2PCENR)), ptr::read_volatile(rcc.add(reg_idx(RCC_APB2PCENR))) | RCC_APB2_AFIOEN);
        ptr::write_volatile(
            afio.add(reg_idx(AFIO_PCFR1)),
            ptr::read_volatile(afio.add(reg_idx(AFIO_PCFR1))) | AFIO_PCFR1_HSE_PA1_PA2,
        );

        ptr::write_volatile(rcc.add(reg_idx(RCC_CTLR)), ptr::read_volatile(rcc.add(reg_idx(RCC_CTLR))) | RCC_HSEON);

        let mut ok = false;
        for _ in 0..HSE_STARTUP_RETRIES {
            if ptr::read_volatile(rcc.add(reg_idx(RCC_CTLR))) & RCC_HSERDY != 0 {
                ok = true;
                break;
            }
        }
        if !ok {
            return;
        }

        ptr::write_volatile(flash.add(reg_idx(FLASH_ACTLR)), FLASH_ACTLR_LATENCY_1);

        let cfgr = rcc.add(reg_idx(RCC_CFGR0));
        ptr::write_volatile(cfgr, ptr::read_volatile(cfgr) | RCC_HPRE_DIV1);
        ptr::write_volatile(cfgr, (ptr::read_volatile(cfgr) & !RCC_PLLSRC_HSE_MUL2) | RCC_PLLSRC_HSE_MUL2);

        ptr::write_volatile(rcc.add(reg_idx(RCC_CTLR)), ptr::read_volatile(rcc.add(reg_idx(RCC_CTLR))) | RCC_PLLON);
        while ptr::read_volatile(rcc.add(reg_idx(RCC_CTLR))) & RCC_PLLRDY == 0 {}

        ptr::write_volatile(cfgr, (ptr::read_volatile(cfgr) & !RCC_SW) | RCC_SW_PLL);
        while ptr::read_volatile(cfgr) & RCC_SWS != RCC_SWS_PLL_VAL {}
    }
}
