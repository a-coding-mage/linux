/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

// Dependency supplied by the corresponding SOF header.

/* ssc1: TINTE */
pub const SOF_DAI_INTEL_SSP_QUIRK_TINTE: u32 = 1u32 << 0;
/* ssc1: PINTE */
pub const SOF_DAI_INTEL_SSP_QUIRK_PINTE: u32 = 1u32 << 1;
/* ssc2: SMTATF */
pub const SOF_DAI_INTEL_SSP_QUIRK_SMTATF: u32 = 1u32 << 2;
/* ssc2: MMRATF */
pub const SOF_DAI_INTEL_SSP_QUIRK_MMRATF: u32 = 1u32 << 3;
/* ssc2: PSPSTWFDFD */
pub const SOF_DAI_INTEL_SSP_QUIRK_PSPSTWFDFD: u32 = 1u32 << 4;
/* ssc2: PSPSRWFDFD */
pub const SOF_DAI_INTEL_SSP_QUIRK_PSPSRWFDFD: u32 = 1u32 << 5;
/* ssc1: LBM */
pub const SOF_DAI_INTEL_SSP_QUIRK_LBM: u32 = 1u32 << 6;

/* here is the possibility to define others aux macros */

pub const SOF_DAI_INTEL_SSP_FRAME_PULSE_WIDTH_MAX: u32 = 38;
pub const SOF_DAI_INTEL_SSP_SLOT_PADDING_MAX: u32 = 31;

/* SSP clocks control settings
 *
 * Macros for clks_control field in sof_ipc_dai_ssp_params struct.
 */

/* mclk 0 disable */
pub const SOF_DAI_INTEL_SSP_MCLK_0_DISABLE: u32 = 1u32 << 0;
/* mclk 1 disable */
pub const SOF_DAI_INTEL_SSP_MCLK_1_DISABLE: u32 = 1u32 << 1;
/* mclk keep active */
pub const SOF_DAI_INTEL_SSP_CLKCTRL_MCLK_KA: u32 = 1u32 << 2;
/* bclk keep active */
pub const SOF_DAI_INTEL_SSP_CLKCTRL_BCLK_KA: u32 = 1u32 << 3;
/* fs keep active */
pub const SOF_DAI_INTEL_SSP_CLKCTRL_FS_KA: u32 = 1u32 << 4;
/* bclk idle */
pub const SOF_DAI_INTEL_SSP_CLKCTRL_BCLK_IDLE_HIGH: u32 = 1u32 << 5;
/* mclk early start */
pub const SOF_DAI_INTEL_SSP_CLKCTRL_MCLK_ES: u32 = 1u32 << 6;
/* bclk early start */
pub const SOF_DAI_INTEL_SSP_CLKCTRL_BCLK_ES: u32 = 1u32 << 7;
/* mclk always on */
pub const SOF_DAI_INTEL_SSP_CLKCTRL_MCLK_AON: u32 = 1u32 << 8;

/* DMIC max. four controllers for eight microphone channels */
pub const SOF_DAI_INTEL_DMIC_NUM_CTRL: usize = 4;

/* SSP Configuration Request - SOF_IPC_DAI_SSP_CONFIG */
#[repr(C, packed)]
pub struct sof_ipc_dai_ssp_params {
    pub hdr: sof_ipc_hdr,
    pub reserved1: u16,
    pub mclk_id: u16,
    pub mclk_rate: u32, /* mclk frequency in Hz */
    pub fsync_rate: u32, /* fsync frequency in Hz */
    pub bclk_rate: u32, /* bclk frequency in Hz */
    /* TDM */
    pub tdm_slots: u32,
    pub rx_slots: u32,
    pub tx_slots: u32,
    /* data */
    pub sample_valid_bits: u32,
    pub tdm_slot_width: u16,
    pub reserved2: u16, /* alignment */
    /* MCLK */
    pub mclk_direction: u32,
    pub frame_pulse_width: u16,
    pub tdm_per_slot_padding_flag: u16,
    pub clks_control: u32,
    pub quirks: u32,
    pub bclk_delay: u32, /* guaranteed time (ms) for which BCLK
                         * will be driven, before sending data
                         */
}

