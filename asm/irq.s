# supervisor low-level interrupt/exception code for RV32G/RV64G targets
#
# Note: No support for F/D floating point (yet)!
#
# (c) Chris Williams, 2020.
# See LICENSE for usage and copying.

.altmacro

.section .text
.align 8

.global supervisor_irq_handler

# during interrupts and exceptions, reserve space for 32 registers, 32 or 64 bits wide
.if ptrwidth == 32
.equ  IRQ_REGISTER_FRAME_SIZE,   (32 * 4)   # RV32
.elseif ptrwidth == 64
.equ  IRQ_REGISTER_FRAME_SIZE,   (32 * 8)   # RV64
.else
.error "Only 32-bit and 64-bit RISC-V supported (unexpected pointer width)"
.endif

# macro to generate store instructions to push given 'reg' register
.macro PUSH_REG reg
.if ptrwidth == 32
  sw  x\reg, (\reg * 4)(sp)
.else
  sd  x\reg, (\reg * 8)(sp)
.endif
.endm

# macro to generate load instructions to pull given 'reg' register
.macro PULL_REG reg
.if ptrwidth == 32
  lw  x\reg, (\reg * 4)(sp)
.else
  ld  x\reg, (\reg * 8)(sp)
.endif
.endm

# Entry point for supervisor-level handler of interrupts and exceptions
# interrupts are automatically disabled on entry. we could be reentrant in
# future, but for now: only reenable on exit
supervisor_irq_handler:
  # stack general purpose registers, skip zero (x0) and sp (x2)
  addi  sp, sp, -(IRQ_REGISTER_FRAME_SIZE)

  PUSH_REG 1
  .set reg, 3
  .rept 29
    PUSH_REG %reg
    .set reg, reg + 1
  .endr

  # work out what happened using high-level code
  # => a0 = top of the stack containing preserved registers
  addi  a0, sp, IRQ_REGISTER_FRAME_SIZE
  call decode_irq

  # restore registers, skip zero (x0) and sp (x2)
  .set reg, 31
  .rept 29
    PULL_REG %reg
    .set reg, reg - 1
  .endr
  PULL_REG 1

  # fix up stack and exit handler
  addi  sp, sp, IRQ_REGISTER_FRAME_SIZE
  sret