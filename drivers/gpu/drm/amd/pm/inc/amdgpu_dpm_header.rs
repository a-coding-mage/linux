/* Translated from amdgpu_dpm.h. External kernel types are supplied by dependencies. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum gfx_change_state { sGpuChangeState_D0Entry = 1, sGpuChangeState_D3Entry }
#[repr(C)]
pub enum amdgpu_int_thermal_type { THERMAL_TYPE_NONE, THERMAL_TYPE_EXTERNAL, THERMAL_TYPE_EXTERNAL_GPIO, THERMAL_TYPE_RV6XX, THERMAL_TYPE_RV770, THERMAL_TYPE_ADT7473_WITH_INTERNAL, THERMAL_TYPE_EVERGREEN, THERMAL_TYPE_SUMO, THERMAL_TYPE_NI, THERMAL_TYPE_SI, THERMAL_TYPE_EMC2103_WITH_INTERNAL, THERMAL_TYPE_CI, THERMAL_TYPE_KV }
#[repr(C)]
pub enum amdgpu_runpm_mode { AMDGPU_RUNPM_NONE, AMDGPU_RUNPM_PX, AMDGPU_RUNPM_BOCO, AMDGPU_RUNPM_BACO, AMDGPU_RUNPM_BAMACO }
pub const BACO_SUPPORT: u32 = 1 << 0;
pub const MACO_SUPPORT: u32 = 1 << 1;

#[repr(C)] pub struct amdgpu_ps { pub caps:u32, pub class:u32, pub class2:u32, pub vclk:u32, pub dclk:u32, pub evclk:u32, pub ecclk:u32, pub vce_active:bool, pub vce_level: amd_vce_level, pub ps_priv:*mut core::ffi::c_void }
#[repr(C)] pub struct amdgpu_dpm_thermal { pub work: work_struct, pub min_temp:i32, pub max_temp:i32, pub max_edge_emergency_temp:i32, pub min_hotspot_temp:i32, pub max_hotspot_crit_temp:i32, pub max_hotspot_emergency_temp:i32, pub min_mem_temp:i32, pub max_mem_crit_temp:i32, pub max_mem_emergency_temp:i32, pub sw_ctf_threshold:i32, pub high_to_low:bool, pub irq: amdgpu_irq_src }
#[repr(C)] pub struct amdgpu_clock_and_voltage_limits { pub sclk:u32, pub mclk:u32, pub vddc:u16, pub vddci:u16 }
#[repr(C)] pub struct amdgpu_clock_array { pub count:u32, pub values:*mut u32 }
#[repr(C)] pub struct amdgpu_clock_voltage_dependency_entry { pub clk:u32, pub v:u16 }
#[repr(C)] pub struct amdgpu_clock_voltage_dependency_table { pub count:u32, pub entries:*mut amdgpu_clock_voltage_dependency_entry }
#[repr(C)] pub union amdgpu_cac_leakage_entry { pub vddc:u16, pub leakage:u32, pub vddc1:u16, pub vddc2:u16, pub vddc3:u16 }
#[repr(C)] pub struct amdgpu_cac_leakage_table { pub count:u32, pub entries:*mut amdgpu_cac_leakage_entry }
#[repr(C)] pub struct amdgpu_phase_shedding_limits_entry { pub voltage:u16, pub sclk:u32, pub mclk:u32 }
#[repr(C)] pub struct amdgpu_phase_shedding_limits_table { pub count:u32, pub entries:*mut amdgpu_phase_shedding_limits_entry }
#[repr(C)] pub struct amdgpu_uvd_clock_voltage_dependency_entry { pub vclk:u32, pub dclk:u32, pub v:u16 }
#[repr(C)] pub struct amdgpu_uvd_clock_voltage_dependency_table { pub count:u8, pub entries:*mut amdgpu_uvd_clock_voltage_dependency_entry }
#[repr(C)] pub struct amdgpu_vce_clock_voltage_dependency_entry { pub ecclk:u32, pub evclk:u32, pub v:u16 }
#[repr(C)] pub struct amdgpu_vce_clock_voltage_dependency_table { pub count:u8, pub entries:*mut amdgpu_vce_clock_voltage_dependency_entry }
#[repr(C)] pub struct amdgpu_ppm_table { pub ppm_design:u8, pub cpu_core_number:u16, pub platform_tdp:u32, pub small_ac_platform_tdp:u32, pub platform_tdc:u32, pub small_ac_platform_tdc:u32, pub apu_tdp:u32, pub dgpu_tdp:u32, pub dgpu_ulv_power:u32, pub tj_max:u32 }
#[repr(C)] pub struct amdgpu_cac_tdp_table { pub tdp:u16, pub configurable_tdp:u16, pub tdc:u16, pub battery_power_limit:u16, pub small_power_limit:u16, pub low_cac_leakage:u16, pub high_cac_leakage:u16, pub maximum_power_delivery_limit:u16 }
#[repr(C)] pub struct amdgpu_dpm_dynamic_state { pub vddc_dependency_on_sclk:amdgpu_clock_voltage_dependency_table, pub vddci_dependency_on_mclk:amdgpu_clock_voltage_dependency_table, pub vddc_dependency_on_mclk:amdgpu_clock_voltage_dependency_table, pub mvdd_dependency_on_mclk:amdgpu_clock_voltage_dependency_table, pub vddc_dependency_on_dispclk:amdgpu_clock_voltage_dependency_table, pub uvd_clock_voltage_dependency_table:amdgpu_uvd_clock_voltage_dependency_table, pub vce_clock_voltage_dependency_table:amdgpu_vce_clock_voltage_dependency_table, pub samu_clock_voltage_dependency_table:amdgpu_clock_voltage_dependency_table, pub acp_clock_voltage_dependency_table:amdgpu_clock_voltage_dependency_table, pub vddgfx_dependency_on_sclk:amdgpu_clock_voltage_dependency_table, pub valid_sclk_values:amdgpu_clock_array, pub valid_mclk_values:amdgpu_clock_array, pub max_clock_voltage_on_dc:amdgpu_clock_and_voltage_limits, pub max_clock_voltage_on_ac:amdgpu_clock_and_voltage_limits, pub mclk_sclk_ratio:u32, pub sclk_mclk_delta:u32, pub vddc_vddci_delta:u16, pub min_vddc_for_pcie_gen2:u16, pub cac_leakage_table:amdgpu_cac_leakage_table, pub phase_shedding_limits_table:amdgpu_phase_shedding_limits_table, pub ppm_table:*mut amdgpu_ppm_table, pub cac_tdp_table:*mut amdgpu_cac_tdp_table }
#[repr(C)] pub struct amdgpu_dpm_fan { pub t_min:u16, pub t_med:u16, pub t_high:u16, pub pwm_min:u16, pub pwm_med:u16, pub pwm_high:u16, pub t_hyst:u8, pub cycle_delay:u32, pub t_max:u16, pub control_mode:u8, pub default_max_fan_pwm:u16, pub default_fan_output_sensitivity:u16, pub fan_output_sensitivity:u16, pub ucode_fan_control:bool }
#[repr(C)] pub struct amdgpu_dpm { pub ps:*mut amdgpu_ps, pub num_ps:i32, pub current_ps:*mut amdgpu_ps, pub requested_ps:*mut amdgpu_ps, pub boot_ps:*mut amdgpu_ps, pub uvd_ps:*mut amdgpu_ps, pub num_of_vce_states:u32, pub vce_states:[amd_vce_state; AMD_MAX_VCE_LEVELS], pub vce_level:amd_vce_level, pub state:amd_pm_state_type, pub user_state:amd_pm_state_type, pub last_state:amd_pm_state_type, pub last_user_state:amd_pm_state_type, pub platform_caps:u32, pub voltage_response_time:u32, pub backbias_response_time:u32, pub priv_:*mut core::ffi::c_void, pub dyn_state:amdgpu_dpm_dynamic_state, pub fan:amdgpu_dpm_fan, pub tdp_limit:u32, pub near_tdp_limit:u32, pub near_tdp_limit_adjusted:u32, pub sq_ramping_threshold:u32, pub cac_leakage:u32, pub tdp_od_limit:u16, pub tdp_adjustment:u32, pub load_line_slope:u16, pub power_control:bool, pub thermal_active:bool, pub uvd_active:bool, pub vce_active:bool, pub thermal:amdgpu_dpm_thermal, pub forced_level:amd_dpm_forced_level }
#[repr(C)] pub enum ip_power_state { POWER_STATE_UNKNOWN, POWER_STATE_ON, POWER_STATE_OFF }
pub const SMU_DEBUG_HALT_ON_ERROR:u32=1<<0; pub const SMU_DEBUG_POOL_USE_VRAM:u32=1<<1; pub const MAX_SMU_I2C_BUSES:usize=2;
#[repr(C)] pub struct amdgpu_smu_i2c_bus { pub adapter:i2c_adapter, pub adev:*mut amdgpu_device, pub port:i32, pub mutex:mutex }
#[repr(C)] pub struct config_table_setting { pub gfxclk_average_tau:u16, pub socclk_average_tau:u16, pub uclk_average_tau:u16, pub gfx_activity_average_tau:u16, pub mem_activity_average_tau:u16, pub socket_power_average_tau:u16, pub apu_socket_power_average_tau:u16, pub fclk_average_tau:u16 }

/* OD_OPS_SUPPORT_* and the remaining declarations are preserved as external interfaces. */
pub const OD_OPS_SUPPORT_FAN_CURVE_RETRIEVE:u32=1<<0; pub const OD_OPS_SUPPORT_FAN_CURVE_SET:u32=1<<1;
extern "C" { pub fn amdgpu_dpm_read_sensor(adev:*mut amdgpu_device, sensor:amd_pp_sensors, data:*mut core::ffi::c_void, size:*mut u32)->i32; pub fn amdgpu_dpm_get_apu_thermal_limit(adev:*mut amdgpu_device, limit:*mut u32)->i32; pub fn amdgpu_dpm_set_apu_thermal_limit(adev:*mut amdgpu_device, limit:u32)->i32; pub fn amdgpu_dpm_get_sclk(adev:*mut amdgpu_device, low:bool)->i32; pub fn amdgpu_dpm_get_mclk(adev:*mut amdgpu_device, low:bool)->i32; pub fn amdgpu_dpm_baco_reset(adev:*mut amdgpu_device)->i32; pub fn amdgpu_dpm_mode2_reset(adev:*mut amdgpu_device)->i32; pub fn amdgpu_dpm_link_reset(adev:*mut amdgpu_device)->i32; pub fn amdgpu_dpm_enable_gfx_features(adev:*mut amdgpu_device)->i32; pub fn amdgpu_dpm_is_baco_supported(adev:*mut amdgpu_device)->i32; pub fn amdgpu_dpm_baco_exit(adev:*mut amdgpu_device)->i32; pub fn amdgpu_dpm_baco_enter(adev:*mut amdgpu_device)->i32; pub fn amdgpu_dpm_compute_clocks(adev:*mut amdgpu_device); pub fn amdgpu_dpm_enable_uvd(adev:*mut amdgpu_device, enable:bool); pub fn amdgpu_dpm_enable_vce(adev:*mut amdgpu_device, enable:bool); pub fn amdgpu_dpm_enable_jpeg(adev:*mut amdgpu_device, enable:bool); pub fn amdgpu_dpm_enable_vpe(adev:*mut amdgpu_device, enable:bool); pub fn amdgpu_pm_acpi_event_handler(adev:*mut amdgpu_device); }

