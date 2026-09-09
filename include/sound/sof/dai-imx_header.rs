/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * Copyright 2019 NXP
 *
 * Author: Daniel Baluta <daniel.baluta@nxp.com>
 */

// Dependency supplied by the corresponding SOF header.

/* ESAI Configuration Request - SOF_IPC_DAI_ESAI_CONFIG */
#[repr(C, packed)]
pub struct sof_ipc_dai_esai_params {
    pub hdr: sof_ipc_hdr,

    /* MCLK */
    pub reserved1: u16,
    pub mclk_id: u16,
    pub mclk_direction: u32,

    pub mclk_rate: u32, /* MCLK frequency in Hz */
    pub fsync_rate: u32, /* FSYNC frequency in Hz */
    pub bclk_rate: u32, /* BCLK frequency in Hz */

    /* TDM */
    pub tdm_slots: u32,
    pub rx_slots: u32,
    pub tx_slots: u32,
    pub tdm_slot_width: u16,
    pub reserved2: u16, /* alignment */
}

/* SAI Configuration Request - SOF_IPC_DAI_SAI_CONFIG */
#[repr(C, packed)]
pub struct sof_ipc_dai_sai_params {
    pub hdr: sof_ipc_hdr,

    /* MCLK */
    pub reserved1: u16,
    pub mclk_id: u16,
    pub mclk_direction: u32,

    pub mclk_rate: u32, /* MCLK frequency in Hz */
    pub fsync_rate: u32, /* FSYNC frequency in Hz */
    pub bclk_rate: u32, /* BCLK frequency in Hz */

    /* TDM */
    pub tdm_slots: u32,
    pub rx_slots: u32,
    pub tx_slots: u32,
    pub tdm_slot_width: u16,
    pub reserved2: u16, /* alignment */
}

/* MICFIL Configuration Request - SOF_IPC_DAI_MICFIL_CONFIG */
#[repr(C, packed)]
pub struct sof_ipc_dai_micfil_params {
    pub pdm_rate: u32,
    pub pdm_ch: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
