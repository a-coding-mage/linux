/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (c) 2024-2025 Intel Corporation
 *
 * These are definitions for the mailbox command interface of CXL subsystem.
 */

// Dependency intent preserved from <linux/types.h>, <linux/stddef.h>, and
// <cxl/features.h>.

/**
 * struct fwctl_rpc_cxl - ioctl(FWCTL_RPC) input for CXL
 * @opcode: CXL mailbox command opcode
 * @flags: Flags for the command (input).
 * @op_size: Size of input payload.
 * @reserved1: Reserved. Must be 0s.
 * @get_sup_feats_in: Get Supported Features input
 * @get_feat_in: Get Feature input
 * @set_feat_in: Set Feature input
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_rpc_cxl_hdr {
    pub opcode: u32,
    pub flags: u32,
    pub op_size: u32,
    pub reserved1: u32,
}

#[repr(C)]
pub union fwctl_rpc_cxl_payload {
    pub get_sup_feats_in: cxl_mbox_get_sup_feats_in,
    pub get_feat_in: cxl_mbox_get_feat_in,
    pub set_feat_in: cxl_mbox_set_feat_in,
}

#[repr(C)]
pub struct fwctl_rpc_cxl {
    pub opcode: u32,
    pub flags: u32,
    pub op_size: u32,
    pub reserved1: u32,
    pub payload: fwctl_rpc_cxl_payload,
}

/**
 * struct fwctl_rpc_cxl_out - ioctl(FWCTL_RPC) output for CXL
 * @size: Size of the output payload
 * @retval: Return value from device
 * @get_sup_feats_out: Get Supported Features output
 * @payload: raw byte stream of payload
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_rpc_cxl_out_hdr {
    pub size: u32,
    pub retval: u32,
}

#[repr(C)]
pub union fwctl_rpc_cxl_out_payload {
    pub get_sup_feats_out: cxl_mbox_get_sup_feats_out,
    pub payload: [u8; 0],
}

#[repr(C)]
pub struct fwctl_rpc_cxl_out {
    pub size: u32,
    pub retval: u32,
    pub payload: fwctl_rpc_cxl_out_payload,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
