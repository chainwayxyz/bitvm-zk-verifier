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

//! BitVM code generator for RISC-V targets

use crate::bitvm;
use crate::riscv::{Instruction, RiscvInstructions::*};

/// Generates BitVM assembly using decoded RISC-V assembly.
pub fn generate_code(instructions: Vec<Instruction>) -> Vec<bitvm::Instruction> {
    let mut bitvm_instructions = Vec::new();

    for i in instructions {
        let instr = match i.instr {
            ADD => add(i),
            ADDI => addi(i),
            SUB => sub(i),
            XOR => xor(i),
            XORI => xori(i),
            AND => and(i),
            ANDI => andi(i),
            OR => or(i),
            ORI => ori(i),
            JAL => jal(i),
            LBU => lbu(i),
            LH => lh(i),
            LB => lb(i),
            LHU => lhu(i),
            LW => lw(i),
            SB => sb(i),
            SH => sh(i),
            SW => sw(i),
            JALR => jalr(i),
            AUIPC => auipc(i),
            LUI => lui(i),
            BEQ => beq(i),
            BNE => bne(i),
            SLLI => slli(i),
            SRLI => srli(i),
            SRAI => srai(i),
            SLT => slt(i),
            SLTU => sltu(i),
            SLTIU => sltiu(i),
            SLTI => slti(i),
            BLT => blt(i),
            BLTU => bltu(i),
            BGE => bge(i),
            BGEU => bgeu(i),
            ECALL => ecall(i),
            EBREAK => ebreak(i),
            SRA => sra(i),
            SRL => srl(i),
            SLL => sll(i),
            _ => panic!("Illegal instruction: {:#?}", i.instr),
        };

        for j in instr {
            bitvm_instructions.push(j);
        }
    }

    bitvm_instructions
}

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

fn add(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_ADD,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    }]
}

fn addi(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_ADDI,
        address_a: reg2mem(instruction.rs1),
        address_b: instruction.imm,
        address_c: reg2mem(instruction.rd),
    }]
}

fn sub(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_SUB,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    }]
}

fn xor(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_XOR,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    }]
}

fn xori(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_XORI,
        address_a: reg2mem(instruction.rs1),
        address_b: instruction.imm,
        address_c: reg2mem(instruction.rd),
    }]
}

fn and(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_AND,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    }]
}

fn andi(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_ANDI,
        address_a: reg2mem(instruction.rs1),
        address_b: instruction.imm,
        address_c: reg2mem(instruction.rd),
    }]
}

fn or(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_OR,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    }]
}

fn ori(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_ORI,
        address_a: reg2mem(instruction.rs1),
        address_b: instruction.imm,
        address_c: reg2mem(instruction.rd),
    }]
}

fn jal(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: 4,
            address_c: reg2mem(instruction.rd),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        },
    ]
}

fn lbu(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        },
        bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a bit
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0xFF,
            address_c: reg2mem(instruction.rd),
        },
    ]
}

fn lh(instruction: Instruction) -> Vec<bitvm::Instruction> {
    let mut ret = vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        },
        bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a bit
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0xFF,
            address_c: reg2mem(instruction.rd),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 1,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: tmp2(),
        },
        bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a byte
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp2(),
        },
    ];

    for _ in 0..8 {
        ret.push(bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: tmp2(),
            address_b: tmp2(),
            address_c: tmp2(),
        });
    }

    ret.push(bitvm::Instruction {
        asm_type: bitvm::ASM_OR,
        address_a: reg2mem(instruction.rd),
        address_b: tmp2(),
        address_c: reg2mem(instruction.rd),
    });

    ret.push(bitvm::Instruction {
        asm_type: bitvm::ASM_ANDI,
        address_a: reg2mem(instruction.rd),
        address_b: 0x8000,
        address_c: tmp(),
    });

    ret.push(bitvm::Instruction {
        asm_type: bitvm::ASM_ADD,
        address_a: tmp(),
        address_b: tmp(),
        address_c: tmp(),
    });

    ret.push(bitvm::Instruction {
        asm_type: bitvm::ASM_SUB,
        address_a: 0,
        address_b: tmp(),
        address_c: tmp(),
    });

    ret.push(bitvm::Instruction {
        asm_type: bitvm::ASM_OR,
        address_a: reg2mem(instruction.rd),
        address_b: tmp(),
        address_c: reg2mem(instruction.rd),
    });

    ret
}

