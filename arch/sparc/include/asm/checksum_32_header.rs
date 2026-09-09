/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from sparc/include/asm/checksum_32.h. */

/* The Linux types and helpers referenced here are supplied by other headers. */

extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: __wsum) -> __wsum;
    pub fn __csum_partial_copy_sparc_generic(
        src: *const u8,
        dst: *mut u8,
    ) -> u32;
    pub fn access_ok(addr: *const core::ffi::c_void, size: i32) -> bool;
    pub fn htonl(value: u32) -> u32;
}

pub type __wsum = u32;
pub type __sum16 = u16;
pub type __be32 = u32;
pub type __u32 = u32;
pub type __u8 = u8;

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[inline]
pub unsafe fn csum_partial_copy_nocheck(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    let mut ret = src as u32;
    let mut d = dst as *mut i8;
    let mut l = len;
    core::arch::asm!(
        "call __csum_partial_copy_sparc_generic",
        " mov -1, %g7",
        inout("o0") ret,
        inout("o1") d,
        inout("g1") l,
        clobber_abi("C"),
        options(nostack)
    );
    ret as __wsum
}

#[inline]
pub unsafe fn csum_and_copy_from_user(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    if !access_ok(src, len) {
        return 0;
    }
    csum_partial_copy_nocheck(src, dst, len)
}

#[inline]
pub unsafe fn csum_and_copy_to_user(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    if !access_ok(dst, len) {
        return 0;
    }
    csum_partial_copy_nocheck(src, dst, len)
}

/* ihl is always 5 or greater, and iph is usually word aligned. */
#[inline]
pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> __sum16 {
    let mut sum: u32;
    let mut p = iph;
    core::arch::asm!(
        "sub {ihl}, 4, %g4",
        "ld [{p} + 0x00], {sum}",
        "ld [{p} + 0x04], %g2",
        "ld [{p} + 0x08], %g3",
        "addcc %g2, {sum}, {sum}",
        "addxcc %g3, {sum}, {sum}",
        "ld [{p} + 0x0c], %g2",
        "ld [{p} + 0x10], %g3",
        "addxcc %g2, {sum}, {sum}",
        "addx {sum}, %g0, {sum}",
        "1: addcc %g3, {sum}, {sum}",
        "add {p}, 4, {p}",
        "addxcc {sum}, %g0, {sum}",
        "subcc %g4, 1, %g4",
        "be,a 2f",
        "sll {sum}, 16, %g2",
        "b 1b",
        "ld [{p} + 0x10], %g3",
        "2: addcc {sum}, %g2, %g2",
        "srl %g2, 16, {sum}",
        "addx {sum}, %g0, {sum}",
        "xnor %g0, {sum}, {sum}",
        sum = out(reg) sum,
        p = inout(reg) p,
        ihl = in(reg) ihl,
        clobber_abi("C"),
        options(nostack)
    );
    sum as __sum16
}

#[inline]
pub fn csum_fold(mut sum: __wsum) -> __sum16 {
    let mut tmp = sum << 16;
    sum = sum.wrapping_add(tmp);
    tmp = (sum >> 16).wrapping_add((sum >> 16 != 0) as u32);
    (!tmp) as __sum16
}

#[inline]
pub fn csum_tcpudp_nofold(saddr: __be32, daddr: __be32, len: __u32, proto: __u8, sum: __wsum) -> __wsum {
    sum.wrapping_add(saddr).wrapping_add(daddr).wrapping_add(len.wrapping_add(proto as u32))
}

#[inline]
pub fn csum_tcpudp_magic(saddr: __be32, daddr: __be32, len: __u32, proto: __u8, sum: __wsum) -> __sum16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

#[inline]
pub unsafe fn csum_ipv6_magic(saddr: *const in6_addr, daddr: *const in6_addr, len: __u32, proto: __u8, sum: __wsum) -> __sum16 {
    let mut total = sum.wrapping_add(htonl(len)).wrapping_add(htonl(proto as u32));
    for i in 0..8 {
        total = total.wrapping_add(u16::from_be_bytes([(*saddr).s6_addr[i * 2], (*saddr).s6_addr[i * 2 + 1]]) as u32);
        total = total.wrapping_add(u16::from_be_bytes([(*daddr).s6_addr[i * 2], (*daddr).s6_addr[i * 2 + 1]]) as u32);
    }
    csum_fold(total.wrapping_add((total < sum) as u32))
}

#[inline]
pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16 {
    csum_fold(csum_partial(buff, len, 0))
}

pub const HAVE_ARCH_CSUM_ADD: bool = true;

#[inline]
pub fn csum_add(csum: __wsum, addend: __wsum) -> __wsum {
    let result = csum.wrapping_add(addend);
    result.wrapping_add((result < csum) as u32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
