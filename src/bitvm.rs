use core::panic;
use std::fs::File;
use std::io::BufReader;

use serde::Serialize;
use serde::Deserialize;

// Number of blocks for a player to respond until the other player wins
pub const TIMEOUT: usize = 1;

// Logarithm of the VM's max trace length
pub const LOG_TRACE_LEN: usize = 32;
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

const BLAKE3_ZERO_HASHES: [&[u8]; 32] = [
    &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    &[53, 43, 210, 102, 218, 229, 60, 110, 106, 41, 36, 64, 17, 207, 160, 41, 129, 61, 10, 184],
    &[14, 197, 186, 149, 175, 113, 2, 91, 20, 109, 234, 197, 62, 150, 255, 154, 220, 3, 51, 116],
    &[123, 165, 20, 225, 166, 31, 181, 18, 73, 109, 253, 173, 14, 47, 173, 31, 12, 218, 0, 39],
    &[82, 61, 104, 220, 208, 183, 60, 48, 79, 61, 175, 177, 252, 170, 52, 238, 255, 117, 213, 107],
    &[72, 140, 160, 179, 218, 17, 50, 97, 84, 100, 196, 127, 34, 102, 179, 127, 151, 106, 113, 167],
    &[166, 67, 214, 177, 239, 83, 5, 11, 103, 155, 73, 165, 251, 43, 111, 52, 205, 65, 150, 35],
    &[247, 47, 45, 135, 11, 235, 169, 168, 53, 34, 245, 161, 163, 104, 100, 50, 162, 193, 242, 47],
    &[9, 120, 198, 46, 234, 15, 50, 191, 206, 166, 225, 163, 96, 231, 255, 169, 117, 175, 33, 217],
    &[44, 30, 169, 50, 129, 209, 16, 51, 106, 210, 12, 20, 192, 197, 68, 26, 139, 68, 98, 119],
    &[50, 194, 3, 16, 119, 72, 217, 167, 108, 77, 176, 182, 143, 140, 60, 50, 46, 196, 190, 54],
    &[214, 106, 11, 1, 217, 47, 78, 191, 102, 138, 119, 149, 154, 164, 16, 155, 231, 90, 241, 8],
    &[50, 212, 236, 31, 118, 230, 95, 150, 96, 172, 59, 210, 165, 212, 73, 94, 179, 10, 34, 52],
    &[120, 69, 24, 212, 103, 40, 242, 93, 170, 42, 205, 198, 148, 120, 77, 105, 85, 249, 15, 167],
    &[140, 152, 146, 107, 41, 35, 5, 248, 245, 222, 142, 113, 11, 60, 133, 168, 66, 247, 69, 250],
    &[146, 98, 31, 219, 248, 208, 102, 175, 203, 72, 238, 116, 119, 111, 143, 229, 133, 113, 199, 104],
    &[170, 8, 63, 159, 114, 151, 170, 208, 43, 60, 49, 200, 152, 11, 181, 49, 60, 14, 28, 246],
    &[68, 42, 136, 103, 53, 0, 62, 203, 161, 44, 174, 133, 134, 147, 200, 8, 185, 235, 161, 25],
    &[18, 249, 88, 207, 105, 42, 160, 144, 171, 155, 28, 237, 176, 96, 24, 249, 58, 40, 200, 198],
    &[93, 130, 192, 51, 239, 160, 237, 131, 205, 97, 200, 251, 166, 70, 147, 232, 31, 241, 46, 54],
    &[139, 207, 173, 210, 203, 100, 201, 243, 150, 170, 220, 144, 42, 31, 5, 145, 199, 252, 212, 243],
    &[172, 227, 69, 150, 192, 201, 186, 15, 87, 73, 60, 37, 226, 154, 75, 191, 238, 247, 164, 155],
    &[120, 226, 111, 210, 33, 225, 86, 119, 242, 234, 36, 161, 159, 121, 135, 200, 198, 113, 195, 214],
    &[239, 218, 162, 17, 119, 46, 225, 125, 1, 1, 0, 47, 69, 103, 62, 214, 69, 66, 154, 250],
    &[159, 213, 142, 48, 175, 36, 4, 129, 231, 60, 198, 204, 187, 234, 213, 33, 217, 168, 8, 14],
    &[45, 43, 205, 130, 123, 113, 98, 70, 214, 226, 160, 65, 184, 54, 222, 181, 173, 2, 252, 180],
    &[140, 36, 59, 194, 132, 87, 76, 131, 242, 19, 240, 59, 21, 163, 38, 136, 105, 2, 98, 165],
    &[184, 133, 175, 205, 215, 15, 59, 58, 254, 71, 41, 81, 79, 210, 201, 76, 154, 110, 80, 232],
    &[114, 166, 60, 77, 185, 171, 97, 67, 86, 59, 197, 174, 101, 45, 90, 130, 5, 28, 5, 210],
    &[34, 120, 233, 212, 114, 16, 11, 103, 53, 157, 95, 78, 236, 38, 101, 236, 188, 121, 224, 218],
    &[26, 49, 251, 232, 222, 186, 112, 212, 126, 49, 226, 245, 139, 60, 127, 223, 218, 83, 191, 136],
    &[72, 221, 68, 125, 140, 116, 102, 238, 215, 196, 221, 46, 203, 102, 234, 201, 246, 141, 65, 60],
];

fn blake3_zero_hash(layer: usize) -> [u8; 20] {
    let mut zero_node = [0u8; 20];
    for i in 0..20 {
        zero_node[i] = BLAKE3_ZERO_HASHES[layer][i];
    }
    return zero_node;
}

