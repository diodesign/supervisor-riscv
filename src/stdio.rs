/* diosix RV32G/RV64G supervisor-level code to input/output with the rest of the world
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

use core::fmt;
use super::sbi;

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
            unsafe { $crate::STDOUT.write_fmt(format_args!($($arg)*)).unwrap(); }
        }
    });
}

pub struct SBIWriter;
pub static mut STDOUT: SBIWriter = SBIWriter {};

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
