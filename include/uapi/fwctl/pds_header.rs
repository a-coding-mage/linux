/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright(c) Advanced Micro Devices, Inc */

/*
 * fwctl interface info for pds_fwctl
 */

/* Dependency equivalent of <linux/types.h>. */

/**
 * struct fwctl_info_pds
 * @uctx_caps:  bitmap of firmware capabilities
 *
 * Return basic information about the FW interface available.
 */
#[repr(C)]
pub struct fwctl_info_pds {
    pub uctx_caps: u32,
}

/**
 * enum pds_fwctl_capabilities
 * @PDS_FWCTL_QUERY_CAP: firmware can be queried for information
 * @PDS_FWCTL_SEND_CAP:  firmware can be sent commands
 */
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum pds_fwctl_capabilities {
    PDS_FWCTL_QUERY_CAP = 0,
    PDS_FWCTL_SEND_CAP = 1,
}

/**
 * struct fwctl_rpc_pds
 * @in.op:       requested operation code
 * @in.ep:       firmware endpoint to operate on
 * @in.rsvd:     reserved
 * @in.len:      length of payload data
 * @in.payload:  address of payload buffer
 * @in:          rpc in parameters
 * @out.retval:  operation result value
 * @out.rsvd:    reserved
 * @out.len:      length of result data buffer
 * @out.payload: address of payload data buffer
 * @out:         rpc out parameters
 */
#[repr(C)]
pub struct fwctl_rpc_pds {
    pub in_: fwctl_rpc_pds_in,
    pub out: fwctl_rpc_pds_out,
}

#[repr(C)]
pub struct fwctl_rpc_pds_in {
    pub op: u32,
    pub ep: u32,
    pub rsvd: u32,
    pub len: u32,
    pub payload: u64,
}

#[repr(C)]
pub struct fwctl_rpc_pds_out {
    pub retval: u32,
    pub rsvd: [u32; 2],
    pub len: u32,
    pub payload: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
