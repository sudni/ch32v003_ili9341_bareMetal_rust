# Project documentation

Bare-metal Rust firmware for **CH32V003** driving an **ILI9341** TFT over **SPI1**, with optional tearing-edge (TE) synchronization via **EXTI**.

| Document | Contents |
|----------|----------|
| [architecture.md](architecture.md) | Software structure, data flow, interrupts |
| [hardware.md](hardware.md) | MCU pinout, optional **ER-TFTM028-4** JP1 map, TE / EXTI, **backlight PWM** (`0..=1023`) |
| [memory.md](memory.md) | Flash/RAM map from the linker script and stack/static budget |

## RISC-V assembly listings

To study the compiler output for this target (`riscv32imc-unknown-none-elf`), generate LLVM GNU-style `.s` files (under `target/`, already covered by `.gitignore`):

| Command | Profile | Output file |
|---------|---------|-------------|
| `cargo gen-asm` | `release` (`opt-level=z`, LTO) | `target/ch32v003-ili9341-release.s` |
| `cargo gen-asm-dev` | `dev` + `debuginfo=2` (`.loc` → Rust source lines) | `target/ch32v003-ili9341-dev.s` |

Aliases are named **`gen-asm`** so they do not shadow a separate `cargo-asm` tool if you have it installed.

The **release** listing matches what you flash; **dev** is usually easier to map back to `src/*.rs`. You can also disassemble the ELF with LLVM or GNU objdump, for example: `llvm-objdump -d target/riscv32imc-unknown-none-elf/release/ch32v003-ili9341`.

For a CH32V003-focused report (flash/RAM vs `memory.x`, linker checks, RV32IMC opcode stats):

```bash
pip install -r tools/requirements.txt
python tools/ch32v003_elf_analyze.py
```
