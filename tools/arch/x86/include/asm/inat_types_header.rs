// SPDX-License-Identifier: GPL-2.0-or-later
// Written by Masami Hiramatsu <mhiramat@redhat.com>.
//! Host-side import of canonical instruction attribute integer types.
#[path = "../../../../../arch/x86/include/asm/inat_types_header.rs"]
mod canonical;
pub(crate) use canonical::*;
// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