/* HDA Configuration Request - SOF_IPC_DAI_HDA_CONFIG */
#[repr(C, packed)]
pub struct sof_ipc_dai_hda_params {
    pub hdr: sof_ipc_hdr,
    pub link_dma_ch: u32,
    pub rate: u32,
    pub channels: u32,
}

/* ALH Configuration Request - SOF_IPC_DAI_ALH_CONFIG */
#[repr(C, packed)]
pub struct sof_ipc_dai_alh_params {
    pub hdr: sof_ipc_hdr,
    pub stream_id: u32,
    pub rate: u32,
    pub channels: u32,
    /* reserved for future use */
    pub reserved: [u32; 13],
}

/* DMIC Configuration Request - SOF_IPC_DAI_DMIC_CONFIG */

/* This struct is defined per 2ch PDM controller available in the platform.
 * Normally it is sufficient to set the used microphone specific enables to 1
 * and keep other parameters as zero. The customizations are:
 *
 * 1. If a device mixes different microphones types with different polarity
 * and/or the absolute polarity matters the PCM signal from a microphone
 * can be inverted with the controls.
 *
 * 2. If the microphones in a stereo pair do not appear in captured stream
 * in desired order due to board schematics choises they can be swapped with
 * the clk_edge parameter.
 *
 * 3. If PDM bit errors are seen in capture (poor quality) the skew parameter
 * that delays the sampling time of data by half cycles of DMIC source clock
 * can be tried for improvement. However there is no guarantee for this to fix
 * data integrity problems.
 */
#[repr(C, packed)]
pub struct sof_ipc_dai_dmic_pdm_ctrl {
    pub hdr: sof_ipc_hdr,
    pub id: u16, /* PDM controller ID */
    pub enable_mic_a: u16, /* Use A (left) channel mic (0 or 1)*/
    pub enable_mic_b: u16, /* Use B (right) channel mic (0 or 1)*/
    pub polarity_mic_a: u16, /* Optionally invert mic A signal (0 or 1) */
    pub polarity_mic_b: u16, /* Optionally invert mic B signal (0 or 1) */
    pub clk_edge: u16, /* Optionally swap data clock edge (0 or 1) */
    pub skew: u16, /* Adjust PDM data sampling vs. clock (0..15) */
    pub reserved: [u16; 3], /* Make sure the total size is 4 bytes aligned */
}

/* Global settings for all 2ch PDM controllers. */
#[repr(C, packed)]
pub struct sof_ipc_dai_dmic_params {
    pub hdr: sof_ipc_hdr,
    pub driver_ipc_version: u32, /* Version (1..N) */
    pub pdmclk_min: u32, /* Minimum microphone clock in Hz (100000..N) */
    pub pdmclk_max: u32, /* Maximum microphone clock in Hz (min...N) */
    pub fifo_fs: u32, /* FIFO sample rate in Hz (8000..96000) */
    pub reserved_1: u32, /* Reserved */
    pub fifo_bits: u16, /* FIFO word length (16 or 32) */
    pub fifo_bits_b: u16, /* Deprecated since firmware ABI 3.0.1 */
    pub duty_min: u16, /* Min. mic clock duty cycle in % (20..80) */
    pub duty_max: u16, /* Max. mic clock duty cycle in % (min..80) */
    pub num_pdm_active: u32, /* Number of active pdm controllers. */
    pub wake_up_time: u32, /* Time from clock start to data (us) */
    pub min_clock_on_time: u32, /* Min. time that clk is kept on (us) */
    pub unmute_ramp_time: u32, /* Length of logarithmic gain ramp (ms) */
    /* reserved for future use */
    pub reserved: [u32; 5],
    /* PDM controllers configuration */
    pub pdm: [sof_ipc_dai_dmic_pdm_ctrl; SOF_DAI_INTEL_DMIC_NUM_CTRL],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
