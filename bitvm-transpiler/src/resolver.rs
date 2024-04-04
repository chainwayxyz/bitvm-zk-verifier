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

//! This module will resolve any label, that can't be resolved while generating
//! code. This operation is not ISA specific.

use crate::bitvm;

pub fn resolve(instructions: Vec<bitvm::UnresolvedInstruction>) -> Vec<bitvm::Instruction> {
    let mut resolved = Vec::new();

	for i in instructions {
		resolved.push(i.instr);
	}

	resolved
}
