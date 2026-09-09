// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * INET An implementation of the TCP/IP protocol suite for the LINUX
 * operating system. IP/TCP/UDP checksumming routines.
 *
 * The original implementation uses m68k inline assembly.  The routines below
 * retain its big-endian word/long traversal, one's-complement carry handling,
 * and raw pointer interfaces.
 */

pub type __wsum = u32;

#[inline(always)]
unsafe fn add_with_carry(sum: &mut u32, value: u32) {
    let (v, carry) = sum.overflowing_add(value);
    let (v, carry2) = v.overflowing_add(carry as u32);
    *sum = v.wrapping_add(carry2 as u32);
}

unsafe fn checksum_bytes(mut src: *const u8, mut dst: *mut u8, mut len: usize, mut sum: u32) -> u32 {
    if (src as usize & 2) != 0 && len >= 2 {
        let word = ((*src as u32) << 8) | *src.add(1) as u32;
        add_with_carry(&mut sum, word << 16);
        if !dst.is_null() {
            *dst = *src;
            *dst.add(1) = *src.add(1);
            dst = dst.add(2);
        }
        src = src.add(2);
        len -= 2;
    }

    while len >= 4 {
        let word = ((*src as u32) << 24)
            | (*src.add(1) as u32) << 16
            | (*src.add(2) as u32) << 8
            | *src.add(3) as u32;
        add_with_carry(&mut sum, word);
        if !dst.is_null() {
            *dst = *src;
            *dst.add(1) = *src.add(1);
            *dst.add(2) = *src.add(2);
            *dst.add(3) = *src.add(3);
            dst = dst.add(4);
        }
        src = src.add(4);
        len -= 4;
    }

    if len >= 2 {
        let word = ((*src as u32) << 8) | *src.add(1) as u32;
        add_with_carry(&mut sum, word << 16);
        if !dst.is_null() {
            *dst = *src;
            *dst.add(1) = *src.add(1);
            dst = dst.add(2);
        }
        src = src.add(2);
        len -= 2;
    }
    if len != 0 {
        add_with_carry(&mut sum, (*src as u32) << 24);
        if !dst.is_null() {
            *dst = *src;
        }
    }
    sum
}

/* computes a partial checksum, e.g. for TCP/UDP fragments */
pub unsafe fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: __wsum) -> __wsum {
    checksum_bytes(buff as *const u8, core::ptr::null_mut(), len as usize, sum)
}

/* copy from user space while checksumming, with exception handling. */
pub unsafe fn csum_and_copy_from_user(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    // The m68k exception table makes a fault return zero. Rust raw-pointer
    // access has no equivalent local exception mechanism.
    checksum_bytes(src as *const u8, dst as *mut u8, len as usize, !0u32)
}

/* copy from kernel space while checksumming, otherwise like csum_partial */
pub unsafe fn csum_partial_copy_nocheck(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    checksum_bytes(src as *const u8, dst as *mut u8, len as usize, 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