fn lb(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        },
        bitvm::Instruction {
            // Load can return up to 32-bit
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0xFF,
            address_c: reg2mem(instruction.rd),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0x80,
            address_c: tmp(),
        },
        bitvm::Instruction {
            // lshift
            asm_type: bitvm::ASM_ADD,
            address_a: tmp(),
            address_b: tmp(),
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_SUB,
            address_a: 0,
            address_b: tmp(),
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_OR,
            address_a: reg2mem(instruction.rd),
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        },
    ]
}

fn lhu(instruction: Instruction) -> Vec<bitvm::Instruction> {
    let mut ret = vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        },
        bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a bit
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0xFF,
            address_c: reg2mem(instruction.rd),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 1,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: tmp2(),
        },
        bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a byte
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp2(),
        },
    ];

    for _ in 0..8 {
        ret.push(bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: tmp2(),
            address_b: tmp2(),
            address_c: tmp2(),
        });
    }

    ret.push(bitvm::Instruction {
        asm_type: bitvm::ASM_OR,
        address_a: reg2mem(instruction.rd),
        address_b: tmp2(),
        address_c: reg2mem(instruction.rd),
    });

    ret
}

fn lw(instruction: Instruction) -> Vec<bitvm::Instruction> {
    let mut ret = vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm + 3,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        },
        bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a bit
            asm_type: bitvm::ASM_ANDI,
            address_a: reg2mem(instruction.rd),
            address_b: 0xFF,
            address_c: reg2mem(instruction.rd),
        },
    ];

    for _ in 0..8 {
        ret.push(bitvm::Instruction {
            asm_type: bitvm::ASM_SUBI,
            address_a: tmp(),
            address_b: 1,
            address_c: tmp(),
        });
        ret.push(bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: tmp2(),
        });
        ret.push(bitvm::Instruction {
            // just to be sure someone didn't sneak in a uint32 value instead of a bit
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp2(),
        });
        for __ in 0..8 {
            ret.push(bitvm::Instruction {
                asm_type: bitvm::ASM_ADD,
                address_a: tmp2(),
                address_b: tmp2(),
                address_c: tmp2(),
            });
        }
        ret.push(bitvm::Instruction {
            asm_type: bitvm::ASM_OR,
            address_a: reg2mem(instruction.rd),
            address_b: tmp2(),
            address_c: reg2mem(instruction.rd),
        });
    }

    ret
}

fn sb(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_STORE,
            address_a: tmp2(),
            address_b: tmp(),
            address_c: 0,
        },
    ]
}

fn sh(instruction: Instruction) -> Vec<bitvm::Instruction> {
    let mut ret = vec![
        bitvm::Instruction {
            // tmp is now rs1 + offset which is the memroy point where we want to store rs2[0:15]
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        },
        bitvm::Instruction {
            // Move rs2 to tmp2, now we want to store tmp2 to the memory location in tmp.
            asm_type: bitvm::ASM_ADD,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            // we get the last 8 bit and store it in tmp2
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            // Finally wrote the last byte, now we have to write rs2[9:15]
            asm_type: bitvm::ASM_STORE,
            address_a: tmp2(),
            address_b: tmp(),
            address_c: 0,
        },
        bitvm::Instruction {
            // We move 1 byte to write the second half.
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 1,
            address_c: tmp(),
        },
        bitvm::Instruction {
            // rs2 is the value we want to write, moved it to tmp2
            asm_type: bitvm::ASM_ADD,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        },
    ];

    for _ in 0..8 {
        ret.push(bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: tmp2(),
            address_b: tmp2(),
            address_c: tmp2(),
        });
    }

    ret.push(bitvm::Instruction {
        asm_type: bitvm::ASM_ANDI,
        address_a: tmp2(),
        address_b: 0xFF,
        address_c: tmp2(),
    });
    ret.push(bitvm::Instruction {
        asm_type: bitvm::ASM_STORE,
        address_a: tmp2(),
        address_b: tmp(),
        address_c: 0,
    });

    ret
}

