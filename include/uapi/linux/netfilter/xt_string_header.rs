/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The C header includes <linux/types.h>; the corresponding fixed-width
// integer types are represented directly by Rust primitive types below.

pub const XT_STRING_MAX_PATTERN_SIZE: usize = 128;
pub const XT_STRING_MAX_ALGO_NAME_SIZE: usize = 16;

pub const XT_STRING_FLAG_INVERT: u32 = 0x01;
pub const XT_STRING_FLAG_IGNORECASE: u32 = 0x02;

// Opaque type supplied by the kernel's other headers.
pub struct ts_config;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_string_info_v0 {
    pub invert: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_string_info_v1 {
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xt_string_info_u {
    pub v0: xt_string_info_v0,
    pub v1: xt_string_info_v1,
}

#[repr(C)]
pub struct xt_string_info {
    pub from_offset: u16,
    pub to_offset: u16,
    pub algo: [u8; XT_STRING_MAX_ALGO_NAME_SIZE],
    pub pattern: [u8; XT_STRING_MAX_PATTERN_SIZE],
    pub patlen: u8,
    pub u: xt_string_info_u,

    /* Used internally by the kernel */
    // The C declaration requests 8-byte alignment for this pointer; raw
    // pointers have this alignment on the target ABI.
    pub config: *mut ts_config,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
