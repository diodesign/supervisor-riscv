/* diosix RV64G supervisor-level exception and interrupt handler
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

#![allow(unused_must_use)]

use spin::Mutex;
use super::sbi;
use riscv::register::scause;
use riscv::register::sepc;

lazy_static!
{
    static ref IRQ_LOCK: Mutex<bool> = Mutex::new(false);
}

extern "C"
{
    fn sventry(thread_id: usize, heap_start: usize, heap_end: usize);
}


/* useful functions for writing out info when the rest of the environment can't be trusted */
fn print_hex(value: usize)
{
    let chars = [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f' ];
    for index in 1..17
    {
        let nibble = value >> (4 * (16 - index)) & 0xf;
        sbi::putc(chars[nibble]);
    }
}

fn print_string(s: &str)
{
    for c in s.chars()
    {
        sbi::putc(c);
    }
}

/* exception handler for memory allocation failures */
#[alloc_error_handler]
fn alloc_failure(attempt: core::alloc::Layout) -> !
{
    let lock = IRQ_LOCK.lock();

    print_string("Internal error: Memory allocation failed, tried to allocate/free 0x");
    print_hex(attempt.size());
    print_string(" bytes. Shutting down...\n");
    
    drop(lock);
    sbi::exit(1);
}

/* environment exception and interrupt handler */
#[no_mangle]
pub extern "C" fn decode_irq(_registers: usize)
{
    let lock = IRQ_LOCK.lock();

    /* print out a message and shut down. just in case println!() stops working,
    write out the error message 'manually' */
    print_string("Internal error: Unhandled exception/irq 0x");
    print_hex(scause::read().bits());
    print_string(" at 0x");
    print_hex(sepc::read());
    print_string(" sventry 0x");
    print_hex(sventry as usize);
    print_string(" Shutting down...\n");

    /* not going anywhere */
    drop(lock);
    sbi::exit(1);
}