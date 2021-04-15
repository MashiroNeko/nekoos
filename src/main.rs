#![no_std] // do not link the Rust stdlib
#![no_main] // disable all rust-level entry points

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// this func is the entry point, since the linker looks for a func named '_start' by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
