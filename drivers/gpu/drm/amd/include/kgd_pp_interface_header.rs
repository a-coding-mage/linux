#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/*
 * Copyright 2017 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */

extern "C" { pub static pp_smu_ip_block: amdgpu_ip_block_version; }
extern "C" { pub static smu_v11_0_ip_block: amdgpu_ip_block_version; }
extern "C" { pub static smu_v12_0_ip_block: amdgpu_ip_block_version; }
extern "C" { pub static smu_v13_0_ip_block: amdgpu_ip_block_version; }
extern "C" { pub static smu_v14_0_ip_block: amdgpu_ip_block_version; }
extern "C" { pub static smu_v15_0_ip_block: amdgpu_ip_block_version; }

#[repr(C)]\n#[derive(Copy, Clone)]\npub smu_temp_metric_type {
	SMU_TEMP_METRIC_BASEBOARD,
	SMU_TEMP_METRIC_GPUBOARD,
	SMU_TEMP_METRIC_MAX,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub smu_event_type {
	SMU_EVENT_RESET_COMPLETE = 0,
};

#[repr(C)]\npub amd_vce_state {
	/* vce clocks */
	evclk: u32;
	ecclk: u32;
	/* gpu clocks */
	sclk: u32;
	mclk: u32;
	clk_idx: u8;
	pstate: u8;
};


#[repr(C)]\n#[derive(Copy, Clone)]\npub amd_dpm_forced_level {
	AMD_DPM_FORCED_LEVEL_AUTO = 01,
	AMD_DPM_FORCED_LEVEL_MANUAL = 02,
	AMD_DPM_FORCED_LEVEL_LOW = 04,
	AMD_DPM_FORCED_LEVEL_HIGH = 08,
	AMD_DPM_FORCED_LEVEL_PROFILE_STANDARD = 010,
	AMD_DPM_FORCED_LEVEL_PROFILE_MIN_SCLK = 020,
	AMD_DPM_FORCED_LEVEL_PROFILE_MIN_MCLK = 040,
	AMD_DPM_FORCED_LEVEL_PROFILE_PEAK = 080,
	AMD_DPM_FORCED_LEVEL_PROFILE_EXIT = 0100,
	AMD_DPM_FORCED_LEVEL_PERF_DETERMINISM = 0200,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub amd_pm_state_type {
	/* not used for dpm */
	POWER_STATE_TYPE_DEFAULT,
	POWER_STATE_TYPE_POWERSAVE,
	/* user selectable states */
	POWER_STATE_TYPE_BATTERY,
	POWER_STATE_TYPE_BALANCED,
	POWER_STATE_TYPE_PERFORMANCE,
	/* internal states */
	POWER_STATE_TYPE_INTERNAL_UVD,
	POWER_STATE_TYPE_INTERNAL_UVD_SD,
	POWER_STATE_TYPE_INTERNAL_UVD_HD,
	POWER_STATE_TYPE_INTERNAL_UVD_HD2,
	POWER_STATE_TYPE_INTERNAL_UVD_MVC,
	POWER_STATE_TYPE_INTERNAL_BOOT,
	POWER_STATE_TYPE_INTERNAL_THERMAL,
	POWER_STATE_TYPE_INTERNAL_ACPI,
	POWER_STATE_TYPE_INTERNAL_ULV,
	POWER_STATE_TYPE_INTERNAL_3DPERF,
};

pub const AMD_MAX_VCE_LEVELS: u64 = 6;

