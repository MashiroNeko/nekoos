use riscv::register::{
    sstatus::Sstatus,
    scause::Scause,
};

#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub x: [usize; 32], // general registers
    pub sstatus: Sstatus, // supervisor status register 
    pub sepc: usize, // supervisor exception program counter
    pub stval: usize, // supervisor trap value
    pub scause: Scause, // scause register: record the cause of exception/interrupt/trap
}

impl TrapFrame {
    pub fn increase_sepc(self: &mut Self) {
        self.sepc = self.sepc + 4;
    }
}