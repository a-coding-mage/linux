/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependency supplied by the surrounding translation unit: adf_accel_devices.h

pub const ADF_RING_CONFIG_NEAR_FULL_WM: u32 = 0x0A;
pub const ADF_RING_CONFIG_NEAR_EMPTY_WM: u32 = 0x05;
pub const ADF_COALESCING_MIN_TIME: u32 = 0x1FF;
pub const ADF_COALESCING_MAX_TIME: u32 = 0xFFFFF;
pub const ADF_COALESCING_DEF_TIME: u32 = 0x27FF;
pub const ADF_RING_NEAR_WATERMARK_512: u32 = 0x08;
pub const ADF_RING_NEAR_WATERMARK_0: u32 = 0x00;
pub const ADF_RING_EMPTY_SIG: u32 = 0x7F7F7F7F;

/* Valid internal ring size values */
pub const ADF_RING_SIZE_128: u32 = 0x01;
pub const ADF_RING_SIZE_256: u32 = 0x02;
pub const ADF_RING_SIZE_512: u32 = 0x03;
pub const ADF_RING_SIZE_4K: u32 = 0x06;
pub const ADF_RING_SIZE_16K: u32 = 0x08;
pub const ADF_RING_SIZE_4M: u32 = 0x10;
pub const ADF_MIN_RING_SIZE: u32 = ADF_RING_SIZE_128;
pub const ADF_MAX_RING_SIZE: u32 = ADF_RING_SIZE_4M;
pub const ADF_DEFAULT_RING_SIZE: u32 = ADF_RING_SIZE_16K;

/* Valid internal msg size values */
pub const ADF_MSG_SIZE_32: u32 = 0x01;
pub const ADF_MSG_SIZE_64: u32 = 0x02;
pub const ADF_MSG_SIZE_128: u32 = 0x04;
pub const ADF_MIN_MSG_SIZE: u32 = ADF_MSG_SIZE_32;
pub const ADF_MAX_MSG_SIZE: u32 = ADF_MSG_SIZE_128;

/* Size to bytes conversion macros for ring and msg size values */
#[inline]
pub const fn ADF_MSG_SIZE_TO_BYTES(size: u32) -> u32 {
    size << 5
}

#[inline]
pub const fn ADF_BYTES_TO_MSG_SIZE(size: u32) -> u32 {
    size >> 5
}

#[inline]
pub const fn ADF_SIZE_TO_RING_SIZE_IN_BYTES(size: u32) -> u32 {
    (1 << (size - 1)) << 7
}

#[inline]
pub const fn ADF_RING_SIZE_IN_BYTES_TO_SIZE(size: u32) -> u32 {
    (1 << (size - 1)) >> 7
}

/* Minimum ring buffer size for memory allocation */
#[inline]
pub const fn ADF_RING_SIZE_BYTES_MIN(size: u32) -> u32 {
    if size < ADF_SIZE_TO_RING_SIZE_IN_BYTES(ADF_RING_SIZE_4K) {
        ADF_SIZE_TO_RING_SIZE_IN_BYTES(ADF_RING_SIZE_4K)
    } else {
        size
    }
}

#[inline]
pub const fn ADF_RING_SIZE_MODULO(size: u32) -> u32 {
    size + 0x6
}

#[inline]
pub const fn ADF_SIZE_TO_POW(size: u32) -> u32 {
    (((size & 0x4) >> 1) | ((size & 0x4) >> 2) | size) & !0x4
}

/* Max outstanding requests */
#[inline]
pub const fn ADF_MAX_INFLIGHTS(ring_size: u32, msg_size: u32) -> u32 {
    ((((1 << (ring_size - 1)) << 3) >> ADF_SIZE_TO_POW(msg_size)) - 1)
}

#[inline]
pub const fn BUILD_RING_CONFIG(size: u32) -> u32 {
    (ADF_RING_NEAR_WATERMARK_0 << ADF_RING_CONFIG_NEAR_FULL_WM)
        | (ADF_RING_NEAR_WATERMARK_0 << ADF_RING_CONFIG_NEAR_EMPTY_WM)
        | size
}

#[inline]
pub const fn BUILD_RESP_RING_CONFIG(
    size: u32,
    watermark_nf: u32,
    watermark_ne: u32,
) -> u32 {
    (watermark_nf << ADF_RING_CONFIG_NEAR_FULL_WM)
        | (watermark_ne << ADF_RING_CONFIG_NEAR_EMPTY_WM)
        | size
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