#[repr(C)]\n#[derive(Copy, Clone)]\npub amd_vce_level {
	AMD_VCE_LEVEL_AC_ALL = 0,     /* AC, All cases */
	AMD_VCE_LEVEL_DC_EE = 1,      /* DC, entropy encoding */
	AMD_VCE_LEVEL_DC_LL_LOW = 2,  /* DC, low latency queue, res <= 720 */
	AMD_VCE_LEVEL_DC_LL_HIGH = 3, /* DC, low latency queue, 1080 >= res > 720 */
	AMD_VCE_LEVEL_DC_GP_LOW = 4,  /* DC, general purpose queue, res <= 720 */
	AMD_VCE_LEVEL_DC_GP_HIGH = 5, /* DC, general purpose queue, 1080 >= res > 720 */
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub amd_fan_ctrl_mode {
	AMD_FAN_CTRL_NONE = 0,
	AMD_FAN_CTRL_MANUAL = 1,
	AMD_FAN_CTRL_AUTO = 2,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub pp_clock_type {
	PP_SCLK,
	PP_MCLK,
	PP_PCIE,
	PP_SOCCLK,
	PP_FCLK,
	PP_DCEFCLK,
	PP_VCLK,
	PP_VCLK1,
	PP_DCLK,
	PP_DCLK1,
	PP_ISPICLK,
	PP_ISPXCLK,
	OD_SCLK,
	OD_MCLK,
	OD_FCLK,
	OD_VDDC_CURVE,
	OD_RANGE,
	OD_VDDGFX_OFFSET,
	OD_CCLK,
	OD_FAN_CURVE,
	OD_ACOUSTIC_LIMIT,
	OD_ACOUSTIC_TARGET,
	OD_FAN_TARGET_TEMPERATURE,
	OD_FAN_MINIMUM_PWM,
	OD_FAN_ZERO_RPM_ENABLE,
	OD_FAN_ZERO_RPM_STOP_TEMP,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub amd_pp_sensors {
	AMDGPU_PP_SENSOR_GFX_SCLK = 0,
	AMDGPU_PP_SENSOR_CPU_CLK,
	AMDGPU_PP_SENSOR_VDDNB,
	AMDGPU_PP_SENSOR_VDDGFX,
	AMDGPU_PP_SENSOR_UVD_VCLK,
	AMDGPU_PP_SENSOR_UVD_DCLK,
	AMDGPU_PP_SENSOR_VCE_ECCLK,
	AMDGPU_PP_SENSOR_GPU_LOAD,
	AMDGPU_PP_SENSOR_MEM_LOAD,
	AMDGPU_PP_SENSOR_GFX_MCLK,
	AMDGPU_PP_SENSOR_GPU_TEMP,
	AMDGPU_PP_SENSOR_EDGE_TEMP = AMDGPU_PP_SENSOR_GPU_TEMP,
	AMDGPU_PP_SENSOR_HOTSPOT_TEMP,
	AMDGPU_PP_SENSOR_MEM_TEMP,
	AMDGPU_PP_SENSOR_VCE_POWER,
	AMDGPU_PP_SENSOR_UVD_POWER,
	AMDGPU_PP_SENSOR_GPU_AVG_POWER, /* milliwatts */
	AMDGPU_PP_SENSOR_GPU_INPUT_POWER, /* milliwatts */
	AMDGPU_PP_SENSOR_SS_APU_SHARE,
	AMDGPU_PP_SENSOR_SS_DGPU_SHARE,
	AMDGPU_PP_SENSOR_STABLE_PSTATE_SCLK,
	AMDGPU_PP_SENSOR_STABLE_PSTATE_MCLK,
	AMDGPU_PP_SENSOR_ENABLED_SMC_FEATURES_MASK,
	AMDGPU_PP_SENSOR_MIN_FAN_RPM,
	AMDGPU_PP_SENSOR_MAX_FAN_RPM,
	AMDGPU_PP_SENSOR_VCN_POWER_STATE,
	AMDGPU_PP_SENSOR_PEAK_PSTATE_SCLK,
	AMDGPU_PP_SENSOR_PEAK_PSTATE_MCLK,
	AMDGPU_PP_SENSOR_VCN_LOAD,
	AMDGPU_PP_SENSOR_VDDBOARD,
	AMDGPU_PP_SENSOR_NODEPOWERLIMIT,
	AMDGPU_PP_SENSOR_NODEPOWER,
	AMDGPU_PP_SENSOR_GPPTRESIDENCY,
	AMDGPU_PP_SENSOR_MAXNODEPOWERLIMIT,
	AMDGPU_PP_SENSOR_UBB_POWER,
	AMDGPU_PP_SENSOR_UBB_POWER_LIMIT,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub amd_pp_task {
	AMD_PP_TASK_DISPLAY_CONFIG_CHANGE,
	AMD_PP_TASK_ENABLE_USER_STATE,
	AMD_PP_TASK_READJUST_POWER_STATE,
	AMD_PP_TASK_COMPLETE_INIT,
	AMD_PP_TASK_MAX
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub PP_SMC_POWER_PROFILE {
	PP_SMC_POWER_PROFILE_UNKNOWN = -1,
	PP_SMC_POWER_PROFILE_BOOTUP_DEFAULT = 00,
	PP_SMC_POWER_PROFILE_FULLSCREEN3D = 01,
	PP_SMC_POWER_PROFILE_POWERSAVING  = 02,
	PP_SMC_POWER_PROFILE_VIDEO        = 03,
	PP_SMC_POWER_PROFILE_VR           = 04,
	PP_SMC_POWER_PROFILE_COMPUTE      = 05,
	PP_SMC_POWER_PROFILE_CUSTOM       = 06,
	PP_SMC_POWER_PROFILE_WINDOW3D     = 07,
	PP_SMC_POWER_PROFILE_CAPPED	  = 08,
	PP_SMC_POWER_PROFILE_UNCAPPED	  = 09,
	PP_SMC_POWER_PROFILE_COUNT,
};

extern "C" { pub static amdgpu_pp_profile_name: [*const core::ffi::c_char; PP_SMC_POWER_PROFILE_COUNT]; }



enum {
	PP_GROUP_UNKNOWN = 0,
	PP_GROUP_GFX = 1,
	PP_GROUP_SYS,
	PP_GROUP_MAX
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub PP_OD_DPM_TABLE_COMMAND {
	PP_OD_EDIT_SCLK_VDDC_TABLE,
	PP_OD_EDIT_MCLK_VDDC_TABLE,
	PP_OD_EDIT_FCLK_TABLE,
	PP_OD_EDIT_CCLK_VDDC_TABLE,
	PP_OD_EDIT_VDDC_CURVE,
	PP_OD_RESTORE_DEFAULT_TABLE,
	PP_OD_COMMIT_DPM_TABLE,
	PP_OD_EDIT_VDDGFX_OFFSET,
	PP_OD_EDIT_FAN_CURVE,
	PP_OD_EDIT_ACOUSTIC_LIMIT,
	PP_OD_EDIT_ACOUSTIC_TARGET,
	PP_OD_EDIT_FAN_TARGET_TEMPERATURE,
	PP_OD_EDIT_FAN_MINIMUM_PWM,
	PP_OD_EDIT_FAN_ZERO_RPM_ENABLE,
	PP_OD_EDIT_FAN_ZERO_RPM_STOP_TEMP,
};

#[repr(C)]\npub pp_states_info {
	nums: u32;
	states: [u32; 16];
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub PP_HWMON_TEMP {
	PP_TEMP_EDGE = 0,
	PP_TEMP_JUNCTION,
	PP_TEMP_MEM,
	PP_TEMP_MAX
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub pp_mp1_state {
	PP_MP1_STATE_NONE,
	PP_MP1_STATE_SHUTDOWN,
	PP_MP1_STATE_UNLOAD,
	PP_MP1_STATE_RESET,
	PP_MP1_STATE_FLR,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub pp_df_cstate {
	DF_CSTATE_DISALLOW = 0,
	DF_CSTATE_ALLOW,
};

/**
 * DOC: amdgpu_pp_power
 *
 * APU power is managed to system-level requirements through the PPT
 * (package power tracking) feature. PPT is intended to limit power to the
 * requirements of the power source and could be dynamically updated to
 * maximize APU performance within the system power budget.
 *
 * Two types of power measurement can be requested, where supported, with
 * :c:type:`pp_power_type <pp_power_type>`.
 */

/**
 * pp_power_limit_level - Used to query the power limits
 *
 * @PP_PWR_LIMIT_MIN: Minimum Power Limit
 * @PP_PWR_LIMIT_CURRENT: Current Power Limit
 * @PP_PWR_LIMIT_DEFAULT: Default Power Limit
 * @PP_PWR_LIMIT_MAX: Maximum Power Limit
 */
#[repr(C)]\n#[derive(Copy, Clone)]\npub pp_power_limit_level {
	PP_PWR_LIMIT_MIN = -1,
	PP_PWR_LIMIT_CURRENT,
	PP_PWR_LIMIT_DEFAULT,
	PP_PWR_LIMIT_MAX,
};

/**
 * pp_power_type - Used to specify the type of the requested power
 *
 * @PP_PWR_TYPE_SUSTAINED: manages the configurable, thermally significant
 * moving average of APU power (default ~5000 ms).
 * @PP_PWR_TYPE_FAST: manages the ~10 ms moving average of APU power,
 * where supported.
 */
#[repr(C)]\n#[derive(Copy, Clone)]\npub pp_power_type {
	PP_PWR_TYPE_SUSTAINED,
	PP_PWR_TYPE_FAST,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub pp_xgmi_plpd_mode {
	XGMI_PLPD_NONE = -1,
	XGMI_PLPD_DISALLOW,
	XGMI_PLPD_DEFAULT,
	XGMI_PLPD_OPTIMIZED,
	XGMI_PLPD_COUNT,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub pp_pm_policy {
	PP_PM_POLICY_NONE = -1,
	PP_PM_POLICY_SOC_PSTATE = 0,
	PP_PM_POLICY_XGMI_PLPD,
	PP_PM_POLICY_NUM,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub pp_policy_soc_pstate {
	SOC_PSTATE_DEFAULT = 0,
	SOC_PSTATE_0,
	SOC_PSTATE_1,
	SOC_PSTATE_2,
	SOC_PSTAT_COUNT,
};

pub const PP_POLICY_MAX_LEVELS: u64 = 5;

pub const PP_GROUP_MASK: u64 = 0F0000000;
pub const PP_GROUP_SHIFT: u64 = 28;

pub const PP_BLOCK_MASK: u64 = 00FFFFF00;
pub const PP_BLOCK_SHIFT: u64 = 8;

pub const PP_BLOCK_GFX_CG: u64 = 001;
pub const PP_BLOCK_GFX_MG: u64 = 002;
pub const PP_BLOCK_GFX_3D: u64 = 004;
pub const PP_BLOCK_GFX_RLC: u64 = 008;
pub const PP_BLOCK_GFX_CP: u64 = 010;
pub const PP_BLOCK_SYS_BIF: u64 = 001;
pub const PP_BLOCK_SYS_MC: u64 = 002;
pub const PP_BLOCK_SYS_ROM: u64 = 004;
pub const PP_BLOCK_SYS_DRM: u64 = 008;
pub const PP_BLOCK_SYS_HDP: u64 = 010;
pub const PP_BLOCK_SYS_SDMA: u64 = 020;

pub const PP_STATE_MASK: u64 = 00000000F;
pub const PP_STATE_SHIFT: u64 = 0;
pub const PP_STATE_SUPPORT_MASK: u64 = 0000000F0;
pub const PP_STATE_SUPPORT_SHIFT: u64 = 0;

pub const PP_STATE_CG: u64 = 001;
pub const PP_STATE_LS: u64 = 002;
pub const PP_STATE_DS: u64 = 004;
pub const PP_STATE_SD: u64 = 008;
pub const PP_STATE_SUPPORT_CG: u64 = 010;
pub const PP_STATE_SUPPORT_LS: u64 = 020;
pub const PP_STATE_SUPPORT_DS: u64 = 040;
pub const PP_STATE_SUPPORT_SD: u64 = 080;

#define PP_CG_MSG_ID(group, block, support, state) \
		((group) << PP_GROUP_SHIFT | (block) << PP_BLOCK_SHIFT | \
		(support) << PP_STATE_SUPPORT_SHIFT | (state) << PP_STATE_SHIFT)

pub const XGMI_MODE_PSTATE_D3: u64 = 0;
pub const XGMI_MODE_PSTATE_D0: u64 = 1;

pub const NUM_HBM_INSTANCES: u64 = 4;
pub const NUM_XGMI_LINKS: u64 = 8;
pub const MAX_GFX_CLKS: u64 = 8;
pub const MAX_CLKS: u64 = 4;
pub const NUM_VCN: u64 = 4;
pub const NUM_JPEG_ENG: u64 = 32;
pub const NUM_JPEG_ENG_V1: u64 = 40;
pub const MAX_XCC: u64 = 8;
pub const NUM_XCP: u64 = 8;
seq_file: pub;
amd_pp_clock_type: pub;
amd_pp_simple_clock_info: pub;
amd_pp_display_configuration: pub;
amd_pp_clock_info: pub;
pp_display_clock_request: pub;
pp_clock_levels_with_voltage: pub;
pp_clock_levels_with_latency: pub;
amd_pp_clocks: pub;
pp_smu_wm_range_sets: pub;
pp_smu_nv_clock_table: pub;
dpm_clocks: pub;

#[repr(C)]\npub amdgpu_xcp_metrics {
	/* Utilization Instantaneous (%) */
	gfx_busy_inst: [u32; MAX_XCC];
	jpeg_busy: [u16; NUM_JPEG_ENG];
	vcn_busy: [u16; NUM_VCN];
	/* Utilization Accumulated (%) */
	gfx_busy_acc: [u64; MAX_XCC];
};

#[repr(C)]\npub amdgpu_xcp_metrics_v1_1 {
	/* Utilization Instantaneous (%) */
	gfx_busy_inst: [u32; MAX_XCC];
	jpeg_busy: [u16; NUM_JPEG_ENG];
	vcn_busy: [u16; NUM_VCN];
	/* Utilization Accumulated (%) */
	gfx_busy_acc: [u64; MAX_XCC];
	/* Total App Clock Counter Accumulated */
	gfx_below_host_limit_acc: [u64; MAX_XCC];
};

#[repr(C)]\npub amdgpu_xcp_metrics_v1_2 {
	/* Utilization Instantaneous (%) */
	gfx_busy_inst: [u32; MAX_XCC];
	jpeg_busy: [u16; NUM_JPEG_ENG_V1];
	vcn_busy: [u16; NUM_VCN];
	/* Utilization Accumulated (%) */
	gfx_busy_acc: [u64; MAX_XCC];
	/* Total App Clock Counter Accumulated */
	gfx_below_host_limit_ppt_acc: [u64; MAX_XCC];
	gfx_below_host_limit_thm_acc: [u64; MAX_XCC];
	gfx_low_utilization_acc: [u64; MAX_XCC];
	gfx_below_host_limit_total_acc: [u64; MAX_XCC];
};

#[repr(C)]\npub amd_pm_funcs {
/* export for dpm on ci and si */
	int (*pre_set_power_state)(*mut core::ffi::c_voidhandle);
	int (*set_power_state)(*mut core::ffi::c_voidhandle);
	void (*post_set_power_state)(*mut core::ffi::c_voidhandle);
	void (*display_configuration_changed)(*mut core::ffi::c_voidhandle);
	void (*print_power_state)(*mut core::ffi::c_voidhandle, *mut core::ffi::c_voidps);
	bool (*vblank_too_short)(*mut core::ffi::c_voidhandle);
	void (*notify_ac_dc)(*mut core::ffi::c_voidhandle);
	int (*check_state_equal)(*mut core::ffi::c_voidhandle,
				*mut core::ffi::c_voidcps,
				*mut core::ffi::c_voidrps,
				bool  *equal);
/* export for sysfs */
	int (*set_fan_control_mode)(*mut core::ffi::c_voidhandle, u32 mode);
	int (*get_fan_control_mode)(*mut core::ffi::c_voidhandle, u32 *fan_mode);
	int (*set_fan_speed_pwm)(*mut core::ffi::c_voidhandle, u32 speed);
	int (*get_fan_speed_pwm)(*mut core::ffi::c_voidhandle, u32 *speed);
	int (*force_clock_level)(*mut core::ffi::c_voidhandle, pp_clock_type type, u32 mask);
	int (*print_clock_levels)(*mut core::ffi::c_voidhandle, pp_clock_type type, *mut core::ffi::c_charbuf);
	int (*emit_clock_levels)(*mut core::ffi::c_voidhandle, pp_clock_type type, *mut core::ffi::c_charbuf, int *offset);
	int (*force_performance_level)(*mut core::ffi::c_voidhandle, amd_dpm_forced_level level);
	int (*get_sclk_od)(*mut core::ffi::c_voidhandle);
	int (*set_sclk_od)(*mut core::ffi::c_voidhandle, u32 value);
	int (*get_mclk_od)(*mut core::ffi::c_voidhandle);
	int (*set_mclk_od)(*mut core::ffi::c_voidhandle, u32 value);
	int (*read_sensor)(*mut core::ffi::c_voidhandle, int idx, *mut core::ffi::c_voidvalue, int *size);
	int (*get_apu_thermal_limit)(*mut core::ffi::c_voidhandle, u32 *limit);
	int (*set_apu_thermal_limit)(*mut core::ffi::c_voidhandle, u32 limit);
	amd_dpm_forced_level (*get_performance_level)(*mut core::ffi::c_voidhandle);
	amd_pm_state_type (*get_current_power_state)(*mut core::ffi::c_voidhandle);
	int (*get_fan_speed_rpm)(*mut core::ffi::c_voidhandle, u32 *rpm);
	int (*set_fan_speed_rpm)(*mut core::ffi::c_voidhandle, u32 rpm);
	int (*get_pp_num_states)(*mut core::ffi::c_voidhandle, pp_states_info *data);
	int (*get_pp_table)(*mut core::ffi::c_voidhandle, *mut core::ffi::c_char*table);
	int (*set_pp_table)(*mut core::ffi::c_voidhandle, *const core::ffi::c_charbuf, usize size);
	void (*debugfs_print_current_performance_level)(*mut core::ffi::c_voidhandle, seq_file *m);
	int (*switch_power_profile)(*mut core::ffi::c_voidhandle, PP_SMC_POWER_PROFILE type, bool en);
	int (*pause_power_profile)(*mut core::ffi::c_voidhandle, bool pause);
/* export to amdgpu */
	amd_vce_state *(*get_vce_clock_state)(*mut core::ffi::c_voidhandle, u32 idx);
	int (*dispatch_tasks)(*mut core::ffi::c_voidhandle, amd_pp_task task_id,
			amd_pm_state_type *user_state);
	int (*load_firmware)(*mut core::ffi::c_voidhandle);
	int (*wait_for_fw_loading_complete)(*mut core::ffi::c_voidhandle);
	int (*set_powergating_by_smu)(*mut core::ffi::c_voidhandle,
				u32 block_type,
				bool gate,
				int inst);
	int (*set_clockgating_by_smu)(*mut core::ffi::c_voidhandle, u32 msg_id);
	int (*set_power_limit)(*mut core::ffi::c_voidhandle, u32 limit_type, u32 n);
	int (*get_power_limit)(*mut core::ffi::c_voidhandle, u32 *limit,
			pp_power_limit_level pp_limit_level,
			pp_power_type power_type);
	int (*get_power_profile_mode)(*mut core::ffi::c_voidhandle, *mut core::ffi::c_charbuf);
	int (*set_power_profile_mode)(*mut core::ffi::c_voidhandle, long *input, u32 size);
	int (*set_fine_grain_clk_vol)(*mut core::ffi::c_voidhandle, u32 type, long *input, u32 size);
	int (*odn_edit_dpm_table)(*mut core::ffi::c_voidhandle, PP_OD_DPM_TABLE_COMMAND type,
				  long *input, u32 size);
	int (*set_mp1_state)(*mut core::ffi::c_voidhandle, pp_mp1_state mp1_state);
	int (*smu_i2c_bus_access)(*mut core::ffi::c_voidhandle, bool acquire);
	int (*gfx_state_change_set)(*mut core::ffi::c_voidhandle, u32 state);
/* export to DC */
	u32 (*get_sclk)(*mut core::ffi::c_voidhandle, bool low);
	u32 (*get_mclk)(*mut core::ffi::c_voidhandle, bool low);
	int (*display_configuration_change)(*mut core::ffi::c_voidhandle,
		const amd_pp_display_configuration *input);
	int (*get_current_clocks)(*mut core::ffi::c_voidhandle,
		amd_pp_clock_info *clocks);
	int (*get_clock_by_type)(*mut core::ffi::c_voidhandle,
		amd_pp_clock_type type,
		amd_pp_clocks *clocks);
	int (*get_clock_by_type_with_latency)(*mut core::ffi::c_voidhandle,
		amd_pp_clock_type type,
		pp_clock_levels_with_latency *clocks);
	int (*get_clock_by_type_with_voltage)(*mut core::ffi::c_voidhandle,
		amd_pp_clock_type type,
		pp_clock_levels_with_voltage *clocks);
	int (*set_watermarks_for_clocks_ranges)(*mut core::ffi::c_voidhandle,
						*mut core::ffi::c_voidclock_ranges);
	int (*display_clock_voltage_request)(*mut core::ffi::c_voidhandle,
				pp_display_clock_request *clock);
	int (*get_display_mode_validation_clocks)(*mut core::ffi::c_voidhandle,
		amd_pp_simple_clock_info *clocks);
	int (*notify_smu_enable_pwe)(*mut core::ffi::c_voidhandle);
	int (*enable_mgpu_fan_boost)(*mut core::ffi::c_voidhandle);
	int (*set_active_display_count)(*mut core::ffi::c_voidhandle, u32 count);
	int (*set_hard_min_dcefclk_by_freq)(*mut core::ffi::c_voidhandle, u32 clock);
	int (*set_hard_min_fclk_by_freq)(*mut core::ffi::c_voidhandle, u32 clock);
	int (*set_min_deep_sleep_dcefclk)(*mut core::ffi::c_voidhandle, u32 clock);
	int (*get_asic_baco_capability)(*mut core::ffi::c_voidhandle);
	int (*get_asic_baco_state)(*mut core::ffi::c_voidhandle, int *state);
	int (*set_asic_baco_state)(*mut core::ffi::c_voidhandle, int state);
	int (*get_ppfeature_status)(*mut core::ffi::c_voidhandle, *mut core::ffi::c_charbuf);
	int (*set_ppfeature_status)(*mut core::ffi::c_voidhandle, u64 ppfeature_masks);
	int (*asic_reset_mode_2)(*mut core::ffi::c_voidhandle);
	int (*asic_reset_enable_gfx_features)(*mut core::ffi::c_voidhandle);
	int (*set_df_cstate)(*mut core::ffi::c_voidhandle, pp_df_cstate state);
	int (*set_xgmi_pstate)(*mut core::ffi::c_voidhandle, u32 pstate);
	isize (*get_gpu_metrics)(*mut core::ffi::c_voidhandle, *mut core::ffi::c_void*table);
	isize (*get_temp_metrics)(*mut core::ffi::c_voidhandle, smu_temp_metric_type type, *mut core::ffi::c_voidtable);
	bool (*temp_metrics_is_supported)(*mut core::ffi::c_voidhandle, smu_temp_metric_type type);
	isize (*get_xcp_metrics)(*mut core::ffi::c_voidhandle, int xcp_id, *mut core::ffi::c_voidtable);
	isize (*get_pm_metrics)(*mut core::ffi::c_voidhandle, *mut core::ffi::c_voidpmmetrics, usize size);
	int (*set_watermarks_for_clock_ranges)(*mut core::ffi::c_voidhandle,
					       pp_smu_wm_range_sets *ranges);
	int (*display_disable_memory_clock_switch)(*mut core::ffi::c_voidhandle,
						   bool disable_memory_clock_switch);
	int (*get_max_sustainable_clocks_by_dc)(*mut core::ffi::c_voidhandle,
						pp_smu_nv_clock_table *max_clocks);
	int (*get_uclk_dpm_states)(*mut core::ffi::c_voidhandle,
				   unsigned int *clock_values_in_khz,
				   unsigned int *num_states);
	int (*get_dpm_clock_table)(*mut core::ffi::c_voidhandle,
				   dpm_clocks *clock_table);
	int (*get_smu_prv_buf_details)(*mut core::ffi::c_voidhandle, *mut core::ffi::c_void*addr, usize *size);
	void (*pm_compute_clocks)(*mut core::ffi::c_voidhandle);
	int (*notify_rlc_state)(*mut core::ffi::c_voidhandle, bool en);
};

#[repr(C)]\npub metrics_table_header {
	structure_size: u16;
	format_revision: u8;
	content_revision: u8;
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub amdgpu_metrics_attr_id {
	AMDGPU_METRICS_ATTR_ID_TEMPERATURE_HOTSPOT,
	AMDGPU_METRICS_ATTR_ID_TEMPERATURE_MEM,
	AMDGPU_METRICS_ATTR_ID_TEMPERATURE_VRSOC,
	AMDGPU_METRICS_ATTR_ID_CURR_SOCKET_POWER,
	AMDGPU_METRICS_ATTR_ID_AVERAGE_GFX_ACTIVITY,
	AMDGPU_METRICS_ATTR_ID_AVERAGE_UMC_ACTIVITY,
	AMDGPU_METRICS_ATTR_ID_MEM_MAX_BANDWIDTH,
	AMDGPU_METRICS_ATTR_ID_ENERGY_ACCUMULATOR,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_CLOCK_COUNTER,
	AMDGPU_METRICS_ATTR_ID_ACCUMULATION_COUNTER,
	AMDGPU_METRICS_ATTR_ID_PROCHOT_RESIDENCY_ACC,
	AMDGPU_METRICS_ATTR_ID_PPT_RESIDENCY_ACC,
	AMDGPU_METRICS_ATTR_ID_SOCKET_THM_RESIDENCY_ACC,
	AMDGPU_METRICS_ATTR_ID_VR_THM_RESIDENCY_ACC,
	AMDGPU_METRICS_ATTR_ID_HBM_THM_RESIDENCY_ACC,
	AMDGPU_METRICS_ATTR_ID_GFXCLK_LOCK_STATUS,
	AMDGPU_METRICS_ATTR_ID_PCIE_LINK_WIDTH,
	AMDGPU_METRICS_ATTR_ID_PCIE_LINK_SPEED,
	AMDGPU_METRICS_ATTR_ID_XGMI_LINK_WIDTH,
	AMDGPU_METRICS_ATTR_ID_XGMI_LINK_SPEED,
	AMDGPU_METRICS_ATTR_ID_GFX_ACTIVITY_ACC,
	AMDGPU_METRICS_ATTR_ID_MEM_ACTIVITY_ACC,
	AMDGPU_METRICS_ATTR_ID_PCIE_BANDWIDTH_ACC,
	AMDGPU_METRICS_ATTR_ID_PCIE_BANDWIDTH_INST,
	AMDGPU_METRICS_ATTR_ID_PCIE_L0_TO_RECOV_COUNT_ACC,
	AMDGPU_METRICS_ATTR_ID_PCIE_REPLAY_COUNT_ACC,
	AMDGPU_METRICS_ATTR_ID_PCIE_REPLAY_ROVER_COUNT_ACC,
	AMDGPU_METRICS_ATTR_ID_PCIE_NAK_SENT_COUNT_ACC,
	AMDGPU_METRICS_ATTR_ID_PCIE_NAK_RCVD_COUNT_ACC,
	AMDGPU_METRICS_ATTR_ID_XGMI_READ_DATA_ACC,
	AMDGPU_METRICS_ATTR_ID_XGMI_WRITE_DATA_ACC,
	AMDGPU_METRICS_ATTR_ID_XGMI_LINK_STATUS,
	AMDGPU_METRICS_ATTR_ID_FIRMWARE_TIMESTAMP,
	AMDGPU_METRICS_ATTR_ID_CURRENT_GFXCLK,
	AMDGPU_METRICS_ATTR_ID_CURRENT_SOCCLK,
	AMDGPU_METRICS_ATTR_ID_CURRENT_VCLK0,
	AMDGPU_METRICS_ATTR_ID_CURRENT_DCLK0,
	AMDGPU_METRICS_ATTR_ID_CURRENT_UCLK,
	AMDGPU_METRICS_ATTR_ID_NUM_PARTITION,
	AMDGPU_METRICS_ATTR_ID_PCIE_LC_PERF_OTHER_END_RECOVERY,
	AMDGPU_METRICS_ATTR_ID_GFX_BUSY_INST,
	AMDGPU_METRICS_ATTR_ID_JPEG_BUSY,
	AMDGPU_METRICS_ATTR_ID_VCN_BUSY,
	AMDGPU_METRICS_ATTR_ID_GFX_BUSY_ACC,
	AMDGPU_METRICS_ATTR_ID_GFX_BELOW_HOST_LIMIT_PPT_ACC,
	AMDGPU_METRICS_ATTR_ID_GFX_BELOW_HOST_LIMIT_THM_ACC,
	AMDGPU_METRICS_ATTR_ID_GFX_LOW_UTILIZATION_ACC,
	AMDGPU_METRICS_ATTR_ID_GFX_BELOW_HOST_LIMIT_TOTAL_ACC,
	AMDGPU_METRICS_ATTR_ID_TEMPERATURE_HBM,
	AMDGPU_METRICS_ATTR_ID_TEMPERATURE_MID,
	AMDGPU_METRICS_ATTR_ID_TEMPERATURE_AID,
	AMDGPU_METRICS_ATTR_ID_TEMPERATURE_XCD,
	AMDGPU_METRICS_ATTR_ID_LABEL_VERSION,
	AMDGPU_METRICS_ATTR_ID_NODE_ID,
	AMDGPU_METRICS_ATTR_ID_NODE_TEMP_RETIMER,
	AMDGPU_METRICS_ATTR_ID_NODE_TEMP_IBC,
	AMDGPU_METRICS_ATTR_ID_NODE_TEMP_IBC_2,
	AMDGPU_METRICS_ATTR_ID_NODE_TEMP_VDD18_VR,
	AMDGPU_METRICS_ATTR_ID_NODE_TEMP_04_HBM_B_VR,
	AMDGPU_METRICS_ATTR_ID_NODE_TEMP_04_HBM_D_VR,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_SOCIO_A,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_SOCIO_C,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_X0,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_X1,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_HBM_B,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_HBM_D,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_04_HBM_B,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_04_HBM_D,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_HBM_B,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_HBM_D,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_075_HBM_B,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_075_HBM_D,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_11_GTA_A,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_11_GTA_C,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDAN_075_GTA_A,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDAN_075_GTA_C,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDCR_075_UCIE,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_065_UCIEAA,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_065_UCIEAM_A,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDIO_065_UCIEAM_C,
	AMDGPU_METRICS_ATTR_ID_VR_TEMP_VDDAN_075,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_FPGA,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_FRONT,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_BACK,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_OAM7,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_IBC,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_UFPGA,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_OAM1,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_0_1_HSC,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_2_3_HSC,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_4_5_HSC,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_6_7_HSC,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_FPGA_0V72_VR,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_UBB_FPGA_3V3_VR,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_0_1_2_3_1V2_VR,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_4_5_6_7_1V2_VR,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_0_1_0V9_VR,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_4_5_0V9_VR,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_2_3_0V9_VR,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_RETIMER_6_7_0V9_VR,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_0_1_2_3_3V3_VR,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_OAM_4_5_6_7_3V3_VR,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_IBC_HSC,
	AMDGPU_METRICS_ATTR_ID_SYSTEM_TEMP_IBC,
	AMDGPU_METRICS_ATTR_ID_MAX,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub amdgpu_metrics_attr_type {
	AMDGPU_METRICS_TYPE_U8,
	AMDGPU_METRICS_TYPE_S8,
	AMDGPU_METRICS_TYPE_U16,
	AMDGPU_METRICS_TYPE_S16,
	AMDGPU_METRICS_TYPE_U32,
	AMDGPU_METRICS_TYPE_S32,
	AMDGPU_METRICS_TYPE_U64,
	AMDGPU_METRICS_TYPE_S64,
	AMDGPU_METRICS_TYPE_MAX,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub amdgpu_metrics_attr_unit {
	/* None */
	AMDGPU_METRICS_UNIT_NONE,
	/* MHz*/
	AMDGPU_METRICS_UNIT_CLOCK_1,
	/* Degree Celsius*/
	AMDGPU_METRICS_UNIT_TEMP_1,
	/* Watts*/
	AMDGPU_METRICS_UNIT_POWER_1,
	/* In nanoseconds*/
	AMDGPU_METRICS_UNIT_TIME_1,
	/* In 10 nanoseconds*/
	AMDGPU_METRICS_UNIT_TIME_2,
	/* Speed in GT/s */
	AMDGPU_METRICS_UNIT_SPEED_1,
	/* Speed in 0.1 GT/s */
	AMDGPU_METRICS_UNIT_SPEED_2,
	/* Bandwidth GB/s */
	AMDGPU_METRICS_UNIT_BW_1,
	/* Data in KB */
	AMDGPU_METRICS_UNIT_DATA_1,
	/* Percentage */
	AMDGPU_METRICS_UNIT_PERCENT,
	AMDGPU_METRICS_UNIT_MAX,
};

pub const AMDGPU_METRICS_ATTR_UNIT_MASK: u64 = 0FF000000;
pub const AMDGPU_METRICS_ATTR_UNIT_SHIFT: u64 = 24;
pub const AMDGPU_METRICS_ATTR_TYPE_MASK: u64 = 000F00000;
pub const AMDGPU_METRICS_ATTR_TYPE_SHIFT: u64 = 20;
pub const AMDGPU_METRICS_ATTR_ID_MASK: u64 = 0000FFC00;
pub const AMDGPU_METRICS_ATTR_ID_SHIFT: u64 = 10;
pub const AMDGPU_METRICS_ATTR_INST_MASK: u64 = 0000003FF;
pub const AMDGPU_METRICS_ATTR_INST_SHIFT: u64 = 0;

pub const AMDGPU_METRICS_ENC_ATTR: () = (); // C function-like macro preserved in comment

/*
 * gpu_metrics_v1_0 is not recommended as it's not naturally aligned.
 * Use gpu_metrics_v1_1 or later instead.
 */
#[repr(C)]\npub gpu_metrics_v1_0 {
	common_header: metrics_table_header;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Temperature */
	temperature_edge: u16;
	temperature_hotspot: u16;
	temperature_mem: u16;
	temperature_vrgfx: u16;
	temperature_vrsoc: u16;
	temperature_vrmem: u16;

	/* Utilization */
	average_gfx_activity: u16;
	average_umc_activity: u16; // memory controller
	average_mm_activity: u16; // UVD or VCN

	/* Power/Energy */
	average_socket_power: u16;
	energy_accumulator: u32;

	/* Average clocks */
	average_gfxclk_frequency: u16;
	average_socclk_frequency: u16;
	average_uclk_frequency: u16;
	average_vclk0_frequency: u16;
	average_dclk0_frequency: u16;
	average_vclk1_frequency: u16;
	average_dclk1_frequency: u16;

	/* Current clocks */
	current_gfxclk: u16;
	current_socclk: u16;
	current_uclk: u16;
	current_vclk0: u16;
	current_dclk0: u16;
	current_vclk1: u16;
	current_dclk1: u16;

	/* Throttle status */
	throttle_status: u32;

	/* Fans */
	current_fan_speed: u16;

	/* Link width/speed */
	pcie_link_width: u8;
	pcie_link_speed: u8; // in 0.1 GT/s
};

#[repr(C)]\npub gpu_metrics_v1_1 {
	common_header: metrics_table_header;

	/* Temperature */
	temperature_edge: u16;
	temperature_hotspot: u16;
	temperature_mem: u16;
	temperature_vrgfx: u16;
	temperature_vrsoc: u16;
	temperature_vrmem: u16;

	/* Utilization */
	average_gfx_activity: u16;
	average_umc_activity: u16; // memory controller
	average_mm_activity: u16; // UVD or VCN

	/* Power/Energy */
	average_socket_power: u16;
	energy_accumulator: u64;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Average clocks */
	average_gfxclk_frequency: u16;
	average_socclk_frequency: u16;
	average_uclk_frequency: u16;
	average_vclk0_frequency: u16;
	average_dclk0_frequency: u16;
	average_vclk1_frequency: u16;
	average_dclk1_frequency: u16;

	/* Current clocks */
	current_gfxclk: u16;
	current_socclk: u16;
	current_uclk: u16;
	current_vclk0: u16;
	current_dclk0: u16;
	current_vclk1: u16;
	current_dclk1: u16;

	/* Throttle status */
	throttle_status: u32;

	/* Fans */
	current_fan_speed: u16;

	/* Link width/speed */
	pcie_link_width: u16;
	pcie_link_speed: u16; // in 0.1 GT/s

	padding: u16;

	gfx_activity_acc: u32;
	mem_activity_acc: u32;

	temperature_hbm: [u16; NUM_HBM_INSTANCES];
};

#[repr(C)]\npub gpu_metrics_v1_2 {
	common_header: metrics_table_header;

	/* Temperature */
	temperature_edge: u16;
	temperature_hotspot: u16;
	temperature_mem: u16;
	temperature_vrgfx: u16;
	temperature_vrsoc: u16;
	temperature_vrmem: u16;

	/* Utilization */
	average_gfx_activity: u16;
	average_umc_activity: u16; // memory controller
	average_mm_activity: u16; // UVD or VCN

	/* Power/Energy */
	average_socket_power: u16;
	energy_accumulator: u64;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Average clocks */
	average_gfxclk_frequency: u16;
	average_socclk_frequency: u16;
	average_uclk_frequency: u16;
	average_vclk0_frequency: u16;
	average_dclk0_frequency: u16;
	average_vclk1_frequency: u16;
	average_dclk1_frequency: u16;

	/* Current clocks */
	current_gfxclk: u16;
	current_socclk: u16;
	current_uclk: u16;
	current_vclk0: u16;
	current_dclk0: u16;
	current_vclk1: u16;
	current_dclk1: u16;

	/* Throttle status (ASIC dependent) */
	throttle_status: u32;

	/* Fans */
	current_fan_speed: u16;

	/* Link width/speed */
	pcie_link_width: u16;
	pcie_link_speed: u16; // in 0.1 GT/s

	padding: u16;

	gfx_activity_acc: u32;
	mem_activity_acc: u32;

	temperature_hbm: [u16; NUM_HBM_INSTANCES];

	/* PMFW attached timestamp (10ns resolution) */
	firmware_timestamp: u64;
};

#[repr(C)]\npub gpu_metrics_v1_3 {
	common_header: metrics_table_header;

	/* Temperature */
	temperature_edge: u16;
	temperature_hotspot: u16;
	temperature_mem: u16;
	temperature_vrgfx: u16;
	temperature_vrsoc: u16;
	temperature_vrmem: u16;

	/* Utilization */
	average_gfx_activity: u16;
	average_umc_activity: u16; // memory controller
	average_mm_activity: u16; // UVD or VCN

	/* Power/Energy */
	average_socket_power: u16;
	energy_accumulator: u64;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Average clocks */
	average_gfxclk_frequency: u16;
	average_socclk_frequency: u16;
	average_uclk_frequency: u16;
	average_vclk0_frequency: u16;
	average_dclk0_frequency: u16;
	average_vclk1_frequency: u16;
	average_dclk1_frequency: u16;

	/* Current clocks */
	current_gfxclk: u16;
	current_socclk: u16;
	current_uclk: u16;
	current_vclk0: u16;
	current_dclk0: u16;
	current_vclk1: u16;
	current_dclk1: u16;

	/* Throttle status */
	throttle_status: u32;

	/* Fans */
	current_fan_speed: u16;

	/* Link width/speed */
	pcie_link_width: u16;
	pcie_link_speed: u16; // in 0.1 GT/s

	padding: u16;

	gfx_activity_acc: u32;
	mem_activity_acc: u32;

	temperature_hbm: [u16; NUM_HBM_INSTANCES];

	/* PMFW attached timestamp (10ns resolution) */
	firmware_timestamp: u64;

	/* Voltage (mV) */
	voltage_soc: u16;
	voltage_gfx: u16;
	voltage_mem: u16;

	padding1: u16;

	/* Throttle status (ASIC independent) */
	indep_throttle_status: u64;
};

#[repr(C)]\npub gpu_metrics_v1_4 {
	common_header: metrics_table_header;

	/* Temperature (Celsius) */
	temperature_hotspot: u16;
	temperature_mem: u16;
	temperature_vrsoc: u16;

	/* Power (Watts) */
	curr_socket_power: u16;

	/* Utilization (%) */
	average_gfx_activity: u16;
	average_umc_activity: u16; // memory controller
	vcn_activity: [u16; NUM_VCN];

	/* Energy (15.259uJ (2^-16) units) */
	energy_accumulator: u64;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Throttle status */
	throttle_status: u32;

	/* Clock Lock Status. Each bit corresponds to clock instance */
	gfxclk_lock_status: u32;

	/* Link width (number of lanes) and speed (in 0.1 GT/s) */
	pcie_link_width: u16;
	pcie_link_speed: u16;

	/* XGMI bus width and bitrate (in Gbps) */
	xgmi_link_width: u16;
	xgmi_link_speed: u16;

	/* Utilization Accumulated (%) */
	gfx_activity_acc: u32;
	mem_activity_acc: u32;

	/*PCIE accumulated bandwidth (GB/sec) */
	pcie_bandwidth_acc: u64;

	/*PCIE instantaneous bandwidth (GB/sec) */
	pcie_bandwidth_inst: u64;

	/* PCIE L0 to recovery state transition accumulated count */
	pcie_l0_to_recov_count_acc: u64;

	/* PCIE replay accumulated count */
	pcie_replay_count_acc: u64;

	/* PCIE replay rollover accumulated count */
	pcie_replay_rover_count_acc: u64;

	/* XGMI accumulated data transfer size(KiloBytes) */
	xgmi_read_data_acc: [u64; NUM_XGMI_LINKS];
	xgmi_write_data_acc: [u64; NUM_XGMI_LINKS];

	/* PMFW attached timestamp (10ns resolution) */
	firmware_timestamp: u64;

	/* Current clocks (Mhz) */
	current_gfxclk: [u16; MAX_GFX_CLKS];
	current_socclk: [u16; MAX_CLKS];
	current_vclk0: [u16; MAX_CLKS];
	current_dclk0: [u16; MAX_CLKS];
	current_uclk: u16;

	padding: u16;
};

#[repr(C)]\npub gpu_metrics_v1_5 {
	common_header: metrics_table_header;

	/* Temperature (Celsius) */
	temperature_hotspot: u16;
	temperature_mem: u16;
	temperature_vrsoc: u16;

	/* Power (Watts) */
	curr_socket_power: u16;

	/* Utilization (%) */
	average_gfx_activity: u16;
	average_umc_activity: u16; // memory controller
	vcn_activity: [u16; NUM_VCN];
	jpeg_activity: [u16; NUM_JPEG_ENG];

	/* Energy (15.259uJ (2^-16) units) */
	energy_accumulator: u64;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Throttle status */
	throttle_status: u32;

	/* Clock Lock Status. Each bit corresponds to clock instance */
	gfxclk_lock_status: u32;

	/* Link width (number of lanes) and speed (in 0.1 GT/s) */
	pcie_link_width: u16;
	pcie_link_speed: u16;

	/* XGMI bus width and bitrate (in Gbps) */
	xgmi_link_width: u16;
	xgmi_link_speed: u16;

	/* Utilization Accumulated (%) */
	gfx_activity_acc: u32;
	mem_activity_acc: u32;

	/*PCIE accumulated bandwidth (GB/sec) */
	pcie_bandwidth_acc: u64;

	/*PCIE instantaneous bandwidth (GB/sec) */
	pcie_bandwidth_inst: u64;

	/* PCIE L0 to recovery state transition accumulated count */
	pcie_l0_to_recov_count_acc: u64;

	/* PCIE replay accumulated count */
	pcie_replay_count_acc: u64;

	/* PCIE replay rollover accumulated count */
	pcie_replay_rover_count_acc: u64;

	/* PCIE NAK sent  accumulated count */
	pcie_nak_sent_count_acc: u32;

	/* PCIE NAK received accumulated count */
	pcie_nak_rcvd_count_acc: u32;

	/* XGMI accumulated data transfer size(KiloBytes) */
	xgmi_read_data_acc: [u64; NUM_XGMI_LINKS];
	xgmi_write_data_acc: [u64; NUM_XGMI_LINKS];

	/* PMFW attached timestamp (10ns resolution) */
	firmware_timestamp: u64;

	/* Current clocks (Mhz) */
	current_gfxclk: [u16; MAX_GFX_CLKS];
	current_socclk: [u16; MAX_CLKS];
	current_vclk0: [u16; MAX_CLKS];
	current_dclk0: [u16; MAX_CLKS];
	current_uclk: u16;

	padding: u16;
};

