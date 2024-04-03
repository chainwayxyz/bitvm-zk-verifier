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

//! # BitVM Transpiler
//!
//! This program can accept certain assembly source and binary files as inputs
//! and outputs BitVM executable files.
//!
//! ## Program Flow
//!
//! 1. Parse CLI flags and options
//! 2. Read input binary file and get instructions
//! 3. Run ISA specific decoder
//! 4. Run ISA specific BitVM assembler
//! 5. Organize memory
//! 6. Write BitVM assembly and memory to a file

mod bitvm;
mod cli;
mod elf_parser;
mod memory;
mod output;
mod riscv;

fn main() {
    let input_file = cli::parse_cli();

    let binary = elf_parser::read_elf_file(input_file);
    let binary_info = elf_parser::BinaryInfo::from(binary.clone());

    let decoded_binary = match binary_info.isa {
        elf_parser::Isa::Riscv32 | elf_parser::Isa::Riscv64 => riscv::decode(binary.clone()),
        elf_parser::Isa::None => panic!("ISA not supported!"),
    };

    let bitvm_instructions = match binary_info.isa {
        elf_parser::Isa::Riscv32 | elf_parser::Isa::Riscv64 => {
            riscv::generate_code(binary_info, decoded_binary)
        }
        elf_parser::Isa::None => panic!("ISA not supported!"),
    };

    let out = memory::organize(binary);

    output::generate_json("bitvm", bitvm_instructions, out);
}
