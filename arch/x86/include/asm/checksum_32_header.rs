/* SPDX-License-Identifier: GPL-2.0 */

// Translated from x86/include/asm/checksum_32.h.
// The original header includes linux/in6.h and linux/uaccess.h; those symbols
// are supplied by the surrounding translation.

extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: __wsum) -> __wsum;
    pub fn csum_partial_copy_generic(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: i32,
    ) -> __wsum;
    pub fn might_sleep();
    pub fn user_access_begin(ptr: *const core::ffi::c_void, len: i32) -> bool;
    pub fn user_access_end();
}

pub type __wsum = u32;
pub type __sum16 = u16;
pub type __be32 = u32;
pub type __u32 = u32;
pub type __u8 = u8;

pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

#[inline]
pub unsafe fn csum_partial_copy_nocheck(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    csum_partial_copy_generic(src, dst, len)
}

#[inline]
pub unsafe fn csum_and_copy_from_user(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    might_sleep();
    if !user_access_begin(src, len) {
        return 0;
    }
    let ret = csum_partial_copy_generic(src, dst, len);
    user_access_end();
    ret
}

// The inline assembly in the C implementation computes the Internet checksum
// over an IPv4 header. This preserves its operations and carry folding.
#[inline]
pub unsafe fn ip_fast_csum(iph: *const u8, ihl: u32) -> __sum16 {
    let words = (ihl as usize) * 4;
    let mut sum: u64 = 0;
    let mut i = 0usize;
    while i < words {
        let word = u32::from_ne_bytes(*iph.add(i..i + 4).as_ptr().cast::<[u8; 4]>());
        sum = (sum & 0xffff_ffff) + word as u64 + (sum >> 32);
        i += 4;
    }
    while (sum >> 32) != 0 {
        sum = (sum & 0xffff_ffff) + (sum >> 32);
    }
    let mut folded = (sum & 0xffff) + (sum >> 16);
    folded = (folded & 0xffff) + (folded >> 16);
    !(folded as u16)
}

#[inline]
pub fn csum_fold(mut sum: __wsum) -> __sum16 {
    sum = sum.wrapping_add((sum << 16).wrapping_add(0xffff));
    (!(sum >> 16)) as u16
}

#[inline]
pub fn csum_tcpudp_nofold(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    mut sum: __wsum,
) -> __wsum {
    sum = sum.wrapping_add(daddr);
    sum = sum.wrapping_add(saddr);
    sum = sum.wrapping_add((len.wrapping_add(proto as u32)) << 8);
    sum = sum.wrapping_add(0);
    sum
}

#[inline]
pub fn csum_tcpudp_magic(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    sum: __wsum,
) -> __sum16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

#[inline]
pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16 {
    csum_fold(csum_partial(buff, len, 0))
}

// struct in6_addr is supplied by linux/in6.h in the surrounding translation.
#[repr(C)]
pub struct in6_addr {
    pub s6_addr32: [u32; 4],
}

#[inline]
pub unsafe fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: __u32,
    proto: __u8,
    mut sum: __wsum,
) -> __sum16 {
    for i in 0..4 {
        sum = sum.wrapping_add((*saddr).s6_addr32[i]);
        sum = sum.wrapping_add((*daddr).s6_addr32[i]);
    }
    sum = sum.wrapping_add(len.to_be());
    sum = sum.wrapping_add((proto as u32).to_be());
    csum_fold(sum)
}

#[inline]
pub unsafe fn csum_and_copy_to_user(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    might_sleep();
    if !user_access_begin(dst, len) {
        return 0;
    }
    let ret = csum_partial_copy_generic(src, dst, len);
    user_access_end();
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
