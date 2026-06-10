//! CH32V003 peripheral bases and GPIO pin names for this LCD board.
//!
//! Matches `docs/hardware.md` (F4P6-style pinout). Only pins used or reserved
//! for this firmware are listed; see the CH32V003 datasheet for the full table.
//!
//! Several names are not referenced in Rust code yet (HSE, SCK/MOSI AF, TE, BL);
//! they stay here as the single board pin map for bring-up and GPIO init.

#![allow(dead_code)]

// --- AHB peripherals (base addresses) ---------------------------------------

pub const SPI1_BASE: u32 = 0x4001_3000;
pub const GPIOA_BASE: u32 = 0x4001_0800;
pub const GPIOC_BASE: u32 = 0x4001_1000;
pub const GPIOD_BASE: u32 = 0x4001_1400;

/// Offset from GPIO port base to `BSHR` (bit set / reset).
pub const GPIO_BSHR_OFFSET: u32 = 0x10;

// --- SPI1 register offsets (from `SPI1_BASE`) --------------------------------

pub const SPI1_SR_OFFSET: u32 = 0x08;
pub const SPI1_DR_OFFSET: u32 = 0x0C;

/// Status: transmit buffer empty (ready for next byte).
pub const SPI1_SR_TXE: u16 = 1 << 1;
/// Status: receive buffer not empty.
pub const SPI1_SR_RXNE: u16 = 1 << 0;

// --- Port A (crystal / HSE — configured in platform init, not toggled here) -

pub const PA1_HSE_OSCI: u8 = 1;
pub const PA2_HSE_OSCO: u8 = 2;

// --- Port C -----------------------------------------------------------------

/// LCD chip select (active low). AF: `SPI1_NSS` / NSS.
pub const PC1_LCD_CS: u8 = 1;
/// SPI1 clock. AF: `SPI1_SCK`.
pub const PC5_SPI1_SCK: u8 = 5;
/// SPI1 MOSI. AF: `SPI1_MOSI`.
pub const PC6_SPI1_MOSI: u8 = 6;

// --- Port D -----------------------------------------------------------------

/// Tearing-effect input → EXTI (`exti7_0_irqhandler`). AF includes `TIM1_CH1N`.
pub const PD0_LCD_TE: u8 = 0;
/// Backlight PWM (TIM1_CH1). Configured in `hw::tim1_backlight_init` via `backlight_set_duty`.
pub const PD2_TIM1_CH1_BL: u8 = 2;
/// ILI9341 D/C (data = high, command = low).
pub const PD3_LCD_DC: u8 = 3;
/// ILI9341 reset (active low).
pub const PD4_LCD_RST: u8 = 4;

// --- GPIO atomic helpers (BSHR: low half SET, high half RESET) ------------

#[inline(always)]
pub const fn gpio_bshr_set(pin: u8) -> u32 {
    1u32 << (pin as u32)
}

#[inline(always)]
pub const fn gpio_bshr_reset(pin: u8) -> u32 {
    1u32 << ((pin as u32) + 16)
}
