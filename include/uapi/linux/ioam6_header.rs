/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  IPv6 IOAM implementation
 *
 *  Author:
 *  Justin Iurman <justin.iurman@uliege.be>
 */

// <asm/byteorder.h> and <linux/types.h> are supplied by the surrounding
// translation environment.

pub const IOAM6_U16_UNAVAILABLE: u16 = u16::MAX;
pub const IOAM6_U32_UNAVAILABLE: u32 = u32::MAX;
pub const IOAM6_U64_UNAVAILABLE: u64 = u64::MAX;

pub const IOAM6_DEFAULT_ID: u32 = IOAM6_U32_UNAVAILABLE >> 8;
pub const IOAM6_DEFAULT_ID_WIDE: u64 = IOAM6_U64_UNAVAILABLE >> 8;
pub const IOAM6_DEFAULT_IF_ID: u16 = IOAM6_U16_UNAVAILABLE;
pub const IOAM6_DEFAULT_IF_ID_WIDE: u32 = IOAM6_U32_UNAVAILABLE;

/* IPv6 IOAM Option Header */
#[repr(C, packed)]
pub struct ioam6_hdr {
    pub opt_type: u8,
    pub opt_len: u8,
    pub reserved: u8,
    pub type_: u8,
}

pub const IOAM6_TYPE_PREALLOC: u8 = 0;

/* IOAM Trace Header */
#[repr(C, packed)]
pub struct ioam6_trace_hdr {
    pub namespace_id: u16, // __be16
    // Bitfield ordering follows __LITTLE_ENDIAN_BITFIELD or
    // __BIG_ENDIAN_BITFIELD from <asm/byteorder.h>.
    pub overflow_nodelen: u8,
    pub remlen: u8,
    pub type_: ioam6_trace_type,
    pub data: [u8; 0],
}

#[repr(C)]
pub union ioam6_trace_type {
    pub type_be32: u32, // __be32
    pub type_bits: ioam6_trace_type_bits,
}

/*
 * C bitfields are retained as their underlying 32-bit storage.  On little
 * endian targets the source declares bits 7..0, 15..8, 23..16 and reserves
 * bits 31..24; on big endian targets it declares bits 0..23 in order and
 * reserves bits 31..24.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioam6_trace_type_bits {
    pub bits: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
