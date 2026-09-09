/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Translated from aie4_host_queue.h.
// The original header depends on Linux fixed-width integer types.

pub const CTX_MAX_CMDS: u32 = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HostQueueHeaderVersion {
    pub major: u16,
    pub minor: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HostQueueHeader {
    pub read_index: u64,
    pub version: HostQueueHeaderVersion,
    pub capacity: u32, /* Queue capacity, must be power of two. */
    pub write_index: u64,
    pub data_address: u64, /* The xdna dev addr for payload. */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
