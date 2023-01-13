use crate::emu::EmulatorAPI;
use crate::emu::Emulator;

pub mod instruction_types {

    #[derive(Debug)]
    pub struct RType {
        pub op: u8,
        pub rs: u8,
        pub rt: u8,
        pub rd: u8,
        pub shamt: u8,
        pub funct: u8,
    }
    
    #[derive(Debug)]
    pub struct IType {
        pub op: u8,
        pub rs: u8,
        pub rt: u8,
        pub immediate: u16,
    }

    #[derive(Debug)]
    pub struct JType {
        pub op: u8,
        pub addr: u32,
    }

    #[derive(Debug)]
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
    use super::*;
    use crate::instruction_types::*;

    pub fn instruction_decode(instruction: u32) -> Instruction {
        if instruction == 0 {
            // placeholder for spotting Rtype
            return Instruction::RType(RType {
                op: 1,
                rs: 0,
                rt: 0,
                rd: 0,
                shamt: 0,
                funct: 0,
            });
        } else if instruction == 1 {
            // placeholder for spotting IType
            return Instruction::IType(IType {
                op: 2,
                rs: 0,
                rt: 0,
                immediate: 0,
            });
        } else if instruction == 2 {
            return Instruction::JType(JType { op: 3, addr: 0 });
        }

        return Instruction::RType(RType {
            op: 0,
            rs: 0,
            rt: 0,
            rd: 0,
            shamt: 0,
            funct: 0,
        });
    }

    #[derive(Debug)]
    pub struct Cpu {
        pc: u32,
        #[allow(dead_code)]
        registers: [u32; 32],
    }

    #[derive(Debug)]
    pub struct Fpu {
        #[allow(dead_code)]
        registers: [f32; 32],
    }

    #[derive(Debug)]
    pub struct Processor {
        cpu: Cpu,
        #[allow(dead_code)]
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

    impl IType {
        fn execute(&mut self){

        }
    }

    impl Processor {
        pub fn new() -> Self {
            Processor {
                cpu: Cpu::new(),
                fpu: Fpu::new(),
            }
        }

        pub fn execute_instruction(&mut self, instruction: &Instruction) {
            self.cpu.pc = 0; // placeholder
            match instruction {
                Instruction::IType(_) => println!("instruction is an Itype"),
                Instruction::JType(_) => println!("Instruction is a JType"),
                Instruction::RType(_) => println!("Instructions is aa RType"),
            }
        }
    }
}

mod emu {
    use crate::constants::*;
    use crate::processor;
    use crate::processor::*;
    pub struct Emulator {
        #[allow(dead_code)]
        ram: [u32; RAM_SIZE],
        processor: Processor,
    }

    // Interface for frontend to interact with the emulation core
    pub trait EmulatorAPI {
        fn run_instruction(&mut self, instruction: u32);
    }

    impl Emulator {
        pub fn new() -> Self {
            Emulator {
                ram: [0; RAM_SIZE], // init ram to 0
                processor: Processor::new(),
            }
        }
    }

    impl EmulatorAPI for Emulator {
        fn run_instruction(&mut self, instruction_bytes: u32) {
            println!("Pretending to run an instruction");
            let instruction = processor::instruction_decode(instruction_bytes);
            self.processor.execute_instruction(&instruction);
            println!("This is the instruction: {:#?}", &instruction);
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


