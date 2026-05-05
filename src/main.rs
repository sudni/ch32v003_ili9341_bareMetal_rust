#![no_std]
#![no_main]

mod hw;
mod ili9341;
mod font;

use panic_halt as _;

static mut TE_FLAG: bool = false;
static mut IRQ_COUNTER: u32 = 0;

#[no_mangle]
fn exti7_0_irqhandler() {
    // TE from TFT module -> PD0 (EXTI); mux + NVIC configured in platform init.
    unsafe {
        IRQ_COUNTER = IRQ_COUNTER.wrapping_add(1);
        TE_FLAG = true;
        // clear EXTI pending (platform specific)
    }
}

#[no_mangle]
fn main() -> ! {
    let lcd = ili9341::Ili9341::new();
    lcd.init();

    let mut tiles = ili9341::Tiles::new();
    let mut buf = [0u16; (ili9341::TILE_W as usize) * (ili9341::TILE_H as usize)];

    loop {
        unsafe {
            if TE_FLAG {
                TE_FLAG = false;

                let irq_count = IRQ_COUNTER;
                semihosting::println!("IRQ counter: {}", irq_count);

                if let Some((tx, ty)) = tiles.next() {
                    render(tx, ty, &mut buf);
                    lcd.push_tile(tx, ty, &buf);
                    tiles.clear(tx, ty);
                }

                // demo text overlay (very simple)
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

fn render(tx: u16, ty: u16, buf: &mut [u16]) {
    let color = ((tx * 31 / 6) << 11) | ((ty * 63 / 8) << 5);
    for p in buf.iter_mut() {
        *p = color;
    }
}