fn hash(left: [u8; 20], right: [u8; 20]) -> [u8; 20] {
    let mut input = [0u8; 40];
    for i in 0..20 {
        input[i] = left[i];
        input[i+20] = right[i];
    }
    let mut hash160 = [0u8; 20];
    let hash = blake3::hash(&input);
    for i in 0..20 {
        hash160[i] = hash.as_bytes()[i];
    }
    hash160
}

pub fn build_tree(leaves: &[u32]) -> [u8; 20] {
    // We need at least one leaf
    if leaves.len() == 0 { panic!("leaves is empty") }

    // Pad each leaf with zeros
    let mut leaves160 = Vec::with_capacity(leaves.len() + 1);
    for leaf in leaves {
        let mut leaf160 = [0u8; 20];
        leaf160[..4].copy_from_slice(&leaf.to_le_bytes());
        leaves160.push(leaf160);
    }

    // Hash from leaves to root
    let mut layer = 0;
    while leaves160.len() > 1 {
        // Use precomputed zero hash
        if (leaves160.len() & 1) == 1 {
            leaves160.push(blake3_zero_hash(layer));
        }
        // Compute next layer
        let mut tmp = Vec::with_capacity(leaves160.len() / 2);
        let mut i = 0;
        while i < leaves160.len() {
            tmp.push(hash(leaves160[i], leaves160[i+1]));
            i += 2;
        }
        leaves160 = tmp;
        layer += 1;
    }
    leaves160.shrink_to(1);
    // Extend to 32 layers
    while layer < 32 {
        // Use precomputed zero hash
        leaves160[0] = hash(leaves160[0], blake3_zero_hash(layer));
        layer += 1;
    }
    // Return root
    leaves160[0]
}

pub fn build_path(leaves: &[u32], index: u32) -> Vec<[u8; 20]> {
    // We need at least one leaf
    if leaves.len() == 0 { panic!("leaves is empty") }

    // Pad each leaf with zeros
    let mut leaves160 = Vec::with_capacity(leaves.len() + 1);
    for leaf in leaves {
        let mut leaf160 = [0u8; 20];
        leaf160[..4].copy_from_slice(&leaf.to_le_bytes());
        leaves160.push(leaf160);
    }

    let mut path = Vec::new();
    let mut index = index;
    let mut layer = 0;
    // Hash from leaves to root
    while leaves160.len() > 1 {
        // Use precomputed zero hash
        if (leaves160.len() & 1) == 1 {
            leaves160.push(blake3_zero_hash(layer));
        }
        path.push(leaves160[(index ^ 1) as usize]);
        // Compute next layer
        let mut tmp = Vec::with_capacity(leaves160.len() / 2);
        let mut i = 0;
        while i < leaves160.len() {
            tmp.push(hash(leaves160[i], leaves160[i+1]));
            i += 2;
        }
        leaves160 = tmp;
        index = index >> 1;
        layer += 1;
    }
    // Extend to 32 layers
    while layer < 32 {
        // Use precomputed zero hash
        path.push(blake3_zero_hash(layer));
        layer += 1;
    }
    // Return path
    path
}

pub fn verify_path(path: Vec<[u8; 20]>, leaf: u32, index: u32) -> [u8; 20] {
    // Pad the leaf with zeros
    let mut leaf160 = [0u8; 20];
    leaf160[..4].copy_from_slice(&leaf.to_le_bytes());
    let mut index = index;
    // Hash the path from leaf to root
    path.into_iter().fold(leaf160, |node, sibling| {
        let hash = if (index & 1) == 0 { hash(node, sibling) } else { hash(sibling, node) };
        index = index >> 1;
        hash
    })
}

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

pub struct MerklePath {
    pub path: Vec<[u8; 20]>,
    pub value: u32,
    pub address: u32,
}

impl MerklePath {
    fn new(snapshot: &Snapshot, address: u32) -> Self {
        Self {
            path: build_path(&snapshot.memory, address),
            value: snapshot.read(address),
            address,
        }
    }

    fn verify_up_to(&self, height: usize) -> [u8; 20] {
        let mut sub_path = self.path.clone();
        sub_path.shrink_to(PATH_LEN - height);
        verify_path(sub_path, self.value, self.address)
    }

    fn get_node(&self, index: usize) -> [u8; 20] {
        self.path[PATH_LEN - 1 - index]
    }
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

    fn path(&self, address: u32) -> MerklePath {
        MerklePath::new(self, address)
    }

    fn root(&self) -> [u8; 20] {
        build_tree(&self.memory)
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
    let snapshot = vm.run(2_usize.pow(40));
    println!("step count: {:?}", snapshot.step_count);
    let u = snapshot.read(28 * 4);
    let v = snapshot.read(33 * 4);
    println!("result: {}, {}", v, u);
}

// #[cfg(test)]
// mod tests {
//     use std::fs::File;
//     use std::io::BufReader;

//     use crate::bitvm::VM;

//     #[test]
//     fn execute_json() {
//         let file = File::open("bitvm.json").unwrap();
//         let reader = BufReader::new(file);
//         let mut vm: VM = serde_json::from_reader(reader).unwrap();
//         let snapshot = vm.run(2_usize.pow(40));
//         println!("step count: {:?}", snapshot.step_count);
//         let u = snapshot.read(28 * 4);
//         let v = snapshot.read(33 * 4);
//         println!("result: {}, {}", v, u);
//     }
// }
