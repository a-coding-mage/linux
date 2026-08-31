/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/bug.h>, <linux/string.h>, <linux/types.h>

pub const VIRTIO_FEATURES_U64S: usize = 2;
pub const VIRTIO_FEATURES_BITS: usize = VIRTIO_FEATURES_U64S * 64;

#[inline]
pub const fn VIRTIO_BIT(b: u32) -> u64 {
    1u64 << (b & 0x3f)
}

#[inline]
pub const fn VIRTIO_U64(b: u32) -> usize {
    (b >> 6) as usize
}

// Translation of VIRTIO_DECLARE_FEATURES(name):
// union {
//     u64 name;
//     u64 name_array[VIRTIO_FEATURES_U64S];
// }
// This macro is C-token-pasting dependent and must be declared at the use site
// in Rust with the desired field names.

#[inline]
pub const fn virtio_features_chk_bit(bit: u32) -> bool {
    (bit as usize) < VIRTIO_FEATURES_BITS
}

#[inline]
pub unsafe fn virtio_features_test_bit(features: *const u64, bit: u32) -> bool {
    virtio_features_chk_bit(bit) && ((*features.add(VIRTIO_U64(bit)) & VIRTIO_BIT(bit)) != 0)
}

#[inline]
pub unsafe fn virtio_features_set_bit(features: *mut u64, bit: u32) {
    if virtio_features_chk_bit(bit) {
        *features.add(VIRTIO_U64(bit)) |= VIRTIO_BIT(bit);
    }
}

#[inline]
pub unsafe fn virtio_features_clear_bit(features: *mut u64, bit: u32) {
    if virtio_features_chk_bit(bit) {
        *features.add(VIRTIO_U64(bit)) &= !VIRTIO_BIT(bit);
    }
}

#[inline]
pub unsafe fn virtio_features_zero(features: *mut u64) {
    core::ptr::write_bytes(features, 0, VIRTIO_FEATURES_U64S);
}

#[inline]
pub unsafe fn virtio_features_from_u64(features: *mut u64, from: u64) {
    virtio_features_zero(features);
    *features.add(0) = from;
}

#[inline]
pub unsafe fn virtio_features_equal(f1: *const u64, f2: *const u64) -> bool {
    let mut i: i32 = 0;

    while i < VIRTIO_FEATURES_U64S as i32 {
        if *f1.add(i as usize) != *f2.add(i as usize) {
            return false;
        }
        i += 1;
    }
    true
}

#[inline]
pub unsafe fn virtio_features_copy(to: *mut u64, from: *const u64) {
    core::ptr::copy_nonoverlapping(from, to, VIRTIO_FEATURES_U64S);
}

#[inline]
pub unsafe fn virtio_features_andnot(to: *mut u64, f1: *const u64, f2: *const u64) {
    let mut i: i32 = 0;

    while i < VIRTIO_FEATURES_U64S as i32 {
        *to.add(i as usize) = *f1.add(i as usize) & !*f2.add(i as usize);
        i += 1;
    }
}
