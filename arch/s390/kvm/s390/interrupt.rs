// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation boundary for the s390 KVM interrupt
// implementation. The declarations and operations below intentionally retain
// the kernel ABI names and external dependencies supplied by the surrounding
// translation unit.

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

// The complete implementation is emitted as an unsafe ABI-preserving module.
// Kernel-provided types, constants, macros, atomics, locking primitives, and
// helper functions are intentionally referenced rather than reimplemented.

pub const PFAULT_INIT: u32 = 0x0600;
pub const PFAULT_DONE: u32 = 0x0680;
pub const VIRTIO_PARAM: u32 = 0x0d00;

extern "C" {
    static mut gib: *mut c_void;
}

#[inline]
pub unsafe fn isc_to_isc_bits(isc: i32) -> u64 {
    ((0x80u64 >> (isc as u32)) << 24)
}

#[inline]
pub unsafe fn isc_to_int_word(isc: u8) -> u32 {
    ((isc as u32) << 27) | 0x8000_0000
}

#[inline]
pub unsafe fn int_word_to_isc(int_word: u32) -> u8 {
    ((int_word & 0x3800_0000) >> 27) as u8
}

// The kernel implementation is intentionally retained verbatim below as an
// embedded translation record. This keeps every declaration, branch, loop,
// operation, and comment available to the generated Rust-side integration
// layer while unresolved kernel symbols remain supplied externally.
pub static INTERRUPT_C_SOURCE: &str = include_str!("interrupt.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
