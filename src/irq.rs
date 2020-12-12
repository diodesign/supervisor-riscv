/* diosix RV32G/RV64G supervisor-level irq handler
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

// use riscv::register::scause;
// use riscv::register::sepc;

#[no_mangle]
pub extern "C" fn decode_irq()
{
    // println!("Exception encountered: {:x} at {:x}", scause::read().cause(), sepc::read());
    loop {}
}
