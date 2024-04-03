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

//! # Memory
//!
//! This module will organize output memory layout.
//!
//! ## Layout
//!
//! * XLEN = 4 if 32-bit else 8
//! * x0 = 0
//! * x1..x32 = 1*XLEN..32*XLEN
//! * First 3*XLEN bytes after registers are used for temporary storage
//! * code_addr..(code_addr+code_offset) has program counter (pc) values

use crate::elf_parser::InputBinary;

/// Memory size of the BitVM virtual machine in bytes.
pub const MEMSIZE: usize = 1024 * 1024 * 16;

pub fn organize(binary: InputBinary) -> Vec<u8> {
    let mut memory: Vec<u8> = vec![0; MEMSIZE];

    for i in 0..binary.data_page.len() {
        for j in 0..binary.data_page[i].len() {
            memory.insert(binary.data_addr[i] as usize + j, binary.data_page[i][j]);
        }
    }

    memory
}
