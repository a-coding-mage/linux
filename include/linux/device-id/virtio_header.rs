/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <linux/types.h> under __KERNEL__; __u32 is represented
// here by Rust's 32-bit unsigned integer type.

pub const VIRTIO_DEV_ANY_ID: u32 = 0xffff_ffff;

#[repr(C)]
pub struct virtio_device_id {
    pub device: u32,
    pub vendor: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
