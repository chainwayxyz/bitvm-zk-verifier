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

//! # Executable and Linkable Format Parser
//!
//! This module reads an ELF input binary file's contents and saves important
//! bits to memory.
//!
//! We only need actual program code in an ELF file. Needed sections include:
//! * .text
//! * .data
//!
//! Note: This parser heavily depends on a 3rd party elf library.

use elf::endian::AnyEndian;
use elf::file::Class;
use elf::section::SectionHeader;
use elf::ElfStream;
use std::path::PathBuf;

/// Parsed binary file content, which is ready for transpiling.
#[derive(Clone, Debug)]
pub struct InputBinary {
    pub isa: Isa,
    pub code_page: Vec<u8>,
    pub code_addr: u64,
    pub data_page: Vec<Vec<u8>>,
    pub data_addr: Vec<u64>,
}

impl InputBinary {
    pub fn new() -> Self {
        Self {
            isa: Isa::None,
            code_page: vec![],
            code_addr: 0,
            data_page: vec![],
            data_addr: vec![],
        }
    }
}

/// Supported ISA's.
#[derive(Clone, Copy, Debug)]
pub enum Isa {
    None,
    Riscv32,
    Riscv64,
}

/// This struct includes summary of the input binary. It is also a subset of
/// `InputBinary`. Therefore it is initialized using the `InputBinary` struct.
#[derive(Clone, Copy)]
pub struct BinaryInfo {
    pub isa: Isa,
    pub pc: u64,
}

impl BinaryInfo {
    pub fn from(input_binary: InputBinary) -> Self {
        Self {
            isa: input_binary.isa,
            pc: input_binary.code_addr,
        }
    }
}

/// Reads file and section headers. Returns ELF target machine information and
/// actual code.
pub fn read_elf_file(input_file: PathBuf) -> InputBinary {
    let mut binary = InputBinary::new();
    let path = std::path::PathBuf::from(input_file);
    let io = std::fs::File::open(path).expect("Could not open ELF file.");
    let mut file =
        ElfStream::<AnyEndian, _>::open_stream(io).expect("Could not open ELF file as a stream.");

    binary.isa = match file.ehdr.e_machine {
        0xF3 => match file.ehdr.class {
            Class::ELF32 => Isa::Riscv32,
            Class::ELF64 => Isa::Riscv64,
        },
        _ => panic!("Unsupported ISA!"),
    };

    let text_section = get_a_section(&mut file, ".text");
    binary.code_page = text_section.0.to_vec();
    binary.code_addr = text_section.1;

    get_data_sections(&mut file, &mut binary);

    binary
}

/// Gets a raw section data and it's offset.
fn get_a_section<'a, T>(file: &'a mut ElfStream<AnyEndian, T>, section: &str) -> (&'a [u8], u64)
where
    T: std::io::Read + std::io::Seek,
{
    let shdr: SectionHeader = *file
        .section_header_by_name(section)
        .expect("section table should be parsable")
        .expect(format!("file should have a {} section", section).as_str());

    (file.section_data(&shdr).unwrap().0, shdr.sh_addr)
}

/// Gets multiple loadable data sections.
fn get_data_sections<'a, T>(file: &'a mut ElfStream<AnyEndian, T>, binary: &mut InputBinary)
where
    T: std::io::Read + std::io::Seek,
{
    // Get both actual section data and section name table.
    let shdr = file.section_headers().clone();

    for hdr in shdr {
        if hdr.sh_addr != binary.code_addr
           && hdr.sh_type == 1 && hdr.sh_addr != 0 {
            binary.data_addr.push(hdr.sh_addr);
            binary.data_page.push(file.section_data(&hdr).unwrap().0.to_vec());
        }
    }
}
