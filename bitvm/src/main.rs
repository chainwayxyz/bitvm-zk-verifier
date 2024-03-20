use core::panic;
use std::fs::File;
use std::io::BufReader;

use serde::Serialize;
use serde::Deserialize;

// Number of blocks for a player to respond until the other player wins
pub const TIMEOUT: usize = 1;

// Logarithm of the VM's max trace length
pub const LOG_TRACE_LEN: usize = 40;
// Max trace length
pub const TRACE_LEN: usize = 1 << LOG_TRACE_LEN;

// Logarithm of the length of a Merkle path
pub const LOG_PATH_LEN: usize = 5;
// Length of a Merkle path
pub const PATH_LEN: usize = 1 << LOG_PATH_LEN;
// Number of memory cells
pub const MEMORY_LEN: usize = 1 << PATH_LEN;

// VM instruction set (emulating rv32i)
pub const ASM_ADD: u8 = 1;
pub const ASM_SUB: u8 = 2;
pub const ASM_MUL: u8 = 3;
pub const ASM_AND: u8 = 4;
pub const ASM_OR: u8 = 5;
pub const ASM_XOR: u8 = 6;
pub const ASM_ADDI: u8 = 7;
pub const ASM_SUBI: u8 = 8;
pub const ASM_ANDI: u8 = 9;
pub const ASM_ORI: u8 = 10;
pub const ASM_XORI: u8 = 11;
pub const ASM_JMP: u8 = 12;
pub const ASM_BEQ: u8 = 13;
pub const ASM_BNE: u8 = 14;
pub const ASM_RSHIFT1: u8 = 15;
pub const ASM_SLTU: u8 = 16;
pub const ASM_SLT: u8 = 17;
pub const ASM_SYSCALL: u8 = 18;
pub const ASM_LOAD: u8 = 19;
pub const ASM_STORE: u8 = 20;
pub const ASM_RSHIFT8: u8 = 21;
pub const ASM_LSHIFT8: u8 = 22;

pub const U32_SIZE: usize = 1 << 32;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Instruction {
    pub asm_type: u8,
    pub address_a: u32,
    pub address_b: u32,
    pub address_c: u32,
}

#[derive(Debug)]
pub struct Snapshot {
    pub pc: u32,
    pub memory: Vec<u32>,
    pub step_count: usize,
    pub instruction: Instruction,
}

impl Snapshot {
    fn new(memory: Vec<u32>, instruction: Instruction, pc: u32) -> Self {
        Self {
            pc,
            memory,
            step_count: 0,
            instruction,
        }
    }

    pub fn read(&self, address: u32) -> u32 {
        self.memory[address as usize]
    }

    fn write(&mut self, address: u32, value: u32) {
        while address >= self.memory.len() as u32 {
            self.memory.push(0);
        }
        self.memory[address as usize] = value;
    }
}

fn execute_instruction(s: &mut Snapshot) {
    match s.instruction.asm_type {
        ASM_ADD => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a)
                    .wrapping_add(s.read(s.instruction.address_b)),
            );
            s.pc += 1
        }
        ASM_SUB => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a)
                    .wrapping_sub(s.read(s.instruction.address_b)),
            );
            s.pc += 1
        }
        ASM_MUL => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a)
                    .wrapping_mul(s.read(s.instruction.address_b)),
            );
            s.pc += 1
        }
        ASM_AND => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a) & s.read(s.instruction.address_b),
            );
            s.pc += 1
        }
        ASM_OR => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a) | s.read(s.instruction.address_b),
            );
            s.pc += 1
        }
        ASM_XOR => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a) ^ s.read(s.instruction.address_b),
            );
            s.pc += 1
        }
        ASM_ADDI => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a)
                    .wrapping_add(s.instruction.address_b),
            );
            s.pc += 1
        }
        ASM_SUBI => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a)
                    .wrapping_sub(s.instruction.address_b),
            );
            s.pc += 1
        }
        ASM_ANDI => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a) & s.instruction.address_b,
            );
            s.pc += 1
        }
        ASM_ORI => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a) | s.instruction.address_b,
            );
            s.pc += 1
        }
        ASM_XORI => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a) ^ s.instruction.address_b,
            );
            s.pc += 1
        }
        ASM_BEQ => {
            if s.read(s.instruction.address_a) == s.read(s.instruction.address_b) {
                s.pc = s.instruction.address_c
            } else {
                s.pc += 1
            }
        }
        ASM_BNE => {
            if s.read(s.instruction.address_a) != s.read(s.instruction.address_b) {
                s.pc = s.instruction.address_c
            } else {
                s.pc += 1
            }
        }
        ASM_JMP => s.pc = s.read(s.instruction.address_a),
        ASM_RSHIFT1 => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a) >> 1,
            );
            s.pc += 1
        }
        ASM_RSHIFT8 => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a) >> 8
            );
            s.pc += 1;
        }
        ASM_LSHIFT8 => {
            s.write(
                s.instruction.address_c,
                s.read(s.instruction.address_a) << 8
            );
            s.pc += 1;
        }
        ASM_SLTU => {
            s.write(
                s.instruction.address_c,
                if s.read(s.instruction.address_a) < s.read(s.instruction.address_b) {
                    1
                } else {
                    0
                },
            );
            s.pc += 1
        }
        ASM_SLT => {
            s.write(
                s.instruction.address_c,
                if (s.read(s.instruction.address_a) as i32)
                    < (s.read(s.instruction.address_b) as i32)
                {
                    1
                } else {
                    0
                },
            );
            s.pc += 1
        }
        ASM_LOAD => {
            s.instruction.address_a = s.read(s.instruction.address_b);
            s.write(s.instruction.address_c, s.read(s.instruction.address_a));
            s.pc += 1
        }
        ASM_STORE => {
            s.instruction.address_c = s.read(s.instruction.address_b);
            s.write(s.instruction.address_c, s.read(s.instruction.address_a));
            s.pc += 1
        }
        ASM_SYSCALL => {
            println!("syscall called");
            s.pc += 1
        }
        _ => panic!("Unknown instuction type {}", s.instruction.asm_type),
    }
}

#[derive(Serialize, Deserialize)]
pub struct VM {
    pub program: Vec<Instruction>,
    pub memory_entries: Vec<u32>,
}

impl VM {
    pub fn new(program_source: &[Instruction], memory_entries: &[u32]) -> Self {
        Self {
            program: program_source.into(),
            memory_entries: memory_entries.into(),
        }
    }

    pub fn run(&mut self, max_steps: usize) -> Snapshot {
        let mut snapshot: Snapshot = Snapshot::new(self.memory_entries.clone(), self.program[0], 0);
        while snapshot.pc < self.program.len() as u32 && snapshot.step_count + 1 < max_steps {
            snapshot.instruction = self.program[snapshot.pc as usize];
            execute_instruction(&mut snapshot);
            snapshot.step_count += 1;
        }
        snapshot
    }
}

fn main() {
    let file = File::open("bitvm.json").unwrap();
    let reader = BufReader::new(file);
    let mut vm: VM = serde_json::from_reader(reader).unwrap();
    let snapshot = vm.run(TRACE_LEN);
    println!("step count: {:?}", snapshot.step_count);
    let u = snapshot.read(28 * 4);
    let v = snapshot.read(33 * 4);
    println!("result: {}, {}", v, u);
}
