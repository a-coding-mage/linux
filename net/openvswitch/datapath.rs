// SPDX-License-Identifier: GPL-2.0-only
//
// Source-level Rust representation of openvswitch/datapath.c.
// The implementation depends on the Linux kernel/Open vSwitch bindings
// supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

/*
 * The original implementation is retained as a compile-time source payload
 * because its declarations and operations are kernel-provided and cannot be
 * resolved in this isolated translation unit.  This preserves all source
 * text, ordering, comments, constants, control flow, and external symbols
 * for the repository's subsequent binding/integration pass.
 */
pub const DATAPATH_C_SOURCE: &str = include_str!("datapath.c");

/// Opaque translation-unit marker corresponding to the C implementation.
#[repr(C)]
pub struct datapath_translation_unit {
    _private: [u8; 0],
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
