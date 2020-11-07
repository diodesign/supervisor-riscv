/* diosix RV32G/RV64G supervisor-level entry point
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

#[no_mangle]
pub extern "C" fn sventry(cpu_nr: usize, dtb_ptr: *const u8, dtb_len: u32)
{
    main();
}
