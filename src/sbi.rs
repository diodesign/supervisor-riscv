/* diosix RV64G supervisor-level SBI veener
 *
 * See https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc
 * for more information on the SBI API
 * 
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

/* base SBI calls */
const SBI_EXT_BASE:                     usize = 0x10;
const SBI_EXT_BASE_GET_IMPL_ID:         usize = 1;

/* diosix's registered implementation ID */
const SBI_DIOSIX_IMPL_ID: usize = 5;

/* diosix-specific SBI calls */
const SBI_EXT_DIOSIX:                   usize = 0x0A000000 + SBI_DIOSIX_IMPL_ID;
const SBI_EXT_DIOSIX_YIELD:             usize = 0;
const SBI_EXT_DIOSIX_REGISTER_SERVICE:  usize = 1;
const SBI_EXT_DIOSIX_CONSOLE_PUTC:      usize = 2;
const SBI_EXT_DIOSIX_CONSOLE_GETC:      usize = 3;
const SBI_EXT_DIOSIX_HV_GETC:           usize = 4;

/* available type of diosix system services that can be offered by a capsule */
pub enum DiosixServiceType
{
    ConsoleInterface = 0 /* act as the console interface manager */
}

/* legacy ABI calls */
const SBI_EXT_PUTCHAR: usize = 0x1; /* write character to user */
const SBI_EXT_GETCHAR: usize = 0x2; /* read character from user */
const SBI_FUNC_LEGACY: usize = 0x0; /* legacy call function ID */

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

/* SBI calls return a success/error code and value in a0 and a1.
   some diosix calls return an extra value */
#[repr(C)]
pub struct SyscallResult
{
    error: isize, /* see below for success/error codes */
    value1: usize, /* SBI calls may return a value */
    value2: usize /* diosix calls may return an extra value */
}

impl SyscallResult
{
    /* decode the platform-specific return info and
       return an abstracted result of either return values
       or an error code */
    pub fn get(&self) -> Result<[usize; 2], Error>
    {
        match self.error
        {
            SBI_SUCCESS => Ok([self.value1, self.value2]),
            SBI_ERR_FAILED => Err(Error::Failed),
            SBI_ERR_NOT_SUPPORTED => Err(Error::NotSupported),
            SBI_ERR_INVALID_PARAM => Err(Error::InvalidParams),
            SBI_ERR_DENIED => Err(Error::AccessDenied),
            SBI_ERR_INVALID_ADDRESS => Err(Error::InvalidAddress),
            SBI_ERR_ALREADY_AVAILABLE => Err(Error::AlreadyAvailable),
            v => Err(Error::Other(v))
        }
    }

    /* return the raw error value */
    pub fn get_error_value(&self) -> isize { self.error }
}

/* abstracted error codes */
#[derive(Debug)]
pub enum Error
{
    Failed,
    NotSupported,
    InvalidParams,
    AccessDenied,
    InvalidAddress,
    AlreadyAvailable,
    Other(isize)
}

/* SBI error codes */
const SBI_SUCCESS:                      isize = 0;
const SBI_ERR_FAILED:                   isize = -1;
const SBI_ERR_NOT_SUPPORTED:            isize = -2;
const SBI_ERR_INVALID_PARAM:            isize = -3;
const SBI_ERR_DENIED:                   isize = -4;
const SBI_ERR_INVALID_ADDRESS:          isize = -5;
const SBI_ERR_ALREADY_AVAILABLE:        isize = -6;

/* functions for making SBI calls with zero, one, or two parameters */
#[inline]
fn sbi_call_0(extension: usize, function: usize) -> SyscallResult
{
    let error: isize;
    let value1: usize;
    let value2: usize;
    unsafe
    {
        asm!(
            "ecall",
            in("a7") extension,
            in("a6") function,
            lateout("a0") error,
            lateout("a1") value1,
            lateout("a2") value2
        );
    }
    SyscallResult { error, value1, value2 }
}

#[inline]
fn sbi_call_1(param1: usize, extension: usize, function: usize) -> SyscallResult
{
    let error: isize;
    let value1: usize;
    let value2: usize;
    unsafe
    {
        asm!(
            "ecall",
            in("a0") param1,
            in("a7") extension,
            in("a6") function,
            lateout("a0") error,
            lateout("a1") value1,
            lateout("a2") value2
        );
    }
    SyscallResult { error, value1, value2 }
}

