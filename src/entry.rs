/* diosix RV32G/RV64G supervisor-level entry point
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

use super::sbi;
use core::sync::atomic::{AtomicUsize, Ordering};

/* the application's entry point */
extern "C"
{
    fn main();
}

/* reference count of cpu threads */
lazy_static!
{
    static ref THREADS_RUNNING: AtomicUsize = AtomicUsize::new(0);
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
    /* keep track of the number of threads entering the application */
    THREADS_RUNNING.fetch_add(1, Ordering::SeqCst);

    /* call the main application's entry point */
    unsafe { main() };

    /* when number of threads left running hits zero, shut down */
    if THREADS_RUNNING.fetch_sub(1, Ordering::SeqCst) == 1
    {
        /* shutdown this service's capsule when all threads have exited */
        sbi::shutdown(sbi::SBI_EXT_SYS_RESET_REASON::NoReason);
    }
}
