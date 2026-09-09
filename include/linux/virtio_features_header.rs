/* SPDX-License-Identifier: GPL-2.0 */

pub const VIRTIO_FEATURES_U64S: usize = 2;
pub const VIRTIO_FEATURES_BITS: usize = VIRTIO_FEATURES_U64S * 64;

#[inline]
pub const fn virtio_bit(b: u32) -> u64 {
    1u64 << (b & 0x3f)
}

#[inline]
pub const fn virtio_u64(b: u32) -> usize {
    (b >> 6) as usize
}

/* C equivalent: union { u64 name; u64 name##_array[VIRTIO_FEATURES_U64S]; } */
#[macro_export]
macro_rules! VIRTIO_DECLARE_FEATURES {
    ($name:ident) => {
        #[repr(C)]
        pub union $name {
            pub name: u64,
            pub name_array: [u64; VIRTIO_FEATURES_U64S],
        }
    };
}

#[inline]
pub fn virtio_features_chk_bit(bit: u32) -> bool {
    if bit >= VIRTIO_FEATURES_BITS as u32 {
        /*
         * For a constant bit, the C BUILD_BUG_ON makes the build fail before
         * any bad features access.  Runtime values correspond to WARN_ON_ONCE.
         */
        return false;
    }
    true
}

#[inline]
pub unsafe fn virtio_features_test_bit(features: *const u64, bit: u32) -> bool {
    virtio_features_chk_bit(bit)
        && ((*features.add(virtio_u64(bit)) & virtio_bit(bit)) != 0)
}

#[inline]
pub unsafe fn virtio_features_set_bit(features: *mut u64, bit: u32) {
    if virtio_features_chk_bit(bit) {
        *features.add(virtio_u64(bit)) |= virtio_bit(bit);
    }
}

#[inline]
pub unsafe fn virtio_features_clear_bit(features: *mut u64, bit: u32) {
    if virtio_features_chk_bit(bit) {
        *features.add(virtio_u64(bit)) &= !virtio_bit(bit);
    }
}

#[inline]
pub unsafe fn virtio_features_zero(features: *mut u64) {
    for i in 0..VIRTIO_FEATURES_U64S {
        *features.add(i) = 0;
    }
}

#[inline]
pub unsafe fn virtio_features_from_u64(features: *mut u64, from: u64) {
    virtio_features_zero(features);
    *features = from;
}

#[inline]
pub unsafe fn virtio_features_equal(f1: *const u64, f2: *const u64) -> bool {
    let mut i = 0;
    while i < VIRTIO_FEATURES_U64S {
        if *f1.add(i) != *f2.add(i) {
            return false;
        }
        i += 1;
    }
    true
}

#[inline]
pub unsafe fn virtio_features_copy(to: *mut u64, from: *const u64) {
    for i in 0..VIRTIO_FEATURES_U64S {
        *to.add(i) = *from.add(i);
    }
}

#[inline]
pub unsafe fn virtio_features_andnot(to: *mut u64, f1: *const u64, f2: *const u64) {
    for i in 0..VIRTIO_FEATURES_U64S {
        *to.add(i) = *f1.add(i) & !*f2.add(i);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
