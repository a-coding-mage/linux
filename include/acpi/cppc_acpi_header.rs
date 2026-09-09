/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * CPPC (Collaborative Processor Performance Control) methods used
 * by CPUfreq drivers.
 *
 * (C) Copyright 2014, 2015 Linaro Ltd.
 * Author: Ashwin Chaugule <ashwin.chaugule@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

/* CPPCv2, CPPCv3 and CPPCv4 support */
pub const CPPC_V2_REV: u32 = 2;
pub const CPPC_V3_REV: u32 = 3;
pub const CPPC_V4_REV: u32 = 4;
pub const CPPC_V2_NUM_ENT: u32 = 21;
pub const CPPC_V3_NUM_ENT: u32 = 23;
pub const CPPC_V4_NUM_ENT: u32 = 25;

pub const PCC_CMD_COMPLETE_MASK: u32 = 1 << 0;
pub const PCC_ERROR_MASK: u32 = 1 << 2;

pub const MAX_CPC_REG_ENT: usize = 23;

/* CPPC specific PCC commands. */
pub const CMD_READ: u32 = 0;
pub const CMD_WRITE: u32 = 1;

pub const CPPC_AUTO_ACT_WINDOW_SIG_BIT_SIZE: u32 = 7;
pub const CPPC_AUTO_ACT_WINDOW_EXP_BIT_SIZE: u32 = 3;
pub const CPPC_AUTO_ACT_WINDOW_MAX_SIG: u32 = (1 << CPPC_AUTO_ACT_WINDOW_SIG_BIT_SIZE) - 1;
pub const CPPC_AUTO_ACT_WINDOW_MAX_EXP: u32 = (1 << CPPC_AUTO_ACT_WINDOW_EXP_BIT_SIZE) - 1;
/* CPPC_AUTO_ACT_WINDOW_MAX_SIG is 127, so 128 and 129 will decay to 127 when writing */
pub const CPPC_AUTO_ACT_WINDOW_SIG_CARRY_THRESH: u32 = 129;

pub const CPPC_EPP_PERFORMANCE_PREF: u32 = 0x00;
pub const CPPC_EPP_ENERGY_EFFICIENCY_PREF: u32 = 0xFF;

pub const CPPC_PERF_LIMITED_DESIRED_EXCURSION: u32 = 1 << 0;
pub const CPPC_PERF_LIMITED_MINIMUM_EXCURSION: u32 = 1 << 1;
pub const CPPC_PERF_LIMITED_MASK: u32 =
    CPPC_PERF_LIMITED_DESIRED_EXCURSION | CPPC_PERF_LIMITED_MINIMUM_EXCURSION;

/* Each register has the folowing format. */
#[repr(C, packed)]
pub struct cpc_reg {
    pub descriptor: u8,
    pub length: u16,
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_width: u8,
    pub address: u64,
}

/*
 * Each entry in the CPC table is either
 * of type ACPI_TYPE_BUFFER or
 * ACPI_TYPE_INTEGER.
 */
#[repr(C)]
pub union cpc_entry_union {
    pub reg_and_lock: cpc_reg_and_lock,
    pub int_value: u64,
}

#[repr(C)]
pub struct cpc_reg_and_lock {
    pub reg: cpc_reg,
    pub use_rmw_lock: bool,
}

#[repr(C)]
pub struct cpc_register_resource {
    pub type_: acpi_object_type,
    pub sys_mem_vaddr: *mut u64,
    pub cpc_entry: cpc_entry_union,
}

/* Container to hold the CPC details for each CPU */
#[repr(C)]
pub struct cpc_desc {
    pub num_entries: i32,
    pub version: i32,
    pub cpu_id: i32,
    pub write_cmd_status: i32,
    pub write_cmd_id: i32,
    /* Lock used for RMW operations in cpc_write() */
    pub rmw_lock: raw_spinlock_t,
    pub cpc_regs: [cpc_register_resource; MAX_CPC_REG_ENT],
    pub domain_info: acpi_psd_package,
    pub kobj: kobject,
}

