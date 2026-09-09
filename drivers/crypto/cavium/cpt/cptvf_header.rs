/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Cavium, Inc.
 */

// Dependencies supplied by the Linux kernel and cpt_common.h are intentionally
// left as external names.

/* Default command queue length */
pub const CPT_CMD_QLEN: u32 = 2046;
pub const CPT_CMD_QCHUNK_SIZE: u32 = 1023;

/* Default command timeout in seconds */
pub const CPT_COMMAND_TIMEOUT: u32 = 4;
pub const CPT_TIMER_THOLD: u32 = 0xFFFF;
pub const CPT_NUM_QS_PER_VF: usize = 1;
pub const CPT_INST_SIZE: u32 = 64;
pub const CPT_NEXT_CHUNK_PTR_SIZE: u32 = 8;

pub const CPT_VF_MSIX_VECTORS: usize = 2;
pub const CPT_VF_INTR_MBOX_MASK: u32 = 1 << 0;
pub const CPT_VF_INTR_DOVF_MASK: u32 = 1 << 1;
pub const CPT_VF_INTR_IRDE_MASK: u32 = 1 << 2;
pub const CPT_VF_INTR_NWRP_MASK: u32 = 1 << 3;
pub const CPT_VF_INTR_SERR_MASK: u32 = 1 << 4;
pub const DMA_DIRECT_DIRECT: u32 = 0; // Input DIRECT, Output DIRECT
pub const DMA_GATHER_SCATTER: u32 = 1;
pub const FROM_DPTR: u32 = 1;

/**
 * Enumeration cpt_vf_int_vec_e
 *
 * CPT VF MSI-X Vector Enumeration
 * Enumerates the MSI-X interrupt vectors.
 */
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum cpt_vf_int_vec_e {
    CPT_VF_INT_VEC_E_MISC = 0x00,
    CPT_VF_INT_VEC_E_DONE = 0x01,
}

#[repr(C)]
pub struct command_chunk {
    pub head: *mut u8,
    pub dma_addr: dma_addr_t,
    pub size: u32, // Chunk size, max CPT_INST_CHUNK_MAX_SIZE
    pub nextchunk: hlist_node,
}

#[repr(C)]
pub struct command_queue {
    pub lock: spinlock_t, // command queue lock
    pub idx: u32, // Command queue host write idx
    pub nchunks: u32, // Number of command chunks
    pub qhead: *mut command_chunk, // Command queue head, instructions are inserted here
    pub chead: hlist_head,
}

#[repr(C)]
pub struct command_qinfo {
    pub cmd_size: u32,
    pub qchunksize: u32, // Command queue chunk size
    pub queue: [command_queue; CPT_NUM_QS_PER_VF],
}

#[repr(C)]
pub struct pending_entry {
    pub busy: u8, // Entry status (free/busy)
    pub completion_addr: *mut u64, // Completion address
    pub post_arg: *mut core::ffi::c_void,
    pub callback: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void)>, // Kernel ASYNC request callabck
    pub callback_arg: *mut core::ffi::c_void, // Kernel ASYNC request callabck arg
}

#[repr(C)]
pub struct pending_queue {
    pub head: *mut pending_entry, // head of the queue
    pub front: u32, // Process work from here
    pub rear: u32, // Append new work here
    pub pending_count: atomic64_t,
    pub lock: spinlock_t, // Queue lock
}

#[repr(C)]
pub struct pending_qinfo {
    pub nr_queues: u32, // Number of queues supported
    pub qlen: u32, // Queue length
    pub queue: [pending_queue; CPT_NUM_QS_PER_VF],
}

#[macro_export]
macro_rules! for_each_pending_queue {
    ($qinfo:expr, $q:ident, $i:ident, $body:block) => {
        for ($i, $q) in (0usize, &mut $qinfo.queue[0]); $i < $qinfo.nr_queues as usize; $i += 1, $q = &mut $qinfo.queue[$i]) $body
    };
}

#[repr(C)]
pub struct cpt_vf {
    pub flags: u16, // Flags to hold device status bits
    pub vfid: u8, // Device Index 0...CPT_MAX_VF_NUM
    pub vftype: u8, // VF type of SE_TYPE(1) or AE_TYPE(1)
    pub vfgrp: u8, // VF group (0 - 8)
    pub node: u8, // Operating node: Bits (46:44) in BAR0 address
    pub priority: u8, // VF priority ring: 1-High proirity round robin ring;0-Low priority round robin ring;
    pub pdev: *mut pci_dev, // pci device handle
    pub reg_base: *mut core::ffi::c_void, // Register start address
    pub wqe_info: *mut core::ffi::c_void, // BH worker info
    pub affinity_mask: [cpumask_var_t; CPT_VF_MSIX_VECTORS],
    pub qsize: u32,
    pub nr_queues: u32,
    pub cqinfo: command_qinfo, // Command queue information
    pub pqinfo: pending_qinfo, // Pending queue information
    pub pf_acked: bool,
    pub pf_nacked: bool,
}

extern "C" {
    pub fn cptvf_send_vf_up(cptvf: *mut cpt_vf) -> i32;
    pub fn cptvf_send_vf_down(cptvf: *mut cpt_vf) -> i32;
    pub fn cptvf_send_vf_to_grp_msg(cptvf: *mut cpt_vf) -> i32;
    pub fn cptvf_send_vf_priority_msg(cptvf: *mut cpt_vf) -> i32;
    pub fn cptvf_send_vq_size_msg(cptvf: *mut cpt_vf) -> i32;
    pub fn cptvf_check_pf_ready(cptvf: *mut cpt_vf) -> i32;
    pub fn cptvf_handle_mbox_intr(cptvf: *mut cpt_vf);
    pub fn cvm_crypto_exit();
    pub fn cvm_crypto_init(cptvf: *mut cpt_vf) -> i32;
    pub fn vq_post_process(cptvf: *mut cpt_vf, qno: u32);
    pub fn cptvf_write_vq_doorbell(cptvf: *mut cpt_vf, val: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
