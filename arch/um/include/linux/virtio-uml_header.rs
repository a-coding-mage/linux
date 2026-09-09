/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2021 Intel Corporation
 * Author: Johannes Berg <johannes@sipsolutions.net>
 */

// C dependency: `struct virtio_device` is supplied by another header.
#[repr(C)]
pub struct virtio_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn virtio_uml_set_no_vq_suspend(
        vdev: *mut virtio_device,
        no_vq_suspend: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
