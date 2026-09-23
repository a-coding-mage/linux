// SPDX-License-Identifier: GPL-2.0-or-later
// Written by Masami Hiramatsu <mhiramat@redhat.com>.
//! Host decoders share the canonical core-only attribute implementation.
#[path = "../../../../arch/x86/lib/inat.rs"]
mod canonical;
pub(crate) use canonical::*;
// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
