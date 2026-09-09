/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies from linux/types.h, linux/blk-mq.h, and linux/wordpart.h
// are supplied by the surrounding translation unit.

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum t10_dif_type {
    T10_PI_TYPE0_PROTECTION = 0x0,
    T10_PI_TYPE1_PROTECTION = 0x1,
    T10_PI_TYPE2_PROTECTION = 0x2,
    T10_PI_TYPE3_PROTECTION = 0x3,
}

/*
 * A T10 PI-capable target device can be formatted with different
 * protection schemes. Currently 0 through 3 are defined:
 *
 * Type 0 is regular (unprotected) I/O
 *
 * Type 1 defines the contents of the guard and reference tags
 *
 * Type 2 defines the contents of the guard and reference tags and
 * uses 32-byte commands to seed the latter
 *
 * Type 3 defines the contents of the guard tag only
 */

/* These layouts mirror the fields accessed by the C inline function. */
#[repr(C)]
pub struct blk_integrity {
    pub interval_exp: u32,
}

#[repr(C)]
pub struct queue_limits {
    pub integrity: blk_integrity,
}

#[repr(C)]
pub struct request_queue {
    pub limits: queue_limits,
}

#[repr(C)]
pub struct request {
    pub q: *mut request_queue,
}

extern "C" {
    pub fn ilog2(value: u32) -> u32;
    pub fn queue_logical_block_size(q: *mut request_queue) -> u32;
    pub fn blk_rq_pos(rq: *const request) -> u64;
}

// CONFIG_BLK_DEV_INTEGRITY is a build-time condition from the C header.
#[inline]
pub unsafe fn full_pi_ref_tag(rq: *const request) -> u64 {
    let mut shift: u32 = ilog2(queue_logical_block_size((*rq).q));

    // Preserves IS_ENABLED(CONFIG_BLK_DEV_INTEGRITY) and its conditional use.
    if (*rq).q.as_ref().unwrap().limits.integrity.interval_exp != 0 {
        shift = (*rq).q.as_ref().unwrap().limits.integrity.interval_exp;
    }
    blk_rq_pos(rq) >> (shift - SECTOR_SHIFT)
}

/*
 * T10 Protection Information tuple.
 */
#[repr(C)]
pub struct t10_pi_tuple {
    pub guard_tag: u16, /* __be16: Checksum */
    pub app_tag: u16,   /* __be16: Opaque storage */
    pub ref_tag: u32,   /* __be32: Target LBA or indirect LBA */
}

pub const T10_PI_APP_ESCAPE: u16 = 0xffffu16.to_be();
pub const T10_PI_REF_ESCAPE: u32 = 0xffffffffu32.to_be();

#[inline]
pub unsafe fn t10_pi_ref_tag(rq: *const request) -> u32 {
    full_pi_ref_tag(rq) as u32
}

#[repr(C)]
pub struct crc64_pi_tuple {
    pub guard_tag: u64, /* __be64 */
    pub app_tag: u16,   /* __be16 */
    pub ref_tag: [u8; 6],
}

/**
 * lower_48_bits() - return bits 0-47 of a number
 * @n: the number we're accessing
 */
#[inline]
pub const fn lower_48_bits(n: u64) -> u64 {
    n & ((1u64 << 48) - 1)
}

#[inline]
pub unsafe fn ext_pi_ref_tag(rq: *const request) -> u64 {
    lower_48_bits(full_pi_ref_tag(rq))
}

pub const SECTOR_SHIFT: u32 = 9;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
