// SPDX-License-Identifier: GPL-2.0
//! i386 kernel and real-mode relocation classification.

use crate::elf::{Relocation, Symbol};
use crate::relocs_header::{matches, relocation_type, SymbolClass, Width};
use crate::{Failure, Result};

pub(crate) fn classify(
    rel: &Relocation,
    sym: &Symbol,
    name: &[u8],
    real: bool,
) -> Result<Option<Width>> {
    let absolute = sym.section == 0xfff1 && !matches(SymbolClass::Relative, name, false, real);
    let accepted_absolute = matches(SymbolClass::Absolute, name, false, real);
    match rel.kind {
        0 | 2 | 4 | 21 | 23 => Ok(None),
        1 if !real => {
            if absolute {
                if accepted_absolute {
                    return Ok(None);
                }
                return Err(Failure::named(
                    "Invalid absolute R_386_32 relocation: ",
                    name,
                    "\n",
                ));
            }
            Ok(Some(Width::Bits32))
        }
        1 if real => {
            if !absolute {
                return Ok(matches(SymbolClass::Linear, name, false, true).then_some(Width::Bits32));
            }
            if accepted_absolute {
                return Ok(None);
            }
            Err(Failure::named(
                "Invalid absolute R_386_32 relocation: ",
                name,
                "\n",
            ))
        }
        20 if real => {
            if absolute {
                if accepted_absolute {
                    return Ok(None);
                }
                if matches(SymbolClass::Segment, name, false, true) {
                    return Ok(Some(Width::Bits16));
                }
            } else if !matches(SymbolClass::Linear, name, false, true) {
                return Ok(None);
            }
            let kind = if absolute { "absolute" } else { "relative" };
            Err(Failure::named(
                &format!("Invalid {kind} R_386_16 relocation: "),
                name,
                "\n",
            ))
        }
        kind => Err(format!(
            "Unsupported relocation type: {} ({kind})\n",
            relocation_type(kind, false)
        )
        .into()),
    }
}
