/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2019,2021 Advanced Micro Devices, Inc.
 *
 * Author: Rijo Thomas <Rijo-john.Thomas@amd.com>
 * Author: Devaraj Rangasamy <Devaraj.Rangasamy@amd.com>
 *
 */

/* This file describes the TEE communication interface between host and AMD
 * Secure Processor
 */

// Dependencies supplied by the surrounding kernel translation.

pub const TEE_DEFAULT_CMD_TIMEOUT: u32 = 10 * MSEC_PER_SEC;
pub const TEE_DEFAULT_RING_TIMEOUT: u32 = 10;
pub const MAX_BUFFER_SIZE: usize = 988;

/**
 * struct tee_init_ring_cmd - Command to init TEE ring buffer
 * @low_addr:  bits [31:0] of the physical address of ring buffer
 * @hi_addr:   bits [63:32] of the physical address of ring buffer
 * @size:      size of ring buffer in bytes
 */
#[repr(C)]
pub struct tee_init_ring_cmd {
    pub low_addr: u32,
    pub hi_addr: u32,
    pub size: u32,
}

pub const MAX_RING_BUFFER_ENTRIES: usize = 32;

/**
 * struct ring_buf_manager - Helper structure to manage ring buffer.
 * @ring_start:  starting address of ring buffer
 * @ring_size:   size of ring buffer in bytes
 * @ring_pa:     physical address of ring buffer
 * @wptr:        index to the last written entry in ring buffer
 */
#[repr(C)]
pub struct ring_buf_manager {
    pub mutex: mutex, /* synchronizes access to ring buffer */
    pub ring_start: *mut core::ffi::c_void,
    pub ring_size: u32,
    pub ring_pa: phys_addr_t,
    pub wptr: u32,
}

#[repr(C)]
pub struct psp_tee_device {
    pub dev: *mut device,
    pub psp: *mut psp_device,
    pub io_regs: *mut core::ffi::c_void,
    pub vdata: *mut tee_vdata,
    pub rb_mgr: ring_buf_manager,
}

/**
 * enum tee_cmd_state - TEE command states for the ring buffer interface
 * @TEE_CMD_STATE_INIT:      initial state of command when sent from host
 * @TEE_CMD_STATE_PROCESS:   command being processed by TEE environment
 * @TEE_CMD_STATE_COMPLETED: command processing completed
 */
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum tee_cmd_state {
    TEE_CMD_STATE_INIT,
    TEE_CMD_STATE_PROCESS,
    TEE_CMD_STATE_COMPLETED,
}

/**
 * enum cmd_resp_state - TEE command's response status maintained by driver
 * @CMD_RESPONSE_INVALID:      initial state when no command is written to ring
 * @CMD_WAITING_FOR_RESPONSE:  driver waiting for response from TEE
 * @CMD_RESPONSE_TIMEDOUT:     failed to get response from TEE
 * @CMD_RESPONSE_COPIED:       driver has copied response from TEE
 */
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum cmd_resp_state {
    CMD_RESPONSE_INVALID,
    CMD_WAITING_FOR_RESPONSE,
    CMD_RESPONSE_TIMEDOUT,
    CMD_RESPONSE_COPIED,
}

/**
 * struct tee_ring_cmd - Structure of the command buffer in TEE ring
 * @cmd_id:      refers to &enum tee_cmd_id. Command id for the ring buffer
 *               interface
 * @cmd_state:   refers to &enum tee_cmd_state
 * @status:      status of TEE command execution
 * @res0:        reserved region
 * @pdata:       private data (currently unused)
 * @res1:        reserved region
 * @buf:         TEE command specific buffer
 * @flag:        refers to &enum cmd_resp_state
 */
#[repr(C, packed)]
pub struct tee_ring_cmd {
    pub cmd_id: u32,
    pub cmd_state: u32,
    pub status: u32,
    pub res0: [u32; 1],
    pub pdata: u64,
    pub res1: [u32; 2],
    pub buf: [u8; MAX_BUFFER_SIZE],
    pub flag: u32,

    /* Total size: 1024 bytes */
}

extern "C" {
    pub fn tee_dev_init(psp: *mut psp_device) -> i32;
    pub fn tee_dev_destroy(psp: *mut psp_device);
    pub fn tee_restore(psp: *mut psp_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
