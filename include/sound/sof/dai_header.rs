/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

// Dependencies supplied by the corresponding SOF headers are intentionally
// referenced here without reimplementing them.

pub const SOF_DAI_FMT_I2S: u32 = 1; // I2S mode
pub const SOF_DAI_FMT_RIGHT_J: u32 = 2; // Right Justified mode
pub const SOF_DAI_FMT_LEFT_J: u32 = 3; // Left Justified mode
pub const SOF_DAI_FMT_DSP_A: u32 = 4; // L data MSB after FRM LRC
pub const SOF_DAI_FMT_DSP_B: u32 = 5; // L data MSB during FRM LRC
pub const SOF_DAI_FMT_PDM: u32 = 6; // Pulse density modulation

pub const SOF_DAI_FMT_CONT: u32 = 1 << 4; // continuous clock
pub const SOF_DAI_FMT_GATED: u32 = 0 << 4; // clock is gated

pub const SOF_DAI_FMT_NB_NF: u32 = 0 << 8; // normal bit clock + frame
pub const SOF_DAI_FMT_NB_IF: u32 = 2 << 8; // normal BCLK + inv FRM
pub const SOF_DAI_FMT_IB_NF: u32 = 3 << 8; // invert BCLK + nor FRM
pub const SOF_DAI_FMT_IB_IF: u32 = 4 << 8; // invert BCLK + FRM

pub const SOF_DAI_FMT_CBP_CFP: u32 = 0 << 12; // codec bclk provider & frame provider
pub const SOF_DAI_FMT_CBC_CFP: u32 = 2 << 12; // codec bclk consumer & frame provider
pub const SOF_DAI_FMT_CBP_CFC: u32 = 3 << 12; // codec bclk provider & frame consumer
pub const SOF_DAI_FMT_CBC_CFC: u32 = 4 << 12; // codec bclk consumer & frame consumer

// Keep old definitions for backwards compatibility.
pub const SOF_DAI_FMT_CBM_CFM: u32 = SOF_DAI_FMT_CBP_CFP;
pub const SOF_DAI_FMT_CBS_CFM: u32 = SOF_DAI_FMT_CBC_CFP;
pub const SOF_DAI_FMT_CBM_CFS: u32 = SOF_DAI_FMT_CBP_CFC;
pub const SOF_DAI_FMT_CBS_CFS: u32 = SOF_DAI_FMT_CBC_CFC;

pub const SOF_DAI_FMT_FORMAT_MASK: u32 = 0x000f;
pub const SOF_DAI_FMT_CLOCK_MASK: u32 = 0x00f0;
pub const SOF_DAI_FMT_INV_MASK: u32 = 0x0f00;
pub const SOF_DAI_FMT_CLOCK_PROVIDER_MASK: u32 = 0xf000;

pub const SOF_DAI_CONFIG_FLAGS_CMD_MASK: u32 = 0xF;
pub const SOF_DAI_CONFIG_FLAGS_NONE: u32 = 0;
pub const SOF_DAI_CONFIG_FLAGS_HW_PARAMS: u32 = 1 << 0;
pub const SOF_DAI_CONFIG_FLAGS_HW_FREE: u32 = 1 << 1;
// DAI_CONFIG sent during pause trigger. Only available ABI 3.20 onwards.
pub const SOF_DAI_CONFIG_FLAGS_PAUSE: u32 = 1 << 2;
pub const SOF_DAI_CONFIG_FLAGS_QUIRK_SHIFT: u32 = 4;
pub const SOF_DAI_CONFIG_FLAGS_QUIRK_MASK: u32 = 0xF << SOF_DAI_CONFIG_FLAGS_QUIRK_SHIFT;
// Used with SOF_DAI_CONFIG_FLAGS_HW_PARAMS for two-step pipeline/DAI DMA stop/pause.
pub const SOF_DAI_CONFIG_FLAGS_2_STEP_STOP: u32 = 1 << 0;

#[repr(u32)]
pub enum sof_ipc_dai_type {
    SOF_DAI_INTEL_NONE = 0,
    SOF_DAI_INTEL_SSP,
    SOF_DAI_INTEL_DMIC,
    SOF_DAI_INTEL_HDA,
    SOF_DAI_INTEL_ALH,
    SOF_DAI_IMX_SAI,
    SOF_DAI_IMX_ESAI,
    SOF_DAI_AMD_BT,
    SOF_DAI_AMD_SP,
    SOF_DAI_AMD_DMIC,
    SOF_DAI_MEDIATEK_AFE,
    SOF_DAI_AMD_HS,
    SOF_DAI_AMD_SP_VIRTUAL,
    SOF_DAI_AMD_HS_VIRTUAL,
    SOF_DAI_IMX_MICFIL,
    SOF_DAI_AMD_SDW,
    SOF_DAI_INTEL_UAOL,
    SOF_DAI_AMD_I2S,
}

#[repr(C)]
pub union sof_ipc_dai_config_hw_specific {
    pub ssp: ::core::mem::ManuallyDrop<sof_ipc_dai_ssp_params>,
    pub dmic: ::core::mem::ManuallyDrop<sof_ipc_dai_dmic_params>,
    pub hda: ::core::mem::ManuallyDrop<sof_ipc_dai_hda_params>,
    pub alh: ::core::mem::ManuallyDrop<sof_ipc_dai_alh_params>,
    pub esai: ::core::mem::ManuallyDrop<sof_ipc_dai_esai_params>,
    pub sai: ::core::mem::ManuallyDrop<sof_ipc_dai_sai_params>,
    pub acpbt: ::core::mem::ManuallyDrop<sof_ipc_dai_acp_params>,
    pub acpsp: ::core::mem::ManuallyDrop<sof_ipc_dai_acp_params>,
    pub acpdmic: ::core::mem::ManuallyDrop<sof_ipc_dai_acpdmic_params>,
    pub acphs: ::core::mem::ManuallyDrop<sof_ipc_dai_acp_params>,
    pub afe: ::core::mem::ManuallyDrop<sof_ipc_dai_mtk_afe_params>,
    pub micfil: ::core::mem::ManuallyDrop<sof_ipc_dai_micfil_params>,
    pub acp_sdw: ::core::mem::ManuallyDrop<sof_ipc_dai_acp_sdw_params>,
    pub acp_i2s: ::core::mem::ManuallyDrop<sof_ipc_dai_acp_params>,
}

#[repr(C, packed)]
pub struct sof_ipc_dai_config {
    pub hdr: sof_ipc_cmd_hdr,
    pub type_: u32,
    pub dai_index: u32,
    pub format: u16,
    pub group_id: u8,
    pub flags: u8,
    pub reserved: [u32; 8],
    pub hw_specific: sof_ipc_dai_config_hw_specific,
}

#[repr(C)]
pub struct sof_dai_private_data {
    pub comp_dai: *mut sof_ipc_comp_dai,
    pub dai_config: *mut sof_ipc_dai_config,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
