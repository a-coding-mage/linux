/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
 *
 * Copyright (c) 2019-2020, The Linux Foundation. All rights reserved.
 * Copyright (c) 2021-2023 Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Dependency intent: symbols such as SZ_4K, DRM_COMMAND_BASE, DRM_IOWR, DRM_IOW,
// and the DRM integer aliases are supplied by the translated DRM dependencies.

pub const QAIC_MANAGE_MAX_MSG_LENGTH: _ = SZ_4K;

pub const QAIC_SEM_INSYNCFENCE: u32 = 2;
pub const QAIC_SEM_OUTSYNCFENCE: u32 = 1;

pub const QAIC_SEM_NOP: u32 = 0;
pub const QAIC_SEM_INIT: u32 = 1;
pub const QAIC_SEM_INC: u32 = 2;
pub const QAIC_SEM_DEC: u32 = 3;
pub const QAIC_SEM_WAIT_EQUAL: u32 = 4;
pub const QAIC_SEM_WAIT_GT_EQ: u32 = 5; // Greater than or equal
pub const QAIC_SEM_WAIT_GT_0: u32 = 6; // Greater than 0

pub const QAIC_TRANS_UNDEFINED: u32 = 0;
pub const QAIC_TRANS_PASSTHROUGH_FROM_USR: u32 = 1;
pub const QAIC_TRANS_PASSTHROUGH_TO_USR: u32 = 2;
pub const QAIC_TRANS_PASSTHROUGH_FROM_DEV: u32 = 3;
pub const QAIC_TRANS_PASSTHROUGH_TO_DEV: u32 = 4;
pub const QAIC_TRANS_DMA_XFER_FROM_USR: u32 = 5;
pub const QAIC_TRANS_DMA_XFER_TO_DEV: u32 = 6;
pub const QAIC_TRANS_ACTIVATE_FROM_USR: u32 = 7;
pub const QAIC_TRANS_ACTIVATE_FROM_DEV: u32 = 8;
pub const QAIC_TRANS_ACTIVATE_TO_DEV: u32 = 9;
pub const QAIC_TRANS_DEACTIVATE_FROM_USR: u32 = 10;
pub const QAIC_TRANS_DEACTIVATE_FROM_DEV: u32 = 11;
pub const QAIC_TRANS_STATUS_FROM_USR: u32 = 12;
pub const QAIC_TRANS_STATUS_TO_USR: u32 = 13;
pub const QAIC_TRANS_STATUS_FROM_DEV: u32 = 14;
pub const QAIC_TRANS_STATUS_TO_DEV: u32 = 15;
pub const QAIC_TRANS_TERMINATE_FROM_DEV: u32 = 16;
pub const QAIC_TRANS_TERMINATE_TO_DEV: u32 = 17;
pub const QAIC_TRANS_DMA_XFER_CONT: u32 = 18;
pub const QAIC_TRANS_VALIDATE_PARTITION_FROM_DEV: u32 = 19;
pub const QAIC_TRANS_VALIDATE_PARTITION_TO_DEV: u32 = 20;

#[repr(C)]
pub struct qaic_manage_trans_hdr { pub type_: u32, pub len: u32 }

#[repr(C)]
pub struct qaic_manage_trans_passthrough { pub hdr: qaic_manage_trans_hdr, pub data: [u8; 0] }

#[repr(C)]
pub struct qaic_manage_trans_dma_xfer { pub hdr: qaic_manage_trans_hdr, pub tag: u32, pub pad: u32, pub addr: u64, pub size: u64 }

#[repr(C)]
pub struct qaic_manage_trans_activate_to_dev { pub hdr: qaic_manage_trans_hdr, pub queue_size: u32, pub eventfd: u32, pub options: u32, pub pad: u32 }

#[repr(C)]
pub struct qaic_manage_trans_activate_from_dev { pub hdr: qaic_manage_trans_hdr, pub status: u32, pub dbc_id: u32, pub options: u64 }

#[repr(C)]
pub struct qaic_manage_trans_deactivate { pub hdr: qaic_manage_trans_hdr, pub dbc_id: u32, pub pad: u32 }

#[repr(C)]
pub struct qaic_manage_trans_status_to_dev { pub hdr: qaic_manage_trans_hdr }

#[repr(C)]
pub struct qaic_manage_trans_status_from_dev { pub hdr: qaic_manage_trans_hdr, pub major: u16, pub minor: u16, pub status: u32, pub status_flags: u64 }

#[repr(C)]
pub struct qaic_manage_msg { pub len: u32, pub count: u32, pub data: u64 }

#[repr(C)]
pub struct qaic_create_bo { pub size: u64, pub handle: u32, pub pad: u32 }

