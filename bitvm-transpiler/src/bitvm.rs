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

#[derive(Copy, Clone)]
pub struct Instruction {
    pub asm_type: u8,
    pub address_a: u32,
    pub address_b: u32,
    pub address_c: u32,
}