#[repr(C)]\npub gpu_metrics_v1_6 {
	common_header: metrics_table_header;

	/* Temperature (Celsius) */
	temperature_hotspot: u16;
	temperature_mem: u16;
	temperature_vrsoc: u16;

	/* Power (Watts) */
	curr_socket_power: u16;

	/* Utilization (%) */
	average_gfx_activity: u16;
	average_umc_activity: u16; // memory controller

	/* Energy (15.259uJ (2^-16) units) */
	energy_accumulator: u64;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Accumulation cycle counter */
	accumulation_counter: u32;

	/* Accumulated throttler residencies */
	prochot_residency_acc: u32;
	ppt_residency_acc: u32;
	socket_thm_residency_acc: u32;
	vr_thm_residency_acc: u32;
	hbm_thm_residency_acc: u32;

	/* Clock Lock Status. Each bit corresponds to clock instance */
	gfxclk_lock_status: u32;

	/* Link width (number of lanes) and speed (in 0.1 GT/s) */
	pcie_link_width: u16;
	pcie_link_speed: u16;

	/* XGMI bus width and bitrate (in Gbps) */
	xgmi_link_width: u16;
	xgmi_link_speed: u16;

	/* Utilization Accumulated (%) */
	gfx_activity_acc: u32;
	mem_activity_acc: u32;

	/*PCIE accumulated bandwidth (GB/sec) */
	pcie_bandwidth_acc: u64;

	/*PCIE instantaneous bandwidth (GB/sec) */
	pcie_bandwidth_inst: u64;

	/* PCIE L0 to recovery state transition accumulated count */
	pcie_l0_to_recov_count_acc: u64;

	/* PCIE replay accumulated count */
	pcie_replay_count_acc: u64;

	/* PCIE replay rollover accumulated count */
	pcie_replay_rover_count_acc: u64;

	/* PCIE NAK sent  accumulated count */
	pcie_nak_sent_count_acc: u32;

	/* PCIE NAK received accumulated count */
	pcie_nak_rcvd_count_acc: u32;

	/* XGMI accumulated data transfer size(KiloBytes) */
	xgmi_read_data_acc: [u64; NUM_XGMI_LINKS];
	xgmi_write_data_acc: [u64; NUM_XGMI_LINKS];

	/* PMFW attached timestamp (10ns resolution) */
	firmware_timestamp: u64;

	/* Current clocks (Mhz) */
	current_gfxclk: [u16; MAX_GFX_CLKS];
	current_socclk: [u16; MAX_CLKS];
	current_vclk0: [u16; MAX_CLKS];
	current_dclk0: [u16; MAX_CLKS];
	current_uclk: u16;

	/* Number of current partition */
	num_partition: u16;

	/* XCP metrics stats */
	xcp_stats: [amdgpu_xcp_metrics; NUM_XCP];

	/* PCIE other end recovery counter */
	pcie_lc_perf_other_end_recovery: u32;
};

