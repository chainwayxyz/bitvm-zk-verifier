// bitvm-transpiler - Convert other assemblies to BitVM assembly.
// Copyright (C) 2024 Chainway Labs
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! # RISC-V Code Generator
//!
//! BitVM code generator for RISC-V targets. Most of the RISC-V instructions are
//! not present in BitVM. So, we need to simulate them with instructions that
//! are present in BitVM.
//!
//! ## Memory Layout
//!
//! * XLEN = 4 if 32-bit else 8
//! * x0 = 0
//! * x1..x32 = 1*XLEN..32*XLEN
//! * First 3*XLEN bytes after registers are used for temporary storage

use crate::bitvm;
use crate::elf_parser::BinaryInfo;
use crate::riscv::{Instruction, RiscvInstructions::*};

fn reg2mem(reg: u32) -> u32 {
    reg * 4
}
fn tmp() -> u32 {
    33 * 4
}
fn tmp2() -> u32 {
    34 * 4
}
fn tmp3() -> u32 {
    35 * 4
}

/// Generates BitVM assembly using decoded RISC-V assembly.
pub fn generate_code(
    mut info: BinaryInfo,
    instructions: Vec<Instruction>,
) -> Vec<bitvm::UnresolvedInstruction> {
    let mut bitvm_instructions = Vec::new();

    for i in instructions {
        let instr = match i.instr {
            ADD => add(i, info),
            ADDI => addi(i, info),
            SUB => sub(i, info),
            XOR => xor(i, info),
            XORI => xori(i, info),
            AND => and(i, info),
            ANDI => andi(i, info),
            OR => or(i, info),
            ORI => ori(i, info),
            JAL => jal(i, info),
            LBU => lbu(i, info),
            LH => lh(i, info),
            LB => lb(i, info),
            LHU => lhu(i, info),
            LW => lw(i, info),
            SB => sb(i, info),
            SH => sh(i, info),
            SW => sw(i, info),
            JALR => jalr(i, info),
            AUIPC => auipc(i, info),
            LUI => lui(i, info),
            BEQ => beq(i, info),
            BNE => bne(i, info),
            SLLI => slli(i, info),
            SRLI => srli(i, info),
            SRAI => srai(i, info),
            SLT => slt(i, info),
            SLTU => sltu(i, info),
            SLTIU => sltiu(i, info),
            SLTI => slti(i, info),
            BLT => blt(i, info),
            BLTU => bltu(i, info),
            BGE => bge(i, info),
            BGEU => bgeu(i, info),
            ECALL => ecall(i, info),
            EBREAK => ebreak(i, info),
            SRA => sra(i, info),
            SRL => srl(i, info),
            SLL => sll(i, info),
            _ => panic!("Illegal instruction: {:#?}", i.instr),
        };

        // Assign actual RISC-V PC to every instruction. This can help resolver
        // to determine new jump location. Also, add instructions one by one to
        // return value.
        for mut j in instr {
            j.real_pc = info.pc;
            bitvm_instructions.push(j);
        }

        // Iterate RISc-V PC.
        info.pc += 1;
    }

    // Assign BitVM program counters.
    for i in 0..bitvm_instructions.len() {
        bitvm_instructions[i].bitvm_pc = i as u64;
    }

    bitvm_instructions
}

fn add(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_ADD,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    })]
}

fn addi(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_ADDI,
        address_a: reg2mem(instruction.rs1),
        address_b: instruction.imm,
        address_c: reg2mem(instruction.rd),
    })]
}

fn sub(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_SUB,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    })]
}

fn xor(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_XOR,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    })]
}

fn xori(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_XORI,
        address_a: reg2mem(instruction.rs1),
        address_b: instruction.imm,
        address_c: reg2mem(instruction.rd),
    })]
}

fn and(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_AND,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    })]
}
fn andi(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_ANDI,
        address_a: reg2mem(instruction.rs1),
        address_b: instruction.imm,
        address_c: reg2mem(instruction.rd),
    })]
}

fn or(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_OR,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    })]
}

fn ori(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_ORI,
        address_a: reg2mem(instruction.rs1),
        address_b: instruction.imm,
        address_c: reg2mem(instruction.rd),
    })]
}

