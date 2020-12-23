/* diosix ELF layout for system service applications on common RV32/RV64 hardware
 * 
 * Do a cargo clean after modifying this file.
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
    *(.entry) /* must be first */
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

  /DISCARD/ : { *(.eh_frame) }

  __application_end = ALIGN(0x1000);
}
