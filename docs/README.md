# Project documentation

Bare-metal Rust firmware for **CH32V003** driving an **ILI9341** TFT over **SPI1**, with optional tearing-edge (TE) synchronization via **EXTI**.

| Document | Contents |
|----------|----------|
| [architecture.md](architecture.md) | Software structure, data flow, interrupts |
| [hardware.md](hardware.md) | Pin and signal wiring implied by the code |
| [memory.md](memory.md) | Flash/RAM map from the linker script and stack/static budget |
