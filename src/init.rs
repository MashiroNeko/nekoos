use crate::clock::init;

global_asm!(include_str!("boot/entry.asm"));

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, dtb: usize) -> ! {
    crate::interrupt::init();
    crate::clock::init();
    crate::memory::init();
    //test_page_table();
    crate::process::init();
    loop {}
}

fn test_page_table() {
    // test read
    let ptr = 0xc0400000 as *const u32;
    let value = unsafe { ptr.read() };
    println!("addr: {:?}, value: {:#x}", ptr, value);

    // test write: page fault!
    unsafe {
        (0xc0000000 as *mut u32).write(0);
    }
}