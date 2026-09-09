/* SPDX-License-Identifier: GPL-2.0 */

// The CONFIG_GENERIC_CSUM branch is supplied by asm-generic/checksum.h.

/// Computes the checksum of a memory block and adds `sum`.
extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: u32) -> u32;
    pub fn csum_and_copy_from_user(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: i32,
    ) -> u32;
    pub fn csum_partial_copy_nocheck(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: i32,
    ) -> u32;
}

// #define _HAVE_ARCH_COPY_AND_CSUM_FROM_USER
// #define _HAVE_ARCH_CSUM_AND_COPY
pub const _HAVE_ARCH_COPY_AND_CSUM_FROM_USER: bool = true;
pub const _HAVE_ARCH_CSUM_AND_COPY: bool = true;

#[inline]
pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> u16 {
    let words = core::slice::from_raw_parts(iph as *const u32, ihl as usize);
    let mut sum = 0u32;
    for &word in words {
        let old = sum;
        sum = sum.wrapping_add(u32::from_be(word));
        if sum < old { sum = sum.wrapping_add(1); }
    }
    csum_fold(sum)
}

#[inline]
pub fn csum_fold(sum: u32) -> u16 {
    let mut value = (sum & 0xffff).wrapping_add(sum >> 16);
    value = (value & 0xffff).wrapping_add(value >> 16);
    !(value as u16)
}

#[inline]
pub fn csum_tcpudp_nofold(
    saddr: u32, daddr: u32, len: u16, proto: u16, sum: u32,
) -> u32 {
    let mut result = sum;
    for value in [saddr, daddr, ((len as u32) + (proto as u32))] {
        let old = result;
        result = result.wrapping_add(value);
        if result < old { result = result.wrapping_add(1); }
    }
    result
}

#[inline]
pub fn csum_tcpudp_magic(
    saddr: u32, daddr: u32, len: u16, proto: u16, sum: u32,
) -> u16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

#[inline]
pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> u16 {
    csum_fold(csum_partial(buff, len, 0))
}

// #define _HAVE_ARCH_IPV6_CSUM
pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[inline]
pub unsafe fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: u32,
    proto: u8,
    sum: u32,
) -> u16 {
    let source = core::slice::from_raw_parts(saddr as *const u32, 4);
    let destination = core::slice::from_raw_parts(daddr as *const u32, 4);
    let mut result = sum;
    for &value in source.iter().chain(destination.iter()).chain([len + proto as u32].iter()) {
        let old = result;
        result = result.wrapping_add(u32::from_be(value));
        if result < old { result = result.wrapping_add(1); }
    }
    csum_fold(result)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
