/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header includes <linux/types.h> and, in kernel builds, defines
 * kernel_ulong_t. Those dependencies are supplied externally.
 */

pub const IEEE1394_MATCH_VENDOR_ID: u32 = 0x0001;
pub const IEEE1394_MATCH_MODEL_ID: u32 = 0x0002;
pub const IEEE1394_MATCH_SPECIFIER_ID: u32 = 0x0004;
pub const IEEE1394_MATCH_VERSION: u32 = 0x0008;

#[repr(C)]
pub union ieee1394_device_id_driver_data {
    pub driver_data: kernel_ulong_t,
    pub driver_data_ptr: *const core::ffi::c_void,
}

#[repr(C)]
pub struct ieee1394_device_id {
    pub match_flags: __u32,
    pub vendor_id: __u32,
    pub model_id: __u32,
    pub specifier_id: __u32,
    pub version: __u32,
    pub driver_data: ieee1394_device_id_driver_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
