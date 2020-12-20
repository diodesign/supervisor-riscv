/* diosix RV32G/RV64G supervisor-level entry point
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

use super::stdio;
use spin::Mutex;

lazy_static!
{
    static ref INIT_DONE: Mutex<bool> = Mutex::new(false);
}

extern "C"
{
    fn main();
}

/* entry point for the rust portion of this crate. this initializes the supervisor
   environment and then calls the application's main function.
   => thread_id = ID number for this virtual hardware thread, counting up from 0
      dtb_ptr = address of the device tree describing the environment in RAM
      dtb_len = length of the device tree
*/
#[no_mangle]
pub extern "C" fn sventry(thread_id: usize, _dtb_ptr: *const u8, _dtb_len: u32)
{
    /* make thread 0 prepare the environment, all other threads wait */
    match thread_id
    {
        0 =>
        {
            /* initialize parts of the library */
            stdio::init();

            /* let other threads run */
            *(INIT_DONE.lock()) = true;
        },
        _ => while *(INIT_DONE.lock()) != true {}
    }

    /* call the main application's entry point */
    unsafe { main() };
}
