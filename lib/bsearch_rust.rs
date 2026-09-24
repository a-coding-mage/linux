// SPDX-License-Identifier: GPL-2.0-only
//! Native bsearch owner, unrestricted export and kprobe exclusion record.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "bsearch.rs"]
mod implementation;

pub use implementation::{bsearch, declarations};
ffi_export::export_symbol!(bsearch, bsearch, "", "");

// __NOKPROBE_SYMBOL stores the function address in a writable pointer-sized
// _kprobe_blacklist record. A real function pointer emits the target relocation
// without integer-address fabrication or architecture-specific assembly.
#[cfg(CONFIG_KPROBES)]
#[used]
#[link_section = "_kprobe_blacklist"]
static mut KPROBE_BLACKLIST: unsafe extern "C" fn(
    *const core::ffi::c_void,
    *const core::ffi::c_void,
    usize,
    usize,
    declarations::BsearchCmp,
) -> *mut core::ffi::c_void = bsearch;
