/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header amd_hsmp.h.
// The original header uses #pragma pack(4) for the following wire structures.

pub const HSMP_MAX_MSG_LEN: usize = 8;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hsmp_message_ids {
    HSMP_TEST = 1,
    HSMP_GET_SMU_VER,
    HSMP_GET_PROTO_VER,
    HSMP_GET_SOCKET_POWER,
    HSMP_SET_SOCKET_POWER_LIMIT,
    HSMP_GET_SOCKET_POWER_LIMIT,
    HSMP_GET_SOCKET_POWER_LIMIT_MAX,
    HSMP_SET_BOOST_LIMIT,
    HSMP_SET_BOOST_LIMIT_SOCKET,
    HSMP_GET_BOOST_LIMIT,
    HSMP_GET_PROC_HOT,
    HSMP_SET_XGMI_LINK_WIDTH,
    HSMP_SET_DF_PSTATE,
    HSMP_SET_AUTO_DF_PSTATE,
    HSMP_GET_FCLK_MCLK,
    HSMP_GET_CCLK_THROTTLE_LIMIT,
    HSMP_GET_C0_PERCENT,
    HSMP_SET_NBIO_DPM_LEVEL,
    HSMP_GET_NBIO_DPM_LEVEL,
    HSMP_GET_DDR_BANDWIDTH,
    HSMP_GET_TEMP_MONITOR,
    HSMP_GET_DIMM_TEMP_RANGE,
    HSMP_GET_DIMM_POWER,
    HSMP_GET_DIMM_THERMAL,
    HSMP_GET_SOCKET_FREQ_LIMIT,
    HSMP_GET_CCLK_CORE_LIMIT,
    HSMP_GET_RAILS_SVI,
    HSMP_GET_SOCKET_FMAX_FMIN,
    HSMP_GET_IOLINK_BANDWITH,
    HSMP_GET_XGMI_BANDWITH,
    HSMP_SET_GMI3_WIDTH,
    HSMP_SET_PCI_RATE,
    HSMP_SET_POWER_MODE,
    HSMP_SET_PSTATE_MAX_MIN,
    HSMP_GET_METRIC_TABLE_VER,
    HSMP_GET_METRIC_TABLE,
    HSMP_GET_METRIC_TABLE_DRAM_ADDR,
    HSMP_SET_XGMI_PSTATE_RANGE,
    HSMP_CPU_RAIL_ISO_FREQ_POLICY,
    HSMP_DFC_ENABLE_CTRL,
    HSMP_PC6_ENABLE,
    HSMP_CC6_ENABLE,
    HSMP_GET_RAPL_UNITS = 0x30,
    HSMP_GET_RAPL_CORE_COUNTER,
    HSMP_GET_RAPL_PACKAGE_COUNTER,
    HSMP_DIMM_SB_RD,
    HSMP_READ_CCD_POWER,
    HSMP_READ_TDELTA,
    HSMP_GET_SVI3_VR_CTRL_TEMP,
    HSMP_GET_ENABLED_HSMP_CMDS,
    HSMP_SET_GET_FLOOR_LIMIT,
    HSMP_DIMM_SB_WR,
    HSMP_SDPS_LIMIT,
    HSMP_PQOS_TRAFFIC_PRIORITY,
    HSMP_PQOS_FLOATING_BW,
    HSMP_MSG_ID_MAX,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct hsmp_message {
    pub msg_id: u32,
    pub num_args: u16,
    pub response_sz: u16,
    pub args: [u32; HSMP_MAX_MSG_LEN],
    pub sock_ind: u16,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hsmp_msg_type {
    HSMP_RSVD = -1,
    HSMP_SET = 0,
    HSMP_GET = 1,
    HSMP_SET_GET = 2,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hsmp_proto_versions {
    HSMP_PROTO_VER2 = 2,
    HSMP_PROTO_VER3,
    HSMP_PROTO_VER4,
    HSMP_PROTO_VER5,
    HSMP_PROTO_VER6,
    HSMP_PROTO_VER7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsmp_msg_desc {
    pub num_args: i32,
    pub response_sz: i32,
    pub type_: hsmp_msg_type,
}

/* Message descriptors indexed by message ID; unsupported messages are reserved. */
pub const hsmp_msg_desc_table: [hsmp_msg_desc; 0x3d] = [
    hsmp_msg_desc { num_args: 0, response_sz: 0, type_: hsmp_msg_type::HSMP_RSVD },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 0, type_: hsmp_msg_type::HSMP_SET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 0, type_: hsmp_msg_type::HSMP_SET },
    hsmp_msg_desc { num_args: 1, response_sz: 0, type_: hsmp_msg_type::HSMP_SET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 0, type_: hsmp_msg_type::HSMP_SET },
    hsmp_msg_desc { num_args: 0, response_sz: 2, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 0, type_: hsmp_msg_type::HSMP_SET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 0, type_: hsmp_msg_type::HSMP_SET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 0, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 3, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 0, type_: hsmp_msg_type::HSMP_SET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 0, type_: hsmp_msg_type::HSMP_RSVD },
    hsmp_msg_desc { num_args: 0, response_sz: 0, type_: hsmp_msg_type::HSMP_RSVD },
    hsmp_msg_desc { num_args: 0, response_sz: 0, type_: hsmp_msg_type::HSMP_RSVD },
    hsmp_msg_desc { num_args: 0, response_sz: 0, type_: hsmp_msg_type::HSMP_RSVD },
    hsmp_msg_desc { num_args: 0, response_sz: 0, type_: hsmp_msg_type::HSMP_RSVD },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 2, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 2, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 0, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 3, type_: hsmp_msg_type::HSMP_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 0, type_: hsmp_msg_type::HSMP_SET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 1, type_: hsmp_msg_type::HSMP_SET_GET },
    hsmp_msg_desc { num_args: 1, response_sz: 2, type_: hsmp_msg_type::HSMP_SET_GET },
];

#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct hsmp_metric_table {
    pub accumulation_counter: u32,
    pub max_socket_temperature: u32,
    pub max_vr_temperature: u32,
    pub max_hbm_temperature: u32,
    pub max_socket_temperature_acc: u64,
    pub max_vr_temperature_acc: u64,
    pub max_hbm_temperature_acc: u64,
    pub socket_power_limit: u32,
    pub max_socket_power_limit: u32,
    pub socket_power: u32,
    pub timestamp: u64,
    pub socket_energy_acc: u64,
    pub ccd_energy_acc: u64,
    pub xcd_energy_acc: u64,
    pub aid_energy_acc: u64,
    pub hbm_energy_acc: u64,
    pub cclk_frequency_limit: u32,
    pub gfxclk_frequency_limit: u32,
    pub fclk_frequency: u32,
    pub uclk_frequency: u32,
    pub socclk_frequency: [u32; 4],
    pub vclk_frequency: [u32; 4],
    pub dclk_frequency: [u32; 4],
    pub lclk_frequency: [u32; 4],
    pub gfxclk_frequency_acc: [u64; 8],
    pub cclk_frequency_acc: [u64; 96],
    pub max_cclk_frequency: u32,
    pub min_cclk_frequency: u32,
    pub max_gfxclk_frequency: u32,
    pub min_gfxclk_frequency: u32,
    pub fclk_frequency_table: [u32; 4],
    pub uclk_frequency_table: [u32; 4],
    pub socclk_frequency_table: [u32; 4],
    pub vclk_frequency_table: [u32; 4],
    pub dclk_frequency_table: [u32; 4],
    pub lclk_frequency_table: [u32; 4],
    pub max_lclk_dpm_range: u32,
    pub min_lclk_dpm_range: u32,
    pub xgmi_width: u32,
    pub xgmi_bitrate: u32,
    pub xgmi_read_bandwidth_acc: [u64; 8],
    pub xgmi_write_bandwidth_acc: [u64; 8],
    pub socket_c0_residency: u32,
    pub socket_gfx_busy: u32,
    pub dram_bandwidth_utilization: u32,
    pub socket_c0_residency_acc: u64,
    pub socket_gfx_busy_acc: u64,
    pub dram_bandwidth_acc: u64,
    pub max_dram_bandwidth: u32,
    pub dram_bandwidth_utilization_acc: u64,
    pub pcie_bandwidth_acc: [u64; 4],
    pub prochot_residency_acc: u32,
    pub ppt_residency_acc: u32,
    pub socket_thm_residency_acc: u32,
    pub vr_thm_residency_acc: u32,
    pub hbm_thm_residency_acc: u32,
    pub spare: u32,
    pub gfxclk_frequency: [u32; 8],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct hsmp_telemetry_data {
    pub buf: u64,
    pub size: u32,
    pub sock_ind: u16,
    pub reserved: u16,
}

pub const HSMP_BASE_IOCTL_NR: u32 = 0xF8;
// _IOWR(HSMP_BASE_IOCTL_NR, 0, struct hsmp_message), using the platform's
// generic ioctl encoding supplied by the consuming environment.
pub const HSMP_IOCTL_CMD: u32 = 0;
// _IOW(HSMP_BASE_IOCTL_NR, 1, struct hsmp_telemetry_data).
pub const HSMP_IOCTL_GET_TELEMETRY_DATA: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