/* These are indexes into the per-cpu cpc_regs[]. Order is important. */
#[repr(C)]
pub enum cppc_regs {
    HIGHEST_PERF,
    NOMINAL_PERF,
    LOW_NON_LINEAR_PERF,
    LOWEST_PERF,
    GUARANTEED_PERF,
    DESIRED_PERF,
    MIN_PERF,
    MAX_PERF,
    PERF_REDUC_TOLERANCE,
    TIME_WINDOW,
    CTR_WRAP_TIME,
    REFERENCE_CTR,
    DELIVERED_CTR,
    PERF_LIMITED,
    ENABLE,
    AUTO_SEL_ENABLE,
    AUTO_ACT_WINDOW,
    ENERGY_PERF,
    REFERENCE_PERF,
    LOWEST_FREQ,
    NOMINAL_FREQ,
    OSPM_NOMINAL_PERF,
    RESOURCE_PRIORITY,
}

/*
 * Categorization of registers as described
 * in the ACPI v.5.1 spec.
 * XXX: Only filling up ones which are used by governors
 * today.
 */
#[repr(C)]
pub struct cppc_perf_caps {
    pub guaranteed_perf: u32,
    pub highest_perf: u32,
    pub nominal_perf: u32,
    pub reference_perf: u32,
    pub lowest_perf: u32,
    pub lowest_nonlinear_perf: u32,
    pub lowest_freq: u32,
    pub nominal_freq: u32,
}

#[repr(C)]
pub struct cppc_perf_ctrls {
    pub max_perf: u32,
    pub min_perf: u32,
    pub desired_perf: u32,
    pub energy_perf: u32,
    pub auto_sel: bool,
}

#[repr(C)]
pub struct cppc_perf_fb_ctrs {
    pub reference: u64,
    pub delivered: u64,
    pub wraparound_time: u64,
}

/* Per CPU container for runtime CPPC management. */
#[repr(C)]
pub struct cppc_cpudata {
    pub perf_caps: cppc_perf_caps,
    pub perf_ctrls: cppc_perf_ctrls,
    pub perf_fb_ctrs: cppc_perf_fb_ctrs,
    pub shared_type: core::ffi::c_uint,
    pub shared_cpu_map: cpumask_var_t,
}

