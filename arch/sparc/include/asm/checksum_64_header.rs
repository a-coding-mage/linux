/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of sparc/include/asm/checksum_64.h. */

pub type __u8 = u8;
pub type __u32 = u32;
pub type __be32 = u32;
pub type __wsum = u32;
pub type __sum16 = u16;

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: __wsum) -> __wsum;
    pub fn csum_partial_copy_nocheck(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: i32,
    ) -> __wsum;
    pub fn csum_and_copy_from_user(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: i32,
    ) -> __wsum;
    pub fn csum_and_copy_to_user(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: i32,
    ) -> __wsum;
    pub fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> __sum16;
}

#[inline]
pub fn csum_fold(mut sum: __wsum) -> __sum16 {
    let mut tmp = sum << 16;
    let (v, carry) = sum.overflowing_add(tmp);
    sum = v;
    tmp = (sum >> 16).wrapping_add(carry as u32);
    (!tmp) as __sum16
}

#[inline]
pub fn csum_tcpudp_nofold(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    mut sum: __wsum,
) -> __wsum {
    let (v, c1) = sum.overflowing_add(saddr);
    sum = v;
    let (v, c2) = sum.overflowing_add(daddr);
    sum = v;
    let (v, c3) = sum.overflowing_add((proto as u32).wrapping_add(len));
    sum = v;
    sum.wrapping_add((c1 as u32) + (c2 as u32) + (c3 as u32))
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

pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

#[inline]
pub unsafe fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: __u32,
    proto: __u8,
    mut sum: __wsum,
) -> __sum16 {
    let add_word = |acc: &mut u32, word: u32| {
        let (v, carry) = acc.overflowing_add(word);
        *acc = v.wrapping_add(carry as u32);
    };
    let mut i = 0usize;
    while i < 16 {
        let sw = u32::from_be_bytes((*saddr).s6_addr[i..i + 4].try_into().unwrap());
        let dw = u32::from_be_bytes((*daddr).s6_addr[i..i + 4].try_into().unwrap());
        add_word(&mut sum, sw);
        add_word(&mut sum, dw);
        i += 4;
    }
    add_word(&mut sum, len.to_be());
    add_word(&mut sum, (proto as u32).to_be());
    csum_fold(sum)
}

#[inline]
pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16 {
    csum_fold(csum_partial(buff, len, 0))
}

pub const HAVE_ARCH_CSUM_ADD: bool = true;

#[inline]
pub fn csum_add(mut csum: __wsum, addend: __wsum) -> __wsum {
    let (v, carry) = csum.overflowing_add(addend);
    csum = v;
    csum.wrapping_add(carry as u32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
