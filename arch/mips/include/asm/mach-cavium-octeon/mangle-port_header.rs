/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003, 2004 Ralf Baechle
 */

// Dependency supplied by asm/byteorder.h in the C source.

#[cfg(target_endian = "big")]
#[inline]
pub unsafe fn __should_swizzle_bits(a: *mut core::ffi::c_void) -> bool {
    unsafe extern "C" {
        static octeon_should_swizzle_table: [bool; 256];
    }
    let did = (((a as usize as u64) >> 40) & 0xff) as usize;
    unsafe { octeon_should_swizzle_table[did] }
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn __swizzle_addr_b(port: u64) -> u64 { port }

#[cfg(target_endian = "big")]
#[inline]
pub const fn __swizzle_addr_w(port: u64) -> u64 { port }

#[cfg(target_endian = "big")]
#[inline]
pub const fn __swizzle_addr_l(port: u64) -> u64 { port }

#[cfg(target_endian = "big")]
#[inline]
pub const fn __swizzle_addr_q(port: u64) -> u64 { port }

#[cfg(target_endian = "little")]
#[inline]
pub const fn __should_swizzle_bits(_a: *mut core::ffi::c_void) -> bool { false }

#[cfg(target_endian = "little")]
#[inline]
pub const fn __should_swizzle_addr(p: u64) -> bool {
    /* boot bus? */
    ((p >> 40) & 0xff) == 0
}

#[cfg(target_endian = "little")]
#[inline]
pub fn __swizzle_addr_b(port: u64) -> u64 {
    if __should_swizzle_addr(port) { port ^ 7 } else { port }
}

#[cfg(target_endian = "little")]
#[inline]
pub fn __swizzle_addr_w(port: u64) -> u64 {
    if __should_swizzle_addr(port) { port ^ 6 } else { port }
}

#[cfg(target_endian = "little")]
#[inline]
pub fn __swizzle_addr_l(port: u64) -> u64 {
    if __should_swizzle_addr(port) { port ^ 4 } else { port }
}

#[inline]
pub fn __swizzle_addr_q(port: u64) -> u64 { port }

#[inline]
pub const fn ioswabb<T>(_: *mut core::ffi::c_void, x: T) -> T { x }

#[inline]
pub const fn __mem_ioswabb<T>(_: *mut core::ffi::c_void, x: T) -> T { x }

#[inline]
pub unsafe fn ioswabw(a: *mut core::ffi::c_void, x: u16) -> u16 {
    if unsafe { __should_swizzle_bits(a) } { x.to_le() } else { x }
}

#[inline]
pub const fn __mem_ioswabw(x: u16) -> u16 { x }

#[inline]
pub unsafe fn ioswabl(a: *mut core::ffi::c_void, x: u32) -> u32 {
    if unsafe { __should_swizzle_bits(a) } { x.to_le() } else { x }
}

#[inline]
pub const fn __mem_ioswabl(x: u32) -> u32 { x }

#[inline]
pub unsafe fn ioswabq(a: *mut core::ffi::c_void, x: u64) -> u64 {
    if unsafe { __should_swizzle_bits(a) } { x.to_le() } else { x }
}

#[inline]
pub const fn __mem_ioswabq(x: u64) -> u64 { x }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
