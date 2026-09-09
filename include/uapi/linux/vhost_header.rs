/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Userspace interface for in-kernel virtio accelerators. */
/*
 * vhost is used to reduce the number of system calls involved in virtio.
 *
 * Existing virtio net code is used in the guest without modification.
 *
 * This header includes interface used by userspace hypervisor for
 * device configuration.
 */

// Dependencies supplied by the corresponding Linux headers are intentionally
// left external: vhost_types, Linux integer types, and ioctl encodings.

pub const VHOST_FILE_UNBIND: i32 = -1;

/* ioctls */
pub const VHOST_VIRTIO: u32 = 0xAF;

/* Features bitmask for forward compatibility.  Transport bits are used for
 * vhost specific features. */
pub const VHOST_GET_FEATURES: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x00, __u64);
pub const VHOST_SET_FEATURES: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x00, __u64);
pub const VHOST_SET_OWNER: _IO_TYPE = _IO(VHOST_VIRTIO, 0x01);
pub const VHOST_RESET_OWNER: _IO_TYPE = _IO(VHOST_VIRTIO, 0x02);
pub const VHOST_SET_MEM_TABLE: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x03, vhost_memory);
pub const VHOST_SET_LOG_BASE: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x04, __u64);
pub const VHOST_SET_LOG_FD: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x07, i32);
pub const VHOST_NEW_WORKER: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x8, vhost_worker_state);
pub const VHOST_FREE_WORKER: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x9, vhost_worker_state);
pub const VHOST_SET_VRING_NUM: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x10, vhost_vring_state);
pub const VHOST_SET_VRING_ADDR: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x11, vhost_vring_addr);
pub const VHOST_SET_VRING_BASE: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x12, vhost_vring_state);
pub const VHOST_GET_VRING_BASE: _IOWR_TYPE = _IOWR(VHOST_VIRTIO, 0x12, vhost_vring_state);
pub const VHOST_VRING_LITTLE_ENDIAN: u32 = 0;
pub const VHOST_VRING_BIG_ENDIAN: u32 = 1;
pub const VHOST_SET_VRING_ENDIAN: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x13, vhost_vring_state);
pub const VHOST_GET_VRING_ENDIAN: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x14, vhost_vring_state);
pub const VHOST_ATTACH_VRING_WORKER: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x15, vhost_vring_worker);
pub const VHOST_GET_VRING_WORKER: _IOWR_TYPE = _IOWR(VHOST_VIRTIO, 0x16, vhost_vring_worker);
pub const VHOST_SET_VRING_KICK: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x20, vhost_vring_file);
pub const VHOST_SET_VRING_CALL: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x21, vhost_vring_file);
pub const VHOST_SET_VRING_ERR: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x22, vhost_vring_file);
pub const VHOST_SET_VRING_BUSYLOOP_TIMEOUT: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x23, vhost_vring_state);
pub const VHOST_GET_VRING_BUSYLOOP_TIMEOUT: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x24, vhost_vring_state);
pub const VHOST_SET_BACKEND_FEATURES: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x25, __u64);
pub const VHOST_GET_BACKEND_FEATURES: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x26, __u64);
pub const VHOST_NET_SET_BACKEND: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x30, vhost_vring_file);
pub const VHOST_SCSI_SET_ENDPOINT: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x40, vhost_scsi_target);
pub const VHOST_SCSI_CLEAR_ENDPOINT: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x41, vhost_scsi_target);
pub const VHOST_SCSI_GET_ABI_VERSION: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x42, i32);
pub const VHOST_SCSI_SET_EVENTS_MISSED: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x43, __u32);
pub const VHOST_SCSI_GET_EVENTS_MISSED: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x44, __u32);
pub const VHOST_VSOCK_SET_GUEST_CID: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x60, __u64);
pub const VHOST_VSOCK_SET_RUNNING: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x61, i32);
pub const VHOST_VDPA_GET_DEVICE_ID: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x70, __u32);
pub const VHOST_VDPA_GET_STATUS: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x71, __u8);
pub const VHOST_VDPA_SET_STATUS: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x72, __u8);
pub const VHOST_VDPA_GET_CONFIG: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x73, vhost_vdpa_config);
pub const VHOST_VDPA_SET_CONFIG: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x74, vhost_vdpa_config);
pub const VHOST_VDPA_SET_VRING_ENABLE: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x75, vhost_vring_state);
pub const VHOST_VDPA_GET_VRING_NUM: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x76, __u16);
pub const VHOST_VDPA_SET_CONFIG_CALL: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x77, i32);
pub const VHOST_VDPA_GET_IOVA_RANGE: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x78, vhost_vdpa_iova_range);
pub const VHOST_VDPA_GET_CONFIG_SIZE: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x79, __u32);
pub const VHOST_VDPA_GET_AS_NUM: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x7A, u32);
pub const VHOST_VDPA_GET_VRING_GROUP: _IOWR_TYPE = _IOWR(VHOST_VIRTIO, 0x7B, vhost_vring_state);
pub const VHOST_VDPA_SET_GROUP_ASID: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x7C, vhost_vring_state);
pub const VHOST_VDPA_SUSPEND: _IO_TYPE = _IO(VHOST_VIRTIO, 0x7D);
pub const VHOST_VDPA_RESUME: _IO_TYPE = _IO(VHOST_VIRTIO, 0x7E);
pub const VHOST_VDPA_GET_VRING_DESC_GROUP: _IOWR_TYPE = _IOWR(VHOST_VIRTIO, 0x7F, vhost_vring_state);
pub const VHOST_VDPA_GET_VQS_COUNT: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x80, __u32);
pub const VHOST_VDPA_GET_GROUP_NUM: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x81, __u32);
pub const VHOST_VDPA_GET_VRING_SIZE: _IOWR_TYPE = _IOWR(VHOST_VIRTIO, 0x82, vhost_vring_state);
pub const VHOST_GET_FEATURES_ARRAY: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x83, vhost_features_array);
pub const VHOST_SET_FEATURES_ARRAY: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x83, vhost_features_array);
pub const VHOST_FORK_OWNER_KTHREAD: u32 = 0;
pub const VHOST_FORK_OWNER_TASK: u32 = 1;
pub const VHOST_SET_FORK_FROM_OWNER: _IOW_TYPE = _IOW(VHOST_VIRTIO, 0x84, __u8);
pub const VHOST_GET_FORK_FROM_OWNER: _IOR_TYPE = _IOR(VHOST_VIRTIO, 0x85, __u8);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
