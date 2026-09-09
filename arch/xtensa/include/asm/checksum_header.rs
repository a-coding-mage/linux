/*
 * include/asm-xtensa/checksum.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// C dependencies: linux/in6.h, linux/uaccess.h, and asm/core.h.

extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: __wsum) -> __wsum;
    pub fn csum_partial_copy_generic(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32) -> __wsum;
    pub fn access_ok(addr: *const core::ffi::c_void, len: i32) -> bool;
    pub fn htonl(value: u32) -> u32;
}

pub type __wsum = u32;
pub type __sum16 = u16;
pub type __be32 = u32;
pub type __u32 = u32;
pub type __u8 = u8;

#[repr(C)]
pub struct in6_addr {
    pub in6_u: [u32; 4],
}

pub const _HAVE_ARCH_CSUM_AND_COPY: bool = true;

pub unsafe fn csum_partial_copy_nocheck(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32) -> __wsum {
    csum_partial_copy_generic(src, dst, len)
}

pub const _HAVE_ARCH_COPY_AND_CSUM_FROM_USER: bool = true;

pub unsafe fn csum_and_copy_from_user(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32) -> __wsum {
    if !access_ok(src, len) { return 0; }
    csum_partial_copy_generic(src, dst, len)
}

/* Fold a partial checksum. */
pub unsafe fn csum_fold(mut sum: __wsum) -> __sum16 {
    let mut dummy: u32;
    core::arch::asm!(
        "extui {dummy}, {sum}, 16, 16",
        "extui {sum}, {sum}, 0, 16",
        "add {sum}, {sum}, {dummy}",
        "slli {dummy}, {sum}, 16",
        "add {sum}, {sum}, {dummy}",
        "extui {sum}, {sum}, 16, 16",
        "neg {sum}, {sum}",
        "addi {sum}, {sum}, -1",
        "extui {sum}, {sum}, 0, 16",
        sum = inout(reg) sum,
        dummy = lateout(reg) dummy,
    );
    sum as __sum16
}

/* This is a version of ip_compute_csum() optimized for IP headers. */
pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> __sum16 {
    let mut sum: u32;
    let mut ptr = iph;
    let mut count = ihl;
    while count != 0 {
        let word = core::ptr::read_unaligned(ptr as *const u32);
        let old = sum;
        sum = sum.wrapping_add(word);
        if sum < old { sum = sum.wrapping_add(1); }
        ptr = ptr.add(4);
        count -= 1;
    }
    csum_fold(sum)
}

pub unsafe fn csum_tcpudp_nofold(saddr: __be32, daddr: __be32, len: __u32, proto: __u8, mut sum: __wsum) -> __wsum {
    // __XTENSA_EL__ uses (len + proto) << 8; __XTENSA_EB__ uses len + proto.
    let len_proto: u32 = (len.wrapping_add(proto as u32)) << 8;
    for value in [len_proto, daddr, saddr] {
        let old = sum;
        sum = sum.wrapping_add(value);
        if sum < old { sum = sum.wrapping_add(1); }
    }
    sum
}

/* Computes the checksum of the TCP/UDP pseudo-header. */
pub unsafe fn csum_tcpudp_magic(saddr: __be32, daddr: __be32, len: __u32, proto: __u8, sum: __wsum) -> __sum16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

/* This routine is used for miscellaneous IP-like checksums. */
pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16 {
    csum_fold(csum_partial(buff, len, 0))
}

pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

pub unsafe fn csum_ipv6_magic(saddr: *const in6_addr, daddr: *const in6_addr, len: __u32, proto: __u8, mut sum: __wsum) -> __sum16 {
    for i in 0..4 {
        for addr in [saddr, daddr] {
            let value = core::ptr::read_unaligned((addr as *const u32).add(i));
            let old = sum;
            sum = sum.wrapping_add(value);
            if sum < old { sum = sum.wrapping_add(1); }
        }
    }
    for value in [htonl(len), htonl(proto as u32)] {
        let old = sum;
        sum = sum.wrapping_add(value);
        if sum < old { sum = sum.wrapping_add(1); }
    }
    csum_fold(sum)
}

/* Copy and checksum to user. */
pub const HAVE_CSUM_COPY_USER: bool = true;

pub unsafe fn csum_and_copy_to_user(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32) -> __wsum {
    if !access_ok(dst, len) { return 0; }
    csum_partial_copy_generic(src, dst, len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