fn sw(instruction: Instruction) -> Vec<bitvm::Instruction> {
    let mut ret = vec![
        bitvm::Instruction {
            // Calculate base address and store it in tmp() for memory access
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        },
        bitvm::Instruction {
            // Temporarily store rs2 value for manipulation
            asm_type: bitvm::ASM_ADD,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        },
    ];

    for i in 0..4 {
        // Isolate the current byte from tmp2() and store it
        ret.push(bitvm::Instruction {
            // Use a new tmp register if necessary
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0xFF,
            address_c: tmp3(),
        });
        ret.push(bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        });
        // Prepare for the next byte unless it's the last byte
        if i < 3 {
            for _ in 0.. {
                ret.push(bitvm::Instruction {
                    asm_type: bitvm::ASM_RSHIFT1,
                    address_a: tmp2(),
                    address_b: tmp2(),
                    address_c: tmp2(),
                });
            }
            ret.push(bitvm::Instruction {
                asm_type: bitvm::ASM_ADDI,
                address_a: tmp(),
                address_b: 1,
                address_c: tmp(),
            });
        }
    }

    ret
}

fn jalr(instruction: Instruction) -> Vec<bitvm::Instruction> {
    let mut ret = vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: instruction.imm,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp(),
            address_b: 0xFFFFFFFE,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_LOAD,
            address_a: 0,
            address_b: tmp(),
            address_c: tmp(),
        },
    ];

    if instruction.rd != 0 {
        ret.push(bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: 4,
            address_c: reg2mem(instruction.rd),
        });
    }

    ret.push(bitvm::Instruction {
        asm_type: bitvm::ASM_JMP,
        address_a: tmp(),
        address_b: 0,
        address_c: 0,
    });

    ret
}

fn auipc(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        // imm is already << 12'ed
        asm_type: bitvm::ASM_ADDI,
        address_a: reg2mem(0),
        address_b: instruction.imm & 0xFFFFFFFF,
        address_c: reg2mem(instruction.rd),
    }]
}

fn lui(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_ADDI,
        address_a: reg2mem(0),
        address_b: (0b0110111 & 0xfffff000) >> 0, // 0110111: lui opcode
        address_c: reg2mem(instruction.rd),
    }]
}

fn beq(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_BEQ,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: 0,
    }]
}

fn bne(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_BNE,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: 0,
    }]
}

fn slli(instruction: Instruction) -> Vec<bitvm::Instruction> {
    let mut ret = vec![bitvm::Instruction {
        asm_type: bitvm::ASM_BNE,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: 0,
    }];

    for _ in 0..(instruction.imm / 8) {
        for __ in 0..8 {
            ret.push(bitvm::Instruction {
                asm_type: bitvm::ASM_ADD,
                address_a: tmp2(),
                address_b: tmp2(),
                address_c: tmp2(),
            });
        }
    }

    for _ in 0..(instruction.imm % 8) {
        ret.push(bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: reg2mem(instruction.rd),
            address_b: reg2mem(instruction.rd),
            address_c: reg2mem(instruction.rd),
        });
    }

    ret
}

