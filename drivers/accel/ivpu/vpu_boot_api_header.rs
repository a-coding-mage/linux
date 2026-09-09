/* SPDX-License-Identifier: MIT */
/* Copyright (c) 2020-2025, Intel Corporation. */

// The source header uses externally supplied C integer aliases (u8/u16/u32/u64/s32).

pub const VPU_BOOT_API_VER_MAJOR: u32 = 3;
pub const VPU_BOOT_API_VER_MINOR: u32 = 29;
pub const VPU_BOOT_API_VER_PATCH: u32 = 5;
pub const VPU_BOOT_API_VER_INDEX: u32 = 0;

pub const VPU_FW_HEADER_SIZE: u32 = 4096;
pub const VPU_FW_HEADER_VERSION: u32 = 0x1;
pub const VPU_FW_VERSION_SIZE: usize = 32;
pub const VPU_FW_API_VER_NUM: usize = 16;

#[repr(C, packed(4))]
pub struct vpu_firmware_header {
    pub header_version: u32,
    pub image_format: u32,
    pub image_load_address: u64,
    pub image_size: u32,
    pub entry_point: u64,
    pub vpu_version: [u8; VPU_FW_VERSION_SIZE],
    pub compression_type: u32,
    pub firmware_version_load_address: u64,
    pub firmware_version_size: u32,
    pub boot_params_load_address: u64,
    pub api_version: [u32; VPU_FW_API_VER_NUM],
    /// Size of memory require for firmware execution
    pub runtime_size: u32,
    pub shave_nn_fw_size: u32,
    /// Size of primary preemption buffer, assuming a 2-job submission queue.
    /// NOTE: host driver is expected to adapt size accordingly to actual submission queue size and device capabilities.
    pub preemption_buffer_1_size: u32,
    /// Size of secondary preemption buffer, assuming a 2-job submission queue.
    /// NOTE: host driver is expected to adapt size accordingly to actual submission queue size and device capabilities.
    pub preemption_buffer_2_size: u32,
    /// Maximum preemption buffer size that the FW can use. A value of 0 means no declared limit.
    pub preemption_buffer_1_max_size: u32,
    pub preemption_buffer_2_max_size: u32,
    /// Space reserved for future preemption-related fields.
    pub preemption_reserved: [u32; 4],
    /// FW image read only section start address, 4KB aligned
    pub ro_section_start_address: u64,
    /// FW image read only section size, 4KB aligned
    pub ro_section_size: u32,
    pub reserved: u32,
}

