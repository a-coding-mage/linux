/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Copyright (c) 2020, Mellanox Technologies inc. All rights reserved. */

// The C header depends on Linux kernel types and bitfield helpers.  The
// corresponding Rust declarations intentionally retain those external names
// and low-level pointer semantics.

#[inline]
pub unsafe fn _iba_get8(ptr: *const u8) -> u32 {
    *ptr as u32
}

#[inline]
pub unsafe fn _iba_set8(ptr: *mut u8, mask: u32, prep_value: u32) {
    *ptr = ((*ptr as u32 & !mask) | prep_value) as u8;
}

#[inline]
pub unsafe fn _iba_get16(ptr: *const u16) -> u16 {
    u16::from_be(*ptr)
}

#[inline]
pub unsafe fn _iba_set16(ptr: *mut u16, mask: u16, prep_value: u16) {
    *ptr = (((u16::from_be(*ptr) & !mask) | prep_value)).to_be();
}

#[inline]
pub unsafe fn _iba_get32(ptr: *const u32) -> u32 {
    u32::from_be(*ptr)
}

#[inline]
pub unsafe fn _iba_set32(ptr: *mut u32, mask: u32, prep_value: u32) {
    *ptr = (u32::from_be(*ptr) & !mask | prep_value).to_be();
}

#[inline]
pub unsafe fn _iba_get64(ptr: *const u64) -> u64 {
    // The mads are constructed so that 32 bit and smaller are naturally
    // aligned, everything larger has a max alignment of 4 bytes.
    u64::from_be(core::ptr::read_unaligned(ptr))
}

#[inline]
pub unsafe fn _iba_set64(ptr: *mut u64, mask: u64, prep_value: u64) {
    core::ptr::write_unaligned(
        ptr,
        ((_iba_get64(ptr as *const u64) & !mask) | prep_value).to_be(),
    );
}

#[inline]
pub unsafe fn iba_field_prep(mask: u64, value: u64) -> u64 {
    (value << mask.trailing_zeros()) & mask
}

#[inline]
pub unsafe fn iba_field_get(mask: u64, value: u64) -> u64 {
    (value & mask) >> mask.trailing_zeros()
}

#[inline]
pub unsafe fn _iba_get_mem_ptr<T>(ptr: *mut u8, field_offset: usize) -> *mut T {
    ptr.add(field_offset) as *mut T
}

#[inline]
pub unsafe fn _iba_set_mem<T>(
    ptr: *mut u8,
    field_offset: usize,
    input: *const T,
    bytes: usize,
    num_bits: usize,
) {
    // FIXME: A set should always set the entire field, meaning we should zero the trailing bytes.
    if bytes * 8 > num_bits {
        debug_assert!(false);
    }
    if !input.is_null() && bytes != 0 {
        core::ptr::copy_nonoverlapping(input as *const u8, ptr.add(field_offset), bytes);
    }
}

#[inline]
pub unsafe fn _iba_get_mem<T>(
    ptr: *const u8,
    field_offset: usize,
    output: *mut T,
    bytes: usize,
    num_bits: usize,
) {
    if bytes * 8 > num_bits {
        debug_assert!(false);
    }
    if !output.is_null() && bytes != 0 {
        core::ptr::copy_nonoverlapping(ptr.add(field_offset), output as *mut u8, bytes);
    }
}

// Rust equivalents of the generated field-description macros.
#[macro_export]
macro_rules! IBA_FIELD_BLOC {
    ($field_struct:ty, $byte_offset:expr, $bit_offset:expr, $num_bits:expr) => {
        ($field_struct, $byte_offset, (0xffu8 >> (8 - $num_bits)) << (7 - $bit_offset - ($num_bits - 1)), 8)
    };
}

#[macro_export]
macro_rules! IBA_FIELD8_LOC {
    ($field_struct:ty, $byte_offset:expr, $num_bits:expr) => {
        IBA_FIELD_BLOC!($field_struct, $byte_offset, 0, $num_bits)
    };
}

#[macro_export]
macro_rules! IBA_FIELD16_LOC {
    ($field_struct:ty, $byte_offset:expr, $num_bits:expr) => {
        ($field_struct, ($byte_offset) & 0xfffe, 0u16, 16)
    };
}

#[macro_export]
macro_rules! IBA_FIELD32_LOC {
    ($field_struct:ty, $byte_offset:expr, $num_bits:expr) => {
        ($field_struct, ($byte_offset) & 0xfffc, 0u32, 32)
    };
}

#[macro_export]
macro_rules! IBA_FIELD64_LOC {
    ($field_struct:ty, $byte_offset:expr) => {
        ($field_struct, $byte_offset, u64::MAX, 64)
    };
}

#[macro_export]
macro_rules! IBA_FIELD_MLOC {
    ($field_struct:ty, $byte_offset:expr, $num_bits:expr, $field_type:ty) => {
        ($field_struct, $byte_offset, $field_type, $num_bits)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
