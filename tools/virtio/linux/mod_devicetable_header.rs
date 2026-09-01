/* SPDX-License-Identifier: GPL-2.0 */

/* Depends on linux/types.h for __u32. */

#[repr(C)]
pub struct virtio_device_id {
    pub device: __u32,
    pub vendor: __u32,
}

pub const VIRTIO_DEV_ANY_ID: u32 = 0xffffffff;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
