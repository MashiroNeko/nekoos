#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod io;

mod init;
mod lang_items;
mod sbi;
mod interrupt;
mod context;
mod clock;
mod memory;
mod consts;

use buddy_system_allocator::LockedHeap;
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
