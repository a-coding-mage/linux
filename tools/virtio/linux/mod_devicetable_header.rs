/* SPDX-License-Identifier: GPL-2.0 */

/* Depends on linux/types.h for __u32. */

#[repr(C)]
pub struct virtio_device_id {
    pub device: __u32,
    pub vendor: __u32,
}

pub const VIRTIO_DEV_ANY_ID: u32 = 0xffffffff;