fn srli(instruction: Instruction) -> Vec<bitvm::Instruction> {
    let mut ret = vec![bitvm::Instruction {
        asm_type: bitvm::ASM_ADDI,
        address_a: reg2mem(instruction.rs1),
        address_b: 0,
        address_c: reg2mem(instruction.rd),
    }];

    for _ in 0..(instruction.imm / 8) {
        for __ in 0..8 {
            ret.push(bitvm::Instruction {
                asm_type: bitvm::ASM_RSHIFT1,
                address_a: reg2mem(instruction.rd),
                address_b: 0,
                address_c: reg2mem(instruction.rd),
            });
        }
    }

    for _ in 0..(instruction.imm % 8) {
        ret.push(bitvm::Instruction {
            asm_type: bitvm::ASM_RSHIFT1,
            address_a: reg2mem(instruction.rd),
            address_b: 0,
            address_c: reg2mem(instruction.rd),
        });
    }

    ret
}

fn srai(instruction: Instruction) -> Vec<bitvm::Instruction> {
    let mut ret = vec![bitvm::Instruction {
        asm_type: bitvm::ASM_ADDI,
        address_a: reg2mem(instruction.rs1),
        address_b: 0,
        address_c: reg2mem(instruction.rd),
    }];

    for _ in 0..(instruction.imm) {
        for __ in 0..8 {
            ret.push(bitvm::Instruction {
                asm_type: bitvm::ASM_ANDI,
                address_a: reg2mem(instruction.rd),
                address_b: 0x80000000,
                address_c: tmp3(),
            });
            ret.push(bitvm::Instruction {
                asm_type: bitvm::ASM_RSHIFT1,
                address_a: reg2mem(instruction.rd),
                address_b: 0,
                address_c: reg2mem(instruction.rd),
            });
            ret.push(bitvm::Instruction {
                asm_type: bitvm::ASM_OR,
                address_a: reg2mem(instruction.rd),
                address_b: tmp3(),
                address_c: reg2mem(instruction.rd),
            });
        }
    }

    ret
}

fn slt(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_SLT,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    }]
}

fn sltu(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![bitvm::Instruction {
        asm_type: bitvm::ASM_SLTU,
        address_a: reg2mem(instruction.rs1),
        address_b: reg2mem(instruction.rs2),
        address_c: reg2mem(instruction.rd),
    }]
}

fn sltiu(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: instruction.imm,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_SLTU,
            address_a: reg2mem(instruction.rs1),
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        },
    ]
}

fn slti(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: instruction.imm,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_SLT,
            address_a: reg2mem(instruction.rs1),
            address_b: tmp(),
            address_c: reg2mem(instruction.rd),
        },
    ]
}

fn blt(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_SLT,
            address_a: reg2mem(instruction.rs1),
            address_b: reg2mem(instruction.rs2),
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BNE,
            address_a: tmp(),
            address_b: reg2mem(0),
            address_c: 0,
        },
    ]
}

fn bltu(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_SLTU,
            address_a: reg2mem(instruction.rs1),
            address_b: reg2mem(instruction.rs2),
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BNE,
            address_a: tmp(),
            address_b: reg2mem(0),
            address_c: 0,
        },
    ]
}

fn bge(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_SLTU,
            address_a: reg2mem(instruction.rs1),
            address_b: reg2mem(instruction.rs2),
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: tmp(),
            address_b: reg2mem(0),
            address_c: 0,
        },
    ]
}

fn bgeu(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_SLTU,
            address_a: reg2mem(instruction.rs1),
            address_b: reg2mem(instruction.rs2),
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: tmp(),
            address_b: reg2mem(0),
            address_c: 0,
        },
    ]
}

fn ecall(_instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: 1,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(10),
            address_b: reg2mem(0),
            address_c: 0,
        },
    ]
}

fn ebreak(_instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: 2,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        },
    ]
}

