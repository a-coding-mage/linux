/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

// Dependency supplied by the corresponding SOF header translation:
// sound/sof/header.h

/*
 * PM
 */

/* PM context element */
#[repr(C, packed)]
pub struct sof_ipc_pm_ctx_elem {
    pub hdr: sof_ipc_hdr,
    pub type_: u32,
    pub size: u32,
    pub addr: u64,
}

/*
 * PM context - SOF_IPC_PM_CTX_SAVE, SOF_IPC_PM_CTX_RESTORE,
 * SOF_IPC_PM_CTX_SIZE
 */
#[repr(C, packed)]
pub struct sof_ipc_pm_ctx {
    pub hdr: sof_ipc_cmd_hdr,
    pub buffer: sof_ipc_host_buffer,
    pub num_elems: u32,
    pub size: u32,

    /* reserved for future use */
    pub reserved: [u32; 8],

    pub elems: [sof_ipc_pm_ctx_elem; 0],
}

/* enable or disable cores - SOF_IPC_PM_CORE_ENABLE */
#[repr(C, packed)]
pub struct sof_ipc_pm_core_config {
    pub hdr: sof_ipc_cmd_hdr,
    pub enable_mask: u32,
}

#[repr(C, packed)]
pub struct sof_ipc_pm_gate {
    pub hdr: sof_ipc_cmd_hdr,
    pub flags: u32, /* platform specific */

    /* reserved for future use */
    pub reserved: [u32; 5],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
