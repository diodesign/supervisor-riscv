/* diosix RV32G/RV64G supervisor-level code to input/output with the rest of the world
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

use core::fmt;
use super::sbi;
use spin::Mutex;

lazy_static!
{
    pub static ref STDOUT: Mutex<SBIWriter> = Mutex::new(SBIWriter::new());
}

#[macro_export]
macro_rules! println
{
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! print
{
    ($($arg:tt)*) =>
    ({
        use core::fmt::Write;
        {
            $crate::stdio::STDOUT.lock().write_fmt(format_args!($($arg)*)).unwrap();
        }
    });
}

/* simple object that writes out bytes to output to the user via SBI calls */
pub struct SBIWriter;
impl SBIWriter
{
    pub fn new() -> SBIWriter
    {
        SBIWriter {}
    }
}

impl fmt::Write for SBIWriter
{
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        for c in s.bytes()
        {
            sbi::console_putchar(c);
        }
        Ok(())
    }
}

/* ensure the STDOUT object is created and initialized */
pub fn init()
{
    let stdio = STDOUT.lock();
    drop(stdio);
}