// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */

/* C header guard omitted in Rust. */

pub type __u8 = u8;
pub type __u32 = u32;
pub type __be16 = u16;
pub type __be32 = u32;
pub type __wsum = u32;
pub type __sum16 = u16;

/* #define __packed __attribute__((__packed__)) */
/* #define __force */

macro_rules! swap {
    ($a:expr, $b:expr) => {{
        let __tmp = $a;
        $a = $b;
        $b = __tmp;
    }};
}

macro_rules! swap_array {
    ($a:expr, $b:expr) => {{
        let mut __tmp = $a;
        core::ptr::copy_nonoverlapping(($a).as_ptr(), __tmp.as_mut_ptr(), ($a).len());
        core::ptr::copy_nonoverlapping(($b).as_ptr(), ($a).as_mut_ptr(), ($a).len());
        core::ptr::copy_nonoverlapping(__tmp.as_ptr(), ($b).as_mut_ptr(), ($a).len());
    }};
}

unsafe extern "C" {
    fn bpf_ntohs(x: u16) -> u16;
    fn bpf_ntohl(x: u32) -> u32;
    fn bpf_htonl(x: u32) -> u32;
}

#[repr(C)]
pub union in6_addr__bindgen_ty_1 {
    pub u6_addr32: [__be32; 4],
}

#[repr(C)]
pub struct in6_addr {
    pub in6_u: in6_addr__bindgen_ty_1,
}

/* linux/unaligned.h */
#[repr(C, packed)]
struct __get_unaligned_t<T: Copy> {
    x: T,
}

pub unsafe fn get_unaligned<T: Copy>(ptr: *const T) -> T {
    core::ptr::read_unaligned(ptr)
}

pub unsafe fn get_unaligned_be16(p: *const core::ffi::c_void) -> u16 {
    unsafe { bpf_ntohs(get_unaligned::<__be16>(p as *const __be16)) }
}

pub unsafe fn get_unaligned_be32(p: *const core::ffi::c_void) -> u32 {
    unsafe { bpf_ntohl(get_unaligned::<__be32>(p as *const __be32)) }
}

/* lib/checksum.c */
pub fn from64to32(mut x: u64) -> u32 {
    /* add up 32-bit and 32-bit for 32+c bit */
    x = (x & 0xffffffff).wrapping_add(x >> 32);
    /* add up carry.. */
    x = (x & 0xffffffff).wrapping_add(x >> 32);
    x as u32
}

pub fn csum_tcpudp_nofold(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    sum: __wsum,
) -> __wsum {
    let mut s: u64 = sum as u32 as u64;

    s = s.wrapping_add(saddr as u32 as u64);
    s = s.wrapping_add(daddr as u32 as u64);
    #[cfg(target_endian = "big")]
    {
        s = s.wrapping_add(proto as u64 + len as u64);
    }
    #[cfg(not(target_endian = "big"))]
    {
        s = s.wrapping_add((proto as u64 + len as u64) << 8);
    }
    from64to32(s) as __wsum
}

/* asm-generic/checksum.h */
pub fn csum_fold(csum: __wsum) -> __sum16 {
    let mut sum: u32 = csum as u32;

    sum = (sum & 0xffff).wrapping_add(sum >> 16);
    sum = (sum & 0xffff).wrapping_add(sum >> 16);
    !sum as __sum16
}

pub fn csum_tcpudp_magic(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    sum: __wsum,
) -> __sum16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

/* net/ipv6/ip6_checksum.c */
pub unsafe fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: __u32,
    proto: __u8,
    csum: __wsum,
) -> __sum16 {
    let mut carry: i32;
    let ulen: __u32;
    let uproto: __u32;
    let mut sum: __u32 = csum as u32;

    let saddr_words = unsafe { (*saddr).in6_u.u6_addr32 };
    let daddr_words = unsafe { (*daddr).in6_u.u6_addr32 };

    sum = sum.wrapping_add(saddr_words[0] as u32);
    carry = (sum < saddr_words[0] as u32) as i32;
    sum = sum.wrapping_add(carry as u32);

    sum = sum.wrapping_add(saddr_words[1] as u32);
    carry = (sum < saddr_words[1] as u32) as i32;
    sum = sum.wrapping_add(carry as u32);

    sum = sum.wrapping_add(saddr_words[2] as u32);
    carry = (sum < saddr_words[2] as u32) as i32;
    sum = sum.wrapping_add(carry as u32);

    sum = sum.wrapping_add(saddr_words[3] as u32);
    carry = (sum < saddr_words[3] as u32) as i32;
    sum = sum.wrapping_add(carry as u32);

    sum = sum.wrapping_add(daddr_words[0] as u32);
    carry = (sum < daddr_words[0] as u32) as i32;
    sum = sum.wrapping_add(carry as u32);

    sum = sum.wrapping_add(daddr_words[1] as u32);
    carry = (sum < daddr_words[1] as u32) as i32;
    sum = sum.wrapping_add(carry as u32);

    sum = sum.wrapping_add(daddr_words[2] as u32);
    carry = (sum < daddr_words[2] as u32) as i32;
    sum = sum.wrapping_add(carry as u32);

    sum = sum.wrapping_add(daddr_words[3] as u32);
    carry = (sum < daddr_words[3] as u32) as i32;
    sum = sum.wrapping_add(carry as u32);

    ulen = unsafe { bpf_htonl(len as __u32) as u32 };
    sum = sum.wrapping_add(ulen);
    carry = (sum < ulen) as i32;
    sum = sum.wrapping_add(carry as u32);

    uproto = unsafe { bpf_htonl(proto as u32) as u32 };
    sum = sum.wrapping_add(uproto);
    carry = (sum < uproto) as i32;
    sum = sum.wrapping_add(carry as u32);

    csum_fold(sum as __wsum)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
