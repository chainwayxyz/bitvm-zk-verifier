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

//! Bit format agnostic RISC-V ISA decoder.

use super::{Instruction, RiscvInstructions};
use crate::elf_parser;

/// Decodes binary to an `Instruction` array.
pub fn decode(binary: elf_parser::InputBinary) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let code_page = binary.code_page;

    for i in (0..(code_page.len() / 4)).step_by(4) {
        let raw: u32 = ((code_page[i + 3] as u32) << 24)
            + ((code_page[i + 2] as u32) << 16)
            + ((code_page[i + 1] as u32) << 8)
            + ((code_page[i + 0] as u32) << 0);
        instructions.push(decode_raw_instruction(raw).unwrap());
    }

    instructions
}

/// Decodes a raw instruction to an `Instruction` struct.
fn decode_raw_instruction(raw_instruction: u32) -> Result<Instruction, String> {
    let mut instr = RiscvInstructions::NONE;
    let opcode = raw_instruction & 0x7f;
    let rd = (raw_instruction >> 7) & 0x1f;
    let rs1 = (raw_instruction >> 15) & 0x1f;
    let rs2 = (raw_instruction >> 20) & 0x1f;
    let funct3 = (raw_instruction >> 12) & 0x7;
    let funct7 = (raw_instruction >> 25) & 0x7f;
    let mut imm: u32 = 0;

    match opcode {
        0x03 => {
            imm = raw_instruction >> 20;
            match funct3 {
                0x0 => instr = RiscvInstructions::LB,
                0x1 => instr = RiscvInstructions::LH,
                0x2 => instr = RiscvInstructions::LW,
                0x3 => instr = RiscvInstructions::LD,
                0x4 => instr = RiscvInstructions::LBU,
                0x5 => instr = RiscvInstructions::LHU,
                0x6 => instr = RiscvInstructions::LWU,
                _ => {
                    return Err(format!(
                        "Line {}: Illegal values for opcode {}",
                        line!(),
                        opcode
                    ))
                }
            }
        }
        0x0f => match funct3 {
            0x0 => instr = RiscvInstructions::FENCE,
            0x1 => instr = RiscvInstructions::FENCEI,
            _ => {
                return Err(format!(
                    "Line {}: Illegal values for opcode {}",
                    line!(),
                    opcode
                ))
            }
        },
        0x13 => {
            imm = (raw_instruction & 0xfff00000) >> 20;
            let shamt = imm & 0x3f;
            match funct3 {
                0x0 => instr = RiscvInstructions::ADDI,
                0x1 => {
                    instr = RiscvInstructions::SLLI;
                    imm = shamt;
                }
                0x2 => instr = RiscvInstructions::SLTI,
                0x3 => instr = RiscvInstructions::SLTIU,
                0x4 => instr = RiscvInstructions::XORI,
                0x5 => match funct7 >> 1 {
                    0x00 => {
                        instr = RiscvInstructions::SRLI;
                        imm = shamt;
                    }
                    0x10 => {
                        instr = RiscvInstructions::SRAI;
                        imm = shamt;
                    }
                    _ => {
                        return Err(format!(
                            "Line {}: Illegal values for opcode {}",
                            line!(),
                            opcode
                        ))
                    }
                },
                0x6 => instr = RiscvInstructions::ORI,
                0x7 => instr = RiscvInstructions::ANDI,
                _ => {
                    return Err(format!(
                        "Line {}: Illegal values for opcode {}",
                        line!(),
                        opcode
                    ))
                }
            }
        }
        0x17 => {
            imm = raw_instruction & 0xfffff000;
            instr = RiscvInstructions::AUIPC;
        }
        0x1b => {
            imm = raw_instruction >> 20;
            let shamt = imm & 0x1f;
            match funct3 {
                0x0 => instr = RiscvInstructions::ADDIW,
                0x1 => {
                    imm = shamt;
                    instr = RiscvInstructions::SLLIW;
                }
                0x5 => match funct7 {
                    0x00 => {
                        imm = shamt;
                        instr = RiscvInstructions::SRLIW;
                    }
                    0x20 => {
                        imm = shamt;
                        instr = RiscvInstructions::SRAIW;
                    }
                    _ => {
                        return Err(format!(
                            "Line {}: Illegal values for opcode {}",
                            line!(),
                            opcode
                        ))
                    }
                },
                _ => {
                    return Err(format!(
                        "Line {}: Illegal values for opcode {}",
                        line!(),
                        opcode
                    ))
                }
            }
        }
        0x23 => {
            imm = ((raw_instruction & 0xfe000000) >> 20) | ((raw_instruction >> 7) & 0x1f);
            match funct3 {
                0x0 => instr = RiscvInstructions::SB,
                0x1 => instr = RiscvInstructions::SH,
                0x2 => instr = RiscvInstructions::SW,
                0x3 => instr = RiscvInstructions::SD,
                _ => {
                    return Err(format!(
                        "Line {}: Illegal values for opcode {}",
                        line!(),
                        opcode
                    ))
                }
            }
        }
        0x2f => {
            let funct5 = (funct7 & 0x7c) >> 2;
            if funct3 == 0x2 && funct5 == 0x00 {
                instr = RiscvInstructions::AMOADDW;
            } else if funct3 == 0x2 && funct5 == 0x01 {
                instr = RiscvInstructions::AMOSWAPW;
            } else if funct3 == 0x2 && funct5 == 0x04 {
                instr = RiscvInstructions::AMOXORW;
            } else if funct3 == 0x2 && funct5 == 0x08 {
                instr = RiscvInstructions::AMOORW;
            } else if funct3 == 0x2 && funct5 == 0x0c {
                instr = RiscvInstructions::AMOANDW;
            } else if funct3 == 0x2 && funct5 == 0x10 {
                instr = RiscvInstructions::AMOMINW;
            } else if funct3 == 0x2 && funct5 == 0x14 {
                instr = RiscvInstructions::AMOMAXW;
            } else if funct3 == 0x2 && funct5 == 0x18 {
                instr = RiscvInstructions::AMOMINUW;
            } else if funct3 == 0x2 && funct5 == 0x1c {
                instr = RiscvInstructions::AMOMAXUW;
            } else if funct3 == 0x3 && funct5 == 0x00 {
                instr = RiscvInstructions::AMOADDW;
            } else if funct3 == 0x3 && funct5 == 0x01 {
                instr = RiscvInstructions::AMOSWAPD;
            } else if funct3 == 0x3 && funct5 == 0x04 {
                instr = RiscvInstructions::AMOXORD;
            } else if funct3 == 0x3 && funct5 == 0x08 {
                instr = RiscvInstructions::AMOORD;
            } else if funct3 == 0x3 && funct5 == 0x0c {
                instr = RiscvInstructions::AMOANDD;
            } else if funct3 == 0x3 && funct5 == 0x10 {
                instr = RiscvInstructions::AMOMIND;
            } else if funct3 == 0x3 && funct5 == 0x14 {
                instr = RiscvInstructions::AMOMAXD;
            } else if funct3 == 0x3 && funct5 == 0x18 {
                instr = RiscvInstructions::AMOMINUD;
            } else if funct3 == 0x3 && funct5 == 0x1c {
                instr = RiscvInstructions::AMOMAXUD;
            } else if funct3 == 0x2 && funct5 == 0x02 {
                instr = RiscvInstructions::LRW;
            } else if funct3 == 0x3 && funct5 == 0x02 {
                instr = RiscvInstructions::LRD;
            } else if funct3 == 0x2 && funct5 == 0x03 {
                instr = RiscvInstructions::SCW;
            } else if funct3 == 0x3 && funct5 == 0x03 {
                instr = RiscvInstructions::SCD;
            }
        }
        0x33 => {
            if funct3 == 0x0 && funct7 == 0x00 {
                instr = RiscvInstructions::ADD;
            } else if funct3 == 0x0 && funct7 == 0x01 {
                instr = RiscvInstructions::MUL;
            } else if funct3 == 0x0 && funct7 == 0x20 {
                instr = RiscvInstructions::SUB;
            } else if funct3 == 0x1 && funct7 == 0x00 {
                instr = RiscvInstructions::SLL;
            } else if funct3 == 0x1 && funct7 == 0x01 {
                instr = RiscvInstructions::MULH;
            } else if funct3 == 0x2 && funct7 == 0x01 {
                instr = RiscvInstructions::MULHSU;
            } else if funct3 == 0x2 && funct7 == 0x00 {
                instr = RiscvInstructions::SLT;
            } else if funct3 == 0x3 && funct7 == 0x00 {
                instr = RiscvInstructions::SLTU;
            } else if funct3 == 0x3 && funct7 == 0x01 {
                instr = RiscvInstructions::MULHU;
            } else if funct3 == 0x4 && funct7 == 0x00 {
                instr = RiscvInstructions::XOR;
            } else if funct3 == 0x4 && funct7 == 0x01 {
                instr = RiscvInstructions::DIV;
            } else if funct3 == 0x5 && funct7 == 0x00 {
                instr = RiscvInstructions::SRL;
            } else if funct3 == 0x5 && funct7 == 0x01 {
                instr = RiscvInstructions::DIVU;
            } else if funct3 == 0x5 && funct7 == 0x20 {
                instr = RiscvInstructions::SRA;
            } else if funct3 == 0x6 && funct7 == 0x01 {
                instr = RiscvInstructions::REM;
            } else if funct3 == 0x6 && funct7 == 0x00 {
                instr = RiscvInstructions::OR;
            } else if funct3 == 0x7 && funct7 == 0x00 {
                instr = RiscvInstructions::AND;
            } else if funct3 == 0x7 && funct7 == 0x01 {
                instr = RiscvInstructions::REMU;
            }
        }
        0x37 => {
            imm = (raw_instruction & 0xfffff000) >> 0;
            instr = RiscvInstructions::LUI;
        }
        0x3b => {
            if funct3 == 0x0 && funct7 == 0x00 {
                instr = RiscvInstructions::ADDW;
            } else if funct3 == 0x0 && funct7 == 0x01 {
                instr = RiscvInstructions::MULW;
            } else if funct3 == 0x0 && funct7 == 0x20 {
                instr = RiscvInstructions::SUBW;
            } else if funct3 == 0x1 && funct7 == 0x00 {
                instr = RiscvInstructions::SLLW;
            } else if funct3 == 0x4 && funct7 == 0x01 {
                instr = RiscvInstructions::DIVW;
            } else if funct3 == 0x5 && funct7 == 0x00 {
                instr = RiscvInstructions::SRLW;
            } else if funct3 == 0x5 && funct7 == 0x01 {
                instr = RiscvInstructions::DIVUW;
            } else if funct3 == 0x5 && funct7 == 0x20 {
                instr = RiscvInstructions::SRAW;
            } else if funct3 == 0x6 && funct7 == 0x01 {
                instr = RiscvInstructions::REMW;
            } else if funct3 == 0x7 && funct7 == 0x01 {
                instr = RiscvInstructions::REMUW;
            }
        }
        0x63 => {
            imm = ((raw_instruction & 0x80000000) >> 19)
                | ((raw_instruction & 0x80) << 4)
                | ((raw_instruction >> 20) & 0x7e0)
                | ((raw_instruction >> 7) & 0x1e);
            match funct3 {
                0x0 => instr = RiscvInstructions::BEQ,
                0x1 => instr = RiscvInstructions::BNE,
                0x4 => instr = RiscvInstructions::BLT,
                0x5 => instr = RiscvInstructions::BGE,
                0x6 => instr = RiscvInstructions::BLTU,
                0x7 => instr = RiscvInstructions::BGEU,
                _ => {
                    return Err(format!(
                        "Line {}: Illegal values for opcode {}",
                        line!(),
                        opcode
                    ))
                }
            }
        }
        0x67 => {
            instr = RiscvInstructions::JALR;
            imm = (raw_instruction & 0xfff00000) >> 20;
        }
        0x6f => {
            instr = RiscvInstructions::JAL;
            imm = ((raw_instruction & 0x80000000) >> 11)
                | (raw_instruction & 0xff000)
                | ((raw_instruction >> 9) & 0x800)
                | ((raw_instruction >> 20) & 0x7fe);
        }
        0x73 => {
            imm = (raw_instruction & 0xfff00000) >> 20;
            match funct3 {
                0x0 => {
                    if rs2 == 0x0 && funct7 == 0x0 {
                        instr = RiscvInstructions::ECALL;
                    } else if rs2 == 0x1 && funct7 == 0x0 {
                        instr = RiscvInstructions::EBREAK;
                    }
                }
                0x1 => instr = RiscvInstructions::CSRRW,
                0x2 => instr = RiscvInstructions::CSRRS,
                0x3 => instr = RiscvInstructions::CSRRC,
                0x5 => instr = RiscvInstructions::CSRRWI,
                0x6 => instr = RiscvInstructions::CSRRSI,
                0x7 => instr = RiscvInstructions::CSRRCI,
                _ => {
                    return Err(format!(
                        "Line {}: Illegal values for opcode {}",
                        line!(),
                        opcode
                    ))
                }
            }
        }
        _ => {
            return Err(format!(
                "Line {}: Illegal values for opcode {}",
                line!(),
                opcode
            ))
        }
    }

    Ok(Instruction {
        instr,
        rs1,
        rs2,
        rd,
        imm,
    })
}

