/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2016 Broadcom
 *
 * Common header for Broadcom mailbox messages which is shared across
 * Broadcom SoCs and Broadcom mailbox client drivers.
 */

// Dependency supplied by the Linux scatterlist headers in the containing build.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum brcm_message_type {
    BRCM_MESSAGE_UNKNOWN = 0,
    BRCM_MESSAGE_BATCH,
    BRCM_MESSAGE_SPU,
    BRCM_MESSAGE_SBA,
    BRCM_MESSAGE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_sba_command {
    pub cmd: u64,
    pub cmd_dma: *mut u64,
    pub cmd_dma_addr: dma_addr_t,
    pub flags: u64,
    pub resp: dma_addr_t,
    pub resp_len: usize,
    pub data: dma_addr_t,
    pub data_len: usize,
}

pub const BRCM_SBA_CMD_TYPE_A: u64 = 1 << 0;
pub const BRCM_SBA_CMD_TYPE_B: u64 = 1 << 1;
pub const BRCM_SBA_CMD_TYPE_C: u64 = 1 << 2;
pub const BRCM_SBA_CMD_HAS_RESP: u64 = 1 << 3;
pub const BRCM_SBA_CMD_HAS_OUTPUT: u64 = 1 << 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_message_batch {
    pub msgs: *mut brcm_message,
    pub msgs_queued: ::core::ffi::c_uint,
    pub msgs_count: ::core::ffi::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_message_spu {
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_message_sba {
    pub cmds: *mut brcm_sba_command,
    pub cmds_count: ::core::ffi::c_uint,
}

#[repr(C)]
pub union brcm_message_data {
    pub batch: brcm_message_batch,
    pub spu: brcm_message_spu,
    pub sba: brcm_message_sba,
}

#[repr(C)]
pub struct brcm_message {
    pub type_: brcm_message_type,
    pub data: brcm_message_data,
    pub ctx: *mut ::core::ffi::c_void,
    pub error: ::core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
