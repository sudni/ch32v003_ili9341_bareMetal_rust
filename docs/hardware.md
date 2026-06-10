# Hardware connections

This board uses a **CH32V003** with an **ILI9341** TFT over **SPI1**. **MISO is not connected** (write-only path). Pins below match the **CH32V003 TFT module** wiring you use.

## East Rising **ER-TFTM028-4** (optional exact module)

If your panel is the **ER-TFTM028-4** breakout (*Datasheet V2.0*, buydisplay.com / East Rising), it matches this project:

| Item | Spec (from module DS) |
|------|------------------------|
| Size / resolution | **2.8″**, **240 × RGB × 320** |
| Controller | **ILI9341** |
| Bus options | 8080 parallel (8/9/16/18), **3-wire SPI**, **4-wire SPI**, RGB — this firmware uses **4-wire SPI** (separate **CS**, **SCK**, **SDI**, **D/C**) |
| **TE** | **JP1 pin 22** — tearing-effect **output** from the panel (frame sync); tie to **PD0** + EXTI as in your schematic |
| **CS** | **JP1 pin 23** `LCD_/CS`, active low |
| **SCK** | **JP1 pin 24** `D/C(SCL)` acts as **serial clock** in 4-wire SPI |
| **D/C** | **JP1 pin 25** `/WR(D/C)` = **command/data** select in 4-line serial |
| **SDI** | **JP1 pin 27** `LCD_SDI` (MOSI) |
| **RESET** | **JP1 pin 21** `/RESET`, active low (module may have RC reset; wire if you drive reset from MCU) |
| **Backlight** | **JP1 pin 29** `BL_ON/OFF` — high on / low off (or power mode per jumper **J9–J12**; see module §4.3) |
| **SDO** | **JP1 pin 28** — not needed for write-only |

**Critical — solder jumpers:** The module ships for **8080 16-bit parallel by default** (§4.3). For this firmware you must set **4-wire SPI**: **J2, J3, J4, J5 short** and **J1, J6, J7, J8 open**, with the resistor stuffing called out in the same table. If jumpers stay on parallel mode, SPI bit-banging from the MCU will not drive the ILI9341 correctly.

**Power:** Module supports **3.3 V or 5 V** `VDD` (JP / strap per §4.3); **I/O** is **3.3 V logic** (`VDDIO`). Match **CH32V003** at **3.3 V** and follow East Rising notes for `VDD` strap vs backlight jumpers.

## Module pinout (your design — MCU side)


| TFT / module signal | CH32V003 pin | Direction | GPIO mode | Description |
|---------------------|--------------|-----------|------------|-------------|
| **VCC** | 3.3 V supply | — | — | Power (**3.3 V only** — do **not** use 5 V) |
| **GND** | GND | — | — | Common ground |
| **Crystal (X1)** | **PA1 / PA2** | IN/OUT | Analog | 24 MHz external crystal → 48 MHz system clock |
| **CS** | **PC1** | OUT | Push-pull | Chip-select (**active LOW**) |
| **RESET** | **PD4** | OUT | Push-pull | Hardware reset (**active LOW**) |
| **DC/RS** | **PD3** | OUT | Push-pull | Data/command (**H** = data, **L** = command) |
| **SDI / MOSI** | **PC6** | OUT | AF-PP | **SPI1 MOSI** |
| **SCK** | **PC5** | OUT | AF-PP | **SPI1** clock |
| **LED** (backlight) | **PD2** | OUT | AF-PP | Backlight **PWM** (**TIM1_CH1**) |
| **TE** (tearing effect) | **PD0** | IN | Input (floating or pull‑up per panel) | Panel frame sync → **EXTI** (see below) |
| **SDO / MISO** | **NC** | — | — | Not connected |

The firmware drives **PC1** (CS), **PD3** (D/C), and **PD4** (RESET) via GPIO **BSHR**; **PC5** / **PC6** are **SPI1** alternate-function push-pull. **PD2** backlight uses **TIM1_CH1 PWM** (see below). **PD0** is the **TE** input; enable **EXTI** on **PD0** and NVIC for `exti7_0_irqhandler`.

## Backlight PWM (PD2 / TIM1_CH1)

Implemented in `src/hw.rs`:

