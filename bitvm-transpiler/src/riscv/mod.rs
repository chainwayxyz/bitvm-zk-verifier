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

//! # RISC-V Assembly Decoder and BitVM Code Generator
//!
//! This crate includes a RISC-V binary decoder and a BitVM code generator for
//! RISC-V ISA.
//!
//! ## About 32-bit and 64-bit Support
//!
//! RISC-V 32-bit ISA is not a subset of 64-bit ISA. Therefore we need different
//! code generators for both targets. But we can use the same decoder for both
//! targets.

mod decoder;
mod generator;

pub use decoder::decode;
pub use generator::generate_code;

/// A decoded RISC-V instruction, ready for the use of RISC-V BitVM assembler.
#[derive(Debug)]
pub struct Instruction {
    pub instr: RiscvInstructions,
    pub rs1: u32,
    pub rs2: u32,
    pub rd: u32,
    pub imm: u32,
}

/// Instruction list for RISC-V targets.
#[allow(dead_code)]
#[derive(Debug)]
pub enum RiscvInstructions {
    NONE,
    // RV32I/RV64I
    LUI,
    AUIPC,
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    FENCE,
    ECALL,
    EBREAK,
    // RV64I
    LWU,
    LD,
    SD,
    ADDIW,
    SLLIW,
    SRLIW,
    SRAIW,
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,
    // RV32/RV64 Zifencei
    FENCEI,
    // RV32/RV64 Zicsr
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,
    // RV32M/RV64M
    MUL,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVU,
    REM,
    REMU,
    // RV64M
    MULW,
    DIVW,
    DIVUW,
    REMW,
    REMUW,
    // RV32A/RV64A
    LRW,
    SCW,
    AMOSWAPW,
    AMOADDW,
    AMOXORW,
    AMOANDW,
    AMOORW,
    AMOMINW,
    AMOMAXW,
    AMOMINUW,
    AMOMAXUW,
    // RV64A
    LRD,
    SCD,
    AMOSWAPD,
    AMOADDD,
    AMOXORD,
    AMOANDD,
    AMOORD,
    AMOMIND,
    AMOMAXD,
    AMOMINUD,
    AMOMAXUD,
    // RV32F/RV64F
    FLW,
    FSW,
    FMADDS,
    FMSUBS,
    FNMSUBS,
    FNMADDS,
    FADDS,
    FSUBS,
    FMULS,
    FDIVS,
    FSQRTS,
    FSGNJS,
    FSGNJNS,
    FSGNJXS,
    FMINS,
    FMAXS,
    FCVTWS,
    FCVTWUS,
    FMVXW,
    FEQS,
    FLTS,
    FLES,
    FCLASSS,
    FCVTSW,
    FCVTSWU,
    FMVWX,
    // RV64F
    FCVTLS,
    FCVTLUS,
    FCVTSL,
    FCVTSLU,
    // RV32D/RV64D
    FLD,
    FSD,
    FMADDD,
    FMSUBD,
    FNMSUBD,
    FNMADDD,
    FADDD,
    FSUBD,
    FMULD,
    FDIVD,
    FSQRTD,
    FSGNJD,
    FSGNJND,
    FSGNJXD,
    FMIND,
    FMAXD,
    FCVTSD,
    FCVTDS,
    FEQD,
    FLTD,
    FLED,
    FCLASSD,
    FCVTWD,
    FCVTWUD,
    FCVTDW,
    FCVTDWU,
    // RV64D
    FCVTLD,
    FCVTLUD,
    FMVXD,
    FCVTDL,
    FCVTDLU,
    FMVDX,
    // RV32Q/RV64Q
    FLQ,
    FSQ,
    FMADDQ,
    FMSUBQ,
    FNMSUBQ,
    FNMADDQ,
    FADDQ,
    FSUBQ,
    FMULQ,
    FDIVQ,
    FSQRTQ,
    FSGNJQ,
    FSGNJNQ,
    FSGNJXQ,
    FMINQ,
    FMAXQ,
    FCVTSQ,
    FCVTQS,
    FCVTDQ,
    FCVTQD,
    FEQQ,
    FLTQ,
    FLEQ,
    FCLASSQ,
    FCVTWQ,
    FCVTWUQ,
    FCVTQW,
    FCVTQWU,
    // RV64Q
    FCVTLQ,
    FCVTLUQ,
    FCVTQL,
    FCVTQLU,
}
