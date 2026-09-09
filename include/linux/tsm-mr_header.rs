/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// use crypto::hash_info::{enum hash_algo, ...};

/**
 * struct tsm_measurement_register - describes an architectural measurement
 * register (MR)
 * @mr_name: name of the MR
 * @mr_value: buffer containing the current value of the MR
 * @mr_size: size of the MR - typically the digest size of @mr_hash
 * @mr_flags: bitwise OR of one or more flags, detailed below
 * @mr_hash: optional hash identifier defined in include/uapi/linux/hash_info.h.
 *
 * A CC guest driver encloses an array of this structure in struct
 * tsm_measurements to detail the measurement facility supported by the
 * underlying CC hardware.
 *
 * @mr_name and @mr_value must stay valid until this structure is no longer in
 * use.
 *
 * @mr_flags is the bitwise-OR of zero or more of the flags below.
 */
#[repr(C)]
pub struct tsm_measurement_register {
    pub mr_name: *const core::ffi::c_char,
    pub mr_value: *mut core::ffi::c_void,
    pub mr_size: u32,
    pub mr_flags: u32,
    pub mr_hash: hash_algo,
}

pub const TSM_MR_F_NOHASH: u32 = 1;
pub const TSM_MR_F_WRITABLE: u32 = 2;
pub const TSM_MR_F_READABLE: u32 = 4;
pub const TSM_MR_F_LIVE: u32 = 8;
pub const TSM_MR_F_RTMR: u32 = TSM_MR_F_LIVE | TSM_MR_F_WRITABLE;

// C initializer macro: the digest-size and hash-algorithm identifiers are
// supplied by the surrounding kernel translation.
#[macro_export]
macro_rules! TSM_MR_ {
    ($mr:ident, $hash:ident) => {
        .mr_name = concat!(stringify!($mr), "\0").as_ptr() as *const core::ffi::c_char,
        .mr_size = $hash##_DIGEST_SIZE,
        .mr_hash = HASH_ALGO_##$hash,
        .mr_flags = TSM_MR_F_READABLE,
    };
}

/**
 * struct tsm_measurements - defines the CC architecture specific measurement
 * facility and methods for updating measurement registers (MRs)
 */
#[repr(C)]
pub struct tsm_measurements {
    pub mrs: *const tsm_measurement_register,
    pub nr_mrs: usize,
    pub refresh: Option<unsafe extern "C" fn(tm: *const tsm_measurements) -> i32>,
    pub write: Option<unsafe extern "C" fn(
        tm: *const tsm_measurements,
        mr: *const tsm_measurement_register,
        data: *const u8,
    ) -> i32>,
}

extern "C" {
    pub fn tsm_mr_create_attribute_group(
        tm: *const tsm_measurements,
    ) -> *const attribute_group;
    pub fn tsm_mr_free_attribute_group(attr_grp: *const attribute_group);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
