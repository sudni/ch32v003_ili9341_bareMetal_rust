# Hardware connections

This board uses a **CH32V003** with an **ILI9341** TFT over **SPI1**. **MISO is not connected** (write-only path). Pins below match the **CH32V003 TFT module** wiring you use.

## Module pinout (your design)

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

The firmware in this repo drives **PC1** (CS), **PD3** (D/C), and **PD4** (RESET) via GPIO **BSHR**; **PC5** / **PC6** must be configured in your init (or BSP) as **SPI1** alternate-function push-pull. **PD0** is the **TE** input (not toggled in `hw.rs`); enable **EXTI** on **PD0** and NVIC for `exti7_0_irqhandler`. **PD2** backlight PWM is **not** implemented here yet—GPIO/timer init for **TIM1_CH1** is still needed if you want dimming.

## Peripheral addresses (as coded)

| Peripheral | Base / register | Purpose |
|------------|------------------|---------|
| **SPI1** | `0x40013000` | 8-bit TX to the panel (`SR` poll / `DR` write) |
| **GPIOC BSHR** | `0x40011010` | **CS** on **PC1** (atomic set/reset) |
| **GPIOD BSHR** | `0x40011410` | **D/C** on **PD3**, **RST** on **PD4** |

Register bases follow the usual CH32V00x map: GPIO port A `0x40010800`, B `0x40010C00`, C `0x40011000`, D `0x40011400`; **BSHR** is at offset **+0x10**.

## Tearing effect (TE) / EXTI

| Signal | MCU pin | Notes |
|--------|---------|--------|
| **TE** | **PD0** | Input from the ILI9341/module **TE** pad; route through **EXTI** so `exti7_0_irqhandler` runs on the panel’s tearing/blanking edge (configure rising vs falling per your panel and scan direction). |

The symbol `exti7_0_irqhandler` matches the WCH **EXTI line 0–7** interrupt. Map **PD0** to the correct **EXTI line** and source in the CH32V003 reference manual (AFIO / EXTI pin mux), enable the interrupt in **NVIC**, and set trigger polarity to match **TE**.

**In this repository**, EXTI pending clear in the ISR is still a TODO (“platform specific”). Add the correct clear for the line wired to **PD0** if the IRQ sticks or re-enters incorrectly.

## Electrical notes

- CH32V003 I/O is **3.3 V**; the table assumes a 3.3 V module supply.
- One-way SPI to the ILI9341 is normal with **MISO NC**.
