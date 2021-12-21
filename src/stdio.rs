/* diosix RV64G supervisor-level code to input/output with the rest of the world
 *
 * (c) Chris Williams, 2020-2021.
 *
 * See LICENSE for usage and copying.
 */

use spinning::{Mutex, Lazy};
use core::fmt;
use super::sbi;

/* system standard output device */
pub static STDOUT: Lazy<Mutex<Stdout>> = Lazy::new(|| Mutex::new(Stdout {}));

/* print a formatted string to stdout with an automatic newline added */
#[macro_export]
macro_rules! println
{
    ($fmt:expr) => (stdout!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => (stdout!(concat!($fmt, "\n"), $($arg)*));
}

/* print a formatted string to stdout without a newline added */
#[macro_export]
macro_rules! print
{
    ($fmt:expr) => (stdout!("{}", $fmt));
    ($fmt:expr, $($arg:tt)*) => (stdout!(concat!($fmt), $($arg)*));
}

#[macro_export]
macro_rules! stdout
{
    ($($arg:tt)*) =>
    ({
        use core::fmt::Write;
        {
            let mut lock = $crate::stdio::STDOUT.lock();
            lock.write_fmt(format_args!($($arg)*)).unwrap();
        }
    });
}

pub struct Stdout;

impl fmt::Write for Stdout
{
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        for c in s.chars()
        {
            loop
            {
                /* if putc() simply failed then try again. if it failed
                for a specific reason (no permissions, not implemented, etc)
                then bail out */
                match sbi::putc(c)
                {
                    Ok(_) => break,
                    Err(sbi::Error::Failed) => (),
                    Err(_) => return Err(fmt::Error),
                }
            }
        }
        Ok(())
    }
}
