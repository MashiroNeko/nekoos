use crate::clock::init;

global_asm!(include_str!("boot/entry.asm"));

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub fn rust_main() -> ! {
    crate::interrupt::init();
    crate::clock::init();
    crate::memory::init();
    loop {}
}