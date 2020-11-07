/* diosix ELF layout for system service applications on common RV32/RV64 hardware
 * 
 * (c) Chris Williams, 2020.
 * See LICENSE for usage and copying.
 */

OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
  __application_start = .;

  .text : ALIGN(0x8)
  {
    *(.text*)
    *(.rodata*)
  }

  .data : ALIGN(0x8)
  {
    *(.data*)
  }

  /* this needs to be zeroed */
  __bss_start = .;
  .bss : ALIGN(0x8)
  {
    *(.*bss*)
  }
  __bss_end = ALIGN(0x8);

  __application_end = ALIGN(0x1000);
}