#[repr(C)]\npub gpu_metrics_v1_7 {
	common_header: metrics_table_header;

	/* Temperature (Celsius) */
	temperature_hotspot: u16;
	temperature_mem: u16;
	temperature_vrsoc: u16;

	/* Power (Watts) */
	curr_socket_power: u16;

	/* Utilization (%) */
	average_gfx_activity: u16;
	average_umc_activity: u16; // memory controller

	/* VRAM max bandwidthi (in GB/sec) at max memory clock */
	mem_max_bandwidth: u64;

	/* Energy (15.259uJ (2^-16) units) */
	energy_accumulator: u64;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Accumulation cycle counter */
	accumulation_counter: u32;

	/* Accumulated throttler residencies */
	prochot_residency_acc: u32;
	ppt_residency_acc: u32;
	socket_thm_residency_acc: u32;
	vr_thm_residency_acc: u32;
	hbm_thm_residency_acc: u32;

	/* Clock Lock Status. Each bit corresponds to clock instance */
	gfxclk_lock_status: u32;

	/* Link width (number of lanes) and speed (in 0.1 GT/s) */
	pcie_link_width: u16;
	pcie_link_speed: u16;

	/* XGMI bus width and bitrate (in Gbps) */
	xgmi_link_width: u16;
	xgmi_link_speed: u16;

	/* Utilization Accumulated (%) */
	gfx_activity_acc: u32;
	mem_activity_acc: u32;

	/*PCIE accumulated bandwidth (GB/sec) */
	pcie_bandwidth_acc: u64;

	/*PCIE instantaneous bandwidth (GB/sec) */
	pcie_bandwidth_inst: u64;

	/* PCIE L0 to recovery state transition accumulated count */
	pcie_l0_to_recov_count_acc: u64;

	/* PCIE replay accumulated count */
	pcie_replay_count_acc: u64;

	/* PCIE replay rollover accumulated count */
	pcie_replay_rover_count_acc: u64;

	/* PCIE NAK sent  accumulated count */
	pcie_nak_sent_count_acc: u32;

	/* PCIE NAK received accumulated count */
	pcie_nak_rcvd_count_acc: u32;

	/* XGMI accumulated data transfer size(KiloBytes) */
	xgmi_read_data_acc: [u64; NUM_XGMI_LINKS];
	xgmi_write_data_acc: [u64; NUM_XGMI_LINKS];

	/* XGMI link status(active/inactive) */
	xgmi_link_status: [u16; NUM_XGMI_LINKS];

	padding: u16;

	/* PMFW attached timestamp (10ns resolution) */
	firmware_timestamp: u64;

	/* Current clocks (Mhz) */
	current_gfxclk: [u16; MAX_GFX_CLKS];
	current_socclk: [u16; MAX_CLKS];
	current_vclk0: [u16; MAX_CLKS];
	current_dclk0: [u16; MAX_CLKS];
	current_uclk: u16;

	/* Number of current partition */
	num_partition: u16;

	/* XCP metrics stats */
	xcp_stats: [amdgpu_xcp_metrics_v1_1; NUM_XCP];

	/* PCIE other end recovery counter */
	pcie_lc_perf_other_end_recovery: u32;
};

