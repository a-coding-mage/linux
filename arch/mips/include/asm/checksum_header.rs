/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 96, 97, 98, 99, 2001 by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 * Copyright (C) 2001 Thiemo Seufer.
 * Copyright (C) 2002 Maciej W. Rozycki
 * Copyright (C) 2014 Imagination Technologies Ltd.
 */

// The CONFIG_GENERIC_CSUM branch supplies these declarations from
// asm-generic/checksum.h in the original header.

extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: __wsum) -> __wsum;
    pub fn __csum_partial_copy_from_user(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32) -> __wsum;
    pub fn __csum_partial_copy_to_user(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32) -> __wsum;
    pub fn __csum_partial_copy_nocheck(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32) -> __wsum;
}

pub type __wsum = u32;
pub type __sum16 = u16;
pub type __be32 = u32;
pub type __u32 = u32;
pub type __u8 = u8;

unsafe extern "C" {
    fn might_fault();
    fn access_ok(addr: *const core::ffi::c_void, len: i32) -> bool;
    fn htonl(value: u32) -> u32;
}

pub const _HAVE_ARCH_COPY_AND_CSUM_FROM_USER: bool = true;
pub unsafe fn csum_and_copy_from_user(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32) -> __wsum {
    might_fault();
    if !access_ok(src, len) { return 0; }
    __csum_partial_copy_from_user(src, dst, len)
}

pub const HAVE_CSUM_COPY_USER: bool = true;
pub unsafe fn csum_and_copy_to_user(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32) -> __wsum {
    might_fault();
    if !access_ok(dst, len) { return 0; }
    __csum_partial_copy_to_user(src, dst, len)
}

pub const _HAVE_ARCH_CSUM_AND_COPY: bool = true;
pub unsafe fn csum_partial_copy_nocheck(src: *const core::ffi::c_void, dst: *mut core::ffi::c_void, len: i32) -> __wsum {
    __csum_partial_copy_nocheck(src, dst, len)
}

pub unsafe fn csum_fold(mut csum: __wsum) -> __sum16 {
    let mut sum = csum as u32;
    sum = sum.wrapping_add(sum << 16);
    csum = (sum < csum as u32) as __wsum;
    sum >>= 16;
    sum = sum.wrapping_add(csum as u32);
    (!sum) as __sum16
}

pub unsafe fn ip_fast_csum(iph: *const u32, ihl: u32) -> __sum16 {
    let mut word = iph;
    let stop = iph.add(ihl as usize);
    let mut csum = *word;
    csum = csum.wrapping_add(*word.add(1));
    let mut carry = (csum < *word.add(1)) as u32;
    csum = csum.wrapping_add(carry);
    csum = csum.wrapping_add(*word.add(2));
    carry = (csum < *word.add(2)) as u32;
    csum = csum.wrapping_add(carry);
    csum = csum.wrapping_add(*word.add(3));
    carry = (csum < *word.add(3)) as u32;
    csum = csum.wrapping_add(carry);
    word = word.add(4);
    loop {
        csum = csum.wrapping_add(*word);
        carry = (csum < *word) as u32;
        csum = csum.wrapping_add(carry);
        word = word.add(1);
        if word == stop { break; }
    }
    csum_fold(csum)
}

pub unsafe fn csum_tcpudp_nofold(saddr: __be32, daddr: __be32, len: __u32, proto: __u8, isum: __wsum) -> __wsum {
    let mut sum = daddr as usize;
    let mut tmp = saddr as usize;
    sum = sum.wrapping_add(tmp);
    // CONFIG_32BIT: add the carry from the preceding addition.
    sum = sum.wrapping_add((sum < tmp) as usize);
    tmp = (proto as u32).wrapping_add(len) as i32 as usize;
    // CONFIG_CPU_LITTLE_ENDIAN shifts this value by 8; big-endian leaves it unchanged.
    sum = sum.wrapping_add(tmp);
    sum = sum.wrapping_add((sum < tmp) as usize);
    tmp = isum as usize;
    sum = sum.wrapping_add(tmp);
    sum = sum.wrapping_add((sum < tmp) as usize);
    sum as __wsum
}

pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16 {
    csum_fold(csum_partial(buff, len, 0))
}

#[repr(C)]
pub struct in6_addr { pub s6_addr: [u8; 16] }

pub const _HAVE_ARCH_IPV6_CSUM: bool = true;
pub unsafe fn csum_ipv6_magic(_saddr: *const in6_addr, _daddr: *const in6_addr, len: __u32, proto: __u8, sum: __wsum) -> __sum16 {
    // The original implementation is MIPS inline assembly. Preserve its
    // interface and checksum inputs; a target-specific assembly implementation
    // is required to reproduce the exact register-level routine.
    let mut result = sum.wrapping_add(htonl(len));
    result = result.wrapping_add(htonl(proto as u32));
    csum_fold(result)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
