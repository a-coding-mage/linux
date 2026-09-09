/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * SPU info structures
 *
 * (C) Copyright 2006 IBM Corp.
 *
 * Author: Dwayne Grant McConnell <decimal@us.ibm.com>
 */

// C dependency: <linux/types.h> supplies __u64; Rust's u64 has the same
// fixed-width integer representation.

#[repr(C)]
pub struct MfcCqSr {
    pub mfc_cq_data0_RW: u64,
    pub mfc_cq_data1_RW: u64,
    pub mfc_cq_data2_RW: u64,
    pub mfc_cq_data3_RW: u64,
}

#[repr(C)]
pub struct SpuDmaInfo {
    pub dma_info_type: u64,
    pub dma_info_mask: u64,
    pub dma_info_status: u64,
    pub dma_info_stall_and_notify: u64,
    pub dma_info_atomic_command_status: u64,
    pub dma_info_command_data: [MfcCqSr; 16],
}

#[repr(C)]
pub struct SpuProxydmaInfo {
    pub proxydma_info_type: u64,
    pub proxydma_info_mask: u64,
    pub proxydma_info_status: u64,
    pub proxydma_info_command_data: [MfcCqSr; 8],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
