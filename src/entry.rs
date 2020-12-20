/* diosix RV32G/RV64G supervisor-level entry point
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

extern "C"
{
    fn main();
}

/* entry point for the rust portion of this crate. perform any initialization
   for the environment and call the application's main function.
   => thread_id = ID number for this virtual hardware thread, counting up from 0
      dtb_ptr = address of the device tree describing the environment in RAM
      dtb_len = length of the device tree
*/
#[no_mangle]
pub extern "C" fn sventry(_thread_id: usize, _dtb_ptr: *const u8, _dtb_len: u32)
{
    /* call the main application's entry point */
    unsafe { main() };
}
