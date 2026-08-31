// Dependencies from the C header:
// #include <linux/bug.h>
// #include <linux/string.h>
// #include <linux/virtio_features.h>

// C fallback:
// #ifndef VIRTIO_FEATURES_BITS
// #define VIRTIO_FEATURES_BITS 128
// #endif
pub const VIRTIO_FEATURES_BITS: usize = 128;

// C fallback:
// #ifndef VIRTIO_U64
// #define VIRTIO_U64(b)           ((b) >> 6)
// #endif
#[inline]
pub const fn VIRTIO_U64(b: u64) -> u64 {
    b >> 6
}
