/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Checksums for x86-64
 * Copyright 2002 by Andi Kleen, SuSE Labs
 * with some code from asm-x86/checksum.h
 */

/// Fold and invert a 32-bit checksum.
#[inline]
pub unsafe fn csum_fold(mut sum: __wsum) -> __sum16 {
    let high = (sum as u32) & 0xffff0000;
    let addend = (sum as u32) << 16;
    let (v, carry) = high.overflowing_add(addend);
    sum = v.wrapping_add(carry as u32) as __wsum;
    ((!((sum as u32))) >> 16) as __sum16
}

/*
 * This is a version of ip_compute_csum() optimized for IP headers,
 * which always checksum on 4 octet boundaries.
 *
 * By Jorge Cwik <jorge@laser.satlink.net>, adapted for linux by
 * Arnt Gulbrandsen.
 */
#[inline]
pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> __sum16 {
    let words = iph as *const u32;
    let mut sum = core::ptr::read_unaligned(words);
    let count = ihl.wrapping_sub(1);
    if count != 0 {
        let mut i = 1u32;
        while i < ihl {
            let (v, carry) = sum.overflowing_add(core::ptr::read_unaligned(words.add(i as usize)));
            sum = v.wrapping_add(carry as u32);
            i = i.wrapping_add(1);
        }
        let (v, carry) = sum.overflowing_add(0);
        sum = v.wrapping_add(carry as u32);
        sum = (sum & 0xffff).wrapping_add(sum >> 16);
        sum = sum.wrapping_add((sum >> 16) & 1);
        sum = !sum;
    }
    sum as __sum16
}

#[inline]
pub unsafe fn csum_tcpudp_nofold(
    saddr: __be32, daddr: __be32, len: __u32, proto: __u8, mut sum: __wsum,
) -> __wsum {
    let values = [daddr as u32, saddr as u32, (len.wrapping_add(proto as u32)) << 8];
    for value in values {
        let (v, carry) = (sum as u32).overflowing_add(value);
        sum = v.wrapping_add(carry as u32) as __wsum;
    }
    sum
}

#[inline]
pub unsafe fn csum_tcpudp_magic(
    saddr: __be32, daddr: __be32, len: __u32, proto: __u8, sum: __wsum,
) -> __sum16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

pub unsafe extern "C" fn csum_partial(
    buff: *const core::ffi::c_void, len: i32, sum: __wsum,
) -> __wsum;

/* Do not call this directly. Use the wrappers below */
pub unsafe extern "C" fn csum_partial_copy_generic(
    src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32,
) -> __wsum;

pub unsafe extern "C" fn csum_and_copy_from_user(
    src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32,
) -> __wsum;
pub unsafe extern "C" fn csum_and_copy_to_user(
    src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32,
) -> __wsum;
pub unsafe extern "C" fn csum_partial_copy_nocheck(
    src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32,
) -> __wsum;

pub unsafe extern "C" fn ip_compute_csum(
    buff: *const core::ffi::c_void, len: i32,
) -> __sum16;

#[inline]
pub fn add32_with_carry(a: u32, b: u32) -> u32 {
    let (v, carry) = a.overflowing_add(b);
    v.wrapping_add(carry as u32)
}

pub const _HAVE_ARCH_IPV6_CSUM: u32 = 1;

#[inline]
pub unsafe fn csum_ipv6_magic(
    _saddr: *const in6_addr, _daddr: *const in6_addr,
    len: __u32, proto: __u8, sum: __wsum,
) -> __sum16 {
    let saddr = _saddr as *const usize;
    let daddr = _daddr as *const usize;
    let mut sum64 = (htonl(len) as u64)
        .wrapping_add(htons(proto as u16) as u64)
        .wrapping_add(sum as u64);
    for p in [saddr, saddr.add(1), daddr, daddr.add(1)] {
        let (v, carry) = sum64.overflowing_add(core::ptr::read_unaligned(p) as u64);
        sum64 = v.wrapping_add(carry as u64);
    }
    csum_fold(add32_with_carry(sum64 as u32, (sum64 >> 32) as u32) as __wsum)
}

pub const HAVE_ARCH_CSUM_ADD: bool = true;

#[inline]
pub fn csum_add(csum: __wsum, addend: __wsum) -> __wsum {
    add32_with_carry(csum as u32, addend as u32) as __wsum
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