fn jal(instruction: Instruction, info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: info.pc as u32 + 4,
            address_c: reg2mem(instruction.rd),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // TODO: this needs to be relative to pc
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        }),
    ]
}

fn lbu(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a bit
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0xFF,
            address_c: reg2mem(instruction.rd),
        }),
    ]
}

fn lh(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    let mut ret = vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a bit
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0xFF,
            address_c: reg2mem(instruction.rd),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 1,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a byte
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp2(),
        }),
    ];

    for _ in 0..8 {
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: tmp2(),
            address_b: tmp2(),
            address_c: tmp2(),
        }));
    }

    ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_OR,
        address_a: reg2mem(instruction.rd),
        address_b: tmp2(),
        address_c: reg2mem(instruction.rd),
    }));

    ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_ANDI,
        address_a: reg2mem(instruction.rd),
        address_b: 0x8000,
        address_c: tmp(),
    }));

    ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_ADD,
        address_a: tmp(),
        address_b: tmp(),
        address_c: tmp(),
    }));

    ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_SUB,
        address_a: 0,
        address_b: tmp(),
        address_c: tmp(),
    }));

    ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_OR,
        address_a: reg2mem(instruction.rd),
        address_b: tmp(),
        address_c: reg2mem(instruction.rd),
    }));

    ret
}

fn lb(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // Load can return up to 32-bit
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0xFF,
            address_c: reg2mem(instruction.rd),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // get MSB
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0x80,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // lshift
            asm_type: bitvm::ASM_ADD,
            address_a: tmp(),
            address_b: tmp(),
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_SUB,
            address_a: 0,
            address_b: tmp(),
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_OR,
            address_a: reg2mem(instruction.rd),
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        }),
    ]
}

fn lhu(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    let mut ret = vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a bit
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0xFF,
            address_c: reg2mem(instruction.rd),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 1,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a byte
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp2(),
        }),
    ];

    for _ in 0..8 {
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: tmp2(),
            address_b: tmp2(),
            address_c: tmp2(),
        }));
    }

    ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_OR,
        address_a: reg2mem(instruction.rd),
        address_b: tmp2(),
        address_c: reg2mem(instruction.rd),
    }));

    ret
}

fn lw(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    let mut ret = vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm + 3,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a bit
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0xFF,
            address_c: reg2mem(instruction.rd),
        }),
    ];

    for _ in 0..3 {
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_SUBI,
            address_a: tmp(),
            address_b: 1,
            address_c: tmp(),
        }));
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: tmp2(),
        }));
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a bit
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp2(),
        }));
        for __ in 0..8 {
            ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
                asm_type: bitvm::ASM_ADD,
                address_a: tmp2(),
                address_b: tmp2(),
                address_c: tmp2(),
            }));
        }
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_OR,
            address_a: reg2mem(instruction.rd),
            address_b: tmp2(),
            address_c: reg2mem(instruction.rd),
        }));
    }

    ret
}

fn sb(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_STORE,
            address_a: tmp2(),
            address_b: tmp(),
            address_c: 0,
        }),
    ]
}

fn sh(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    let mut ret = vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // tmp is now rs1 + offset which is the memroy point where we want to store rs2[0:15]
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // Move rs2 to tmp2, now we want to store tmp2 to the memory location in tmp.
            asm_type: bitvm::ASM_ADD,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // we get the last 8 bit and store it in tmp2
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // Finally wrote the last byte, now we have to write rs2[9:15]
            asm_type: bitvm::ASM_STORE,
            address_a: tmp2(),
            address_b: tmp(),
            address_c: 0,
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // We move 1 byte to write the second half.
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 1,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // rs2 is the value we want to write, moved it to tmp2
            asm_type: bitvm::ASM_ADD,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        }),
    ];

    for _ in 0..8 {
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_RSHIFT1,
            address_a: tmp2(),
            address_b: tmp2(),
            address_c: tmp2(),
        }));
    }

    ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_ANDI,
        address_a: tmp2(),
        address_b: 0xFF,
        address_c: tmp2(),
    }));
    ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_STORE,
        address_a: tmp2(),
        address_b: tmp(),
        address_c: 0,
    }));

    ret
}

