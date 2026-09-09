/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license. When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2021 Advanced Micro Devices, Inc.. All rights reserved.
 */

// Dependency supplied by the surrounding SOF headers.

/* ACP Configuration Request - SOF_IPC_DAI_AMD_CONFIG */
#[repr(C, packed)]
pub struct sof_ipc_dai_acp_params {
    pub hdr: sof_ipc_hdr,

    pub fsync_rate: u32, /* FSYNC frequency in Hz */
    pub tdm_slots: u32,
    pub tdm_mode: u32,
    pub format: u32,
}

/* ACPDMIC Configuration Request - SOF_IPC_DAI_AMD_CONFIG */
#[repr(C, packed)]
pub struct sof_ipc_dai_acpdmic_params {
    pub pdm_rate: u32,
    pub pdm_ch: u32,
}

/* ACP_SDW Configuration Request - SOF_IPC_DAI_AMD_SDW_CONFIG */
#[repr(C, packed)]
pub struct sof_ipc_dai_acp_sdw_params {
    pub hdr: sof_ipc_hdr,
    pub rate: u32,
    pub channels: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
