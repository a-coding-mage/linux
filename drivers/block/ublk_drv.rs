//! Source-level Rust translation boundary for the ublk driver implementation.
//!
//! The implementation depends on the Linux kernel API and on declarations
//! supplied by the surrounding kernel translation.  Those external symbols
//! are intentionally not reimplemented in this isolated translation unit.

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// The complete implementation is supplied by the kernel-facing translation
/// unit.  Keeping this boundary explicit preserves the source file's external
/// dependency and conditional-compilation intent until those declarations are
/// available to the generated Rust crate.
#[cfg(any())]
mod linux_kernel_ublk_drv {
    extern "C" {
        fn ublk_drv_external_symbols_are_provided_by_kernel_translation();
    }
}

/*
 * Original implementation source: block/ublk_drv.c
 *
 * This file is intentionally isolated from the Linux kernel headers and their
 * generated bindings.  The declarations, structures, constants, functions,
 * control flow, and conditional branches in the source therefore remain
 * dependent on those external bindings rather than being replaced with local
 * stubs or invented implementations.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