#[repr(C)]\npub gpu_metrics_v1_8 {
	common_header: metrics_table_header;

	/* Temperature (Celsius) */
	temperature_hotspot: u16;
	temperature_mem: u16;
	temperature_vrsoc: u16;

	/* Power (Watts) */
	curr_socket_power: u16;

	/* Utilization (%) */
	average_gfx_activity: u16;
	average_umc_activity: u16; // memory controller

	/* VRAM max bandwidthi (in GB/sec) at max memory clock */
	mem_max_bandwidth: u64;

	/* Energy (15.259uJ (2^-16) units) */
	energy_accumulator: u64;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Accumulation cycle counter */
	accumulation_counter: u32;

	/* Accumulated throttler residencies */
	prochot_residency_acc: u32;
	ppt_residency_acc: u32;
	socket_thm_residency_acc: u32;
	vr_thm_residency_acc: u32;
	hbm_thm_residency_acc: u32;

	/* Clock Lock Status. Each bit corresponds to clock instance */
	gfxclk_lock_status: u32;

	/* Link width (number of lanes) and speed (in 0.1 GT/s) */
	pcie_link_width: u16;
	pcie_link_speed: u16;

	/* XGMI bus width and bitrate (in Gbps) */
	xgmi_link_width: u16;
	xgmi_link_speed: u16;

	/* Utilization Accumulated (%) */
	gfx_activity_acc: u32;
	mem_activity_acc: u32;

	/*PCIE accumulated bandwidth (GB/sec) */
	pcie_bandwidth_acc: u64;

	/*PCIE instantaneous bandwidth (GB/sec) */
	pcie_bandwidth_inst: u64;

	/* PCIE L0 to recovery state transition accumulated count */
	pcie_l0_to_recov_count_acc: u64;

	/* PCIE replay accumulated count */
	pcie_replay_count_acc: u64;

	/* PCIE replay rollover accumulated count */
	pcie_replay_rover_count_acc: u64;

	/* PCIE NAK sent  accumulated count */
	pcie_nak_sent_count_acc: u32;

	/* PCIE NAK received accumulated count */
	pcie_nak_rcvd_count_acc: u32;

	/* XGMI accumulated data transfer size(KiloBytes) */
	xgmi_read_data_acc: [u64; NUM_XGMI_LINKS];
	xgmi_write_data_acc: [u64; NUM_XGMI_LINKS];

	/* XGMI link status(active/inactive) */
	xgmi_link_status: [u16; NUM_XGMI_LINKS];

	padding: u16;

	/* PMFW attached timestamp (10ns resolution) */
	firmware_timestamp: u64;

	/* Current clocks (Mhz) */
	current_gfxclk: [u16; MAX_GFX_CLKS];
	current_socclk: [u16; MAX_CLKS];
	current_vclk0: [u16; MAX_CLKS];
	current_dclk0: [u16; MAX_CLKS];
	current_uclk: u16;

	/* Number of current partition */
	num_partition: u16;

	/* XCP metrics stats */
	xcp_stats: [amdgpu_xcp_metrics_v1_2; NUM_XCP];

	/* PCIE other end recovery counter */
	pcie_lc_perf_other_end_recovery: u32;
};

