/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from asm/checksum_32.h. */

/* linux/in6.h and linux/uaccess.h are supplied by surrounding dependencies. */

extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: __wsum) -> __wsum;
    pub fn csum_partial_copy_generic(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: i32,
    ) -> __wsum;
    pub fn access_ok(addr: *const core::ffi::c_void, len: i32) -> bool;
}

pub const _HAVE_ARCH_CSUM_AND_COPY: bool = true;

#[inline]
pub unsafe fn csum_partial_copy_nocheck(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    csum_partial_copy_generic(src, dst, len)
}

pub const _HAVE_ARCH_COPY_AND_CSUM_FROM_USER: bool = true;

#[inline]
pub unsafe fn csum_and_copy_from_user(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    if !access_ok(src, len) {
        return 0;
    }
    csum_partial_copy_generic(src, dst, len)
}

/* Fold a partial checksum. */
#[inline]
pub unsafe fn csum_fold(mut sum: __wsum) -> __sum16 {
    let mut dummy: u32;
    core::arch::asm!(
        "swap.w {0}, {1}", "extu.w {0}, {0}", "extu.w {1}, {1}",
        "add {1}, {0}", "swap.w {0}, {1}", "add {1}, {0}", "not {0}, {0}",
        inout(reg) sum, lateout(reg) dummy, options(nostack)
    );
    sum as __sum16
}

/* Optimized checksum for IP headers, which are always on 4-octet boundaries. */
#[inline]
pub unsafe fn ip_fast_csum(mut iph: *const core::ffi::c_void, mut ihl: u32) -> __sum16 {
    let mut sum: __wsum;
    let mut dummy0: u32;
    let mut dummy1: u32;
    core::arch::asm!(
        "mov.l @{1}+, {0}", "mov.l @{1}+, {3}", "add #-2, {2}", "clrt",
        "1: addc {3}, {0}", "movt {4}", "mov.l @{1}+, {3}", "dt {2}",
        "bf/s 1b", "cmp/eq #1, {4}", "addc {3}, {0}", "addc {2}, {0}",
        lateout(reg) sum, inout(reg) iph, inout(reg) ihl,
        lateout(reg) dummy0, lateout(reg) dummy1, options(nostack)
    );
    csum_fold(sum)
}

#[inline]
pub unsafe fn csum_tcpudp_nofold(
    saddr: __be32, daddr: __be32, len: __u32, proto: __u8, mut sum: __wsum,
) -> __wsum {
    /* __LITTLE_ENDIAN__ is a build-time condition from the original header. */
    let mut len_proto: usize = ((proto as usize).wrapping_add(len as usize)) << 8;
    let mut carry: usize;
    core::arch::asm!(
        "clrt", "addc {0}, {1}", "addc {2}, {1}", "addc {3}, {1}",
        "movt {0}", "add {1}, {0}",
        inout(reg) carry, inout(reg) len_proto,
        in(reg) daddr, in(reg) saddr, inout(reg) sum,
        options(nostack)
    );
    sum
}

#[inline]
pub unsafe fn csum_tcpudp_magic(
    saddr: __be32, daddr: __be32, len: __u32, proto: __u8, sum: __wsum,
) -> __sum16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

#[inline]
pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16 {
    csum_fold(csum_partial(buff, len, 0))
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
    let mut dummy: u32;
    core::arch::asm!(
        "clrt", "mov.l @(0,{2}), {1}", "addc {1}, {0}",
        "mov.l @(4,{2}), {1}", "addc {1}, {0}", "mov.l @(8,{2}), {1}",
        "addc {1}, {0}", "mov.l @(12,{2}), {1}", "addc {1}, {0}",
        "mov.l @(0,{3}), {1}", "addc {1}, {0}", "mov.l @(4,{3}), {1}",
        "addc {1}, {0}", "mov.l @(8,{3}), {1}", "addc {1}, {0}",
        "mov.l @(12,{3}), {1}", "addc {1}, {0}", "addc {4}, {0}",
        "addc {5}, {0}", "movt {1}", "add {1}, {0}",
        inout(reg) sum, lateout(reg) dummy, in(reg) saddr, in(reg) daddr,
        in(reg) htonl(len), in(reg) htonl(proto as __u32), options(nostack)
    );
    csum_fold(sum)
}

pub const HAVE_CSUM_COPY_USER: bool = true;

#[inline]
pub unsafe fn csum_and_copy_to_user(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    if !access_ok(dst, len) {
        return 0;
    }
    csum_partial_copy_generic(src, dst, len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
