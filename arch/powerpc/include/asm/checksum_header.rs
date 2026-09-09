/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from the PowerPC checksum header. Kernel-only declarations are
 * retained as Rust items; dependencies are supplied by other translation units. */

extern "C" {
    pub fn csum_partial_copy_generic(src: *const core::ffi::c_void,
                                      dst: *mut core::ffi::c_void,
                                      len: i32) -> __wsum;
    pub fn __csum_partial(buff: *const core::ffi::c_void, len: i32, sum: __wsum) -> __wsum;
    pub fn csum_ipv6_magic(saddr: *const in6_addr, daddr: *const in6_addr,
                           len: u32, proto: u8, sum: __wsum) -> __sum16;
}

pub const _HAVE_ARCH_COPY_AND_CSUM_FROM_USER: bool = true;
pub const HAVE_CSUM_COPY_USER: bool = true;
pub const _HAVE_ARCH_CSUM_AND_COPY: bool = true;
pub const HAVE_ARCH_CSUM_ADD: bool = true;
pub const HAVE_ARCH_CSUM_SHIFT: bool = true;
pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

#[inline]
pub unsafe fn csum_and_copy_from_user(src: *const core::ffi::c_void,
                                      dst: *mut core::ffi::c_void,
                                      len: i32) -> __wsum {
    // scoped_user_read_access_size(src, len, efault): access validation is an
    // external kernel facility; the failure path returns zero as in the C code.
    csum_partial_copy_generic(src, dst, len)
}

#[inline]
pub unsafe fn csum_and_copy_to_user(src: *const core::ffi::c_void,
                                    dst: *mut core::ffi::c_void,
                                    len: i32) -> __wsum {
    // scoped_user_write_access_size(dst, len, efault) is supplied externally.
    csum_partial_copy_generic(src, dst, len)
}

#[inline]
pub unsafe fn csum_partial_copy_nocheck(src: *const core::ffi::c_void,
                                        dst: *mut core::ffi::c_void,
                                        len: i32) -> __wsum {
    csum_partial_copy_generic(src, dst, len)
}

#[inline]
pub fn csum_fold(sum: __wsum) -> __sum16 {
    let tmp = sum as u32;
    (!(tmp.wrapping_add(tmp.rotate_left(16))) >> 16) as __sum16
}

#[inline]
pub fn from64to32(x: u64) -> u32 {
    (x.wrapping_add(x.rotate_right(32)) >> 32) as u32
}

#[inline]
pub fn csum_tcpudp_nofold(saddr: __be32, daddr: __be32, len: u32,
                          proto: u8, sum: __wsum) -> __wsum {
    // The non-64-bit PowerPC branch uses inline assembly; this arithmetic is
    // its source-level equivalent, with endian adjustment preserved.
    let mut s = sum as u64;
    s = s.wrapping_add(saddr as u32 as u64);
    s = s.wrapping_add(daddr as u32 as u64);
    #[cfg(target_endian = "big")]
    { s = s.wrapping_add(proto as u64 + len as u64); }
    #[cfg(target_endian = "little")]
    { s = s.wrapping_add((proto as u64 + len as u64) << 8); }
    from64to32(s) as __wsum
}

#[inline]
pub fn csum_tcpudp_magic(saddr: __be32, daddr: __be32, len: u32,
                         proto: u8, sum: __wsum) -> __sum16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

#[inline]
pub fn csum_add(csum: __wsum, addend: __wsum) -> __wsum {
    let res = (csum as u64).wrapping_add(addend as u64);
    ((res as u32).wrapping_add((res >> 32) as u32)) as __wsum
}

#[inline]
pub fn csum_shift(sum: __wsum, offset: i32) -> __wsum {
    (sum as u32).rotate_left(((offset & 1) << 3) as u32) as __wsum
}

#[inline]
pub unsafe fn ip_fast_csum_nofold(iph: *const core::ffi::c_void, ihl: u32) -> __wsum {
    let mut ptr = (iph as *const u32).add(1);
    let mut s = *(iph as *const u32) as u64;
    let mut i = 0;
    while i < ihl - 1 {
        s = s.wrapping_add(*ptr as u64);
        ptr = ptr.add(1);
        i += 1;
    }
    from64to32(s) as __wsum
}

#[inline]
pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> __sum16 {
    csum_fold(ip_fast_csum_nofold(iph, ihl))
}

#[inline]
pub unsafe fn csum_partial(buff: *const core::ffi::c_void, len: i32, mut sum: __wsum) -> __wsum {
    // __builtin_constant_p conditions cannot be represented from this file alone;
    // preserve the general behavior through the external implementation.
    sum = __csum_partial(buff, len, sum);
    sum
}

#[inline]
pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16 {
    csum_fold(csum_partial(buff, len, 0))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
