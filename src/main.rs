use crate::emu::EmulatorAPI;


pub mod instruction_types {
    pub struct RType {
        pub op: u8,
        pub rs: u8,
        pub rt: u8,
        pub rd: u8,
        pub shamt: u8,
        pub funct: u8,
    }

    pub struct IType {
        pub op: u8,
        pub rs: u8,
        pub rt: u8,
        pub immediate: u16,
    }

    pub struct JType {
        pub op: u8,
        pub addr: u32,
    }

    pub enum Instruction {
        RType(RType),
        IType(IType),
        JType(JType),
    }
}

mod constants {
    pub const RAM_SIZE: usize = 10000;
}

mod processor {
    use crate::instruction_types::*;


    pub fn instruction_decode(instruction: u32) -> Instruction {
        if (instruction == 0) { // placeholder for spotting Rtype
            return Instruction::RType(RType {op: 0, rs: 0, rt: 0, rd: 0, shamt: 0, funct: 0});
        } else if (instruction == 1) { // placeholder for spotting IType
            return Instruction::IType(IType {op: 0, rs: 0, rt: 0, immediate: 0});
        } else if(instruction == 2) {
            return Instruction::JType(JType {op: 0, addr: 0});
        }

        return Instruction::RType(RType {op: 0, rs: 0, rt: 0, rd: 0, shamt: 0, funct: 0});
    }

    pub struct Cpu {
        pc: u32,
        registers: [u32; 32],
    }

    pub struct Fpu {
        registers: [f32; 32],
    }

    pub struct Processor {
        cpu: Cpu,
        fpu: Fpu,
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

    impl Processor {
        pub fn new() -> Self {
            Processor {
                cpu: Cpu::new(),
                fpu: Fpu::new(),
            }
        }
    }
}

mod emu {
    use crate::constants::*;
    use crate::processor;
    use crate::processor::*;
    pub struct Emulator {
        ram: [u32; RAM_SIZE],
        processor: Processor,
    }

    // This should be the interface the the frontend uses to interact with
    // the emulator
    pub trait EmulatorAPI {
        fn run_instruction(&self, instruction: u32);
    }

    impl Emulator {
        pub fn new() -> Self{
            Emulator {
                ram: [0; RAM_SIZE], // init ram to 0
                processor: Processor::new(),
            }
        }
    }

    impl EmulatorAPI for Emulator {
        fn run_instruction(&self, instruction: u32) {
            println!("Pretending to run an instruction");
            let thing = processor::instruction_decode(instruction);
        }
    }
}


fn main() {
    println!("Hello, world!");
    let mut emu_core = emu::Emulator::new();
    emu_core.run_instruction(34);
    emu_core.run_instruction(1);
    emu_core.run_instruction(2);
    emu_core.run_instruction(3);
}
