# RAM and Flash usage

## Linker script (`memory.x`)

As checked into this project:

| Region | Origin | Size |
|--------|--------|------|
| **FLASH** | `0x00000000` | **16 KiB** |
| **RAM** | `0x20000000` | **2 KiB** |

The script discards `.eh_frame`, `.comment`, and `.riscv.attributes`; it does **not** define stack size or `.bss` placement explicitly—those rely on the linker’s default layout for this target once `-Tmemory.x` is applied.

## Major RAM consumers (from source)

These are **approximate** sizes for planning; exact `.bss`/stack usage depends on the linker and optimization level.

| Item | Size (approx.) | Location |
|------|------------------|----------|
| Tile RGB565 buffer | 40 × 40 × 2 = **3 200 bytes** | `main`: `buf: [u16; TILE_W * TILE_H]` |
| `Tiles::dirty` | **48 bytes** | `[bool; 48]` |
| `IRQ_COUNTER`, `TE_FLAG` | 5 bytes + alignment | `static mut` in `main` |
| Stack | Implementation-defined | Calls, locals, semihosting |

**Important:** A **3.2 KiB** tile buffer **alone** exceeds the **2 KiB** RAM declared in `memory.x`. If you target a standard CH32V003 with 2 KiB SRAM, you must **shrink tiles**, **stream fewer lines at a time**, or **use a part / external RAM** consistent with your real hardware. Treat the current tile size as a logical design parameter to reconcile with the actual MCU.

## Flash (code size)

Firmware size depends on optimization (`-C opt-level=z`, LTO, etc. in `Cargo.toml`) and whether **semihosting** is linked for release builds.

To measure after a successful link into normal flash addresses:

```bash
cargo build --release
llvm-size -A target/riscv32imc-unknown-none-elf/release/ch32v003-ili9341
```

(or the GNU `size` / `riscv-none-elf-size` equivalent.)

Typical sections of interest:

- **`.text`** — program code in flash  
- **`.rodata`** — constants  
- **`.data`** — initialized globals (stored in flash, copied to RAM at startup)  
- **`.bss`** — zero-initialized RAM  

If `llvm-size` reports **no `.text`** or **entry address `0x0`**, the ELF may not be fully linked for embedded; fix linker flags / `memory.x` / scatter file until load segments map into `0x00000000` flash.

## Summary table (design limits vs. source)

| Resource | `memory.x` | Watchpoint in code |
|----------|------------|---------------------|
| Flash | 16 KiB | Track `.text` + `.rodata` + init `.data` |
| RAM | 2 KiB | Tile buffer 3.2 KiB — **must align with real part** |
