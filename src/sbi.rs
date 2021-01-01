/* diosix RV32G/RV64G supervisor-level SBI veener
 *
 * See https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc
 * for more information on the SBI API
 * 
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

/* write character to output port */
const SBI_EXT_PUTCHAR: usize = 0x1;
const SBI_FUNC_LEGACY: usize = 0x0;

/* shutdown or restart the environment */
const SBI_EXT_SYS_RESET:                usize = 0x53525354;
const SBI_EXT_SYS_RESET_FUNC:           usize = 0;
const SBI_EXT_SYS_RESET_SHUTDOWN:       usize = 0;
const SBI_EXT_SYS_RESET_COLD_REBOOT:    usize = 1;

#[allow(non_camel_case_types)]
pub enum SBI_EXT_SYS_RESET_REASON
{
    NoReason,
    SystemFailure
}

impl SBI_EXT_SYS_RESET_REASON
{
    pub fn to_usize(&self) -> usize
    {
        match self
        {
            Self::NoReason => 0,
            Self::SystemFailure => 1
        }
    }
}

extern "C"
{
    fn sbi_call_1(param1: usize, extension: usize, function: usize); /* single parameter sbi call */
    fn sbi_call_2(param1: usize, param2: usize, extension: usize, function: usize); /* two parameter sbi call */
}

/* write character c to the output device */
pub fn console_putchar(c: u8)
{
    unsafe { sbi_call_1(c as usize, SBI_EXT_PUTCHAR, SBI_FUNC_LEGACY) };
}

/* terminate this environment. reason = code for termination */
pub fn shutdown(reason: SBI_EXT_SYS_RESET_REASON)
{
    unsafe { sbi_call_2(SBI_EXT_SYS_RESET_SHUTDOWN, reason.to_usize(), SBI_EXT_SYS_RESET, SBI_EXT_SYS_RESET_FUNC) };   
}

/* restart this environment from scratch. reason = code for restart */
pub fn restart(reason: SBI_EXT_SYS_RESET_REASON)
{
    unsafe { sbi_call_2(SBI_EXT_SYS_RESET_COLD_REBOOT, reason.to_usize(), SBI_EXT_SYS_RESET, SBI_EXT_SYS_RESET_FUNC) };
}