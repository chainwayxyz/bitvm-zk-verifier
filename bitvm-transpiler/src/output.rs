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

//! Output file creation for transpiled instructions.

use crate::bitvm::Instruction;
use std::fs::File;
use std::io::prelude::*;

pub fn generate_json(file_name: &str, input: Vec<Instruction>, memory: Vec<u8>) {
    let serialized = serde_json::to_string(&input).unwrap();
    let memory_serialized = serde_json::to_string(&memory).unwrap();

    let mut file = File::create(format!("{}.json", file_name)).unwrap();
    let out = String::from("{\n  \"program\": ")
        + serialized.as_str()
        + ",\n  \"memory_entries\": "
        + memory_serialized.as_str()
        + "\n}";

    file.write_all(out.as_bytes()).unwrap();
}
