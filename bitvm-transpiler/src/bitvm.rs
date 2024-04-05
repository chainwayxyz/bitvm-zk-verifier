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

//! BitVM machine description. Parsers must target these descriptions. Most of
//! the stuff here are copied from the official BitVM implementation.

use serde::{Deserialize, Serialize};

pub const ASM_ADD: u8 = 1;
pub const ASM_SUB: u8 = 2;
pub const _ASM_MUL: u8 = 3;
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
pub const _ASM_SYSCALL: u8 = 18;
pub const ASM_LOAD: u8 = 19;
pub const ASM_STORE: u8 = 20;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    pub asm_type: u8,
    pub address_a: u32,
    pub address_b: u32,
    pub address_c: u32,
}

/// Possible labels for a RISC-V instruction.
#[derive(Debug, Clone, Copy)]
pub enum Labels {
    None,
    /// Represents program counter of the real RISC-V instruction.
    Pc(u64),
    /// Represents real program end.
    _ProgramEnd,
}

/// This struct will be used by the resolver to generate final BitVM assembly
/// instructions.
#[derive(Debug, Clone, Copy)]
pub struct UnresolvedInstruction {
    pub instr: Instruction,
    /// Real program counter in the actual binary.
    pub real_pc: u64,
    /// After code generation, this will be updated.
    pub bitvm_pc: u64,
    /// Metadata for resolver. If set other than Labels::None, it will be
    /// resolved by the resolver.
    pub label: Labels,
}

impl UnresolvedInstruction {
    /// Creates a new unresolved instruction.
    pub fn _new() -> Self {
        Self {
            instr: Instruction {
                asm_type: 0,
                address_a: 0,
                address_b: 0,
                address_c: 0,
            },
            real_pc: 0,
            bitvm_pc: 0,
            label: Labels::None,
        }
    }

    /// Creates an unresolved instruction, using a BitVM instruction.
    pub fn from(instr: Instruction) -> Self {
        Self {
            instr: instr,
            real_pc: 0,
            bitvm_pc: 0,
            label: Labels::None,
        }
    }

    /// Returns the generated BitVM instruction, without actually resolving.
    pub fn _lazy_resolve(&self) -> Instruction {
        self.instr
    }
}
