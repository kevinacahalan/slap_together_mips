use crate::emu::EmulatorAPI;

mod constants {
    pub const RAM_SIZE: usize = 10000;
}

mod processor {
    use crate::constants;

    pub struct Cpu {
        pc: u32,
        registers: [u32; 32],
    }

    pub struct Fpu {
        registers: [f32; 32],
    }

    impl Cpu {
        pub fn new() -> Cpu {
            Cpu {
                pc: 0x0,
                registers: [0; 32], // start all registers at 0
            }
        }
    }

    impl Fpu {
        pub fn new() -> Fpu {
            Fpu {
                registers: [0.0; 32],
            }
        }
    }
}

mod emu {
    use crate::constants::*;
    use crate::processor::*;
    pub struct Emulator {
        ram: [u32; RAM_SIZE],
        cpu: Cpu,
        fpu: Fpu,
    }

    // This should be the interface the the frontend uses to interact with
    // the emulator
    pub trait EmulatorAPI {
        fn run_instruction(&self, instruction: u32) {

        }
    }

    impl Emulator {
        pub fn new() -> Self{
            Emulator {
                ram: [0; RAM_SIZE], // init ram to 0
                cpu: Cpu::new(),
                fpu: Fpu::new(),
            }
        }
    }

    impl EmulatorAPI for Emulator {
        fn run_instruction(&self, instruction: u32) {
            println!("Pretending to run an instruction");
        }
    }
}


fn main() {
    println!("Hello, world!");
    let mut emu_core = emu::Emulator::new();
    emu_core.run_instruction(34);
}
