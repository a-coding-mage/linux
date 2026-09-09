/* SPDX-License-Identifier: GPL-2.0 */
/* IP checksum routines; translated from arch/arm/include/asm/checksum.h. */

/* Dependencies supplied by the surrounding kernel translation. */

extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: __wsum) -> __wsum;
    pub fn csum_partial_copy_nocheck(src: *const core::ffi::c_void,
                                     dst: *mut core::ffi::c_void, len: i32) -> __wsum;
    pub fn csum_partial_copy_from_user(src: *const core::ffi::c_void,
                                       dst: *mut core::ffi::c_void, len: i32) -> __wsum;
    pub fn access_ok(src: *const core::ffi::c_void, len: i32) -> bool;
    pub fn __csum_ipv6_magic(saddr: *const in6_addr, daddr: *const in6_addr,
                             len: __be32, proto: __be32, sum: __wsum) -> __wsum;
    pub fn htonl(value: u32) -> u32;
}

pub type __wsum = u32;
pub type __sum16 = u16;
pub type __be32 = u32;
pub type __u32 = u32;
pub type __u8 = u8;
pub type u32 = u32;

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

pub const _HAVE_ARCH_COPY_AND_CSUM_FROM_USER: bool = true;
pub const _HAVE_ARCH_CSUM_AND_COPY: bool = true;
pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

pub unsafe fn csum_and_copy_from_user(src: *const core::ffi::c_void,
                                      dst: *mut core::ffi::c_void, len: i32) -> __wsum {
    if !access_ok(src, len) {
        return 0;
    }
    csum_partial_copy_from_user(src, dst, len)
}

pub fn csum_fold(mut sum: __wsum) -> __sum16 {
    sum = sum.wrapping_add(sum.rotate_right(16));
    (!(sum as u32) >> 16) as __sum16
}

pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> __sum16 {
    let mut p = iph as *const u32;
    let mut sum = p.read(); p = p.add(1);
    let mut tmp = p.read(); p = p.add(1);
    let mut carry = (sum as u64) + (tmp as u64);
    sum = carry as u32;
    let mut words = ihl.wrapping_sub(5);
    tmp = p.read(); p = p.add(1);
    carry = (sum as u64) + (tmp as u64) + (carry >> 32);
    sum = carry as u32;
    tmp = p.read(); p = p.add(1);
    carry = (sum as u64) + (tmp as u64) + (carry >> 32);
    sum = carry as u32;
    loop {
        tmp = p.read(); p = p.add(1);
        carry = (sum as u64) + (tmp as u64) + (carry >> 32);
        sum = carry as u32;
        tmp = p.read(); p = p.add(1);
        if (words & 15) != 0 { words = words.wrapping_sub(1); } else { break; }
    }
    carry = (sum as u64) + (tmp as u64) + (carry >> 32);
    sum = carry as u32 + (carry >> 32) as u32;
    csum_fold(sum)
}

pub fn csum_tcpudp_nofold(saddr: __be32, daddr: __be32, len: __u32,
                          proto: __u8, sum: __wsum) -> __wsum {
    let lenprot = len.wrapping_add(proto as u32);
    let second = if sum == 0 { saddr } else { saddr };
    let second = if cfg!(target_endian = "little") { second.rotate_right(8) } else { second };
    let (v, c) = (sum as u64).overflowing_add(daddr as u64);
    let (v, c2) = v.overflowing_add(second as u64);
    let (v, c3) = v.overflowing_add(lenprot as u64);
    (v as u32).wrapping_add((c as u32) + (c2 as u32) + (c3 as u32))
}

pub fn csum_tcpudp_magic(saddr: __be32, daddr: __be32, len: __u32,
                         proto: __u8, sum: __wsum) -> __sum16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16 {
    csum_fold(csum_partial(buff, len, 0))
}

pub unsafe fn csum_ipv6_magic(saddr: *const in6_addr, daddr: *const in6_addr,
                              len: u32, proto: u8, sum: __wsum) -> __sum16 {
    csum_fold(__csum_ipv6_magic(saddr, daddr, htonl(len), htonl(proto as u32), sum))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
