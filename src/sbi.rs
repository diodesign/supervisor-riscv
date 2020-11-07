/* diosix RV32G/RV64G supervisor-level SBI veener
 *
 * See https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc
 * for more information on the SBI API
 * 
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

const SBI_EXT_PUTCHAR: usize = 0x1;
const SBI_FUNC_LEGACY: usize = 0x0;

extern "C"
{
    fn sbi_call_1(param1: usize, extension: usize, function: usize); /* single parameter sbi call */
}

pub fn console_putchar(c: u8)
{
    unsafe { sbi_call_1(c as usize, SBI_EXT_PUTCHAR, SBI_FUNC_LEGACY) };
}
