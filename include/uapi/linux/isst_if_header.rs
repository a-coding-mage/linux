/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Intel Speed Select Interface: OS to hardware Interface */

// Translated from isst_if.h. Linux ioctl encoding uses the platform ABI.

#[repr(C)]
pub struct isst_if_platform_info {
    pub api_version: u16,
    pub driver_version: u16,
    pub max_cmds_per_ioctl: u16,
    pub mbox_supported: u8,
    pub mmio_supported: u8,
}

#[repr(C)]
pub struct isst_if_cpu_map { pub logical_cpu: u32, pub physical_cpu: u32 }

#[repr(C)]
pub struct isst_if_cpu_maps {
    pub cmd_count: u32,
    pub cpu_map: [isst_if_cpu_map; 1],
}

#[repr(C)]
pub struct isst_if_io_reg {
    pub read_write: u32,
    pub logical_cpu: u32,
    pub reg: u32,
    pub value: u32,
}

#[repr(C)]
pub struct isst_if_io_regs {
    pub req_count: u32,
    pub io_reg: [isst_if_io_reg; 1],
}

#[repr(C)]
pub struct isst_if_mbox_cmd {
    pub logical_cpu: u32,
    pub parameter: u32,
    pub req_data: u32,
    pub resp_data: u32,
    pub command: u16,
    pub sub_command: u16,
    pub reserved: u32,
}

#[repr(C)]
pub struct isst_if_mbox_cmds {
    pub cmd_count: u32,
    pub mbox_cmd: [isst_if_mbox_cmd; 1],
}

#[repr(C)]
pub struct isst_if_msr_cmd {
    pub read_write: u32,
    pub logical_cpu: u32,
    pub msr: u64,
    pub data: u64,
}

#[repr(C)]
pub struct isst_if_msr_cmds {
    pub cmd_count: u32,
    pub msr_cmd: [isst_if_msr_cmd; 1],
}

#[repr(C)]
pub struct isst_core_power {
    pub get_set: u8,
    pub socket_id: u8,
    pub power_domain_id: u8,
    pub enable: u8,
    pub supported: u8,
    pub priority_type: u8,
}

#[repr(C)]
pub struct isst_clos_param {
    pub get_set: u8,
    pub socket_id: u8,
    pub power_domain_id: u8,
    pub clos: u8,
    pub min_freq_mhz: u16,
    pub max_freq_mhz: u16,
    pub prop_prio: u8,
}

#[repr(C)]
pub struct isst_if_clos_assoc {
    pub socket_id: u8,
    pub power_domain_id: u8,
    pub logical_cpu: u16,
    pub clos: u16,
}

#[repr(C)]
pub struct isst_if_clos_assoc_cmds {
    pub cmd_count: u16,
    pub get_set: u16,
    pub punit_cpu_map: u16,
    pub assoc_info: [isst_if_clos_assoc; 1],
}

#[repr(C)]
pub struct isst_tpmi_instance_count {
    pub socket_id: u8,
    pub count: u8,
    pub valid_mask: u16,
}

#[repr(C)]
pub struct isst_perf_level_info {
    pub socket_id: u8,
    pub power_domain_id: u8,
    pub max_level: u8,
    pub feature_rev: u8,
    pub level_mask: u8,
    pub current_level: u8,
    pub feature_state: u8,
    pub locked: u8,
    pub enabled: u8,
    pub sst_tf_support: u8,
    pub sst_bf_support: u8,
}

#[repr(C)]
pub struct isst_perf_level_control { pub socket_id: u8, pub power_domain_id: u8, pub level: u8 }

#[repr(C)]
pub struct isst_perf_feature_control { pub socket_id: u8, pub power_domain_id: u8, pub feature: u8 }

pub const TRL_MAX_BUCKETS: usize = 8;
pub const TRL_MAX_LEVELS: usize = 6;

