/* diosix ELF layout for system service applications on common RV64 hardware
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
  .text : ALIGN(0x1000)
  {
    __application_start = .;
    *(.entry) /* must be first */
    *(.text*)
    *(.rodata*)
  }

  .data : ALIGN(0x1000)
  {
    *(.data*)
  }

  /* this needs to be zeroed */
  .bss : ALIGN(0x1000)
  {
    __bss_start = .;
    *(.*bss*)
  }
  __bss_end = ALIGN(0x8);

  __application_end = ALIGN(0x1000);
}