fn sw(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    let mut ret = vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // Calculate base address and store it in tmp() for memory access
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // Temporarily store rs2 value for manipulation
            asm_type: bitvm::ASM_ADD,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        }),
    ];

    for i in 0..4 {
        // Isolate the current byte from tmp2() and store it
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // Use a new tmp register if necessary
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp3(),
        }));
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_STORE,
            address_a: tmp3(),
            address_b: tmp(),
            address_c: 0,
        }));
        // Prepare for the next byte unless it's the last byte
        if i < 3 {
            for _ in 0..8 {
                ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
                    asm_type: bitvm::ASM_RSHIFT1,
                    address_a: tmp2(),
                    address_b: tmp2(),
                    address_c: tmp2(),
                }));
            }
            ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
                asm_type: bitvm::ASM_ADDI,
                address_a: tmp(),
                address_b: 1,
                address_c: tmp(),
            }));
        }
    }

    ret
}

fn jalr(instruction: Instruction, info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    let mut ret = vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp(),
            address_b: 0xFFFFFFFE,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: tmp(),
        }),
    ];

    if instruction.rd != 0 {
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: info.pc as u32 + 4,
            address_c: reg2mem(instruction.rd),
        }));
    }

    ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_JMP,
        address_a: tmp(),
        address_b: 0,
        address_c: 0,
    }));

    ret
}

fn auipc(instruction: Instruction, info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        // imm is already << 12'ed
        asm_type: bitvm::ASM_ADDI,
        address_a: reg2mem(0),
        address_b: (instruction.imm + info.pc as u32) & 0xFFFFFFFF,
        address_c: reg2mem(instruction.rd),
    })]
}

/// ISA data-sheet: LUI places the U-immediate value in the top 20 bits of the
/// destination register rd, filling in the lowest 12 bits with zeros.
///
/// This is provided by decoder as the immediate value.
fn lui(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_ADDI,
        address_a: reg2mem(0),
        address_b: instruction.imm,
        address_c: reg2mem(instruction.rd),
    })]
}

/// BitVM compares address_a and address_b for BEQ. If equal, changes PC to
/// address_c. Therefore we add immediate value and RISC-V PC for the address_c.
fn beq(instruction: Instruction, info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_BEQ,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: info.pc as u32 + instruction.imm, // TODO: check pc relative addressing
    })]
}

/// BNE works same way as the BEQ.
fn bne(instruction: Instruction, info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_BNE,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: info.pc as u32 + instruction.imm, // TODO: check pc relative addressing
    })]
}

fn slli(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    let mut ret = vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_ADDI,
        address_a: reg2mem(instruction.rs1),
        address_b: 0,
        address_c: reg2mem(instruction.rd),
    })];

    // Calculate the number of times to apply ASM_LSHIFT8 based on the immediate
    // value.
    for _ in 0..(instruction.imm / 8) {
        for __ in 0..8 {
            ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
                asm_type: bitvm::ASM_ADD,
                address_a: reg2mem(instruction.rd),
                address_b: reg2mem(instruction.rd),
                address_c: reg2mem(instruction.rd),
            }));
        }
    }

    // Calculate any remaining shifts that are less than 8 bits.
    for _ in 0..(instruction.imm % 8) {
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: reg2mem(instruction.rd),
            address_b: reg2mem(instruction.rd),
            address_c: reg2mem(instruction.rd),
        }));
    }

    ret
}

fn srli(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    let mut ret = vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_ADDI,
        address_a: reg2mem(instruction.rs1),
        address_b: 0,
        address_c: reg2mem(instruction.rd),
    })];

    // Calculate the number of times to apply ASM_RSHIFT8 based on the immediate
    // value.
    for _ in 0..(instruction.imm / 8) {
        for __ in 0..8 {
            ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
                asm_type: bitvm::ASM_RSHIFT1,
                address_a: reg2mem(instruction.rd),
                address_b: 0,
                address_c: reg2mem(instruction.rd),
            }));
        }
    }

    // Calculate any remaining shifts that are less than 8 bits.
    for _ in 0..(instruction.imm % 8) {
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_RSHIFT1,
            address_a: reg2mem(instruction.rd),
            address_b: 0,
            address_c: reg2mem(instruction.rd),
        }));
    }

    ret
}

