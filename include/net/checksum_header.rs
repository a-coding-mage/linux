/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Checksumming functions for IP, TCP, UDP and so on. */

// C header dependencies: linux/errno.h, asm/types.h, asm/byteorder.h,
// asm/checksum.h, and linux/uaccess.h where the architecture does not provide
// the corresponding operations.

pub type __wsum = u32;
pub type __sum16 = u16;
pub type __be16 = u16;
pub type __be32 = u32;

extern "C" {
    pub fn copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    pub fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    pub fn csum_partial(buff: *const core::ffi::c_void, len: usize, sum: __wsum) -> __wsum;
    pub fn csum_fold(sum: __wsum) -> __sum16;
    pub fn inet_proto_csum_replace4(sum: *mut __sum16, skb: *mut sk_buff,
                                    from: __be32, to: __be32, pseudohdr: bool);
    pub fn inet_proto_csum_replace16(sum: *mut __sum16, skb: *mut sk_buff,
                                     from: *const __be32, to: *const __be32,
                                     pseudohdr: bool);
    pub fn inet_proto_csum_replace_by_diff(sum: *mut __sum16, skb: *mut sk_buff,
                                           diff: __wsum, pseudohdr: bool,
                                           ipv6: bool);
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[inline(always)]
pub unsafe fn csum_and_copy_from_user(src: *const core::ffi::c_void,
                                      dst: *mut core::ffi::c_void, len: i32) -> __wsum {
    if copy_from_user(dst, src, len as usize) != 0 { return 0; }
    csum_partial(dst, len as usize, !0u32)
}

#[inline(always)]
pub unsafe fn csum_and_copy_to_user(src: *const core::ffi::c_void,
                                    dst: *mut core::ffi::c_void, len: i32) -> __wsum {
    let sum = csum_partial(src, len as usize, !0u32);
    if copy_to_user(dst, src, len as usize) == 0 { sum } else { 0 }
}

#[inline(always)]
pub unsafe fn csum_partial_copy_nocheck(src: *const core::ffi::c_void,
                                        dst: *mut core::ffi::c_void, len: i32) -> __wsum {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len as usize);
    csum_partial(dst, len as usize, 0)
}

#[inline(always)]
pub fn csum_add(csum: __wsum, addend: __wsum) -> __wsum {
    let res = csum.wrapping_add(addend);
    res.wrapping_add((res < addend) as u32)
}

#[inline(always)]
pub fn csum_sub(csum: __wsum, addend: __wsum) -> __wsum { csum_add(csum, !addend) }

#[inline(always)]
pub fn csum16_add(csum: __sum16, addend: __be16) -> __sum16 {
    let res = csum.wrapping_add(addend);
    res.wrapping_add((res < addend) as u16)
}

#[inline(always)]
pub fn csum16_sub(csum: __sum16, addend: __be16) -> __sum16 { csum16_add(csum, !addend) }

#[inline(always)]
pub fn csum_shift(sum: __wsum, offset: i32) -> __wsum {
    if offset & 1 != 0 { sum.rotate_right(8) } else { sum }
}

#[inline(always)]
pub fn csum_block_add(csum: __wsum, csum2: __wsum, offset: i32) -> __wsum {
    csum_add(csum, csum_shift(csum2, offset))
}

#[inline(always)]
pub fn csum_block_sub(csum: __wsum, csum2: __wsum, offset: i32) -> __wsum {
    csum_block_add(csum, !csum2, offset)
}

#[inline(always)]
pub fn csum_unfold(n: __sum16) -> __wsum { n as __wsum }

pub const CSUM_MANGLED_0: __sum16 = 0xffff;

#[inline(always)]
pub unsafe fn csum_replace_by_diff(sum: *mut __sum16, diff: __wsum) {
    *sum = csum_fold(csum_add(diff, !csum_unfold(*sum)));
}

#[inline(always)]
pub unsafe fn csum_replace4(sum: *mut __sum16, from: __be32, to: __be32) {
    let tmp = csum_sub(!csum_unfold(*sum), from);
    *sum = csum_fold(csum_add(tmp, to));
}

#[inline(always)]
pub unsafe fn csum_replace2(sum: *mut __sum16, old: __be16, new: __be16) {
    *sum = !csum16_add(csum16_sub(!*sum, old), new);
}

#[inline]
pub unsafe fn csum_replace(csum: *mut __wsum, old: __wsum, new: __wsum) {
    *csum = csum_add(csum_sub(*csum, old), new);
}

#[inline]
pub fn csum_from32to16(mut sum: u32) -> u16 {
    sum = sum.wrapping_add((sum >> 16) | (sum << 16));
    (sum >> 16) as u16
}

#[inline(always)]
pub unsafe fn inet_proto_csum_replace2(sum: *mut __sum16, skb: *mut sk_buff,
                                       from: __be16, to: __be16, pseudohdr: bool) {
    inet_proto_csum_replace4(sum, skb, from as __be32, to as __be32, pseudohdr);
}

#[inline(always)]
pub unsafe fn remcsum_adjust(ptr: *mut u8, mut csum: __wsum,
                             start: i32, offset: i32) -> __wsum {
    let psum = ptr.add(offset as usize) as *mut __sum16;
    csum = csum_sub(csum, csum_partial(ptr as *const _, start as usize, 0));
    let delta = csum_sub(csum_fold(csum) as __wsum, *psum as __wsum);
    *psum = csum_fold(csum);
    delta
}

#[inline(always)]
pub unsafe fn remcsum_unadjust(psum: *mut __sum16, delta: __wsum) {
    *psum = csum_fold(csum_sub(delta, *psum as __wsum));
}

#[inline(always)]
pub fn wsum_negate(val: __wsum) -> __wsum { 0u32.wrapping_sub(val) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
