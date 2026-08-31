/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_uint, c_void};

pub type time_t = core::ffi::c_long;

unsafe extern "C" {
    pub fn rand() -> c_int;
    pub fn srand(seed: c_uint);
    pub fn time(timer: *mut time_t) -> time_t;
}

#[inline]
pub unsafe fn bpf_rand_mask(mask: u64) -> u64 {
    unsafe {
        (((rand() as u32) as u64) | (((rand() as u32) as u64) << 32)) & mask
    }
}

/* Generated from bpf_rand_ux(x, m). */

#[inline]
pub unsafe fn bpf_rand_u8(shift: c_int) -> u64 {
    unsafe { bpf_rand_mask(0xffu64).wrapping_shl(shift as u32) }
}

#[inline]
pub unsafe fn bpf_rand_u16(shift: c_int) -> u64 {
    unsafe { bpf_rand_mask(0xffffu64).wrapping_shl(shift as u32) }
}

#[inline]
pub unsafe fn bpf_rand_u24(shift: c_int) -> u64 {
    unsafe { bpf_rand_mask(0xffffffu64).wrapping_shl(shift as u32) }
}

#[inline]
pub unsafe fn bpf_rand_u32(shift: c_int) -> u64 {
    unsafe { bpf_rand_mask(0xffffffffu64).wrapping_shl(shift as u32) }
}

#[inline]
pub unsafe fn bpf_rand_u40(shift: c_int) -> u64 {
    unsafe { bpf_rand_mask(0xffffffffffu64).wrapping_shl(shift as u32) }
}

#[inline]
pub unsafe fn bpf_rand_u48(shift: c_int) -> u64 {
    unsafe { bpf_rand_mask(0xffffffffffffu64).wrapping_shl(shift as u32) }
}

#[inline]
pub unsafe fn bpf_rand_u56(shift: c_int) -> u64 {
    unsafe { bpf_rand_mask(0xffffffffffffffu64).wrapping_shl(shift as u32) }
}

#[inline]
pub unsafe fn bpf_rand_u64(shift: c_int) -> u64 {
    unsafe { bpf_rand_mask(0xffffffffffffffffu64).wrapping_shl(shift as u32) }
}

#[inline]
pub unsafe fn bpf_semi_rand_init() {
    unsafe {
        srand(time(core::ptr::null_mut::<c_void>() as *mut time_t) as c_uint);
    }
}

#[inline]
pub unsafe fn bpf_semi_rand_get() -> u64 {
    unsafe {
        match rand() % 39 {
            0 => 0x000000ff00000000u64 | bpf_rand_u8(0),
            1 => 0xffffffff00000000u64 | bpf_rand_u16(0),
            2 => 0x00000000ffff0000u64 | bpf_rand_u16(0),
            3 => 0x8000000000000000u64 | bpf_rand_u32(0),
            4 => 0x00000000f0000000u64 | bpf_rand_u32(0),
            5 => 0x0000000100000000u64 | bpf_rand_u24(0),
            6 => 0x800ff00000000000u64 | bpf_rand_u32(0),
            7 => 0x7fffffff00000000u64 | bpf_rand_u32(0),
            8 => 0xffffffffffffff00u64 ^ bpf_rand_u32(24),
            9 => 0xffffffffffffff00u64 | bpf_rand_u8(0),
            10 => 0x0000000010000000u64 | bpf_rand_u32(0),
            11 => 0xf000000000000000u64 | bpf_rand_u8(0),
            12 => 0x0000f00000000000u64 | bpf_rand_u8(8),
            13 => 0x000000000f000000u64 | bpf_rand_u8(16),
            14 => 0x0000000000000f00u64 | bpf_rand_u8(32),
            15 => 0x00fff00000000f00u64 | bpf_rand_u8(48),
            16 => 0x00007fffffffffffu64 ^ bpf_rand_u32(1),
            17 => 0xffff800000000000u64 | bpf_rand_u8(4),
            18 => 0xffff800000000000u64 | bpf_rand_u8(20),
            19 => (0xffffffc000000000u64 + 0x80000u64) | bpf_rand_u32(0),
            20 => (0xffffffc000000000u64 - 0x04000000u64) | bpf_rand_u32(0),
            21 => 0x0000000000000000u64 | bpf_rand_u8(55) | bpf_rand_u32(20),
            22 => 0xffffffffffffffffu64 ^ bpf_rand_u8(3) ^ bpf_rand_u32(40),
            23 => 0x0000000000000000u64 | bpf_rand_u8((bpf_rand_u8(0) % 64) as c_int),
            24 => 0x0000000000000000u64 | bpf_rand_u16((bpf_rand_u8(0) % 64) as c_int),
            25 => 0xffffffffffffffffu64 ^ bpf_rand_u8((bpf_rand_u8(0) % 64) as c_int),
            26 => 0xffffffffffffffffu64 ^ bpf_rand_u40((bpf_rand_u8(0) % 64) as c_int),
            27 => 0x0000800000000000u64,
            28 => 0x8000000000000000u64,
            29 => 0x0000000000000000u64,
            30 => 0xffffffffffffffffu64,
            31 => bpf_rand_u16((bpf_rand_u8(0) % 64) as c_int),
            32 => bpf_rand_u24((bpf_rand_u8(0) % 64) as c_int),
            33 => bpf_rand_u32((bpf_rand_u8(0) % 64) as c_int),
            34 => bpf_rand_u40((bpf_rand_u8(0) % 64) as c_int),
            35 => bpf_rand_u48((bpf_rand_u8(0) % 64) as c_int),
            36 => bpf_rand_u56((bpf_rand_u8(0) % 64) as c_int),
            37 => bpf_rand_u64((bpf_rand_u8(0) % 64) as c_int),
            _ => bpf_rand_u64(0),
        }
    }
}
