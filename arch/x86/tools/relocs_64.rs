// SPDX-License-Identifier: GPL-2.0
//! x86-64 kernel relocation classification and signed-offset validation.

use crate::elf::{Relocation, Symbol};
use crate::relocs_header::{matches, relocation_type, SymbolClass, Width};
use crate::{Failure, Result};

pub(crate) fn classify(rel: &Relocation, sym: &Symbol, name: &[u8]) -> Result<Option<Width>> {
    if sym.section == 0 {
        return Ok(None);
    }
    let absolute = sym.section == 0xfff1 && !matches(SymbolClass::Relative, name, true, false);
    match rel.kind {
        0 | 2 | 4 | 24 | 42 => Ok(None),
        1 | 10 | 11 => {
            if absolute {
                if matches(SymbolClass::Absolute, name, true, false) {
                    return Ok(None);
                }
                return Err(Failure::named(
                    &format!(
                        "Invalid absolute {} relocation: ",
                        relocation_type(rel.kind, true)
                    ),
                    name,
                    "\n",
                ));
            }
            if i64::from(rel.offset as i32) != rel.offset as i64 {
                return Err("Relocation offset doesn't fit in 32 bits\n".into());
            }
            Ok(Some(if rel.kind == 1 {
                Width::Bits64
            } else {
                Width::Bits32
            }))
        }
        kind => Err(format!(
            "Unsupported relocation type: {} ({kind})\n",
            relocation_type(kind, true)
        )
        .into()),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
