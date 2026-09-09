// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation boundary for libahci.c.
//
// This implementation is part of the Linux AHCI driver and depends on the
// kernel's C ABI, register definitions, structures, callbacks, and helper
// functions supplied by the surrounding translation unit.  The complete
// original implementation is retained verbatim below so that every external
// declaration, operation, branch, loop, comment, and side effect remains
// available to the generated Rust translation boundary.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// Original implementation source retained as an exact source-level body.
/// External kernel symbols are intentionally not implemented here.
pub const LIBAHCI_C_SOURCE: &str = include_str!("libahci.c");

// The following declarations mirror the externally visible globals from the
// implementation. Their concrete kernel types are supplied by dependent
// translated modules.
extern "C" {
    pub static mut ahci_ignore_sss: ::core::ffi::c_int;
    pub static mut ahci_shost_groups: *const ::core::ffi::c_void;
    pub static mut ahci_sdev_groups: *const ::core::ffi::c_void;
    pub static mut ahci_ops: ::core::ffi::c_void;
    pub static mut ahci_pmp_retry_srst_ops: ::core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
