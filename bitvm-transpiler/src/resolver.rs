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
use crate::bitvm::Labels;

pub fn resolve(instructions: Vec<bitvm::UnresolvedInstruction>) -> Vec<bitvm::Instruction> {
    let mut resolved = Vec::new();
    let program_end = instructions.len() - 1;

    for mut i in instructions.clone() {
        match i.label {
            // This instruction does not need any afterwork.
            Labels::None => (),
            // This instruction needs a new PC assignment.
            Labels::Pc(pc) => {
                for j in instructions.clone() {
                    if j.real_pc == pc {
                        // We need to convert real pc to bitvm counterpart.
                        i.instr.address_c = j.bitvm_pc as u32;
                        break;
                    } else if j.real_pc > pc {
                        // No need to keep looking: PC will only go upwards.
                        break;
                    }
                    // TODO: If this loop can't find what it's looking for, it
                    // will silently skip. Is it safe?
                }
            }
            Labels::ProgramEnd => {
                i.instr.address_c = program_end as u32;
            }
            _ => (),
        }

        resolved.push(i.instr);
    }

    resolved
}
