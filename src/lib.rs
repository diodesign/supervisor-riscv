/* diosix RV64G supervisor-level environment code
 *
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

/* we're on our own */
#![no_std]
#![feature(asm)]
#![feature(alloc_error_handler)]
#![feature(box_syntax)]
#![allow(unused_imports)]
#![feature(global_asm)]

#[macro_use]
extern crate lazy_static;
extern crate spin;

/* common routines for working with RISC-V targets */
extern crate riscv;

/* heap allocator */
extern crate linked_list_allocator;

/* bring in the platform-specific assembly code */
global_asm!(include_str!("../asm/entry.s"));
global_asm!(include_str!("../asm/irq.s"));

#[macro_use]
pub mod stdio;
pub mod entry;
pub mod irq;
pub mod sbi;
pub mod panic;