fn srai(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    let mut ret = vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_ADDI,
        address_a: reg2mem(instruction.rs1),
        address_b: 0,
        address_c: reg2mem(instruction.rd),
    })];

    for _ in 0..(instruction.imm) {
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0x80000000,
            address_c: tmp3(),
        }));
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_RSHIFT1,
            address_a: reg2mem(instruction.rd),
            address_b: 0,
            address_c: reg2mem(instruction.rd),
        }));
        ret.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_OR,
            address_a: reg2mem(instruction.rd),
            address_b: tmp3(),
            address_c: reg2mem(instruction.rd),
        }));
    }

    ret
}

fn slt(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_SLT,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    })]
}

fn sltu(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
        asm_type: bitvm::ASM_SLTU,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    })]
}

fn sltiu(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: instruction.imm,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_SLTU,
            address_a: reg2mem(instruction.rs1),
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        }),
    ]
}

fn slti(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: instruction.imm,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_SLT,
            address_a: reg2mem(instruction.rs1),
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        }),
    ]
}

fn blt(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_SLT,
            address_a: reg2mem(instruction.rs1),
            address_b: reg2mem(instruction.rs2),
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_BNE,
            address_a: tmp(),
            address_b: reg2mem(0),
            address_c: 0, // TODO: Check pc relative addressing
        }),
    ]
}

fn bltu(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_SLTU,
            address_a: reg2mem(instruction.rs1),
            address_b: reg2mem(instruction.rs2),
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_BNE,
            address_a: tmp(),
            address_b: reg2mem(0),
            address_c: 0, // TODO: Check pc relative addressing
        }),
    ]
}

fn bge(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_SLT,
            address_a: reg2mem(instruction.rs1),
            address_b: reg2mem(instruction.rs2),
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: tmp(),
            address_b: reg2mem(0),
            address_c: 0, // TODO: Check pc relative addressing
        }),
    ]
}

fn bgeu(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_SLTU,
            address_a: reg2mem(instruction.rs1),
            address_b: reg2mem(instruction.rs2),
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: tmp(),
            address_b: reg2mem(0),
            address_c: 0,
        }),
    ]
}

fn ecall(_instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // tmp() acts as our status buffer: 0 = unknown 1 = OK, 2 = not OK
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: 1,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // if x10 / a0 is 0, finish program
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(10),
            address_b: reg2mem(0),
            address_c: 0, // TODO: Check label
        }),
    ]
}

fn ebreak(_instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: 2,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0, // TODO: Check label
        }),
    ]
}

fn sra(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: 0,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // shift amount
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0x1F,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0, // TODO: Check label
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: tmp2(),
            address_c: 0, // TODO: Check label
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp(),
            address_b: 0x80000000,
            address_c: tmp3(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_RSHIFT1,
            address_a: tmp(),
            address_b: 0,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // add MSB
            asm_type: bitvm::ASM_OR,
            address_a: tmp(),
            address_b: tmp3(),
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_SUBI,
            address_a: tmp2(),
            address_b: 1,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // TODO: Check label
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // TODO: Check label
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 0,
            address_c: reg2mem(instruction.rd),
        }),
    ]
}

fn srl(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: 0,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // shift amount
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0x1F,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // TODO: Check label
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // TODO: Check label
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: tmp2(),
            address_c: 0,
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_RSHIFT1,
            address_a: tmp(),
            address_b: 0,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_SUBI,
            address_a: tmp2(),
            address_b: 1,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // TODO: Check label
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // TODO: Check label
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 0,
            address_c: reg2mem(instruction.rd),
        }),
    ]
}

fn sll(instruction: Instruction, _info: BinaryInfo) -> Vec<bitvm::UnresolvedInstruction> {
    vec![
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: 0,
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // shift amount
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0x1F,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0, // TODO: Check label
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: tmp2(),
            address_c: 0, // TODO: Check label
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: tmp(),
            address_b: tmp(),
            address_c: tmp(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_SUBI,
            address_a: tmp2(),
            address_b: 1,
            address_c: tmp2(),
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0, // TODO: Check label
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        }),
        bitvm::UnresolvedInstruction::from(bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 0,
            address_c: reg2mem(instruction.rd),
        }),
    ]
}

// #[cfg(test)]
// mod tests {
//     #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
//     use super::*;
//     use crate::elf_parser::Isa;
//     use crate::riscv::{Instruction, RiscvInstructions::*};
//     use crate::{bitvm, riscv};

