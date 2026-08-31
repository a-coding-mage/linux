// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of asm-generic/bitsperlong.h.
// C header guards and include directives are omitted; this file depends on the
// UAPI asm-generic bits-per-long definitions supplied elsewhere.

pub const BITS_PER_LONG: usize = core::mem::size_of::<core::ffi::c_long>() * (u8::BITS as usize);

// C checked:
//     #if BITS_PER_LONG != __BITS_PER_LONG
//     #error Inconsistent word size. Check asm/bitsperlong.h
//     #endif
// Preserve this as a build-time dependency/consistency requirement on the
// external UAPI __BITS_PER_LONG definition.

pub const BITS_PER_LONG_LONG: usize = 64;

pub const fn small_const_nbits(nbits: usize) -> bool {
    nbits <= BITS_PER_LONG && nbits > 0
}
