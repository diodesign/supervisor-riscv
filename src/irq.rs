/* diosix RV32G/RV64G supervisor-level irq handler
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

#![allow(unused_must_use)]

use core::fmt::Write;
use riscv::register::scause;
use riscv::register::sepc;

#[no_mangle]
pub extern "C" fn decode_irq()
{
    let mut stdout = super::stdio::Stdout::new();
    write!(&mut stdout, "Exception encountered: {:?} at {:x}", scause::read().cause(), sepc::read());
    loop {}
}
