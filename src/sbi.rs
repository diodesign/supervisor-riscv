/* diosix RV32G/RV64G supervisor-level SBI veener
 *
 * See https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc
 * for more information on the SBI API
 * 
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

/* diosix's registered implementation ID */
const SBI_DIOSIX_IMPL_ID: usize = 5;

/* base SBI functionality */
const SBI_EXT_BASE:                     usize = 0x10;
const SBI_EXT_BASE_GET_IMPL_ID:         usize = 1;

/* available type of services that can be offered by a capsule */
pub enum DiosixServiceType
{
    ConsoleInterface = 0 /* act as the console interface manager */
}

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
    NoReason = 0,
    SystemFailure = 1
}

/* SBI error codes */
const SBI_SUCCESS:                      usize = 0;

/* SBI calls return a success/error code and value in a0 and a1 */
#[repr(C)]
pub struct Result
{
    pub error: usize,
    pub value: usize
}

/* reasons why an SBI call would fail */
pub enum Error
{

}

extern "C"
{
    fn sbi_call_0(extension: usize, function: usize) -> Result; /* no parameter sbi call */
    fn sbi_call_1(param1: usize, extension: usize, function: usize) -> Result; /* single parameter sbi call */
    fn sbi_call_2(param1: usize, param2: usize, extension: usize, function: usize) -> Result; /* two parameter sbi call */
}

/* check we're running on diosix. true if so, false if not */
pub fn is_diosix() -> bool
{
    let result = unsafe { sbi_call_0(SBI_EXT_BASE, SBI_EXT_BASE_GET_IMPL_ID) };
    if result.value == SBI_DIOSIX_IMPL_ID && result.error == SBI_SUCCESS
    {
        return true;
    }
    false
}

/* attempt to register the given service type
pub fn register_service(stype: DiosixServiceType) -> Result<(), Error>
{
}*/

/* write character c to the output device */
pub fn console_putchar(c: u8)
{
    unsafe { sbi_call_1(c as usize, SBI_EXT_PUTCHAR, SBI_FUNC_LEGACY) };
}

/* terminate this environment. reason = code for termination
   if this call returns, it didn't work. */
pub fn shutdown(reason: SBI_EXT_SYS_RESET_REASON)
{
    unsafe { sbi_call_2(SBI_EXT_SYS_RESET_SHUTDOWN, reason as usize, SBI_EXT_SYS_RESET, SBI_EXT_SYS_RESET_FUNC) };   
}

/* restart this environment from scratch. reason = code for restart
   if this call returns, it didn't work. */
pub fn restart(reason: SBI_EXT_SYS_RESET_REASON)
{
    unsafe { sbi_call_2(SBI_EXT_SYS_RESET_COLD_REBOOT, reason as usize, SBI_EXT_SYS_RESET, SBI_EXT_SYS_RESET_FUNC) };
}