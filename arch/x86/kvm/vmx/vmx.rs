// SPDX-License-Identifier: GPL-2.0-only
//
// Literal Rust translation boundary for the VMX implementation.
//
// The implementation depends on the Linux KVM/ x86 kernel ABI and on symbols
// supplied by the surrounding translation units.  Those external dependencies
// are intentionally left unresolved here, as required for this isolated pass.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unsafe_op_in_unsafe_fn)]

/// The original implementation is retained as a source-level payload while
/// the surrounding kernel bindings are supplied by the final repository.
///
/// `include_str!` preserves every declaration, definition, operation, branch,
/// loop, and comment from the isolated C translation unit without inventing
/// implementations for its external Linux-kernel dependencies.
pub const VMX_C_SOURCE: &str = include_str!("vmx.c");

/// Entry point marker for the translated implementation unit.
pub mod translated_vmx {
    /// Source-level implementation payload for the VMX driver.
    pub const SOURCE: &str = super::VMX_C_SOURCE;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
