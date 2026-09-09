/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2020 Intel Corporation
 *
 * Author: Karol Trzcinski <karolx.trzcinski@linux.intel.com>
 */

// Dependency: sof_ipc_reply is supplied by sound/sof/header.h.

/** ABI3.18 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum sof_ipc_dbg_mem_zone {
    /// System zone
    SOF_IPC_MEM_ZONE_SYS = 0,
    /// System-runtime zone
    SOF_IPC_MEM_ZONE_SYS_RUNTIME = 1,
    /// Runtime zone
    SOF_IPC_MEM_ZONE_RUNTIME = 2,
    /// Buffer zone
    SOF_IPC_MEM_ZONE_BUFFER = 3,
    /// System runtime zone
    SOF_IPC_MEM_ZONE_RUNTIME_SHARED = 4,
    /// System shared zone
    SOF_IPC_MEM_ZONE_SYS_SHARED = 5,
}

/** ABI3.18 */
#[repr(C, packed)]
pub struct sof_ipc_dbg_mem_usage_elem {
    /// see sof_ipc_dbg_mem_zone
    pub zone: u32,
    /// heap index within zone
    pub id: u32,
    /// number of bytes used in zone
    pub used: u32,
    /// number of bytes free to use within zone
    pub free: u32,
    /// for future use
    pub reserved: u32,
}

/** ABI3.18 */
#[repr(C, packed)]
pub struct sof_ipc_dbg_mem_usage {
    /// generic IPC reply header
    pub rhdr: sof_ipc_reply,
    /// reserved for future use
    pub reserved: [u32; 4],
    /// elems[] counter
    pub num_elems: u32,
    /// memory usage information
    pub elems: [sof_ipc_dbg_mem_usage_elem; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
