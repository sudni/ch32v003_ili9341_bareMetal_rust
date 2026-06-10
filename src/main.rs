#![no_std]
#![no_main]

mod font;
mod hw;
mod ili9341;
mod pins;
mod rcc;
mod regs;

use panic_halt as _;
use riscv_rt::entry;

static mut TE_FLAG: bool = false;
static mut IRQ_COUNTER: u32 = 0;

#[no_mangle]
fn exti7_0_irqhandler() {
    // TE: `pins::PD0_LCD_TE` → EXTI; mux + NVIC in platform init.
    unsafe {
        IRQ_COUNTER = IRQ_COUNTER.wrapping_add(1);
        TE_FLAG = true;
    }
}

#[entry]
fn main() -> ! {
    let lcd = ili9341::Ili9341::new();
    lcd.init();

    let mut tiles = ili9341::Tiles::new();

    loop {
        unsafe {
            if TE_FLAG {
                TE_FLAG = false;

                let irq_count = IRQ_COUNTER;
                semihosting::println!("IRQ counter: {}", irq_count);

                if let Some((tx, ty)) = tiles.next() {
                    let color = tile_color(tx, ty);
                    lcd.push_solid_tile(tx, ty, color);
                    tiles.clear(tx, ty);
                } else {
                    tiles.mark_all();
                }

                if tx_demo_condition() {
                    font::draw_text(&lcd, "HELLO", 10, 10, 0xFFFF, 0x0000);
                }
            }
        }
    }
}

fn tx_demo_condition() -> bool {
    true
}

fn tile_color(tx: u16, ty: u16) -> u16 {
    let tiles_x = ili9341::WIDTH / ili9341::TILE_W;
    let tiles_y = ili9341::HEIGHT / ili9341::TILE_H;
    ((tx * 31 / tiles_x) << 11) | ((ty * 63 / tiles_y) << 5)
}
