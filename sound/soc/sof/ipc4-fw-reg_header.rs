/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2022 Intel Corporation
 */

pub const SOF_IPC4_INVALID_STREAM_POSITION: u64 = u64::MAX;

/**
 * struct sof_ipc4_pipeline_registers - Pipeline start and end information in fw
 * @stream_start_offset: Stream start offset (LPIB) reported by mixin
 * module allocated on pipeline attached to Host Output Gateway when
 * first data is being mixed to mixout module. When data is not mixed
 * (right after creation/after reset) value "(u64)-1" is reported
 * @stream_end_offset: Stream end offset (LPIB) reported by mixin
 * module allocated on pipeline attached to Host Output Gateway
 * during transition from RUNNING to PAUSED. When data is not mixed
 * (right after creation or after reset) value "(u64)-1" is reported.
 * When first data is mixed then value "0"is reported.
 */
#[repr(C, packed(4))]
pub struct sof_ipc4_pipeline_registers {
    pub stream_start_offset: u64,
    pub stream_end_offset: u64,
}

pub const SOF_IPC4_PV_MAX_SUPPORTED_CHANNELS: usize = 8;

/**
 * struct sof_ipc4_peak_volume_regs - Volume information in fw
 * @peak_meter: Peak volume value in fw
 * @current_volume: Current volume value in fw
 * @target_volume: Target volume value in fw
 */
#[repr(C, packed(4))]
pub struct sof_ipc4_peak_volume_regs {
    pub peak_meter: [u32; SOF_IPC4_PV_MAX_SUPPORTED_CHANNELS],
    pub current_volume: [u32; SOF_IPC4_PV_MAX_SUPPORTED_CHANNELS],
    pub target_volume: [u32; SOF_IPC4_PV_MAX_SUPPORTED_CHANNELS],
}

/**
 * struct sof_ipc4_llp_reading - Llp information in fw
 * @llp_l: Lower part of 64-bit LLP
 * @llp_u: Upper part of 64-bit LLP
 * @wclk_l: Lower part of 64-bit Wallclock
 * @wclk_u: Upper part of 64-bit Wallclock
 */
#[repr(C, packed(4))]
pub struct sof_ipc4_llp_reading {
    pub llp_l: u32,
    pub llp_u: u32,
    pub wclk_l: u32,
    pub wclk_u: u32,
}

/**
 * struct of sof_ipc4_llp_reading_extended - Extended llp info
 * @llp_reading: Llp information in memory window
 * @tpd_low: Total processed data (low part)
 * @tpd_high: Total processed data (high part)
 */
#[repr(C, packed(4))]
pub struct sof_ipc4_llp_reading_extended {
    pub llp_reading: sof_ipc4_llp_reading,
    pub tpd_low: u32,
    pub tpd_high: u32,
}

/**
 * struct sof_ipc4_llp_reading_slot - Llp slot information in memory window
 * @node_id: Dai gateway node id
 * @reading: Llp information in memory window
 */
#[repr(C, packed(4))]
pub struct sof_ipc4_llp_reading_slot {
    pub node_id: u32,
    pub reading: sof_ipc4_llp_reading,
}

/* ROM information */
pub const SOF_IPC4_FW_FUSE_VALUE_MASK: u32 = 0x0000_00ff;
pub const SOF_IPC4_FW_LOAD_METHOD_MASK: u32 = 1u32 << 8;
pub const SOF_IPC4_FW_DOWNLINK_IPC_USE_DMA_MASK: u32 = 1u32 << 9;
pub const SOF_IPC4_FW_LOAD_METHOD_REV_MASK: u32 = 0x0000_0c00;
pub const SOF_IPC4_FW_REVISION_MIN_MASK: u32 = 0x0000_f000;
pub const SOF_IPC4_FW_REVISION_MAJ_MASK: u32 = 0x000f_0000;
pub const SOF_IPC4_FW_VERSION_MIN_MASK: u32 = 0x00f0_0000;
pub const SOF_IPC4_FW_VERSION_MAJ_MASK: u32 = 0x0f00_0000;

/* Number of dsp core supported in FW Regs. */
pub const SOF_IPC4_MAX_SUPPORTED_ADSP_CORES: usize = 8;

/* Number of host pipeline registers slots in FW Regs. */
pub const SOF_IPC4_MAX_PIPELINE_REG_SLOTS: usize = 16;

/* Number of PeakVol registers slots in FW Regs. */
pub const SOF_IPC4_MAX_PEAK_VOL_REG_SLOTS: usize = 16;

/* Number of GPDMA LLP Reading slots in FW Regs. */
pub const SOF_IPC4_MAX_LLP_GPDMA_READING_SLOTS: usize = 24;

/* Number of Aggregated SNDW Reading slots in FW Regs. */
pub const SOF_IPC4_MAX_LLP_SNDW_READING_SLOTS: usize = 15;

/* Current ABI version of the Fw registers layout. */
pub const SOF_IPC4_FW_REGS_ABI_VER: u32 = 1;

/**
 * struct sof_ipc4_fw_registers - FW Registers exposes additional
 * DSP / FW state information to the driver
 * @fw_status: Current ROM / FW status
 * @lec: Last ROM / FW error code
 * @fps: Current DSP clock status
 * @lnec: Last Native Error Code(from external library)
 * @ltr: Copy of LTRC HW register value(FW only)
 * @rsvd0: Reserved0
 * @rom_info: ROM info
 * @abi_ver: Version of the layout, set to the current FW_REGS_ABI_VER
 * @slave_core_sts: Slave core states
 * @rsvd2: Reserved2
 * @pipeline_regs: State of pipelines attached to host output  gateways
 * @peak_vol_regs: State of PeakVol instances indexed by the PeakVol's instance_id
 * @llp_gpdma_reading_slots: LLP Readings for single link gateways
 * @llp_sndw_reading_slots: SNDW aggregated link gateways
 * @llp_evad_reading_slot: LLP Readings for EVAD gateway
 */
#[repr(C, packed(4))]
pub struct sof_ipc4_fw_registers {
    pub fw_status: u32,
    pub lec: u32,
    pub fps: u32,
    pub lnec: u32,
    pub ltr: u32,
    pub rsvd0: u32,
    pub rom_info: u32,
    pub abi_ver: u32,
    pub slave_core_sts: [u8; SOF_IPC4_MAX_SUPPORTED_ADSP_CORES],
    pub rsvd2: [u32; 6],

    pub pipeline_regs: [sof_ipc4_pipeline_registers; SOF_IPC4_MAX_PIPELINE_REG_SLOTS],

    pub peak_vol_regs: [sof_ipc4_peak_volume_regs; SOF_IPC4_MAX_PEAK_VOL_REG_SLOTS],

    pub llp_gpdma_reading_slots:
        [sof_ipc4_llp_reading_slot; SOF_IPC4_MAX_LLP_GPDMA_READING_SLOTS],

    pub llp_sndw_reading_slots:
        [sof_ipc4_llp_reading_slot; SOF_IPC4_MAX_LLP_SNDW_READING_SLOTS],

    pub llp_evad_reading_slot: sof_ipc4_llp_reading_slot,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