fn sra(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: 0,
            address_c: tmp(),
        },
        bitvm::Instruction {
            // shift amount
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0x1F,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: tmp2(),
            address_c: 0,
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp(),
            address_b: 0x80000000,
            address_c: tmp3(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_RSHIFT1,
            address_a: tmp(),
            address_b: 0,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_OR,
            address_a: tmp(),
            address_b: tmp3(),
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_SUBI,
            address_a: tmp2(),
            address_b: 1,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        },
        bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 0,
            address_c: reg2mem(instruction.rd),
        },
    ]
}

fn srl(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: 0,
            address_c: tmp(),
        },
        bitvm::Instruction {
            // shift amount
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0x1F,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: tmp2(),
            address_c: 0,
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_RSHIFT1,
            address_a: tmp(),
            address_b: 0,
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_SUBI,
            address_a: tmp2(),
            address_b: 1,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        },
        bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 0,
            address_c: reg2mem(instruction.rd),
        },
    ]
}

fn sll(instruction: Instruction) -> Vec<bitvm::Instruction> {
    vec![
        bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs1),
            address_b: 0,
            address_c: tmp(),
        },
        bitvm::Instruction {
            // shift amount
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(instruction.rs2),
            address_b: 0,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ANDI,
            address_a: tmp2(),
            address_b: 0x1F,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: tmp2(),
            address_c: 0,
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADD,
            address_a: tmp(),
            address_b: tmp(),
            address_c: tmp(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_SUBI,
            address_a: tmp2(),
            address_b: 1,
            address_c: tmp2(),
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_BEQ,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        },
        bitvm::Instruction {
            asm_type: bitvm::ASM_ADDI,
            address_a: reg2mem(0),
            address_b: reg2mem(0),
            address_c: 0,
        },
        bitvm::Instruction {
            // result
            asm_type: bitvm::ASM_ADDI,
            address_a: tmp(),
            address_b: 0,
            address_c: reg2mem(instruction.rd),
        },
    ]
}

#[cfg(test)]
mod tests {
    #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
    use super::*;
    use crate::bitvm;
    use crate::riscv::{Instruction, RiscvInstructions::*};

    // For most of the instructions, this is a good default.
    const INPUT: Instruction = Instruction {
        instr: ADD,
        rs1: 1,
        rs2: 2,
        rd: 3,
        imm: 4,
    };

    /// Update input instruction's values.
    fn update_instruction(instr: &mut Instruction, rs1: u32, rs2: u32, rd: u32, imm: u32) {
        instr.rs1 = rs1;
        instr.rs2 = rs2;
        instr.rd = rd;
        instr.imm = imm;
    }

    /// Convert input instruction values to zero.
    fn make_it_zero(instr: &mut Instruction) {
        update_instruction(instr, 0, 0, 0, 0);
    }

    #[test]
    fn test_add() {
        let mut input = INPUT;
        input.instr = ADD;

        assert_eq!(
            add(input),
            vec![bitvm::Instruction {
                asm_type: bitvm::ASM_ADD,
                address_a: reg2mem(1),
                address_b: reg2mem(2),
                address_c: reg2mem(3)
            }]
        );

        make_it_zero(&mut input);
        assert_eq!(
            add(input),
            vec![bitvm::Instruction {
                asm_type: bitvm::ASM_ADD,
                address_a: reg2mem(0),
                address_b: reg2mem(0),
                address_c: reg2mem(0)
            }]
        );
    }

    #[test]
    fn test_addi() {
        let mut input = INPUT;
        input.instr = ADDI;

        assert_eq!(
            addi(input),
            vec![bitvm::Instruction {
                asm_type: bitvm::ASM_ADDI,
                address_a: reg2mem(1),
                address_b: 4,
                address_c: reg2mem(3)
            }]
        );

        make_it_zero(&mut input);
        assert_eq!(
            addi(input),
            vec![bitvm::Instruction {
                asm_type: bitvm::ASM_ADDI,
                address_a: reg2mem(0),
                address_b: 0,
                address_c: reg2mem(0)
            }]
        );
    }
}