/* Remaining header declarations retain their C ABI names and external linkage. */
extern "C" {
    pub fn amdgpu_dpm_set_xgmi_pstate(adev:*mut amdgpu_device, pstate:u32)->i32;
    pub fn amdgpu_dpm_pause_power_profile(adev:*mut amdgpu_device, pause:bool)->i32;
    pub fn amdgpu_dpm_set_powergating_by_smu(adev:*mut amdgpu_device, block_type:u32, gate:bool, inst:i32)->i32;
    pub fn amdgpu_dpm_set_clockgating_by_smu(adev:*mut amdgpu_device, msg_id:u32)->i32;
    pub fn amdgpu_dpm_smu_i2c_bus_access(adev:*mut amdgpu_device, acquire:bool)->i32;
    pub fn amdgpu_dpm_enable_vcn(adev:*mut amdgpu_device, enable:bool, inst:i32);
    pub fn amdgpu_pm_load_smu_firmware(adev:*mut amdgpu_device, smu_version:*mut u32)->i32;
    pub fn amdgpu_dpm_get_fan_control_mode(adev:*mut amdgpu_device, fan_mode:*mut u32)->i32;
    pub fn amdgpu_dpm_set_fan_speed_pwm(adev:*mut amdgpu_device, speed:u32)->i32;
    pub fn amdgpu_dpm_get_fan_speed_pwm(adev:*mut amdgpu_device, speed:*mut u32)->i32;
    pub fn amdgpu_dpm_get_fan_speed_rpm(adev:*mut amdgpu_device, speed:*mut u32)->i32;
    pub fn amdgpu_dpm_set_fan_speed_rpm(adev:*mut amdgpu_device, speed:u32)->i32;
    pub fn amdgpu_dpm_set_fan_control_mode(adev:*mut amdgpu_device, mode:u32)->i32;
    pub fn amdgpu_dpm_is_overdrive_supported(adev:*mut amdgpu_device)->i32;
    pub fn amdgpu_dpm_is_overdrive_enabled(adev:*mut amdgpu_device)->i32;
    pub fn amdgpu_dpm_reset_sdma(adev:*mut amdgpu_device, inst_mask:u32)->i32;
    pub fn amdgpu_dpm_reset_vcn(adev:*mut amdgpu_device, inst_mask:u32)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
