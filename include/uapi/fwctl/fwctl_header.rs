/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES. */

// C dependencies: <linux/types.h> and <linux/ioctl.h>.

pub const FWCTL_TYPE: u32 = 0x9A;

pub const FWCTL_CMD_BASE: u32 = 0;
pub const FWCTL_CMD_INFO: u32 = 0;
pub const FWCTL_CMD_RPC: u32 = 1;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum fwctl_device_type {
    FWCTL_DEVICE_TYPE_ERROR = 0,
    FWCTL_DEVICE_TYPE_MLX5 = 1,
    FWCTL_DEVICE_TYPE_CXL = 2,
    FWCTL_DEVICE_TYPE_BNXT = 3,
    FWCTL_DEVICE_TYPE_PDS = 4,
}

/**
 * struct fwctl_info - ioctl(FWCTL_INFO)
 * @size: sizeof(struct fwctl_info)
 * @flags: Must be 0
 * @out_device_type: Returns the type of the device from enum fwctl_device_type
 * @device_data_len: On input the length of the out_device_data memory. On
 * output the size of the kernel's device_data which may be larger or
 * smaller than the input. Maybe 0 on input.
 * @out_device_data: Pointer to a memory of device_data_len bytes. Kernel will
 * fill the entire memory, zeroing as required.
 *
 * Returns basic information about this fwctl instance, particularly what driver
 * is being used to define the device_data format.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_info {
    pub size: u32,
    pub flags: u32,
    pub out_device_type: u32,
    pub device_data_len: u32,
    pub out_device_data: u64,
}

// _IO(FWCTL_TYPE, FWCTL_CMD_INFO), from Linux's ioctl encoding.
pub const FWCTL_INFO: u32 = FWCTL_TYPE | (FWCTL_CMD_INFO << 8);

/**
 * enum fwctl_rpc_scope - Scope of access for the RPC
 *
 * Refer to fwctl.rst for a more detailed discussion of these scopes.
 */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum fwctl_rpc_scope {
    /** Device configuration access scope */
    FWCTL_RPC_CONFIGURATION = 0,
    /** Read only access to debug information */
    FWCTL_RPC_DEBUG_READ_ONLY = 1,
    /** Writable access to lockdown compatible debug information */
    FWCTL_RPC_DEBUG_WRITE = 2,
    /** Write access to all debug information */
    FWCTL_RPC_DEBUG_WRITE_FULL = 3,
}

/**
 * struct fwctl_rpc - ioctl(FWCTL_RPC)
 * @size: sizeof(struct fwctl_rpc)
 * @scope: One of enum fwctl_rpc_scope, required scope for the RPC
 * @in_len: Length of the in memory
 * @out_len: Length of the out memory
 * @in: Request message in device specific format
 * @out: Response message in device specific format
 *
 * Deliver a Remote Procedure Call to the device FW and return the response. The
 * call's parameters and return are marshaled into linear buffers of memory. Any
 * errno indicates that delivery of the RPC to the device failed. Return status
 * originating in the device during a successful delivery must be encoded into
 * out.
 *
 * The format of the buffers matches the out_device_type from FWCTL_INFO.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_rpc {
    pub size: u32,
    pub scope: u32,
    pub in_len: u32,
    pub out_len: u32,
    pub in_: u64,
    pub out: u64,
}

// _IO(FWCTL_TYPE, FWCTL_CMD_RPC), from Linux's ioctl encoding.
pub const FWCTL_RPC: u32 = FWCTL_TYPE | (FWCTL_CMD_RPC << 8);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