| API | Role |
|-----|------|
| `backlight_on()` | Full brightness — calls `backlight_set_duty(1023)` |
| `backlight_set_duty(duty)` | Set duty cycle; values above **1023** are clamped |

**Timer setup:** TIM1 CH1 on **PD2**, PSC = 0, `ATRLR` = **1023** → PWM frequency ≈ **48 MHz / 1024 ≈ 46.9 kHz** (10-bit resolution).

**Duty range:** `0` = off, **`1023` = maximum** (`BACKLIGHT_PWM_MAX`). The compare register is `TIM1->CH1CVR`.

| `backlight_set_duty(n)` | Approx. brightness |
|-------------------------|-------------------|
| `0` | Off |
| `256` | ~25 % (256 / 1023) — **not** the maximum |
| `512` | ~50 % |
| **`1023`** | **100 %** (full on) |

At the end of `ili9341::init()`, the firmware calls `backlight_on()` so the panel starts at full brightness. To dim:

```rust
hw::backlight_set_duty(512); // ~50 %
```

## Peripheral addresses (as coded)

| Peripheral | Base / register | Purpose |
|------------|------------------|---------|
| **SPI1** | `0x40013000` | 8-bit TX to the panel (`SR` poll / `DR` write) |
| **GPIOC BSHR** | `0x40011010` | **CS** on **PC1** (atomic set/reset) |
| **GPIOD BSHR** | `0x40011410` | **D/C** on **PD3**, **RST** on **PD4** |
| **TIM1** | `0x40012C00` | Backlight **PWM** on **PD2** (`CH1CVR` duty) |

Register bases follow the usual CH32V00x map: GPIO port A `0x40010800`, B `0x40010C00`, C `0x40011000`, D `0x40011400`; **BSHR** is at offset **+0x10**.

## Tearing effect (TE) / EXTI

| Signal | MCU pin | Notes |
|--------|---------|--------|
| **TE** | **PD0** | Input from the ILI9341/module **TE** pad; route through **EXTI** so `exti7_0_irqhandler` runs on the panel’s tearing/blanking edge (configure rising vs falling per your panel and scan direction). |

The symbol `exti7_0_irqhandler` matches the WCH **EXTI line 0–7** interrupt. Map **PD0** to the correct **EXTI line** and source in the CH32V003 reference manual (AFIO / EXTI pin mux), enable the interrupt in **NVIC**, and set trigger polarity to match **TE**.

**In this repository**, EXTI pending clear in the ISR is still a TODO (“platform specific”). Add the correct clear for the line wired to **PD0** if the IRQ sticks or re-enters incorrectly.

## WCH datasheet (CH32V003DS0) alignment

Official **CH32V003** datasheet *CH32V003DS0* (e.g. V1.8) matches this board as follows (pin names from **Chapter 2**, **CH32V003F4P6** pinout):

| Your net | Datasheet pin functions (abridged) |
|----------|-------------------------------------|
| **CS** on **PC1** | `PC1` / **NSS** (SPI NSS), plus I2C/timer remap options — use as GPIO push-pull CS if NSS not used in hardware mode. |
| **SCK** / **MOSI** | **PC5** = **SCK**; **PC6** = **MOSI** (same SPI). |
| **D/C** **PD3** | `PD3` — general-purpose I/O / alternate functions (`A4`, `UCTS`, etc.). |
| **RST** **PD4** | `PD4` — includes `UCK`, timer / SPI-related remaps; fine as GPIO reset. |
| **BL PWM** **PD2** | `PD2` — **TIM1_CH1** (`T1CH1`) matches backlight on **TIM1_CH1**. |
| **TE** **PD0** | `PD0` — **TIM1_CH1N** and remappable `SDA`/`UTX`; configure as **input + EXTI** for TE. |
| **Crystal** **PA1** / **PA2** | **OSCI** / **OSCO** for **HSE** 4–25 MHz (your 24 MHz crystal fits). |

Device memory from the same document: **2 KB SRAM**, **16 KB CodeFlash** — consistent with `memory.x`.

**EXTI (§1.4.8):** eight external interrupt lines; multiple port pins can be tied to one EXTI source — configure **PD0** and the EXTI mux in the **RM** (datasheet is summary only).

## Electrical notes

- CH32V003 I/O is **3.3 V**; the table assumes a 3.3 V module supply.
- One-way SPI to the ILI9341 is normal with **MISO NC**.
