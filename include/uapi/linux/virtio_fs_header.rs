/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */

/*
 * Dependencies supplied by the corresponding Linux UAPI headers:
 * linux/types.h, linux/virtio_ids.h, linux/virtio_config.h,
 * linux/virtio_types.h.
 */

#[repr(C, packed)]
pub struct virtio_fs_config {
    /* Filesystem name (UTF-8, not NUL-terminated, padded with NULs) */
    pub tag: [u8; 36],

    /* Number of request queues */
    pub num_request_queues: u32,
}

/* For the id field in virtio_pci_shm_cap */
pub const VIRTIO_FS_SHMCAP_ID_CACHE: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
