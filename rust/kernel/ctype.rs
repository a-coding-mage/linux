// SPDX-License-Identifier: GPL-2.0
//! Safe access to the kernel's shared byte-classification table and helpers.
//!
//! Native Rust callers should import this module instead of including the
//! table-owning `lib/ctype.rs` source. The linked table may be supplied by the
//! original C implementation or by `CONFIG_RUST_CTYPE`; neither the public API
//! nor the classification rules depend on that choice. This module defines
//! no second table or unmangled symbol.
//!
//! These helpers preserve the kernel's historical byte/Latin-1 rules, not
//! Unicode or locale-aware behavior. Most integer inputs are truncated to
//! a byte; [`isdigit`] instead compares the full integer. There is no special
//! interpretation of C EOF.
//!
//! The shared helpers retain their `const fn` signatures, but calls that
//! inspect the foreign table cannot be evaluated in a Rust constant: its
//! contents are resolved at link time. Table-independent operations such as
//! [`isdigit`], [`isodigit`], [`isascii`], [`toascii`], and [`_tolower`] remain
//! usable in constant expressions.

#[path = "../../include/linux/ctype_header.rs"]
mod classification;

pub use classification::*;

const fn ctype_mask(byte: u8) -> u8 {
    // Bindgen represents the public C declaration `_ctype[]` as `[u8; 0]`.
    // Keep a raw pointer rather than constructing a reference whose apparent
    // length disagrees with that declaration. wrapping_add also avoids an
    // in-bounds arithmetic assumption based on the incomplete array type.
    let table = core::ptr::addr_of!(crate::bindings::_ctype).cast::<u8>();
    // SAFETY: both selectable implementations supply the same immutable
    // 256-byte `_ctype` allocation for the kernel's entire lifetime. A u8
    // index is always within that actual allocation. Reading one byte needs
    // no stronger alignment, creates no aliasing reference, and cannot race
    // with a writer. No table data is accessed while constructing the pointer.
    unsafe { table.wrapping_add(byte as usize).read() }
}
