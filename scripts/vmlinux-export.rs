// SPDX-License-Identifier: GPL-2.0-only
//! Final vmlinux exported-symbol tables and built-in device aliases.
//!
//! The included data comes from modpost's normal collected symbols and CRCs.
//! References are assembler linker names, not fabricated Rust declarations.
//! This is built-in metadata, not a loadable module or a second symbol owner.

#![no_std]

#[path = "../include/linux/export-internal_header.rs"]
mod export_internal;

include!(env!("VMLINUX_EXPORT_DATA"));