//     // For most of the instructions, this is a good default.
//     pub const INPUT: Instruction = Instruction {
//         instr: ADD,
//         rs1: 1,
//         rs2: 2,
//         rd: 3,
//         imm: 4,
//     };

//     pub const INFO: BinaryInfo = BinaryInfo {
//         isa: Isa::Riscv32,
//         pc: 0,
//     };

//     /// Update input instruction's values.
//     pub fn update_instruction(instr: &mut Instruction, rs1: u32, rs2: u32, rd: u32, imm: u32) {
//         instr.rs1 = rs1;
//         instr.rs2 = rs2;
//         instr.rd = rd;
//         instr.imm = imm;
//     }

//     /// Convert input instruction values to zero.
//     pub fn make_it_zero(instr: &mut Instruction) {
//         update_instruction(instr, 0, 0, 0, 0);
//     }

// #[test]
// fn test_generate_code() {
//     let input_instr = tests::INPUT;

//     // Single add instruction.
//     let input = vec![input_instr];
//     assert_eq!(
//         generate_code(tests::INFO, input),
//         vec![bitvm::UnresolvedInstruction::(Instruction {
//             asm_type: bitvm::ASM_ADD,
//             address_a: reg2mem(1),
//             address_b: reg2mem(2),
//             address_c: reg2mem(3)
//         })]
//     );

//     let input = vec![input_instr, input_instr];
//     assert_eq!(
//         generate_code(tests::INFO, input),
//         vec![
//             bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//                 asm_type: bitvm::ASM_ADD,
//                 address_a: reg2mem(1),
//                 address_b: reg2mem(2),
//                 address_c: reg2mem(3)
//             },
//             bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//                 asm_type: bitvm::ASM_ADD,
//                 address_a: reg2mem(1),
//                 address_b: reg2mem(2),
//                 address_c: reg2mem(3)
//             }
//         ]
//     );

//     let input = vec![
//         input_instr,
//         Instruction {
//             instr: ADDI,
//             rs1: 1,
//             rs2: 2,
//             rd: 3,
//             imm: 4,
//         },
//     ];
//     assert_eq!(
//         generate_code(tests::INFO, input),
//         vec![
//             bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//                 asm_type: bitvm::ASM_ADD,
//                 address_a: reg2mem(1),
//                 address_b: reg2mem(2),
//                 address_c: reg2mem(3)
//             },
//             bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//                 asm_type: bitvm::ASM_ADDI,
//                 address_a: reg2mem(1),
//                 address_b: 4,
//                 address_c: reg2mem(3)
//             }
//         ]
//     );

//     let input = vec![
//         input_instr,
//         Instruction {
//             instr: ADDI,
//             rs1: 1,
//             rs2: 2,
//             rd: 3,
//             imm: 4,
//         },
//         Instruction {
//             instr: SUB,
//             rs1: 1,
//             rs2: 2,
//             rd: 3,
//             imm: 4,
//         },
//     ];
//     assert_eq!(
//         generate_code(tests::INFO, input),
//         vec![
//             bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//                 asm_type: bitvm::ASM_ADD,
//                 address_a: reg2mem(1),
//                 address_b: reg2mem(2),
//                 address_c: reg2mem(3)
//             },
//             bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//                 asm_type: bitvm::ASM_ADDI,
//                 address_a: reg2mem(1),
//                 address_b: 4,
//                 address_c: reg2mem(3)
//             },
//             bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//                 asm_type: bitvm::ASM_SUB,
//                 address_a: reg2mem(1),
//                 address_b: reg2mem(2),
//                 address_c: reg2mem(3)
//             }
//         ]
//     );

//     // Test whether big number of instructions are a problem or not.
//     // WARNING: This will take a long time to finish and use so much memory.
//     let mut input: Vec<Instruction> = Vec::new();
//     let mut expected: Vec<bitvm::Instruction> = Vec::new();
//     let count = 5_000_000;
//     for _ in 0..count {
//         input.push(input_instr);
//     }
//     for _ in 0..count {
//         expected.push(bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_ADD,
//             address_a: reg2mem(1),
//             address_b: reg2mem(2),
//             address_c: reg2mem(3),
//         });
//     }
//     assert_eq!(generate_code(tests::INFO, input), expected);
// }

