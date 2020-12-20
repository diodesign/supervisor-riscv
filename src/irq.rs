/* diosix RV32G/RV64G supervisor-level irq handler
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

use riscv::register::scause;
use riscv::register::sepc;

#[no_mangle]
pub extern "C" fn decode_irq()
{
    super::sbi::console_putchar('0' as u8);
    super::sbi::console_putchar('x' as u8);
    let chars = [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f' ];
    let ptr = scause::read().bits();
    for index in 1..17
    {
        let nibble = ptr >> (4 * (16 - index)) & 0xf;
        super::sbi::console_putchar(chars[nibble] as u8);
    }
    super::sbi::console_putchar('\n' as u8);

    super::sbi::console_putchar('0' as u8);
    super::sbi::console_putchar('x' as u8);
    let ptr = sepc::read();
    for index in 1..17
    {
        let nibble = ptr >> (4 * (16 - index)) & 0xf;
        super::sbi::console_putchar(chars[nibble] as u8);
    }
    super::sbi::console_putchar('\n' as u8);

    // println!("Exception encountered: {:?} at {:x}", scause::read().cause(), sepc::read());
    loop {}
}
