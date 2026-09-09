/* Translated from si_dpm.h. */

// C dependencies: amdgpu_atombios.h and sislands_smc.h

pub const MC_CG_CONFIG: u32 = 0x96f;
pub const MC_ARB_CG: u32 = 0x9fa;
#[inline]
pub const fn CG_ARB_REQ(x: u32) -> u32 { x << 0 }
pub const CG_ARB_REQ_MASK: u32 = 0xff << 0;
pub const MC_ARB_DRAM_TIMING_1: u32 = 0x9fc;
pub const MC_ARB_DRAM_TIMING_2: u32 = 0x9fd;
pub const MC_ARB_DRAM_TIMING_3: u32 = 0x9fe;
pub const MC_ARB_DRAM_TIMING2_1: u32 = 0x9ff;
pub const MC_ARB_DRAM_TIMING2_2: u32 = 0xa00;
pub const MC_ARB_DRAM_TIMING2_3: u32 = 0xa01;
pub const NISLANDS_MAX_SMC_PERFORMANCE_LEVELS_PER_SWSTATE: usize = 16;
pub const RV770_ASI_DFLT: u32 = 1000;
pub const CYPRESS_HASI_DFLT: u32 = 400000;
pub const PCIE_PERF_REQ_PECI_GEN1: u32 = 2;
pub const PCIE_PERF_REQ_PECI_GEN2: u32 = 3;
pub const PCIE_PERF_REQ_PECI_GEN3: u32 = 4;
pub const RV770_DEFAULT_VCLK_FREQ: u32 = 53300;
pub const RV770_DEFAULT_DCLK_FREQ: u32 = 40000;
pub const SMC_STROBE_RATIO: u32 = 0x0F;
pub const SMC_STROBE_ENABLE: u32 = 0x10;
pub const SMC_MC_EDC_RD_FLAG: u32 = 0x01;
pub const SMC_MC_EDC_WR_FLAG: u32 = 0x02;
pub const SMC_MC_RTT_ENABLE: u32 = 0x04;
pub const SMC_MC_STUTTER_EN: u32 = 0x08;
pub const SISLANDS_MCREGISTERTABLE_INITIAL_SLOT: u32 = 0;
pub const SISLANDS_MCREGISTERTABLE_ACPI_SLOT: u32 = 1;
pub const SISLANDS_MCREGISTERTABLE_ULV_SLOT: u32 = 2;
pub const SISLANDS_MCREGISTERTABLE_FIRST_DRIVERSTATE_SLOT: u32 = 3;
pub const SISLANDS_LEAKAGE_INDEX0: u32 = 0xff01;
pub const SISLANDS_MAX_LEAKAGE_COUNT: usize = 4;
pub const SISLANDS_MAX_HARDWARE_POWERLEVELS: u32 = 5;
pub const SISLANDS_INITIAL_STATE_ARB_INDEX: u32 = 0;
pub const SISLANDS_ACPI_STATE_ARB_INDEX: u32 = 1;
pub const SISLANDS_ULV_STATE_ARB_INDEX: u32 = 2;
pub const SISLANDS_DRIVER_STATE_ARB_INDEX: u32 = 3;
pub const SISLANDS_DPM2_MAX_PULSE_SKIP: u32 = 256;
pub const SISLANDS_DPM2_NEAR_TDP_DEC: u32 = 10;
pub const SISLANDS_DPM2_ABOVE_SAFE_INC: u32 = 5;
pub const SISLANDS_DPM2_BELOW_SAFE_INC: u32 = 20;
pub const SISLANDS_DPM2_TDP_SAFE_LIMIT_PERCENT: u32 = 80;
pub const SISLANDS_DPM2_MAXPS_PERCENT_H: u32 = 99;
pub const SISLANDS_DPM2_MAXPS_PERCENT_M: u32 = 99;
pub const SISLANDS_DPM2_SQ_RAMP_MAX_POWER: u32 = 0x3FFF;
pub const SISLANDS_DPM2_SQ_RAMP_MIN_POWER: u32 = 0x12;
pub const SISLANDS_DPM2_SQ_RAMP_MAX_POWER_DELTA: u32 = 0x15;
pub const SISLANDS_DPM2_SQ_RAMP_STI_SIZE: u32 = 0x1E;
pub const SISLANDS_DPM2_SQ_RAMP_LTI_RATIO: u32 = 0xF;
pub const SISLANDS_DPM2_PWREFFICIENCYRATIO_MARGIN: u32 = 10;
pub const SISLANDS_VRC_DFLT: u32 = 0xC000B3;
pub const SISLANDS_ULVVOLTAGECHANGEDELAY_DFLT: u32 = 1687;
pub const SISLANDS_CGULVPARAMETER_DFLT: u32 = 0x00040035;
pub const SISLANDS_CGULVCONTROL_DFLT: u32 = 0x1f007550;
pub const SI_ASI_DFLT: u32 = 10000;
pub const SI_BSP_DFLT: u32 = 0x41EB;
pub const SI_BSU_DFLT: u32 = 0x2;
pub const SI_AH_DFLT: u32 = 5;
pub const SI_RLP_DFLT: u32 = 25;
pub const SI_RMP_DFLT: u32 = 65;
pub const SI_LHP_DFLT: u32 = 40;
pub const SI_LMP_DFLT: u32 = 15;
pub const SI_TD_DFLT: u32 = 0;
pub const SI_UTC_DFLT_00: u32 = 0x24;
pub const SI_UTC_DFLT_01: u32 = 0x22;
pub const SI_UTC_DFLT_02: u32 = 0x22;
pub const SI_UTC_DFLT_03: u32 = 0x22;
pub const SI_UTC_DFLT_04: u32 = 0x22;
pub const SI_UTC_DFLT_05: u32 = 0x22;
pub const SI_UTC_DFLT_06: u32 = 0x22;
pub const SI_UTC_DFLT_07: u32 = 0x22;
pub const SI_UTC_DFLT_08: u32 = 0x22;
pub const SI_UTC_DFLT_09: u32 = 0x22;
pub const SI_UTC_DFLT_10: u32 = 0x22;
pub const SI_UTC_DFLT_11: u32 = 0x22;
pub const SI_UTC_DFLT_12: u32 = 0x22;
pub const SI_UTC_DFLT_13: u32 = 0x22;
pub const SI_UTC_DFLT_14: u32 = 0x22;
pub const SI_DTC_DFLT_00: u32 = 0x24;
pub const SI_DTC_DFLT_01: u32 = 0x22;
pub const SI_DTC_DFLT_02: u32 = 0x22;
pub const SI_DTC_DFLT_03: u32 = 0x22;
pub const SI_DTC_DFLT_04: u32 = 0x22;
pub const SI_DTC_DFLT_05: u32 = 0x22;
pub const SI_DTC_DFLT_06: u32 = 0x22;
pub const SI_DTC_DFLT_07: u32 = 0x22;
pub const SI_DTC_DFLT_08: u32 = 0x22;
pub const SI_DTC_DFLT_09: u32 = 0x22;
pub const SI_DTC_DFLT_10: u32 = 0x22;
pub const SI_DTC_DFLT_11: u32 = 0x22;
pub const SI_DTC_DFLT_12: u32 = 0x22;
pub const SI_DTC_DFLT_13: u32 = 0x22;
pub const SI_DTC_DFLT_14: u32 = 0x22;
pub const SI_VRC_DFLT: u32 = 0x0000C003;
pub const SI_VOLTAGERESPONSETIME_DFLT: u32 = 1000;
pub const SI_BACKBIASRESPONSETIME_DFLT: u32 = 1000;
pub const SI_VRU_DFLT: u32 = 0x3;
pub const SI_SPLLSTEPTIME_DFLT: u32 = 0x1000;
pub const SI_SPLLSTEPUNIT_DFLT: u32 = 0x3;
pub const SI_TPU_DFLT: u32 = 0;
pub const SI_TPC_DFLT: u32 = 0x200;
pub const SI_SSTU_DFLT: u32 = 0;
pub const SI_SST_DFLT: u32 = 0x00C8;
pub const SI_GICST_DFLT: u32 = 0x200;
pub const SI_FCT_DFLT: u32 = 0x0400;
pub const SI_FCTU_DFLT: u32 = 0;
pub const SI_CTXCGTT3DRPHC_DFLT: u32 = 0x20;
pub const SI_CTXCGTT3DRSDC_DFLT: u32 = 0x40;
pub const SI_VDDC3DOORPHC_DFLT: u32 = 0x100;
pub const SI_VDDC3DOORSDC_DFLT: u32 = 0x7;
pub const SI_VDDC3DOORSU_DFLT: u32 = 0;
pub const SI_MPLLLOCKTIME_DFLT: u32 = 100;
pub const SI_MPLLRESETTIME_DFLT: u32 = 150;
pub const SI_VCOSTEPPCT_DFLT: u32 = 20;
pub const SI_ENDINGVCOSTEPPCT_DFLT: u32 = 5;
pub const SI_REFERENCEDIVIDER_DFLT: u32 = 4;
pub const SI_PM_NUMBER_OF_TC: usize = 15;
pub const SI_PM_NUMBER_OF_SCLKS: usize = 20;
pub const SI_PM_NUMBER_OF_MCLKS: usize = 4;
pub const SI_PM_NUMBER_OF_VOLTAGE_LEVELS: usize = 4;
pub const SI_PM_NUMBER_OF_ACTIVITY_LEVELS: usize = 3;
pub const SI_TEMP_RANGE_MIN: u32 = 90 * 1000;
pub const SI_TEMP_RANGE_MAX: u32 = 120 * 1000;
pub const FDO_PWM_MODE_STATIC: u32 = 1;
pub const FDO_PWM_MODE_STATIC_RPM: u32 = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ni_dc_cac_level { NISLANDS_DCCAC_LEVEL_0 = 0, NISLANDS_DCCAC_LEVEL_1, NISLANDS_DCCAC_LEVEL_2, NISLANDS_DCCAC_LEVEL_3, NISLANDS_DCCAC_LEVEL_4, NISLANDS_DCCAC_LEVEL_5, NISLANDS_DCCAC_LEVEL_6, NISLANDS_DCCAC_LEVEL_7, NISLANDS_DCCAC_MAX_LEVELS }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum si_cac_config_reg_type { SISLANDS_CACCONFIG_MMR = 0, SISLANDS_CACCONFIG_CGIND, SISLANDS_CACCONFIG_MAX }
extern "C" { pub static si_smu_ip_block: amdgpu_ip_block_version; }