#[cfg(feature = "CONFIG_ACPI_CPPC_LIB")]
extern "C" {
    pub fn cppc_get_desired_perf(cpunum: i32, desired_perf: *mut u64) -> i32;
    pub fn cppc_get_nominal_perf(cpunum: i32, nominal_perf: *mut u64) -> i32;
    pub fn cppc_get_highest_perf(cpunum: i32, highest_perf: *mut u64) -> i32;
    pub fn cppc_get_perf_ctrs(cpu: i32, perf_fb_ctrs: *mut cppc_perf_fb_ctrs) -> i32;
    pub fn cppc_get_perf(cpu: i32, perf_ctrls: *mut cppc_perf_ctrls) -> i32;
    pub fn cppc_set_perf(cpu: i32, perf_ctrls: *mut cppc_perf_ctrls) -> i32;
    pub fn cppc_set_enable(cpu: i32, enable: bool) -> i32;
    pub fn cppc_get_perf_caps(cpu: i32, caps: *mut cppc_perf_caps) -> i32;
    pub fn cppc_perf_ctrs_in_pcc_cpu(cpu: core::ffi::c_uint) -> bool;
    pub fn cppc_perf_ctrs_in_pcc() -> bool;
    pub fn cppc_get_dmi_max_khz() -> u64;
    pub fn cppc_perf_to_khz(caps: *mut cppc_perf_caps, perf: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn cppc_khz_to_perf(caps: *mut cppc_perf_caps, freq: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn acpi_cpc_valid() -> bool;
    pub fn cppc_allow_fast_switch(cpus: *const cpumask) -> bool;
    pub fn acpi_get_psd_map(cpu: core::ffi::c_uint, cpu_data: *mut cppc_cpudata) -> i32;
    pub fn cppc_get_transition_latency(cpu: i32) -> i32;
    pub fn cpc_ffh_supported() -> bool;
    pub fn cpc_supported_by_cpu() -> bool;
    pub fn cpc_read_ffh(cpunum: i32, reg: *mut cpc_reg, val: *mut u64) -> i32;
    pub fn cpc_read_ffh_fb_ctrs(cpu: i32, reg1: *mut cpc_reg, val1: *mut u64, reg2: *mut cpc_reg, val2: *mut u64) -> i32;
    pub fn cpc_write_ffh(cpunum: i32, reg: *mut cpc_reg, val: u64) -> i32;
    pub fn cppc_get_epp_perf(cpunum: i32, epp_perf: *mut u64) -> i32;
    pub fn cppc_set_epp_perf(cpu: i32, perf_ctrls: *mut cppc_perf_ctrls, enable: bool) -> i32;
    pub fn cppc_set_epp(cpu: i32, epp_val: u64) -> i32;
    pub fn cppc_get_auto_act_window(cpu: i32, auto_act_window: *mut u64) -> i32;
    pub fn cppc_set_auto_act_window(cpu: i32, auto_act_window: u64) -> i32;
    pub fn cppc_get_auto_sel(cpu: i32, enable: *mut bool) -> i32;
    pub fn cppc_set_auto_sel(cpu: i32, enable: bool) -> i32;
    pub fn cppc_get_perf_limited(cpu: i32, perf_limited: *mut u64) -> i32;
    pub fn cppc_set_perf_limited(cpu: i32, bits_to_clear: u64) -> i32;
    pub fn amd_get_highest_perf(cpu: core::ffi::c_uint, highest_perf: *mut u32) -> i32;
    pub fn amd_get_boost_ratio_numerator(cpu: core::ffi::c_uint, numerator: *mut u64) -> i32;
    pub fn amd_detect_prefcore(detected: *mut bool) -> i32;
}

#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_get_desired_perf(_: i32, _: *mut u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_get_nominal_perf(_: i32, _: *mut u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_get_highest_perf(_: i32, _: *mut u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_get_perf_ctrs(_: i32, _: *mut cppc_perf_fb_ctrs) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_get_perf(_: i32, _: *mut cppc_perf_ctrls) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_set_perf(_: i32, _: *mut cppc_perf_ctrls) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_set_enable(_: i32, _: bool) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_get_perf_caps(_: i32, _: *mut cppc_perf_caps) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_perf_ctrs_in_pcc_cpu(_: core::ffi::c_uint) -> bool { false }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_perf_ctrs_in_pcc() -> bool { false }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn acpi_cpc_valid() -> bool { false }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_allow_fast_switch(_: *const cpumask) -> bool { false }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_get_transition_latency(_: i32) -> i32 { -ENODATA }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cpc_ffh_supported() -> bool { false }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cpc_read_ffh(_: i32, _: *mut cpc_reg, _: *mut u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cpc_read_ffh_fb_ctrs(_: i32, _: *mut cpc_reg, _: *mut u64, _: *mut cpc_reg, _: *mut u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cpc_write_ffh(_: i32, _: *mut cpc_reg, _: u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_set_epp_perf(_: i32, _: *mut cppc_perf_ctrls, _: bool) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_get_epp_perf(_: i32, _: *mut u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_set_epp(_: i32, _: u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_get_auto_act_window(_: i32, _: *mut u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_set_auto_act_window(_: i32, _: u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_get_auto_sel(_: i32, _: *mut bool) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_set_auto_sel(_: i32, _: bool) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_get_perf_limited(_: i32, _: *mut u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn cppc_set_perf_limited(_: i32, _: u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn amd_get_highest_perf(_: core::ffi::c_uint, _: *mut u32) -> i32 { -ENODEV }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn amd_get_boost_ratio_numerator(_: core::ffi::c_uint, _: *mut u64) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_ACPI_CPPC_LIB"))]
pub unsafe fn amd_detect_prefcore(_: *mut bool) -> i32 { -ENODEV }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
