// SPDX-License-Identifier: GPL-2.0-or-later
//! Host-tool instruction types are the canonical decoder's native Rust types.
#[path = "../../../../../arch/x86/lib/insn.rs"]
mod canonical;
#[allow(unused_imports)] // Retain one type/algorithm definition for every user.
pub(crate) use canonical::*;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
