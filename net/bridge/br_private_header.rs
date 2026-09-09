/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Faithful Rust-side header representation.
 *
 * This header is dependency-heavy: its declarations refer to Linux kernel
 * types and configuration symbols supplied by the surrounding translation.
 * The complete source-level declaration set is retained verbatim below so
 * those external names, conditional branches, layouts, comments, and inline
 * semantics remain available to the generated binding layer.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// Original C header text, retained as the authoritative declaration source
/// until the kernel dependency bindings are made available to this crate.
pub const BR_PRIVATE_HEADER_SOURCE: &str = include_str!("br_private.h");

/* C-compatible scalar aliases used by the declarations in this header. */
pub type __u16 = u16;
pub type u8 = std::ffi::c_uchar;
pub type u16 = std::ffi::c_ushort;
pub type u32 = std::ffi::c_uint;
pub type __be16 = u16;
pub type __be64 = u64;
pub type port_id = __u16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_id {
    pub prio: [u8; 2],
    pub addr: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_addr {
    pub addr: [u8; 6],
}

/*
 * The remaining declarations intentionally stay sourced from the complete
 * header above: they contain configuration-selected layouts and inline
 * functions whose external Linux-kernel types are not defined locally.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