#[repr(C)]
pub struct qaic_mmap_bo { pub handle: u32, pub pad: u32, pub offset: u64 }

#[repr(C)]
pub struct qaic_sem { pub val: u16, pub index: u8, pub presync: u8, pub cmd: u8, pub flags: u8, pub pad: u16 }

#[repr(C)]
pub struct qaic_attach_slice_entry { pub size: u64, pub sem0: qaic_sem, pub sem1: qaic_sem, pub sem2: qaic_sem, pub sem3: qaic_sem, pub dev_addr: u64, pub db_addr: u64, pub db_data: u32, pub db_len: u32, pub offset: u64 }

#[repr(C)]
pub struct qaic_attach_slice_hdr { pub count: u32, pub dbc_id: u32, pub handle: u32, pub dir: u32, pub size: u64 }

#[repr(C)]
pub struct qaic_attach_slice { pub hdr: qaic_attach_slice_hdr, pub data: u64 }

#[repr(C)]
pub struct qaic_execute_entry { pub handle: u32, pub dir: u32 }

#[repr(C)]
pub struct qaic_partial_execute_entry { pub handle: u32, pub dir: u32, pub resize: u64 }

#[repr(C)]
pub struct qaic_execute_hdr { pub count: u32, pub dbc_id: u32 }

#[repr(C)]
pub struct qaic_execute { pub hdr: qaic_execute_hdr, pub data: u64 }

#[repr(C)]
pub struct qaic_wait { pub handle: u32, pub timeout: u32, pub dbc_id: u32, pub pad: u32 }

#[repr(C)]
pub struct qaic_perf_stats_hdr { pub count: u16, pub pad: u16, pub dbc_id: u32 }

#[repr(C)]
pub struct qaic_perf_stats { pub hdr: qaic_perf_stats_hdr, pub data: u64 }

#[repr(C)]
pub struct qaic_perf_stats_entry { pub handle: u32, pub queue_level_before: u32, pub num_queue_element: u32, pub submit_latency_us: u32, pub device_latency_us: u32, pub pad: u32 }

#[repr(C)]
pub struct qaic_detach_slice { pub handle: u32, pub pad: u32 }

pub const DRM_QAIC_MANAGE: u32 = 0x00;
pub const DRM_QAIC_CREATE_BO: u32 = 0x01;
pub const DRM_QAIC_MMAP_BO: u32 = 0x02;
pub const DRM_QAIC_ATTACH_SLICE_BO: u32 = 0x03;
pub const DRM_QAIC_EXECUTE_BO: u32 = 0x04;
pub const DRM_QAIC_PARTIAL_EXECUTE_BO: u32 = 0x05;
pub const DRM_QAIC_WAIT_BO: u32 = 0x06;
pub const DRM_QAIC_PERF_STATS_BO: u32 = 0x07;
pub const DRM_QAIC_DETACH_SLICE_BO: u32 = 0x08;

pub const DRM_IOCTL_QAIC_MANAGE: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_QAIC_MANAGE, qaic_manage_msg);
pub const DRM_IOCTL_QAIC_CREATE_BO: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_QAIC_CREATE_BO, qaic_create_bo);
pub const DRM_IOCTL_QAIC_MMAP_BO: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_QAIC_MMAP_BO, qaic_mmap_bo);
pub const DRM_IOCTL_QAIC_ATTACH_SLICE_BO: _ = DRM_IOW(DRM_COMMAND_BASE + DRM_QAIC_ATTACH_SLICE_BO, qaic_attach_slice);
pub const DRM_IOCTL_QAIC_EXECUTE_BO: _ = DRM_IOW(DRM_COMMAND_BASE + DRM_QAIC_EXECUTE_BO, qaic_execute);
pub const DRM_IOCTL_QAIC_PARTIAL_EXECUTE_BO: _ = DRM_IOW(DRM_COMMAND_BASE + DRM_QAIC_PARTIAL_EXECUTE_BO, qaic_execute);
pub const DRM_IOCTL_QAIC_WAIT_BO: _ = DRM_IOW(DRM_COMMAND_BASE + DRM_QAIC_WAIT_BO, qaic_wait);
pub const DRM_IOCTL_QAIC_PERF_STATS_BO: _ = DRM_IOWR(DRM_COMMAND_BASE + DRM_QAIC_PERF_STATS_BO, qaic_perf_stats);
pub const DRM_IOCTL_QAIC_DETACH_SLICE_BO: _ = DRM_IOW(DRM_COMMAND_BASE + DRM_QAIC_DETACH_SLICE_BO, qaic_detach_slice);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