#[repr(C)]
pub struct isst_perf_level_data_info {
    pub socket_id: u8, pub power_domain_id: u8, pub level: u16, pub tdp_ratio: u16,
    pub base_freq_mhz: u16, pub base_freq_avx2_mhz: u16, pub base_freq_avx512_mhz: u16,
    pub base_freq_amx_mhz: u16, pub thermal_design_power_w: u16, pub tjunction_max_c: u16,
    pub max_memory_freq_mhz: u16, pub cooling_type: u16, pub p0_freq_mhz: u16,
    pub p1_freq_mhz: u16, pub pn_freq_mhz: u16, pub pm_freq_mhz: u16,
    pub p0_fabric_freq_mhz: u16, pub p1_fabric_freq_mhz: u16, pub pn_fabric_freq_mhz: u16,
    pub pm_fabric_freq_mhz: u16, pub max_buckets: u16, pub max_trl_levels: u16,
    pub bucket_core_counts: [u16; TRL_MAX_BUCKETS],
    pub trl_freq_mhz: [[u16; TRL_MAX_BUCKETS]; TRL_MAX_LEVELS],
}

pub const MAX_FABRIC_COUNT: usize = 8;

#[repr(C)]
pub struct isst_perf_level_fabric_info {
    pub socket_id: u8, pub power_domain_id: u8, pub level: u16, pub max_fabrics: u16,
    pub p0_fabric_freq_mhz: [u16; MAX_FABRIC_COUNT],
    pub p1_fabric_freq_mhz: [u16; MAX_FABRIC_COUNT],
    pub pm_fabric_freq_mhz: [u16; MAX_FABRIC_COUNT],
}

#[repr(C)]
pub struct isst_perf_level_cpu_mask {
    pub socket_id: u8, pub power_domain_id: u8, pub level: u8, pub punit_cpu_map: u8,
    pub mask: u64, pub cpu_buffer_size: u16, pub cpu_buffer: [i8; 1],
}

#[repr(C)]
pub struct isst_base_freq_info {
    pub socket_id: u8, pub power_domain_id: u8, pub level: u16,
    pub high_base_freq_mhz: u16, pub low_base_freq_mhz: u16,
    pub tjunction_max_c: u16, pub thermal_design_power_w: u16,
}

#[repr(C)]
pub struct isst_turbo_freq_info {
    pub socket_id: u8, pub power_domain_id: u8, pub level: u16,
    pub max_clip_freqs: u16, pub max_buckets: u16, pub max_trl_levels: u16,
    pub lp_clip_freq_mhz: [u16; TRL_MAX_LEVELS],
    pub bucket_core_counts: [u16; TRL_MAX_BUCKETS],
    pub trl_freq_mhz: [[u16; TRL_MAX_BUCKETS]; TRL_MAX_LEVELS],
}

pub const ISST_IF_MAGIC: u8 = 0xFE;

// _IOR/_IOW/_IOWR are supplied by the target Linux ABI; these declarations
// preserve the ioctl interfaces and their source-level intent.
pub const ISST_IF_GET_PLATFORM_INFO: u32 = 0;
pub const ISST_IF_GET_PHY_ID: u32 = 1;
pub const ISST_IF_IO_CMD: u32 = 2;
pub const ISST_IF_MBOX_COMMAND: u32 = 3;
pub const ISST_IF_MSR_COMMAND: u32 = 4;
pub const ISST_IF_COUNT_TPMI_INSTANCES: u32 = 5;
pub const ISST_IF_CORE_POWER_STATE: u32 = 6;
pub const ISST_IF_CLOS_PARAM: u32 = 7;
pub const ISST_IF_CLOS_ASSOC: u32 = 8;
pub const ISST_IF_PERF_LEVELS: u32 = 9;
pub const ISST_IF_PERF_SET_LEVEL: u32 = 10;
pub const ISST_IF_PERF_SET_FEATURE: u32 = 11;
pub const ISST_IF_GET_PERF_LEVEL_INFO: u32 = 12;
pub const ISST_IF_GET_PERF_LEVEL_CPU_MASK: u32 = 13;
pub const ISST_IF_GET_BASE_FREQ_INFO: u32 = 14;
pub const ISST_IF_GET_BASE_FREQ_CPU_MASK: u32 = 15;
pub const ISST_IF_GET_TURBO_FREQ_INFO: u32 = 16;
pub const ISST_IF_GET_PERF_LEVEL_FABRIC_INFO: u32 = 17;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
