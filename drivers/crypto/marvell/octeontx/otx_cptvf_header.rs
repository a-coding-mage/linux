/* SPDX-License-Identifier: GPL-2.0
 * Marvell OcteonTX CPT driver
 *
 * Copyright (C) 2019 Marvell International Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// C dependencies: linux/list.h, linux/interrupt.h, linux/device.h,
// otx_cpt_common.h, and otx_cptvf_reqmgr.h.

/* Flags to indicate the features supported */
pub const OTX_CPT_FLAG_DEVICE_READY: u16 = 1u16 << 1;

#[inline]
pub unsafe fn otx_cpt_device_ready(cpt: *const otx_cptvf) -> u16 {
    (*cpt).flags & OTX_CPT_FLAG_DEVICE_READY
}

/* Default command queue length */
pub const OTX_CPT_CMD_QLEN: u32 = 4 * 2046;
pub const OTX_CPT_CMD_QCHUNK_SIZE: u32 = 1023;
pub const OTX_CPT_NUM_QS_PER_VF: usize = 1;

#[repr(C)]
pub struct otx_cpt_cmd_chunk {
    pub head: *mut u8,
    pub dma_addr: dma_addr_t,
    pub size: u32, /* Chunk size, max OTX_CPT_INST_CHUNK_MAX_SIZE */
    pub nextchunk: list_head,
}

#[repr(C)]
pub struct otx_cpt_cmd_queue {
    pub idx: u32, /* Command queue host write idx */
    pub num_chunks: u32, /* Number of command chunks */
    pub qhead: *mut otx_cpt_cmd_chunk, /* Command queue head, instructions are inserted here */
    pub base: *mut otx_cpt_cmd_chunk,
    pub chead: list_head,
}

#[repr(C)]
pub struct otx_cpt_cmd_qinfo {
    pub qchunksize: u32, /* Command queue chunk size */
    pub queue: [otx_cpt_cmd_queue; OTX_CPT_NUM_QS_PER_VF],
}

#[repr(C)]
pub struct otx_cpt_pending_qinfo {
    pub num_queues: u32, /* Number of queues supported */
    pub queue: [otx_cpt_pending_queue; OTX_CPT_NUM_QS_PER_VF],
}

/* C macro: for_each_pending_queue(qinfo, q, i) */
// for i = 0, q = &qinfo.queue[i]; i < qinfo.num_queues; i++, q = &qinfo.queue[i]

#[repr(C)]
pub struct otx_cptvf_wqe {
    pub twork: tasklet_struct,
    pub cptvf: *mut otx_cptvf,
}

#[repr(C)]
pub struct otx_cptvf_wqe_info {
    pub vq_wqe: [otx_cptvf_wqe; OTX_CPT_NUM_QS_PER_VF],
}

#[repr(C)]
pub struct otx_cptvf {
    pub flags: u16, /* Flags to hold device status bits */
    pub vfid: u8, /* Device Index 0...OTX_CPT_MAX_VF_NUM */
    pub num_vfs: u8, /* Number of enabled VFs */
    pub vftype: u8, /* VF type of SE_TYPE(2) or AE_TYPE(1) */
    pub vfgrp: u8, /* VF group (0 - 8) */
    pub node: u8, /* Operating node: Bits (46:44) in BAR0 address */
    pub priority: u8, /* VF priority ring: 1-High proirity round robin ring;0-Low priority round robin ring; */
    pub pdev: *mut pci_dev, /* Pci device handle */
    pub reg_base: *mut core::ffi::c_void, /* Register start address */
    pub wqe_info: *mut core::ffi::c_void, /* BH worker info */
    /* MSI-X */
    pub affinity_mask: [cpumask_var_t; OTX_CPT_VF_MSIX_VECTORS],
    /* Command and Pending queues */
    pub qsize: u32,
    pub num_queues: u32,
    pub cqinfo: otx_cpt_cmd_qinfo, /* Command queue information */
    pub pqinfo: otx_cpt_pending_qinfo, /* Pending queue information */
    /* VF-PF mailbox communication */
    pub pf_acked: bool,
    pub pf_nacked: bool,
}

extern "C" {
    pub fn otx_cptvf_send_vf_up(cptvf: *mut otx_cptvf) -> i32;
    pub fn otx_cptvf_send_vf_down(cptvf: *mut otx_cptvf) -> i32;
    pub fn otx_cptvf_send_vf_to_grp_msg(cptvf: *mut otx_cptvf, group: i32) -> i32;
    pub fn otx_cptvf_send_vf_priority_msg(cptvf: *mut otx_cptvf) -> i32;
    pub fn otx_cptvf_send_vq_size_msg(cptvf: *mut otx_cptvf) -> i32;
    pub fn otx_cptvf_check_pf_ready(cptvf: *mut otx_cptvf) -> i32;
    pub fn otx_cptvf_handle_mbox_intr(cptvf: *mut otx_cptvf);
    pub fn otx_cptvf_write_vq_doorbell(cptvf: *mut otx_cptvf, val: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
