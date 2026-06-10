//! CH32V003 register bases and bit masks (from WCH `ch32v00x.h`).

pub const PERIPH_BASE: u32 = 0x4000_0000;
pub const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x1_0000;
pub const AHBPERIPH_BASE: u32 = PERIPH_BASE + 0x2_0000;

pub const AFIO_BASE: u32 = APB2PERIPH_BASE;
pub const GPIOC_BASE: u32 = APB2PERIPH_BASE + 0x1000;
pub const GPIOD_BASE: u32 = APB2PERIPH_BASE + 0x1400;
pub const SPI1_BASE: u32 = APB2PERIPH_BASE + 0x3000;
pub const TIM1_BASE: u32 = APB2PERIPH_BASE + 0x2C00;

pub const DMA1_BASE: u32 = AHBPERIPH_BASE;
pub const DMA1_CH3_BASE: u32 = AHBPERIPH_BASE + 0x0030;
pub const RCC_BASE: u32 = AHBPERIPH_BASE + 0x1000;
pub const FLASH_R_BASE: u32 = AHBPERIPH_BASE + 0x2000;

pub const GPIO_CFGR_OFFSET: u32 = 0x00;
pub const GPIO_BSHR_OFFSET: u32 = 0x10;

pub const RCC_CTLR: u32 = 0x00;
pub const RCC_CFGR0: u32 = 0x04;
pub const RCC_AHBPCENR: u32 = 0x14;
pub const RCC_APB2PCENR: u32 = 0x18;

pub const RCC_HSEON: u32 = 1 << 16;
pub const RCC_HSERDY: u32 = 1 << 17;
pub const RCC_PLLON: u32 = 1 << 24;
pub const RCC_PLLRDY: u32 = 1 << 25;
pub const RCC_PLLSRC_HSE_MUL2: u32 = 1 << 16;
pub const RCC_SW: u32 = 0x03;
pub const RCC_SW_PLL: u32 = 0x02;
pub const RCC_SWS: u32 = 0x0C;
pub const RCC_SWS_PLL_VAL: u32 = 0x08;
pub const RCC_HPRE_DIV1: u32 = 0x0000_0000;

pub const RCC_AHB_DMA1EN: u32 = 1 << 0;
pub const RCC_APB2_AFIOEN: u32 = 1 << 0;
pub const RCC_APB2_IOPCEN: u32 = 1 << 4;
pub const RCC_APB2_IOPDEN: u32 = 1 << 5;
pub const RCC_APB2_SPI1EN: u32 = 1 << 12;
pub const RCC_APB2_TIM1EN: u32 = 1 << 11;

pub const AFIO_PCFR1: u32 = 0x04;
pub const AFIO_PCFR1_HSE_PA1_PA2: u32 = 1 << 15;

pub const FLASH_ACTLR: u32 = 0x00;
pub const FLASH_ACTLR_LATENCY_1: u32 = 0x01;

pub const SPI_CTLR1: u32 = 0x00;
pub const SPI_CTLR2: u32 = 0x04;
pub const SPI_STATR: u32 = 0x08;
pub const SPI_DATAR: u32 = 0x0C;

pub const SPI_CTLR1_MSTR: u16 = 1 << 2;
pub const SPI_CTLR1_SPE: u16 = 1 << 6;
pub const SPI_CTLR1_SSI: u16 = 1 << 8;
pub const SPI_CTLR1_SSM: u16 = 1 << 9;
pub const SPI_CTLR1_DFF: u16 = 1 << 11;

pub const SPI_CTLR2_TXDMAEN: u16 = 1 << 1;

pub const SPI_STATR_TXE: u16 = 1 << 1;
pub const SPI_STATR_BSY: u16 = 1 << 7;

pub const DMA_CFGR: u32 = 0x00;
pub const DMA_CNTR: u32 = 0x04;
pub const DMA_PADDR: u32 = 0x08;
pub const DMA_MADDR: u32 = 0x0C;

pub const DMA1_INTFR: u32 = 0x00;
pub const DMA1_INTFCR: u32 = 0x04;

pub const DMA_CFGR_EN: u16 = 1 << 0;
pub const DMA_CFGR_DIR: u16 = 1 << 4;
pub const DMA_CFGR_MINC: u16 = 1 << 7;
pub const DMA_CFGR_PSIZE_0: u16 = 1 << 8;
pub const DMA_CFGR_MSIZE_0: u16 = 1 << 10;
pub const DMA_CFGR_PL_0: u16 = 1 << 12;
pub const DMA_CFGR_PL_1: u16 = 1 << 13;

pub const DMA_TCIF3: u32 = 1 << 9;
pub const DMA_CGIF3: u32 = 1 << 8;

/// TIM1 register byte offsets (from `TIM1_BASE`).
pub const TIM_CTLR1: u32 = 0x00;
pub const TIM_SWEVGR: u32 = 0x14;
pub const TIM_CHCTLR1: u32 = 0x18;
pub const TIM_CCER: u32 = 0x20;
pub const TIM_PSC: u32 = 0x28;
pub const TIM_ATRLR: u32 = 0x2C;
pub const TIM_CH1CVR: u32 = 0x34;
pub const TIM_BDTR: u32 = 0x44;

pub const TIM_CEN: u16 = 1 << 0;
pub const TIM_ARPE: u16 = 1 << 7;
pub const TIM_UG: u8 = 1 << 0;
pub const TIM_OC1M_1: u16 = 1 << 5;
pub const TIM_OC1M_2: u16 = 1 << 6;
pub const TIM_OC1PE: u16 = 1 << 3;
pub const TIM_CC1E: u16 = 1 << 0;
pub const TIM_MOE: u16 = 1 << 15;

/// GPIO CFGLR: output push-pull 50 MHz (MODE=11, CNF=00).
pub const GPIO_MODE_OUT_50MHZ: u32 = 0b11;
/// GPIO CFGLR: alternate-function push-pull (CNF=10).
pub const GPIO_CNF_AF_PP: u32 = 0b10;

/// Byte register offset → word index for `ptr::add`.
#[inline(always)]
pub const fn reg_idx(byte_offset: u32) -> usize {
    (byte_offset / 4) as usize
}