// #[test]
// fn test_add() {
//     let mut input = tests::INPUT;

//     assert_eq!(
//         add(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_ADD,
//             address_a: reg2mem(1),
//             address_b: reg2mem(2),
//             address_c: reg2mem(3)
//         })]
//     );

//     tests::make_it_zero(&mut input);
//     assert_eq!(
//         add(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_ADD,
//             address_a: reg2mem(0),
//             address_b: reg2mem(0),
//             address_c: reg2mem(0)
//         })]
//     );
// }

// #[test]
// fn test_addi() {
//     let mut input = tests::INPUT;

//     assert_eq!(
//         addi(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_ADDI,
//             address_a: reg2mem(1),
//             address_b: 4,
//             address_c: reg2mem(3)
//         })]
//     );

//     tests::make_it_zero(&mut input);
//     assert_eq!(
//         addi(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_ADDI,
//             address_a: reg2mem(0),
//             address_b: 0,
//             address_c: reg2mem(0)
//         })]
//     );
// }

// #[test]
// fn test_sub() {
//     let mut input = tests::INPUT;

//     assert_eq!(
//         sub(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_SUB,
//             address_a: reg2mem(1),
//             address_b: reg2mem(2),
//             address_c: reg2mem(3)
//         })]
//     );

//     tests::make_it_zero(&mut input);
//     assert_eq!(
//         sub(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_SUB,
//             address_a: reg2mem(0),
//             address_b: reg2mem(0),
//             address_c: reg2mem(0)
//         })]
//     );
// }

// #[test]
// fn test_xor() {
//     let mut input = tests::INPUT;

//     assert_eq!(
//         xor(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_XOR,
//             address_a: reg2mem(1),
//             address_b: reg2mem(2),
//             address_c: reg2mem(3)
//         })]
//     );

//     tests::make_it_zero(&mut input);
//     assert_eq!(
//         xor(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_XOR,
//             address_a: reg2mem(0),
//             address_b: reg2mem(0),
//             address_c: reg2mem(0)
//         })]
//     );
// }

// #[test]
// fn test_xori() {
//     let mut input = tests::INPUT;

//     assert_eq!(
//         xori(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_XORI,
//             address_a: reg2mem(1),
//             address_b: 4,
//             address_c: reg2mem(3)
//         })]
//     );

//     tests::make_it_zero(&mut input);
//     assert_eq!(
//         xori(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_XORI,
//             address_a: reg2mem(0),
//             address_b: 0,
//             address_c: reg2mem(0)
//         })]
//     );
// }

// #[test]
// fn test_andi() {
//     let mut input = tests::INPUT;

//     assert_eq!(
//         andi(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_ANDI,
//             address_a: reg2mem(1),
//             address_b: 4,
//             address_c: reg2mem(3)
//         })]
//     );

//     tests::make_it_zero(&mut input);
//     assert_eq!(
//         andi(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_ANDI,
//             address_a: reg2mem(0),
//             address_b: 0,
//             address_c: reg2mem(0)
//         })]
//     );
// }

// #[test]
// fn test_or() {
//     let mut input = tests::INPUT;

//     assert_eq!(
//         or(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_OR,
//             address_a: reg2mem(1),
//             address_b: reg2mem(2),
//             address_c: reg2mem(3)
//         })]
//     );

//     tests::make_it_zero(&mut input);
//     assert_eq!(
//         or(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_OR,
//             address_a: reg2mem(0),
//             address_b: reg2mem(0),
//             address_c: reg2mem(0)
//         })]
//     );
// }

// #[test]
// fn test_ori() {
//     let mut input = tests::INPUT;

//     assert_eq!(
//         ori(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_ORI,
//             address_a: reg2mem(1),
//             address_b: 4,
//             address_c: reg2mem(3)
//         })]
//     );

//     tests::make_it_zero(&mut input);
//     assert_eq!(
//         ori(input, tests::INFO),
//         vec![bitvm::UnresolvedInstruction::from(bitvm::Instruction {
//             asm_type: bitvm::ASM_ORI,
//             address_a: reg2mem(0),
//             address_b: 0,
//             address_c: reg2mem(0)
//         })]
//     );
// }

// }
