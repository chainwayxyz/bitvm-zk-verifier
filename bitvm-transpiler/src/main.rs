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
//! 5. Write BitVM assembly to a file

mod bitvm;
mod cli;
mod elf_parser;
mod output;
mod riscv;

fn main() {
    let input_file = cli::parse_cli();

    let binary = elf_parser::read_elf_file(input_file);
    let isa = binary.isa;

    let decoded_binary = match isa {
        elf_parser::Isa::Riscv32 | elf_parser::Isa::Riscv64 => riscv::decode(binary),
        elf_parser::Isa::None => panic!("ISA not supported!"),
    };

    let bitvm_instructions = match isa {
        elf_parser::Isa::Riscv32 | elf_parser::Isa::Riscv64 => riscv::generate_code(decoded_binary),
        elf_parser::Isa::None => panic!("ISA not supported!"),
    };

    output::generate_json("bitvm", bitvm_instructions)
}
