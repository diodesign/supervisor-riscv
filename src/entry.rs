/* diosix RV64G supervisor-level entry point
 *
 * (c) Chris Williams, 2020-2021.
 *
 * See LICENSE for usage and copying.
 */

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use super::sbi;

/* the application's entry point */
extern "C"
{
    fn main(theead_id: usize);
}

/* reference count of cpu threads */
lazy_static!
{
    static ref THREADS_RUNNING: AtomicUsize = AtomicUsize::new(0);
    static ref INITIALIZED: AtomicBool = AtomicBool::new(false);
}

/* initialize the environment, starting with the heap allocator */
fn init(heap_start: usize, heap_end: usize)
{
    unsafe { ALLOCATOR.lock().init(heap_start, heap_end - heap_start); }
}

/* entry point for the rust portion of this crate. perform any initialization
   for the environment and call the application's main function.
   => thread_id = ID number for this virtual hardware thread, counting up from 0
      heap_start = start of RAM available for the heap
      heap_end = end of RAM available for the heap
*/
#[no_mangle]
pub extern "C" fn sventry(thread_id: usize, heap_start: usize, heap_end: usize)
{
    /* keep track of the number of threads entering the application */
    THREADS_RUNNING.fetch_add(1, Ordering::SeqCst);

    /* if we're running on diosix, let's go */
    if sbi::is_diosix()
    {
        /* let the boot thread (ID 0) take care of initializing the environment */
        if thread_id == 0
        {
            init(heap_start, heap_end);
            INITIALIZED.swap(true, Ordering::SeqCst);
        }
        else
        {
            /* wait until the boot thread (ID 0) has finished initializing the environment */
            loop
            {
                /* if we get back true instead of false, we're good to go.
                let the next thread know it's ok to exit the loop
                and then exit the loop ourselves */
                if INITIALIZED.swap(false, Ordering::SeqCst) == true
                {
                    INITIALIZED.swap(true, Ordering::SeqCst);
                    break;
                }
            }
        }

        /* call the main application's entry point */
        unsafe { main(thread_id) };
    }

    /* when number of threads left running hits zero, shut down */
    if THREADS_RUNNING.fetch_sub(1, Ordering::SeqCst) == 1
    {
        /* close down this service's capsule when all threads have exited */
        sbi::exit(0);
    }
}
