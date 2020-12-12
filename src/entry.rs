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

#[no_mangle]
pub extern "C" fn sventry(_cpu_nr: usize, _dtb_ptr: *const u8, _dtb_len: u32)
{
    /* call the main application's entry point */
    unsafe { main() };
}