pub const VPU_BOOT_TYPE_COLDBOOT: u32 = 0;
pub const VPU_BOOT_TYPE_WARMBOOT: u32 = 1;
pub const VPU_BOOT_PARAMS_MAGIC: u32 = 0x10000;
pub const VPU_SCHEDULING_MODE_OS: u32 = 0;
pub const VPU_SCHEDULING_MODE_HW: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum VPU_BOOT_L2_CACHE_CFG_TYPE {
    VPU_BOOT_L2_CACHE_CFG_UPA = 0,
    VPU_BOOT_L2_CACHE_CFG_NN = 1,
    VPU_BOOT_L2_CACHE_CFG_NUM = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum VPU_BOOT_MCA_ECC_SIGNAL_TYPE {
    VPU_BOOT_MCA_ECC_NONE = 0,
    VPU_BOOT_MCA_ECC_CORR = 1,
    VPU_BOOT_MCA_ECC_FATAL = 2,
    VPU_BOOT_MCA_ECC_BOTH = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vpu_trace_destination {
    VPU_TRACE_DESTINATION_PIPEPRINT = 0x1,
    VPU_TRACE_DESTINATION_VERBOSE_TRACING = 0x2,
    VPU_TRACE_DESTINATION_NORTH_PEAK = 0x4,
}

pub const VPU_TRACE_PROC_BIT_RESERVED: u32 = 0;
pub const VPU_TRACE_PROC_BIT_LRT: u32 = 1;
pub const VPU_TRACE_PROC_BIT_LNN: u32 = 2;
pub const VPU_TRACE_PROC_BIT_SHV_0: u32 = 3;
pub const VPU_TRACE_PROC_BIT_SHV_1: u32 = 4;
pub const VPU_TRACE_PROC_BIT_SHV_2: u32 = 5;
pub const VPU_TRACE_PROC_BIT_SHV_3: u32 = 6;
pub const VPU_TRACE_PROC_BIT_SHV_4: u32 = 7;
pub const VPU_TRACE_PROC_BIT_SHV_5: u32 = 8;
pub const VPU_TRACE_PROC_BIT_SHV_6: u32 = 9;
pub const VPU_TRACE_PROC_BIT_SHV_7: u32 = 10;
pub const VPU_TRACE_PROC_BIT_SHV_8: u32 = 11;
pub const VPU_TRACE_PROC_BIT_SHV_9: u32 = 12;
pub const VPU_TRACE_PROC_BIT_SHV_10: u32 = 13;
pub const VPU_TRACE_PROC_BIT_SHV_11: u32 = 14;
pub const VPU_TRACE_PROC_BIT_SHV_12: u32 = 15;
pub const VPU_TRACE_PROC_BIT_SHV_13: u32 = 16;
pub const VPU_TRACE_PROC_BIT_SHV_14: u32 = 17;
pub const VPU_TRACE_PROC_BIT_SHV_15: u32 = 18;
pub const VPU_TRACE_PROC_BIT_ACT_SHV_0: u32 = 19;
pub const VPU_TRACE_PROC_BIT_ACT_SHV_1: u32 = 20;
pub const VPU_TRACE_PROC_BIT_ACT_SHV_2: u32 = 21;
pub const VPU_TRACE_PROC_BIT_ACT_SHV_3: u32 = 22;
pub const VPU_TRACE_PROC_NO_OF_HW_DEVS: u32 = 23;
pub const VPU_TRACE_PROC_BIT_30XX_FIRST: u32 = VPU_TRACE_PROC_BIT_LRT;
pub const VPU_TRACE_PROC_BIT_30XX_LAST: u32 = VPU_TRACE_PROC_BIT_SHV_15;

#[repr(C, packed(4))]
pub struct vpu_boot_l2_cache_config { pub use_: u8, pub cfg: u8 }

pub const VPU_PRESENT_CALL_PERIOD_MS_DEFAULT: u32 = 50;
pub const VPU_PRESENT_CALL_PERIOD_MS_MIN: u32 = 16;
pub const VPU_PRESENT_CALL_PERIOD_MS_MAX: u32 = 10000;
pub const POWER_PROFILE_SURVIVABILITY: u32 = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vpu_governor {
    VPU_GOV_DEFAULT = 0,
    VPU_GOV_MAX_PERFORMANCE = 1,
    VPU_GOV_ON_DEMAND = 2,
    VPU_GOV_POWER_SAVE = 3,
    VPU_GOV_ON_DEMAND_PRIORITY_AWARE = 4,
}

#[repr(C, packed(4))]
pub struct power_states_timestamps {
    pub vpu_active_state_requested: u64,
    pub vpu_active_state_achieved: u64,
    pub vpu_idle_state_requested: u64,
    pub vpu_idle_state_achieved: u64,
    pub vpu_standby_state_requested: u64,
    pub vpu_standby_state_achieved: u64,
}

#[repr(C, packed(4))]
pub struct vpu_boot_params {
    pub magic: u32, pub vpu_id: u32, pub vpu_count: u32, pub reserved_0: [u32; 5],
    pub frequency: u32, pub reserved_1: [u32; 12], pub perf_clk_frequency: u32, pub reserved_2: [u32; 42],
    pub ipc_header_area_start: u64, pub ipc_header_area_size: u32, pub shared_region_base: u64, pub shared_region_size: u32,
    pub ipc_payload_area_start: u64, pub ipc_payload_area_size: u32, pub global_aliased_pio_base: u64, pub global_aliased_pio_size: u32,
    pub autoconfig: u32, pub cache_defaults: [vpu_boot_l2_cache_config; 2], pub reserved_3: [u32; 3],
    pub shave_nn_fw_base: u64, pub save_restore_ret_address: u64, pub reserved_4: [u32; 43],
    pub watchdog_irq_mss: i32, pub watchdog_irq_nce: i32, pub host_to_vpu_irq: u32, pub job_done_irq: u32, pub reserved_5: [u32; 60],
    pub host_version_id: u32, pub si_stepping: u32, pub device_id: u64, pub feature_exclusion: u64, pub sku: u64,
    pub min_freq_pll_ratio: u32, pub max_freq_pll_ratio: u32, pub default_trace_level: u32, pub boot_type: u32,
    pub punit_telemetry_sram_base: u64, pub punit_telemetry_sram_size: u64, pub vpu_telemetry_enable: u32,
    pub crit_tracing_buff_addr: u64, pub crit_tracing_buff_size: u32, pub verbose_tracing_buff_addr: u64, pub verbose_tracing_buff_size: u32,
    pub verbose_tracing_sw_component_mask: u64, pub trace_destination_mask: u32, pub trace_hw_component_mask: u64,
    pub tracing_buff_message_format_mask: u64, pub trace_reserved_1: [u64; 2], pub reserved_6: u32, pub pn_freq_pll_ratio: u32,
    pub dvfs_mode: u32, pub dvfs_param: u64, pub d0i3_delayed_entry: u32, pub d0i3_residency_time_us: u64,
    pub d0i3_entry_vpu_ts: u64, pub system_time_us: u64, pub reserved_7: [u32; 2], pub device_time_delta_ticks: u64, pub reserved_8: [u32; 30],
    pub power_states_timestamps: power_states_timestamps, pub vpu_scheduling_mode: u32, pub vpu_focus_present_timer_ms: u32,
    pub vpu_uses_ecc_mca_signal: u32, pub power_profile: u32, pub dct_active_us: u32, pub dct_inactive_us: u32, pub reserved_9: [u32; 734],
}

pub const VPU_TRACING_BUFFER_CANARY: u32 = 0xCAFECAFE;
pub const VPU_TRACING_FORMAT_STRING: u32 = 0;
pub const VPU_TRACING_FORMAT_MIPI: u32 = 2;

#[repr(C, packed(4))]
pub struct vpu_tracing_buffer_header {
    pub host_canary_start: u32, pub read_index: u32, pub read_wrap_count: u32, pub pad_to_cache_line_size_0: [u32; 13],
    pub vpu_canary_start: u32, pub write_index: u32, pub wrap_count: u32, pub reserved_0: u32, pub size: u32,
    pub header_version: u16, pub header_size: u16, pub format: u32, pub alignment: u32, pub name: [i8; 16],
    pub pad_to_cache_line_size_1: [u32; 4],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