#[repr(C)] pub struct ni_leakage_coeffients { pub at: u32, pub bt: u32, pub av: u32, pub bv: u32, pub t_slope: i32, pub t_intercept: i32, pub t_ref: u32 }
#[repr(C)] pub struct SMC_NIslands_MCRegisterAddress { pub s0: u16, pub s1: u16 }
pub type SMC_NIslands_MCRegisterAddress_t = SMC_NIslands_MCRegisterAddress;
#[repr(C)] pub struct rv7xx_power_info { pub voltage_control: bool, pub mvdd_control: bool, pub sclk_ss: bool, pub mclk_ss: bool, pub dynamic_ss: bool, pub thermal_protection: bool, pub mvdd_split_frequency: u32, pub max_vddc: u16, pub max_vddc_in_table: u16, pub min_vddc_in_table: u16, pub acpi_vddc: u16, pub ref_div: u32, pub active_auto_throttle_sources: u32, pub mclk_stutter_mode_threshold: u32, pub mclk_strobe_mode_threshold: u32, pub mclk_edc_enable_threshold: u32, pub bsp: u32, pub bsu: u32, pub pbsp: u32, pub pbsu: u32, pub dsp: u32, pub psp: u32, pub asi: u32, pub pasi: u32, pub vrc: u32 }
#[repr(C)] pub enum si_pcie_gen { SI_PCIE_GEN1 = 0, SI_PCIE_GEN2 = 1, SI_PCIE_GEN3 = 2, SI_PCIE_GEN_INVALID = 0xffff }
#[repr(C)] pub struct rv7xx_pl { pub sclk: u32, pub mclk: u32, pub vddc: u16, pub vddci: u16, pub flags: u32, pub pcie_gen: si_pcie_gen }
#[repr(C)] pub struct si_ps { pub performance_level_count: u16, pub dc_compatible: bool, pub performance_levels: [rv7xx_pl; NISLANDS_MAX_SMC_PERFORMANCE_LEVELS_PER_SWSTATE] }
#[repr(C)] pub struct evergreen_power_info { pub rv7xx: rv7xx_power_info, pub vddci_control: bool, pub dynamic_ac_timing: bool, pub abm: bool, pub mcls: bool, pub pcie_performance_request: bool, pub sclk_deep_sleep: bool, pub smu_uvd_hs: bool, pub uvd_enabled: bool, pub acpi_vddci: u16, pub mclk_edc_wr_enable_threshold: u32, pub vddc_voltage_table: atom_voltage_table, pub vddci_voltage_table: atom_voltage_table, pub current_rps: amdgpu_ps, pub requested_rps: amdgpu_ps }
#[repr(C)] pub struct ni_power_info { pub eg: evergreen_power_info, pub mclk_rtt_mode_threshold: u32, pub support_cac_long_term_average: bool, pub cac_enabled: bool, pub cac_configuration_required: bool, pub driver_calculate_cac_leakage: bool, pub enable_power_containment: bool, pub enable_cac: bool, pub enable_sq_ramping: bool, pub current_ps: si_ps, pub requested_ps: si_ps }
#[repr(C)] pub struct si_cac_config_reg { pub offset: u32, pub mask: u32, pub shift: u32, pub value: u32, pub type_: si_cac_config_reg_type }
#[repr(C)] pub struct si_powertune_data { pub cac_window: u32, pub l2_lta_window_size_default: u32, pub lts_truncate_default: u8, pub shift_n_default: u8, pub operating_temp: u8, pub leakage_coefficients: ni_leakage_coeffients, pub fixed_kt: u32, pub lkge_lut_v0_percent: u32, pub dc_cac: [u8; 9], pub enable_powertune_by_default: bool }
#[repr(C)] pub struct si_dyn_powertune_data { pub cac_leakage: u32, pub leakage_minimum_temperature: i32, pub wintime: u32, pub l2_lta_window_size: u32, pub lts_truncate: u8, pub shift_n: u8, pub dc_pwr_value: u8, pub disable_uvd_powertune: bool }
#[repr(C)] pub struct si_dte_data { pub tau: [u32; SMC_SISLANDS_DTE_MAX_FILTER_STAGES], pub r: [u32; SMC_SISLANDS_DTE_MAX_FILTER_STAGES], pub k: u32, pub t0: u32, pub max_t: u32, pub window_size: u8, pub temp_select: u8, pub dte_mode: u8, pub tdep_count: u8, pub t_limits: [u8; SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE], pub tdep_tau: [u32; SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE], pub tdep_r: [u32; SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE], pub t_threshold: u32, pub enable_dte_by_default: bool }
#[repr(C)] pub struct si_clock_registers { pub cg_spll_func_cntl: u32, pub cg_spll_func_cntl_2: u32, pub cg_spll_func_cntl_3: u32, pub cg_spll_func_cntl_4: u32, pub cg_spll_spread_spectrum: u32, pub cg_spll_spread_spectrum_2: u32, pub dll_cntl: u32, pub mclk_pwrmgt_cntl: u32, pub mpll_ad_func_cntl: u32, pub mpll_dq_func_cntl: u32, pub mpll_func_cntl: u32, pub mpll_func_cntl_1: u32, pub mpll_func_cntl_2: u32, pub mpll_ss1: u32, pub mpll_ss2: u32 }
#[repr(C)] pub struct si_mc_reg_entry { pub mclk_max: u32, pub mc_data: [u32; SMC_SISLANDS_MC_REGISTER_ARRAY_SIZE] }
#[repr(C)] pub struct si_mc_reg_table { pub last: u8, pub num_entries: u8, pub valid_flag: u16, pub mc_reg_table_entry: [si_mc_reg_entry; MAX_AC_TIMING_ENTRIES], pub mc_reg_address: [SMC_NIslands_MCRegisterAddress; SMC_SISLANDS_MC_REGISTER_ARRAY_SIZE] }
#[repr(C)] pub struct si_leakage_voltage_entry { pub voltage: u16, pub leakage_index: u16 }
#[repr(C)] pub struct si_leakage_voltage { pub count: u16, pub entries: [si_leakage_voltage_entry; SISLANDS_MAX_LEAKAGE_COUNT] }
#[repr(C)] pub struct si_ulv_param { pub supported: bool, pub cg_ulv_control: u32, pub cg_ulv_parameter: u32, pub volt_change_delay: u32, pub pl: rv7xx_pl, pub one_pcie_lane_in_ulv: bool }
#[repr(C)] pub struct si_power_info { pub ni: ni_power_info, pub clock_registers: si_clock_registers, pub mc_reg_table: si_mc_reg_table, pub mvdd_voltage_table: atom_voltage_table, pub vddc_phase_shed_table: atom_voltage_table, pub leakage_voltage: si_leakage_voltage, pub mvdd_bootup_value: u16, pub ulv: si_ulv_param, pub max_cu: u32, pub force_pcie_gen: si_pcie_gen, pub boot_pcie_gen: si_pcie_gen, pub acpi_pcie_gen: si_pcie_gen, pub sys_pcie_mask: u32, pub enable_dte: bool, pub enable_ppm: bool, pub vddc_phase_shed_control: bool, pub pspp_notify_required: bool, pub sclk_deep_sleep_above_low: bool, pub voltage_control_svi2: bool, pub vddci_control_svi2: bool, pub sram_end: u32, pub state_table_start: u32, pub soft_regs_start: u32, pub mc_reg_table_start: u32, pub arb_table_start: u32, pub cac_table_start: u32, pub dte_table_start: u32, pub spll_table_start: u32, pub papm_cfg_table_start: u32, pub fan_table_start: u32, pub cac_weights: *const si_cac_config_reg, pub lcac_config: *const si_cac_config_reg, pub cac_override: *const si_cac_config_reg, pub powertune_data: *const si_powertune_data, pub dyn_powertune_data: si_dyn_powertune_data, pub dte_data: si_dte_data, pub smc_mc_reg_table: SMC_SIslands_MCRegisters, pub smc_statetable: SISLANDS_SMC_STATETABLE, pub papm_parm: PP_SIslands_PAPMParameters, pub svd_gpio_id: u8, pub svc_gpio_id: u8, pub fan_ctrl_is_in_default_mode: bool, pub t_min: u32, pub fan_ctrl_default_mode: u32, pub fan_is_controlled_by_smc: bool }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