#[repr(C)]\npub gpu_metrics_attr {
	/* Field type encoded with AMDGPU_METRICS_ENC_ATTR */
	attr_encoding: u64;
	/* Attribute value, depends on attr_encoding */
	*mut core::ffi::c_voidattr_value;
};

#[repr(C)]\npub gpu_metrics_v1_9 {
	common_header: metrics_table_header;
	attr_count: int;
	gpu_metrics_attr metrics_attrs: [u8; 0];
};

/*
 * gpu_metrics_v2_0 is not recommended as it's not naturally aligned.
 * Use gpu_metrics_v2_1 or later instead.
 */
#[repr(C)]\npub gpu_metrics_v2_0 {
	common_header: metrics_table_header;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Temperature */
	temperature_gfx: u16; // gfx temperature on APUs
	temperature_soc: u16; // soc temperature on APUs
	temperature_core: [u16; 8]; // CPU core temperature on APUs
	temperature_l3: [u16; 2];

	/* Utilization */
	average_gfx_activity: u16;
	average_mm_activity: u16; // UVD or VCN

	/* Power/Energy */
	average_socket_power: u16; // dGPU + APU power on A + A platform
	average_cpu_power: u16;
	average_soc_power: u16;
	average_gfx_power: u16;
	average_core_power: [u16; 8]; // CPU core power on APUs

	/* Average clocks */
	average_gfxclk_frequency: u16;
	average_socclk_frequency: u16;
	average_uclk_frequency: u16;
	average_fclk_frequency: u16;
	average_vclk_frequency: u16;
	average_dclk_frequency: u16;

	/* Current clocks */
	current_gfxclk: u16;
	current_socclk: u16;
	current_uclk: u16;
	current_fclk: u16;
	current_vclk: u16;
	current_dclk: u16;
	current_coreclk: [u16; 8]; // CPU core clocks
	current_l3clk: [u16; 2];

	/* Throttle status */
	throttle_status: u32;

	/* Fans */
	fan_pwm: u16;

	padding: u16;
};

