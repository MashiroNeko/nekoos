use riscv::register::{
    sstatus::Sstatus,
    scause::Scause,
};
use core::mem::zeroed;
use riscv::register::sstatus;

#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub x: [usize; 32], // general registers
    pub sstatus: Sstatus, // supervisor status register 
    pub sepc: usize, // supervisor exception program counter
    pub stval: usize, // supervisor trap value
    pub scause: Scause, // scause register: record the cause of exception/interrupt/trap
}

#[repr(C)]
pub struct Context {
    content_addr: usize 
}

#[repr(C)]
struct ContextContent {
    ra: usize, 
    satp: usize, 
    s: [usize; 12], 
}

impl TrapFrame {
    pub fn increase_sepc(self: &mut Self) {
        self.sepc = self.sepc + 4;
    }
}

impl Context {
    #[naked]
    #[inline(never)]
    pub unsafe extern "C" fn switch(&mut self, target: &mut Context) {
        asm!(include_str!("process/switch.asm") :::: "volatile");
    }
    pub unsafe fn null() -> Context {
        Context { content_addr: 0 }
    }

    pub unsafe fn new_kernel_thread(
        entry: extern "C" fn(usize) -> !,
        arg: usize,
        kstack_top: usize,
        satp: usize ) -> Context {
        ContextContent::new_kernel_thread(entry, arg, kstack_top, satp).push_at(kstack_top)
    }
}
impl ContextContent {
    fn new_kernel_thread(entry: extern "C" fn(usize) -> !, arg: usize , kstack_top: usize, satp: usize) -> ContextContent {
        let mut content: ContextContent = unsafe { zeroed() };
        content.ra = entry as usize;
        content.satp = satp;
        content.s[0] = arg;
        let mut sstatus_ = sstatus::read();
        sstatus_.set_spp(sstatus::SPP::Supervisor); 
        content.s[1] = sstatus_.bits();
        content
    }

    unsafe fn push_at(self, stack_top: usize) -> Context {
        let ptr = (stack_top as *mut ContextContent).sub(1);
        *ptr = self; 
        Context { content_addr: ptr as usize }
    }
}