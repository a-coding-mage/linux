/* SPDX-License-Identifier: GPL-2.0 */
/*
 * AMD AE4DMA driver
 *
 * Copyright (c) 2024, Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * Author: Basavaraj Natikar <Basavaraj.Natikar@amd.com>
 */

// Dependencies supplied by the Linux kernel and the included driver headers:
// linux/device.h, linux/dmaengine.h, linux/dmapool.h, linux/list.h,
// linux/mutex.h, linux/pci.h, linux/spinlock.h, linux/wait.h,
// ../ptdma/ptdma.h, and ../../virt-dma.h.

pub const MAX_AE4_HW_QUEUES: usize = 16;

pub const AE4_DESC_COMPLETED: u32 = 0x03;

pub const AE4_MAX_IDX_OFF: u32 = 0x08;
pub const AE4_RD_IDX_OFF: u32 = 0x0c;
pub const AE4_WR_IDX_OFF: u32 = 0x10;
pub const AE4_INTR_STS_OFF: u32 = 0x14;
pub const AE4_Q_BASE_L_OFF: u32 = 0x18;
pub const AE4_Q_BASE_H_OFF: u32 = 0x1c;
pub const AE4_Q_SZ: u32 = 0x20;

pub const AE4_DMA_VERSION: u32 = 4;
pub const CMD_AE4_DESC_DW0_VAL: u32 = 2;

pub const AE4_TIME_OUT: u32 = 5000;

#[repr(C)]
pub struct ae4_msix {
    pub msix_count: core::ffi::c_int,
    pub msix_entry: [msix_entry; MAX_AE4_HW_QUEUES],
}

#[repr(C)]
pub struct ae4_cmd_queue {
    pub ae4: *mut ae4_device,
    pub cmd_q: pt_cmd_queue,
    pub cmd: list_head,
    /* protect command operations */
    pub cmd_lock: mutex,
    pub p_work: delayed_work,
    pub pws: *mut workqueue_struct,
    pub cmp: completion,
    pub q_w: wait_queue_head_t,
    pub intr_cnt: atomic64_t,
    pub done_cnt: atomic64_t,
    pub q_cmd_count: u64,
    pub dridx: u32,
    pub tail_wi: u32,
    pub id: u32,
}

#[repr(C)]
pub struct dword0 {
    pub byte0: u8,
    pub byte1: u8,
    pub timestamp: u16,
}

#[repr(C)]
pub union dwou {
    pub dw0: u32,
    pub dws: dword0,
}

#[repr(C)]
pub struct dword1 {
    pub status: u8,
    pub err_code: u8,
    pub desc_id: u16,
}

#[repr(C)]
pub struct ae4dma_desc {
    pub dwouv: dwou,
    pub dw1: dword1,
    pub length: u32,
    pub rsvd: u32,
    pub src_hi: u32,
    pub src_lo: u32,
    pub dst_hi: u32,
    pub dst_lo: u32,
}

#[repr(C)]
pub struct ae4_device {
    pub pt: pt_device,
    pub ae4_msix: *mut ae4_msix,
    pub ae4cmd_q: [ae4_cmd_queue; MAX_AE4_HW_QUEUES],
    pub ae4_irq: [core::ffi::c_uint; MAX_AE4_HW_QUEUES],
    pub cmd_q_count: core::ffi::c_uint,
}

unsafe extern "C" {
    pub fn ae4_core_init(ae4: *mut ae4_device) -> core::ffi::c_int;
    pub fn ae4_destroy_work(ae4: *mut ae4_device);
    pub fn ae4_check_status_error(ae4cmd_q: *mut ae4_cmd_queue, idx: core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