#[repr(C)]\npub gpu_metrics_v2_1 {
	common_header: metrics_table_header;

	/* Temperature */
	temperature_gfx: u16; // gfx temperature on APUs
	temperature_soc: u16; // soc temperature on APUs
	temperature_core: [u16; 8]; // CPU core temperature on APUs
	temperature_l3: [u16; 2];

	/* Utilization */
	average_gfx_activity: u16;
	average_mm_activity: u16; // UVD or VCN

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Power/Energy */
	average_socket_power: u16; // dGPU + APU power on A + A platform
	average_cpu_power: u16;
	average_soc_power: u16;
	average_gfx_power: u16;
	average_core_power: [u16; 8]; // CPU core power on APUs

	/* Average clocks */
	average_gfxclk_frequency: u16;
	average_socclk_frequency: u16;
	average_uclk_frequency: u16;
	average_fclk_frequency: u16;
	average_vclk_frequency: u16;
	average_dclk_frequency: u16;

	/* Current clocks */
	current_gfxclk: u16;
	current_socclk: u16;
	current_uclk: u16;
	current_fclk: u16;
	current_vclk: u16;
	current_dclk: u16;
	current_coreclk: [u16; 8]; // CPU core clocks
	current_l3clk: [u16; 2];

	/* Throttle status */
	throttle_status: u32;

	/* Fans */
	fan_pwm: u16;

	padding: [u16; 3];
};

#[repr(C)]\npub gpu_metrics_v2_2 {
	common_header: metrics_table_header;

	/* Temperature */
	temperature_gfx: u16; // gfx temperature on APUs
	temperature_soc: u16; // soc temperature on APUs
	temperature_core: [u16; 8]; // CPU core temperature on APUs
	temperature_l3: [u16; 2];

	/* Utilization */
	average_gfx_activity: u16;
	average_mm_activity: u16; // UVD or VCN

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Power/Energy */
	average_socket_power: u16; // dGPU + APU power on A + A platform
	average_cpu_power: u16;
	average_soc_power: u16;
	average_gfx_power: u16;
	average_core_power: [u16; 8]; // CPU core power on APUs

	/* Average clocks */
	average_gfxclk_frequency: u16;
	average_socclk_frequency: u16;
	average_uclk_frequency: u16;
	average_fclk_frequency: u16;
	average_vclk_frequency: u16;
	average_dclk_frequency: u16;

	/* Current clocks */
	current_gfxclk: u16;
	current_socclk: u16;
	current_uclk: u16;
	current_fclk: u16;
	current_vclk: u16;
	current_dclk: u16;
	current_coreclk: [u16; 8]; // CPU core clocks
	current_l3clk: [u16; 2];

	/* Throttle status (ASIC dependent) */
	throttle_status: u32;

	/* Fans */
	fan_pwm: u16;

	padding: [u16; 3];

	/* Throttle status (ASIC independent) */
	indep_throttle_status: u64;
};

#[repr(C)]\npub gpu_metrics_v2_3 {
	common_header: metrics_table_header;

	/* Temperature */
	temperature_gfx: u16; // gfx temperature on APUs
	temperature_soc: u16; // soc temperature on APUs
	temperature_core: [u16; 8]; // CPU core temperature on APUs
	temperature_l3: [u16; 2];

	/* Utilization */
	average_gfx_activity: u16;
	average_mm_activity: u16; // UVD or VCN

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Power/Energy */
	average_socket_power: u16; // dGPU + APU power on A + A platform
	average_cpu_power: u16;
	average_soc_power: u16;
	average_gfx_power: u16;
	average_core_power: [u16; 8]; // CPU core power on APUs

	/* Average clocks */
	average_gfxclk_frequency: u16;
	average_socclk_frequency: u16;
	average_uclk_frequency: u16;
	average_fclk_frequency: u16;
	average_vclk_frequency: u16;
	average_dclk_frequency: u16;

	/* Current clocks */
	current_gfxclk: u16;
	current_socclk: u16;
	current_uclk: u16;
	current_fclk: u16;
	current_vclk: u16;
	current_dclk: u16;
	current_coreclk: [u16; 8]; // CPU core clocks
	current_l3clk: [u16; 2];

	/* Throttle status (ASIC dependent) */
	throttle_status: u32;

	/* Fans */
	fan_pwm: u16;

	padding: [u16; 3];

	/* Throttle status (ASIC independent) */
	indep_throttle_status: u64;

	/* Average Temperature */
	average_temperature_gfx: u16; // average gfx temperature on APUs
	average_temperature_soc: u16; // average soc temperature on APUs
	average_temperature_core: [u16; 8]; // average CPU core temperature on APUs
	average_temperature_l3: [u16; 2];
};

#[repr(C)]\npub gpu_metrics_v2_4 {
	common_header: metrics_table_header;

	/* Temperature (unit: centi-Celsius) */
	temperature_gfx: u16;
	temperature_soc: u16;
	temperature_core: [u16; 8];
	temperature_l3: [u16; 2];

	/* Utilization (unit: centi) */
	average_gfx_activity: u16;
	average_mm_activity: u16;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Power/Energy (unit: mW) */
	average_socket_power: u16;
	average_cpu_power: u16;
	average_soc_power: u16;
	average_gfx_power: u16;
	average_core_power: [u16; 8];

	/* Average clocks (unit: MHz) */
	average_gfxclk_frequency: u16;
	average_socclk_frequency: u16;
	average_uclk_frequency: u16;
	average_fclk_frequency: u16;
	average_vclk_frequency: u16;
	average_dclk_frequency: u16;

	/* Current clocks (unit: MHz) */
	current_gfxclk: u16;
	current_socclk: u16;
	current_uclk: u16;
	current_fclk: u16;
	current_vclk: u16;
	current_dclk: u16;
	current_coreclk: [u16; 8];
	current_l3clk: [u16; 2];

	/* Throttle status (ASIC dependent) */
	throttle_status: u32;

	/* Fans */
	fan_pwm: u16;

	padding: [u16; 3];

	/* Throttle status (ASIC independent) */
	indep_throttle_status: u64;

	/* Average Temperature (unit: centi-Celsius) */
	average_temperature_gfx: u16;
	average_temperature_soc: u16;
	average_temperature_core: [u16; 8];
	average_temperature_l3: [u16; 2];

	/* Power/Voltage (unit: mV) */
	average_cpu_voltage: u16;
	average_soc_voltage: u16;
	average_gfx_voltage: u16;

	/* Power/Current (unit: mA) */
	average_cpu_current: u16;
	average_soc_current: u16;
	average_gfx_current: u16;
};

