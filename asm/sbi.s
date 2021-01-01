# supervisor-level SBI call veneers on RV32I/RV64I platforms
#
# See https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc
# for more information on the SBI API
#
# (c) Chris Williams, 2020.
# See LICENSE for usage and copying.

.altmacro

.section .text
.align 8

# make an SBI call from rust with up to five parameters
# => a[n to m] = paramters n to (m - 1) for the SBI call
#    a[m] = SBI call extension
#    a[m+1] = SBI call function
# <= a0 = error code
#    a1 = aux error code

# eg, a 2 parameter SBI call will use a0 and a1 for the input arguments,
# a2 and a3 for the extension and function, respectively.

# when calling the SBI provider, the exension and function are moved
# into a7 and a6, respectively, as per the ABI

# create a macro that generates the code for the above
.macro SBI_CALL param_count, ext_reg, func_reg
.global sbi_call_\param_count
sbi_call_\param_count:
  add a7, a\ext_reg, x0
  add a6, a\func_reg, x0
  ecall
  ret
.endm

# repeat the macro for 0 to 5 parameter calls, inclusive
.set parameters, 0
.set ext_reg, 0
.set func_reg, 1
  .rept 6
    SBI_CALL %parameters, %ext_reg, %func_reg
    .set parameters, parameters + 1
    .set ext_reg, ext_reg + 1
    .set func_reg, func_reg + 1
  .endr
