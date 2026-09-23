// SPDX-License-Identifier: GPL-2.0
//! Kernel byte-classification table and safe, allocation-free helpers.
// Copyright (C) 1991, 1992 Linus Torvalds.
//
// Compile this table-owning module only once per kernel, through ctype_rust.rs.
// Independent native Rust consumers use kernel::ctype, not a path import of
// this source. The shared helper component uses a private table accessor;
// the owning crate emits export metadata and Kbuild versions its native DWARF.

#[path = "../include/linux/ctype_header.rs"]
mod classification;
pub use classification::*;

const fn ctype_mask(byte: u8) -> u8 {
    _ctype[byte as usize]
}

/// Original kernel classification masks for all 256 unsigned-byte values.
///
/// This is the sole immutable definition of the C-visible `_ctype` symbol.
#[rustfmt::skip] // Keep the original C byte-range rows directly auditable.
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static _ctype: [u8; 256] = [
    _C, _C, _C, _C, _C, _C, _C, _C,                         /* 0-7 */
    _C, _C | _S, _C | _S, _C | _S, _C | _S, _C | _S, _C, _C, /* 8-15 */
    _C, _C, _C, _C, _C, _C, _C, _C,                         /* 16-23 */
    _C, _C, _C, _C, _C, _C, _C, _C,                         /* 24-31 */
    _S | _SP, _P, _P, _P, _P, _P, _P, _P,                    /* 32-39 */
    _P, _P, _P, _P, _P, _P, _P, _P,                         /* 40-47 */
    _D, _D, _D, _D, _D, _D, _D, _D,                         /* 48-55 */
    _D, _D, _P, _P, _P, _P, _P, _P,                         /* 56-63 */
    _P, _U | _X, _U | _X, _U | _X, _U | _X, _U | _X, _U | _X, _U, /* 64-71 */
    _U, _U, _U, _U, _U, _U, _U, _U,                         /* 72-79 */
    _U, _U, _U, _U, _U, _U, _U, _U,                         /* 80-87 */
    _U, _U, _U, _P, _P, _P, _P, _P,                         /* 88-95 */
    _P, _L | _X, _L | _X, _L | _X, _L | _X, _L | _X, _L | _X, _L, /* 96-103 */
    _L, _L, _L, _L, _L, _L, _L, _L,                         /* 104-111 */
    _L, _L, _L, _L, _L, _L, _L, _L,                         /* 112-119 */
    _L, _L, _L, _P, _P, _P, _P, _C,                         /* 120-127 */
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,         /* 128-143 */
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,         /* 144-159 */
    _S | _SP, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, /* 160-175 */
    _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, _P, /* 176-191 */
    _U, _U, _U, _U, _U, _U, _U, _U, _U, _U, _U, _U, _U, _U, _U, _U, /* 192-207 */
    _U, _U, _U, _U, _U, _U, _U, _P, _U, _U, _U, _U, _U, _U, _U, _L, /* 208-223 */
    _L, _L, _L, _L, _L, _L, _L, _L, _L, _L, _L, _L, _L, _L, _L, _L, /* 224-239 */
    _L, _L, _L, _L, _L, _L, _L, _P, _L, _L, _L, _L, _L, _L, _L, _L, /* 240-255 */
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
