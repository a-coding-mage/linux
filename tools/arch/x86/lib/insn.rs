// SPDX-License-Identifier: GPL-2.0-or-later
//! Host-tool view of the canonical, allocation-free x86 instruction decoder.
#[path = "../../../../arch/x86/lib/insn.rs"]
mod canonical;
#[allow(unused_imports)] // This mirrored module exposes the full shared API.
pub(crate) use canonical::*;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
