/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <linux/list.h>; the referenced type is supplied by another
// translated dependency.

#[repr(C)]
pub struct block_device {
    _private: [u8; 0],
}

extern "C" {
    pub static mut dm_verity_loadpin_trusted_root_digests: crate::list_head;
}

#[repr(C)]
pub struct dm_verity_loadpin_trusted_root_digest {
    pub node: crate::list_head,
    pub len: ::core::ffi::c_uint,
    // C flexible array member: data is counted by len.
    pub data: [u8; 0],
}

// Build-time condition preserved from IS_ENABLED(CONFIG_SECURITY_LOADPIN_VERITY).
#[cfg(feature = "security_loadpin_verity")]
extern "C" {
    pub fn dm_verity_loadpin_is_bdev_trusted(bdev: *mut block_device) -> bool;
}

#[cfg(not(feature = "security_loadpin_verity"))]
#[inline]
pub unsafe fn dm_verity_loadpin_is_bdev_trusted(_bdev: *mut block_device) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