#[repr(C)]\npub gpu_metrics_v3_0 {
	common_header: metrics_table_header;

	/* Temperature */
	/* gfx temperature on APUs */
	temperature_gfx: u16;
	/* soc temperature on APUs */
	temperature_soc: u16;
	/* CPU core temperature on APUs */
	temperature_core: [u16; 16];
	/* skin temperature on APUs */
	temperature_skin: u16;

	/* Utilization */
	/* time filtered GFX busy % [0-100] */
	average_gfx_activity: u16;
	/* time filtered VCN busy % [0-100] */
	average_vcn_activity: u16;
	/* time filtered IPU per-column busy % [0-100] */
	average_ipu_activity: [u16; 8];
	/* time filtered per-core C0 residency % [0-100]*/
	average_core_c0_activity: [u16; 16];
	/* time filtered DRAM read bandwidth [MB/sec] */
	average_dram_reads: u16;
	/* time filtered DRAM write bandwidth [MB/sec] */
	average_dram_writes: u16;
	/* time filtered IPU read bandwidth [MB/sec] */
	average_ipu_reads: u16;
	/* time filtered IPU write bandwidth [MB/sec] */
	average_ipu_writes: u16;

	/* Driver attached timestamp (in ns) */
	system_clock_counter: u64;

	/* Power/Energy */
	/* time filtered power used for PPT/STAPM [APU+dGPU] [mW] */
	average_socket_power: u32;
	/* time filtered IPU power [mW] */
	average_ipu_power: u16;
	/* time filtered APU power [mW] */
	average_apu_power: u32;
	/* time filtered GFX power [mW] */
	average_gfx_power: u32;
	/* time filtered dGPU power [mW] */
	average_dgpu_power: u32;
	/* time filtered sum of core power across all cores in the socket [mW] */
	average_all_core_power: u32;
	/* calculated core power [mW] */
	average_core_power: [u16; 16];
	/* time filtered total system power [mW] */
	average_sys_power: u16;
	/* maximum IRM defined STAPM power limit [mW] */
	stapm_power_limit: u16;
	/* time filtered STAPM power limit [mW] */
	current_stapm_power_limit: u16;

	/* time filtered clocks [MHz] */
	average_gfxclk_frequency: u16;
	average_socclk_frequency: u16;
	average_vpeclk_frequency: u16;
	average_ipuclk_frequency: u16;
	average_fclk_frequency: u16;
	average_vclk_frequency: u16;
	average_uclk_frequency: u16;
	average_mpipu_frequency: u16;

	/* Current clocks */
	/* target core frequency [MHz] */
	current_coreclk: [u16; 16];
	/* CCLK frequency limit enforced on classic cores [MHz] */
	current_core_maxfreq: u16;
	/* GFXCLK frequency limit enforced on GFX [MHz] */
	current_gfx_maxfreq: u16;

	/* Throttle Residency (ASIC dependent) */
	throttle_residency_prochot: u32;
	throttle_residency_spl: u32;
	throttle_residency_fppt: u32;
	throttle_residency_sppt: u32;
	throttle_residency_thm_core: u32;
	throttle_residency_thm_gfx: u32;
	throttle_residency_thm_soc: u32;

	/* Metrics table alpha filter time constant [us] */
	time_filter_alphavalue: u32;
};

#[repr(C)]\npub amdgpu_pmmetrics_header {
	structure_size: u16;
	pad: u16;
	mp1_ip_discovery_version: u32;
	pmfw_version: u32;
	pmmetrics_version: u32;
};

#[repr(C)]\npub amdgpu_pm_metrics {
	common_header: amdgpu_pmmetrics_header;

	u8 data: [u8; 0];
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub amdgpu_vr_temp {
	AMDGPU_VDDCR_VDD0_TEMP,
	AMDGPU_VDDCR_VDD1_TEMP,
	AMDGPU_VDDCR_VDD2_TEMP,
	AMDGPU_VDDCR_VDD3_TEMP,
	AMDGPU_VDDCR_SOC_A_TEMP,
	AMDGPU_VDDCR_SOC_C_TEMP,
	AMDGPU_VDDCR_SOCIO_A_TEMP,
	AMDGPU_VDDCR_SOCIO_C_TEMP,
	AMDGPU_VDD_085_HBM_TEMP,
	AMDGPU_VDDCR_11_HBM_B_TEMP,
	AMDGPU_VDDCR_11_HBM_D_TEMP,
	AMDGPU_VDD_USR_TEMP,
	AMDGPU_VDDIO_11_E32_TEMP,
	AMDGPU_VR_MAX_TEMP_ENTRIES,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub amdgpu_system_temp {
	AMDGPU_UBB_FPGA_TEMP,
	AMDGPU_UBB_FRONT_TEMP,
	AMDGPU_UBB_BACK_TEMP,
	AMDGPU_UBB_OAM7_TEMP,
	AMDGPU_UBB_IBC_TEMP,
	AMDGPU_UBB_UFPGA_TEMP,
	AMDGPU_UBB_OAM1_TEMP,
	AMDGPU_OAM_0_1_HSC_TEMP,
	AMDGPU_OAM_2_3_HSC_TEMP,
	AMDGPU_OAM_4_5_HSC_TEMP,
	AMDGPU_OAM_6_7_HSC_TEMP,
	AMDGPU_UBB_FPGA_0V72_VR_TEMP,
	AMDGPU_UBB_FPGA_3V3_VR_TEMP,
	AMDGPU_RETIMER_0_1_2_3_1V2_VR_TEMP,
	AMDGPU_RETIMER_4_5_6_7_1V2_VR_TEMP,
	AMDGPU_RETIMER_0_1_0V9_VR_TEMP,
	AMDGPU_RETIMER_4_5_0V9_VR_TEMP,
	AMDGPU_RETIMER_2_3_0V9_VR_TEMP,
	AMDGPU_RETIMER_6_7_0V9_VR_TEMP,
	AMDGPU_OAM_0_1_2_3_3V3_VR_TEMP,
	AMDGPU_OAM_4_5_6_7_3V3_VR_TEMP,
	AMDGPU_IBC_HSC_TEMP,
	AMDGPU_IBC_TEMP,
	AMDGPU_SYSTEM_MAX_TEMP_ENTRIES = 32,
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub amdgpu_node_temp {
	AMDGPU_RETIMER_X_TEMP,
	AMDGPU_OAM_X_IBC_TEMP,
	AMDGPU_OAM_X_IBC_2_TEMP,
	AMDGPU_OAM_X_VDD18_VR_TEMP,
	AMDGPU_OAM_X_04_HBM_B_VR_TEMP,
	AMDGPU_OAM_X_04_HBM_D_VR_TEMP,
	AMDGPU_NODE_MAX_TEMP_ENTRIES = 12,
};

#[repr(C)]\npub amdgpu_gpuboard_temp_metrics_v1_0 {
	common_header: metrics_table_header;
	label_version: u16;
	node_id: u16;
	accumulation_counter: u64;
	/* Encoded temperature in Celcius, 24:31 is sensor id 0:23 is temp value */
	node_temp: [u32; AMDGPU_NODE_MAX_TEMP_ENTRIES];
	vr_temp: [u32; AMDGPU_VR_MAX_TEMP_ENTRIES];
};

#[repr(C)]\npub amdgpu_baseboard_temp_metrics_v1_0 {
	common_header: metrics_table_header;
	label_version: u16;
	node_id: u16;
	accumulation_counter: u64;
	/* Encoded temperature in Celcius, 24:31 is sensor id 0:23 is temp value */
	system_temp: [u32; AMDGPU_SYSTEM_MAX_TEMP_ENTRIES];
};

#[repr(C)]\npub amdgpu_partition_metrics_v1_0 {
	common_header: metrics_table_header;
	/* Current clocks (Mhz) */
	current_gfxclk: [u16; MAX_XCC];
	current_socclk: [u16; MAX_CLKS];
	current_vclk0: [u16; MAX_CLKS];
	current_dclk0: [u16; MAX_CLKS];
	current_uclk: u16;
	padding: u16;

	/* Utilization Instantaneous (%) */
	gfx_busy_inst: [u32; MAX_XCC];
	jpeg_busy: [u16; NUM_JPEG_ENG_V1];
	vcn_busy: [u16; NUM_VCN];
	/* Utilization Accumulated (%) */
	gfx_busy_acc: [u64; MAX_XCC];
	/* Total App Clock Counter Accumulated */
	gfx_below_host_limit_ppt_acc: [u64; MAX_XCC];
	gfx_below_host_limit_thm_acc: [u64; MAX_XCC];
	gfx_low_utilization_acc: [u64; MAX_XCC];
	gfx_below_host_limit_total_acc: [u64; MAX_XCC];
};

#[repr(C)]\npub amdgpu_partition_metrics_v1_1 {
	common_header: metrics_table_header;
	attr_count: int;
	gpu_metrics_attr metrics_attrs: [u8; 0];
};

#[repr(C)]\n#[derive(Copy, Clone)]\npub amdgpu_xgmi_link_status {
	AMDGPU_XGMI_LINK_INACTIVE = 0,
	AMDGPU_XGMI_LINK_ACTIVE = 1,
	/* Status not available */
	AMDGPU_XGMI_LINK_NA = 2,
};

#[repr(C)]\npub amdgpu_gpuboard_temp_metrics_v1_1 {
	common_header: metrics_table_header;
	attr_count: int;
	gpu_metrics_attr metrics_attrs: [u8; 0];
};

#[repr(C)]\npub amdgpu_baseboard_temp_metrics_v1_1 {
	common_header: metrics_table_header;
	attr_count: int;
	gpu_metrics_attr metrics_attrs: [u8; 0];
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
