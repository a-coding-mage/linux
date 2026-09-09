/* SPDX-License-Identifier: GPL-2.0 */

/* The C header includes linux/types.h when building in the kernel. */
#[cfg(feature = "__KERNEL__")]
pub type kernel_ulong_t = usize;

pub const ZORRO_WILDCARD: u32 = 0xffff_ffff; /* not official */

pub const ZORRO_DEVICE_MODALIAS_FMT: &str = "zorro:i%08X";

#[repr(C)]
pub union zorro_device_id_data {
    /* Data private to the driver */
    pub driver_data: kernel_ulong_t,
    pub driver_data_ptr: *const core::ffi::c_void,
}

#[repr(C)]
pub struct zorro_device_id {
    pub id: u32, /* Device ID or ZORRO_WILDCARD */
    pub data: zorro_device_id_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
