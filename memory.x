/* CH32V003: 16 KiB flash @ 0x00000000, 2 KiB RAM @ 0x20000000 */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 16K
  RAM   : ORIGIN = 0x20000000, LENGTH = 2K
}

/* Stack grows down from top of RAM (used by riscv-rt link.x) */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* CH32V003 has only 2 KiB RAM; default riscv-rt stack is 2 KiB per hart */
_hart_stack_size = 512;

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

SECTIONS
{
  /DISCARD/ :
  {
    *(.eh_frame*)
    *(.comment)
    *(.riscv.attributes)
  }
}
