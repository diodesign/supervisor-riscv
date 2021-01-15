/* diosix RV32G/RV64G supervisor-level irq handler
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

#![allow(unused_must_use)]

use super::sbi;
use core::fmt::Write;
use riscv::register::scause;
use riscv::register::sepc;

lazy_static!
{
    static ref IRQ_LOCK: spin::Mutex<bool> = spin::Mutex::new(false);
}

#[no_mangle]
pub extern "C" fn decode_irq()
{
    let lock = IRQ_LOCK.lock();

    /* print out a message and shut down */
    let mut stdout = super::stdio::Stdout::new();
    write!(&mut stdout, "Internal error: {:?} at 0x{:x}", scause::read().cause(), sepc::read());
    sbi::shutdown(sbi::SBI_EXT_SYS_RESET_REASON::NoReason);

    drop(lock);
    loop {}
}