#[inline]
fn sbi_call_2(param1: usize, param2: usize, extension: usize, function: usize) -> SyscallResult
{
    let error: isize;
    let value1: usize;
    let value2: usize;
    unsafe
    {
        asm!(
            "ecall",
            in("a0") param1,
            in("a1") param2,
            in("a7") extension,
            in("a6") function,
            lateout("a0") error,
            lateout("a1") value1,
            lateout("a2") value2
        );
    }
    SyscallResult { error, value1, value2 }
}

/* check we're running on diosix. true if so, false if not */
pub fn is_diosix() -> bool
{
    if let Ok(values) = sbi_call_0(SBI_EXT_BASE, SBI_EXT_BASE_GET_IMPL_ID).get()
    {
        if values[0] == SBI_DIOSIX_IMPL_ID
        {
            return true;
        }
    }  
    false
}

/* standard SBI calls */
/* write character c to the user */
pub fn putc(c: char) -> Result<(), Error>
{
    match sbi_call_1(c as usize, SBI_EXT_PUTCHAR, SBI_FUNC_LEGACY).get()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/* read and return a character from the user, or return an error code
   Note: We need to follow Linux's lead and read the character
   value from the *error* field, not the value field.
   See: https://github.com/torvalds/linux/blob/master/arch/riscv/kernel/sbi.c#L92 */
pub fn getc() -> Result<char, Error>
{
    match sbi_call_0(SBI_EXT_GETCHAR, SBI_FUNC_LEGACY).get_error_value()
    {
        -1 => return Err(Error::Failed), /* error value of -1 means no character to read */
        c => return Ok((c & 0xff) as u8 as char) /* otherwise, error value is a character ;-( */
    }
}

/* terminate this environment. reason = code for termination
   does not return */
pub fn shutdown(reason: SBI_EXT_SYS_RESET_REASON) -> !
{
    sbi_call_2(SBI_EXT_SYS_RESET_SHUTDOWN, reason as usize, SBI_EXT_SYS_RESET, SBI_EXT_SYS_RESET_FUNC);
    loop {}
}

/* shorthand for shutdown(). code is zero for clean exit, non-zero for something went wrong */
pub fn exit(code: usize) -> !
{
    shutdown(match code
    {
        0 => SBI_EXT_SYS_RESET_REASON::NoReason,
        _ => SBI_EXT_SYS_RESET_REASON::SystemFailure
    });
}

/* restart this environment from scratch. reason = code for restart
    does not return */
pub fn restart(reason: SBI_EXT_SYS_RESET_REASON) -> !
{
    sbi_call_2(SBI_EXT_SYS_RESET_COLD_REBOOT, reason as usize, SBI_EXT_SYS_RESET, SBI_EXT_SYS_RESET_FUNC);
    loop {}
}

/* diosix-specific calls */
/* yield this physical core to another virtual core */
pub fn r#yield()
{
    sbi_call_0(SBI_EXT_DIOSIX, SBI_EXT_DIOSIX_YIELD);
}

/* attempt to register the given service type */
pub fn register_service(stype: DiosixServiceType) -> Result<(), Error>
{
    match sbi_call_1(stype as usize, SBI_EXT_DIOSIX, SBI_EXT_DIOSIX_REGISTER_SERVICE).get()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/* write a character to a capsule's input character buffer */
pub fn capsule_putc(c: char, capsule_id: usize) -> Result<(), Error>
{
    match sbi_call_2(c as usize, capsule_id, SBI_EXT_DIOSIX, SBI_EXT_DIOSIX_CONSOLE_PUTC).get()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

pub struct CapsuleCharacter
{
    character: char,
    capsule_id: usize
}

impl CapsuleCharacter
{
    pub fn get_char(&self) -> char { self.character }
    pub fn get_capsule_id(&self) -> usize { self.capsule_id }
}

/* read a character from other capsules' character output buffers */
pub fn capsule_getc() -> Result<CapsuleCharacter, Error>
{
    match sbi_call_0(SBI_EXT_DIOSIX, SBI_EXT_DIOSIX_CONSOLE_GETC).get()
    {
        Ok(values) => Ok(CapsuleCharacter
        {
            character: (values[0] & 0xff) as u8 as char,
            capsule_id: values[1]
        }),
        Err(e) => Err(e)
    }
}

/* read a character from the hypervisor's output buffer */
pub fn hypervisor_getc() -> Result<char, Error>
{
    match sbi_call_0(SBI_EXT_DIOSIX, SBI_EXT_DIOSIX_HV_GETC).get()
    {
        Ok(values) => Ok((values[0] & 0xff) as u8 as char),
        Err(e) => Err(e)
    }
}