/* diosix RV32G/RV64G supervisor-level code to input/output with the rest of the world
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

use core::fmt;
use super::sbi;

/* simple object that writes out bytes to the user via SBI calls */
pub struct Stdout;
impl Stdout
{
    pub fn new() -> Stdout
    {
        Stdout {}
    }
}

impl fmt::Write for Stdout
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
