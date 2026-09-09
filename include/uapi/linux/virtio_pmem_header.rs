/* SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause */
/*
 * Definitions for virtio-pmem devices.
 *
 * Copyright (C) 2019 Red Hat, Inc.
 *
 * Author(s): Pankaj Gupta <pagupta@redhat.com>
 */

// Dependency intent from the original header:
// #include <linux/types.h>
// #include <linux/virtio_ids.h>
// #include <linux/virtio_config.h>

/* Feature bits */
/* guest physical address range will be indicated as shared memory region 0 */
pub const VIRTIO_PMEM_F_SHMEM_REGION: u32 = 0;

/* shmid of the shared memory region corresponding to the pmem */
pub const VIRTIO_PMEM_SHMEM_REGION_ID: u32 = 0;

#[repr(C)]
pub struct virtio_pmem_config {
    pub start: __le64,
    pub size: __le64,
}

pub const VIRTIO_PMEM_REQ_TYPE_FLUSH: u32 = 0;

#[repr(C)]
pub struct virtio_pmem_resp {
    /* Host return status corresponding to flush request */
    pub ret: __le32,
}

#[repr(C)]
pub struct virtio_pmem_req {
    /* command type */
    pub r#type: __le32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
