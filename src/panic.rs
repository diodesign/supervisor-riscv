/* diosix RV32G/RV64G supervisor-level panic-handling code
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

use core::panic::PanicInfo;

/* we need to provide these */
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> !
{
    /* just halt here */
    loop {}
}
