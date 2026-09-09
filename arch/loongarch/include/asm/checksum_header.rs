/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 ARM Ltd.
 * Copyright (C) 2023 Loongson Technology Corporation Limited
 */

// C header guard: __ASM_CHECKSUM_H
// Dependencies: linux/bitops.h and linux/in6.h

// CONFIG_64BIT conditional section.
#[cfg(CONFIG_64BIT)]
pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

#[cfg(CONFIG_64BIT)]
unsafe extern "C" {
    pub fn csum_ipv6_magic(
        saddr: *const in6_addr,
        daddr: *const in6_addr,
        len: u32,
        proto: u8,
        sum: __wsum,
    ) -> __sum16;
}

#[cfg(CONFIG_64BIT)]
/*
 * turns a 32-bit partial checksum (e.g. from csum_partial) into a
 * 1's complement 16-bit checksum.
 */
#[inline]
pub unsafe fn csum_fold(sum: __wsum) -> __sum16 {
    let tmp: u32 = sum as u32;

    /*
     * swap the two 16-bit halves of sum
     * if there is a carry from adding the two 16-bit halves,
     * it will carry from the lower half into the upper half,
     * giving us the correct sum in the upper half.
     */
    (!(tmp.wrapping_add(tmp.rotate_left(16))) >> 16) as __sum16
}
// #define csum_fold csum_fold

#[cfg(CONFIG_64BIT)]
/*
 * This is a version of ip_compute_csum() optimized for IP headers,
 * which always checksum on 4 octet boundaries.  ihl is the number
 * of 32-bit words and is always >= 5.
 */
#[inline]
pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> __sum16 {
    let mut sum: u64;
    let mut tmp: u128;
    let mut n = ihl as i32; // we want it signed

    tmp = core::ptr::read_unaligned(iph as *const u128);
    let mut ptr = (iph as *const u8).add(16);
    n -= 4;
    tmp = tmp.wrapping_add((tmp >> 64) | (tmp << 64));
    sum = (tmp >> 64) as u64;
    loop {
        sum = sum.wrapping_add(core::ptr::read_unaligned(ptr as *const u32) as u64);
        ptr = ptr.add(4);
        n -= 1;
        if n <= 0 {
            break;
        }
    }

    sum = sum.wrapping_add(sum.rotate_right(32));
    csum_fold((sum >> 32) as __wsum)
}
// #define ip_fast_csum ip_fast_csum

unsafe extern "C" {
    pub fn do_csum(buff: *const u8, len: i32) -> u32;
}
// #define do_csum do_csum

// Declarations supplied by asm-generic/checksum.h remain external dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
