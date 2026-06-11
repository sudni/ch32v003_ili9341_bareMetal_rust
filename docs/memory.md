# RAM and Flash memory map

## CH32V003 (physical)

Per **CH32V003DS0** and `memory.x`:

| Region | Bus address | Size | Role |
|--------|-------------|------|------|
| **CodeFlash** | `0x0000_0000` | **16 KiB** | Instructions + constants loaded at reset |
| **SRAM** | `0x2000_0000` | **2 KiB** | `.data`, `.bss`, stack |

There is no external RAM/flash in this project.

```
  0x0000_0000  ┌─────────────────────────────┐ 16 KiB
               │  FLASH (.text, .rodata,      │
               │          init .data LMA)    │
  0x0000_4000  └─────────────────────────────┘

  0x2000_0000  ┌─────────────────────────────┐ 2 KiB
               │  .data (if any)             │
               │  .bss (static mut, buffers) │
               │  stack ↓ (grows downward)   │
  0x2000_0800  └─────────────────────────────┘  ← _stack_start
```

## Linker script (`memory.x`)

| Symbol / setting | Value | Meaning |
|------------------|-------|---------|
| `FLASH` | `ORIGIN = 0x00000000`, `LENGTH = 16K` | Program storage |
| `RAM` | `ORIGIN = 0x20000000`, `LENGTH = 2K` | Writable memory |
| `_stack_start` | `ORIGIN(RAM) + LENGTH(RAM)` = **`0x20000800`** | Initial stack pointer (top of RAM) |
| `_hart_stack_size` | **512** | Hint for `riscv-rt` (actual `.stack` reservation may differ) |

Section aliases: `.text`/`.rodata` → FLASH; `.data`/`.bss`/`.stack` → RAM.

Discarded sections: `.eh_frame`, `.comment`, `.riscv.attributes`.

## Flash layout (release build)

Measured with `python tools/ch32v003_elf_analyze.py` after `cargo build --release`:

| Section | Size | Content (main contributors) |
|---------|------|-------------------------------|
| **`.text`** | **4.72 KiB** | Rust code, SPI/DMA, TIM1 PWM, init, `riscv-rt` startup |
| **`.rodata`** | **10.75 KiB** | Font bitmaps (`font/data.rs`: 7×10, 11×18, 16×26) |
| **`.data` LMA** | 0 | No initialized globals copied to RAM at boot |
| **Total flash** | **15.47 KiB / 16 KiB (96.7 %)** | ~544 bytes free |

```
  0x0000_0000  .text (code)
       +
  ~0x0000_12E0  .rodata (fonts ~11 KiB dominate)
       +
  0x0000_3E00  ~544 B free
  0x0000_4000  end of flash
```

Entry point: **`0x00000000`** (vector table / reset in flash).

## RAM layout (release build)

| Section | VMA | Size | Content (main contributors) |
|---------|-----|------|-------------------------------|
| **`.data`** | `0x2000_0000` | **0** | — |
| **`.bss`** | low RAM | **1.01 KiB** | `font::GLYPH_BUF` (1024 B), `Tiles::dirty` (48 B), `TE_FLAG`, `IRQ_COUNTER`, … |
| **`.stack`** | high RAM | **~1 KiB** | Reserved stack (below `_stack_start`) |
| **Total RAM** | | **2.00 KiB / 2 KiB (100 %)** | No headroom |

```
  0x2000_0000  ┌──────────────────┐
               │  .bss            │  GLYPH_BUF [u16;512] ≈ 1024 B
               │                  │  Tiles::dirty [bool;48]
               │                  │  other static mut
  ~0x2000_0408  ├──────────────────┤
               │  (unused gap)    │
  ~0x2000_0400  ├──────────────────┤
               │  .stack          │  grows down toward .bss
  0x2000_0800  └──────────────────┘  SP at reset
```

**Important:** RAM is **fully budgeted**. Adding large buffers (framebuffer, bigger glyph pool, heap) requires shrinking something else or changing hardware.

## Major static objects (source → section)

| Item | Size | Section | File |
|------|------|---------|------|
| `GLYPH_BUF` | 512 × 2 = **1024 B** | `.bss` | `font/mod.rs` |
| Font tables (×3) | **~10.75 KiB** | `.rodata` (flash) | `font/data.rs` |
| `Tiles::dirty` | **48 B** | `.bss` | `ili9341.rs` |
| `TE_FLAG`, `IRQ_COUNTER` | few bytes | `.bss` | `main.rs` |
| Tile draw path | **0 B** tile buffer | — | solid-color DMA fill |

No heap allocator (`no_std`, no `alloc`).

## Startup (conceptual)

1. Reset @ `0x00000000` → `riscv-rt` startup in `.text`.
2. Copy `.data` LMA → VMA (none today).
3. Zero `.bss`.
4. Set `sp = _stack_start` (`0x20000800`).
5. Call `main`.

## How to measure

```bash
cargo build --release
python tools/ch32v003_elf_analyze.py
```

Optional:

```bash
llvm-size -A target/riscv32imc-unknown-none-elf/release/ch32v003-ili9341
```

Sections of interest:

- **`.text`** — code (flash)
- **`.rodata`** — constants (flash)
- **`.data`** — initialized RAM (flash LMA + RAM VMA)
- **`.bss`** — zero-init RAM
- **`.stack`** — stack reservation in RAM

If `.text` is missing or entry is wrong, linker/`memory.x` is not applied correctly.

## Design limits vs. current firmware

| Resource | Hardware / `memory.x` | Current release | Margin |
|----------|------------------------|-----------------|--------|
| Flash | 16 KiB | 15.47 KiB | **~544 B** — fonts dominate; shrink fonts or drop a size to add code |
| RAM | 2 KiB | 2.00 KiB | **0 B** — `GLYPH_BUF` + stack use almost all SRAM |

To free RAM: smaller `GLYPH_BUF` (if max font glyph fits), lower `_hart_stack_size` / stack usage, or reuse one buffer for tiles + glyphs (not concurrent).

To free flash: drop `FONT_16X26` or `FONT_11X18`, or use a compact 6×8 font only.
