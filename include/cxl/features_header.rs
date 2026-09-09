/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2024-2025 Intel Corporation. */

/* Dependencies supplied by the surrounding kernel translation. */

/* Feature UUIDs used by the kernel. UUID_INIT stores the multi-byte fields
 * in big-endian byte order, followed by the remaining bytes. */
pub const CXL_FEAT_PATROL_SCRUB_UUID: [u8; 16] = [
    0x96, 0xda, 0xd7, 0xd6, 0xfd, 0xe8, 0x48, 0x2b,
    0xa7, 0x33, 0x75, 0x77, 0x4e, 0x06, 0xdb, 0x8a,
];

pub const CXL_FEAT_ECS_UUID: [u8; 16] = [
    0xe5, 0xb1, 0x3f, 0x22, 0x23, 0x28, 0x4a, 0x14,
    0xb8, 0xba, 0xb9, 0x69, 0x1e, 0x89, 0x33, 0x86,
];

pub const CXL_FEAT_SPPR_UUID: [u8; 16] = [
    0x89, 0x2b, 0xa4, 0x75, 0xfa, 0xd8, 0x47, 0x4e,
    0x9d, 0x3e, 0x69, 0x2c, 0x91, 0x75, 0x68, 0xbb,
];

pub const CXL_FEAT_HPPR_UUID: [u8; 16] = [
    0x80, 0xea, 0x45, 0x21, 0x78, 0x6f, 0x41, 0x27,
    0xaf, 0xb1, 0xec, 0x74, 0x59, 0xfb, 0x0e, 0x24,
];

pub const CXL_FEAT_CACHELINE_SPARING_UUID: [u8; 16] = [
    0x96, 0xc3, 0x33, 0x86, 0x91, 0xdd, 0x44, 0xc7,
    0x9e, 0xcb, 0xfd, 0xaf, 0x65, 0x03, 0xba, 0xc4,
];

pub const CXL_FEAT_ROW_SPARING_UUID: [u8; 16] = [
    0x45, 0x0e, 0xbf, 0x67, 0xb1, 0x35, 0x4f, 0x97,
    0xa4, 0x98, 0xc2, 0xd5, 0x7f, 0x27, 0x9b, 0xed,
];

pub const CXL_FEAT_BANK_SPARING_UUID: [u8; 16] = [
    0x78, 0xb7, 0x96, 0x36, 0x90, 0xac, 0x4b, 0x64,
    0xa4, 0xef, 0xfa, 0xac, 0x5d, 0x18, 0xa8, 0x63,
];

pub const CXL_FEAT_RANK_SPARING_UUID: [u8; 16] = [
    0x34, 0xdb, 0xaf, 0xf5, 0x05, 0x52, 0x42, 0x81,
    0x8f, 0x76, 0xda, 0x0b, 0x5e, 0x7a, 0x76, 0xa7,
];

/* Feature commands capability supported by a device. */
#[repr(C)]
pub enum cxl_features_capability {
    CXL_FEATURES_NONE = 0,
    CXL_FEATURES_RO,
    CXL_FEATURES_RW,
}

/* struct cxl_features_state - The Features state for the device
 * @cxlds: Pointer to CXL device state
 * @entries: CXL feature entry context */
#[repr(C)]
pub struct cxl_features_state {
    pub cxlds: *mut cxl_dev_state,
    pub entries: *mut cxl_feat_entries,
}

#[repr(C)]
pub struct cxl_feat_entries {
    pub num_features: i32,
    pub num_user_features: i32,
    pub ent: [cxl_feat_entry; 0],
}

#[repr(C)]
pub struct cxl_feat_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cxl_dev_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cxl_mailbox {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cxl_memdev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* CONFIG_CXL_FEATURES is a build-time kernel condition preserved here as a
 * Rust feature condition. */
#[cfg(feature = "CXL_FEATURES")]
extern "C" {
    pub fn to_cxlfs(cxlds: *mut cxl_dev_state) -> *mut cxl_features_state;
    pub fn devm_cxl_setup_features(cxlds: *mut cxl_dev_state) -> i32;
    pub fn devm_cxl_setup_fwctl(host: *mut device, cxlmd: *mut cxl_memdev) -> i32;
}

#[cfg(not(feature = "CXL_FEATURES"))]
pub unsafe fn to_cxlfs(_cxlds: *mut cxl_dev_state) -> *mut cxl_features_state {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CXL_FEATURES"))]
pub unsafe fn devm_cxl_setup_features(_cxlds: *mut cxl_dev_state) -> i32 {
    -95
}

#[cfg(not(feature = "CXL_FEATURES"))]
pub unsafe fn devm_cxl_setup_fwctl(_host: *mut device, _cxlmd: *mut cxl_memdev) -> i32 {
    -95
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