/// These tests currently does not covers all of the instructions. I just chose
/// some random instructions and called it a day. In the future, we can add full
/// coverage if needed.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv::{Instruction, RiscvInstructions::*};

    #[test]
    fn add() {
        // add x10, x2, x5
        let raw: u32 = 0x00510533;
        let result = decode_raw_instruction(raw).unwrap();
        assert_eq!(
            result,
            Instruction {
                instr: ADD,
                rs1: 2,
                rs2: 5,
                rd: 10,
                imm: 0,
            }
        );

        // add x31 x1 x21
        let raw: u32 = 0x01508fb3;
        let result = decode_raw_instruction(raw).unwrap();
        assert_eq!(
            result,
            Instruction {
                instr: ADD,
                rs1: 1,
                rs2: 21,
                rd: 31,
                imm: 0,
            }
        );

        // add x31 x1 x21
        let raw: u32 = 0x01508fb3;
        let result = decode_raw_instruction(raw).unwrap();
        assert_eq!(
            result,
            Instruction {
                instr: ADD,
                rs1: 1,
                rs2: 21,
                rd: 31,
                imm: 0,
            }
        );

        // addi x31 x1 21
        let raw: u32 = 0x01508f93;
        let result = decode_raw_instruction(raw).unwrap();
        assert_ne!(
            result,
            Instruction {
                instr: ADD,
                rs1: 1,
                rs2: 21,
                rd: 31,
                imm: 0,
            }
        );
    }

    #[test]
    fn addi() {
        // addi x11 x1 21
        let raw: u32 = 0x01508593;
        let result = decode_raw_instruction(raw).unwrap();
        assert_eq!(result.instr, ADDI);
        assert_eq!(result.rs1, 1);
        assert_eq!(result.rd, 11);
        assert_eq!(result.imm, 21);

        // addi x31 x30 0
        let raw: u32 = 0x000f0f93;
        let result = decode_raw_instruction(raw).unwrap();
        assert_eq!(result.instr, ADDI);
        assert_eq!(result.rs1, 30);
        assert_eq!(result.rd, 31);
        assert_eq!(result.imm, 0);

        // addi x14 x30 345
        let raw: u32 = 0x159f0713;
        let result = decode_raw_instruction(raw).unwrap();
        assert_eq!(result.instr, ADDI);
        assert_eq!(result.rs1, 30);
        assert_eq!(result.rd, 14);
        assert_eq!(result.imm, 345);

        // addi x14 x15 9999999 -> addi x14 x15 1663
        let raw: u32 = 0x67f78713;
        let result = decode_raw_instruction(raw).unwrap();
        assert_eq!(result.instr, ADDI);
        assert_eq!(result.rs1, 15);
        assert_eq!(result.rd, 14);
        assert_ne!(result.imm, 9999999);
        assert_eq!(result.imm, 1663);

        // add x31 x1 x21
        let raw: u32 = 0x01508fb3;
        let result = decode_raw_instruction(raw).unwrap();
        assert_eq!(result.instr, ADD);
        assert_eq!(result.rs1, 1);
        assert_eq!(result.rs2, 21);
        assert_eq!(result.rd, 31);
    }
}
