# supervisor-level SBI call veneers on RV32I/RV64I platforms
#
# See https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc
# for more information on the SBI API
#
# (c) Chris Williams, 2020.
# See LICENSE for usage and copying.

.section .text
.align 4

.global sbi_call_1

# make an SBI call to the hypervisor with one paramter
# a0 = first paramter to the SBI call
# a1 = SBI call extension
# a2 = SBI call function
sbi_call_1:
  # store extension and function in a7 and a6, respectively, as per the ABI
  add a7, a1, x0
  add a6, a2, x0
  ecall
  ret
