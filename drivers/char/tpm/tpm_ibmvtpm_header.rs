/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 IBM Corporation
 *
 * Author: Ashley Lai <ashleydlai@gmail.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * Device driver for TCG/TCPA TPM (trusted platform module).
 * Specifications at www.trustedcomputinggroup.org
 */

/* vTPM Message Format 1 */
#[repr(C, packed(8))]
pub struct ibmvtpm_crq {
    pub valid: u8,
    pub msg: u8,
    pub len: __be16,
    pub data: __be32,
    pub reserved: __be64,
}

#[repr(C)]
pub struct ibmvtpm_crq_queue {
    pub crq_addr: *mut ibmvtpm_crq,
    pub index: u32,
    pub num_entry: u32,
    pub wq: wait_queue_head_t,
}

#[repr(C)]
pub struct ibmvtpm_dev {
    pub dev: *mut device,
    pub vdev: *mut vio_dev,
    pub crq_queue: ibmvtpm_crq_queue,
    pub crq_dma_handle: dma_addr_t,
    pub rtce_size: u32,
    pub rtce_buf: *mut core::ffi::c_void,
    pub rtce_dma_handle: dma_addr_t,
    pub rtce_lock: spinlock_t,
    pub wq: wait_queue_head_t,
    pub res_len: u16,
    pub vtpm_version: u32,
    pub tpm_processing_cmd: u8,
}

pub const CRQ_RES_BUF_SIZE: usize = PAGE_SIZE;

/* Initialize CRQ */
pub const INIT_CRQ_CMD: u64 = 0xC001000000000000; /* Init cmd */
pub const INIT_CRQ_COMP_CMD: u64 = 0xC002000000000000; /* Init complete cmd */
pub const INIT_CRQ_RES: u8 = 0x01; /* Init respond */
pub const INIT_CRQ_COMP_RES: u8 = 0x02; /* Init complete respond */
pub const VALID_INIT_CRQ: u8 = 0xC0; /* Valid command for init crq */

/* vTPM CRQ response is the message type | 0x80 */
pub const VTPM_MSG_RES: u8 = 0x80;
pub const IBMVTPM_VALID_CMD: u8 = 0x80;

/* vTPM CRQ message types */
pub const VTPM_GET_VERSION: u8 = 0x01;
pub const VTPM_GET_VERSION_RES: u8 = 0x01 | VTPM_MSG_RES;

pub const VTPM_TPM_COMMAND: u8 = 0x02;
pub const VTPM_TPM_COMMAND_RES: u8 = 0x02 | VTPM_MSG_RES;

pub const VTPM_GET_RTCE_BUFFER_SIZE: u8 = 0x03;
pub const VTPM_GET_RTCE_BUFFER_SIZE_RES: u8 = 0x03 | VTPM_MSG_RES;

pub const VTPM_PREPARE_TO_SUSPEND: u8 = 0x04;
pub const VTPM_PREPARE_TO_SUSPEND_RES: u8 = 0x04 | VTPM_MSG_RES;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
