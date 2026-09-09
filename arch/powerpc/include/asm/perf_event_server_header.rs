/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Performance event support - PowerPC classic/server specific definitions.
 *
 * Copyright 2008-2009 Paul Mackerras, IBM Corporation.
 */

// C dependencies supplied by the surrounding kernel translation.

/* Update perf_event_print_debug() if this changes */
pub const MAX_HWEVENTS: ::core::ffi::c_int = 8;
pub const MAX_EVENT_ALTERNATIVES: ::core::ffi::c_int = 8;
pub const MAX_LIMITED_HWCOUNTERS: ::core::ffi::c_int = 2;

pub struct perf_event;

#[repr(C)]
pub struct mmcr_regs {
    pub mmcr0: ::core::ffi::c_ulong,
    pub mmcr1: ::core::ffi::c_ulong,
    pub mmcr2: ::core::ffi::c_ulong,
    pub mmcra: ::core::ffi::c_ulong,
    pub mmcr3: ::core::ffi::c_ulong,
}

/*
 * This struct provides the constants and functions needed to
 * describe the PMU on a particular POWER-family CPU.
 */
#[repr(C)]
pub struct power_pmu {
    pub name: *const ::core::ffi::c_char,
    pub n_counter: ::core::ffi::c_int,
    pub max_alternatives: ::core::ffi::c_int,
    pub add_fields: ::core::ffi::c_ulong,
    pub test_adder: ::core::ffi::c_ulong,
    pub compute_mmcr: Option<unsafe extern "C" fn(
        events: *mut u64, n_ev: ::core::ffi::c_int,
        hwc: *mut u32, mmcr: *mut mmcr_regs,
        pevents: *mut *mut perf_event, flags: u32,
    ) -> ::core::ffi::c_int>,
    pub get_constraint: Option<unsafe extern "C" fn(
        event_id: u64, mskp: *mut ::core::ffi::c_ulong,
        valp: *mut ::core::ffi::c_ulong, event_config1: u64,
    ) -> ::core::ffi::c_int>,
    pub get_alternatives: Option<unsafe extern "C" fn(
        event_id: u64, flags: u32, alt: *mut u64,
    ) -> ::core::ffi::c_int>,
    pub get_mem_data_src: Option<unsafe extern "C" fn(
        dsrc: *mut union_perf_mem_data_src, flags: u32, regs: *mut pt_regs,
    )>,
    pub get_mem_weight: Option<unsafe extern "C" fn(weight: *mut u64, ty: u64)>,
    pub group_constraint_mask: ::core::ffi::c_ulong,
    pub group_constraint_val: ::core::ffi::c_ulong,
    pub bhrb_filter_map: Option<unsafe extern "C" fn(branch_sample_type: u64) -> u64>,
    pub config_bhrb: Option<unsafe extern "C" fn(pmu_bhrb_filter: u64)>,
    pub disable_pmc: Option<unsafe extern "C" fn(pmc: u32, mmcr: *mut mmcr_regs)>,
    pub limited_pmc_event: Option<unsafe extern "C" fn(event_id: u64) -> ::core::ffi::c_int>,
    pub flags: u32,
    pub attr_groups: *const *const attribute_group,
    pub n_generic: ::core::ffi::c_int,
    pub generic_events: *mut ::core::ffi::c_int,
    pub cache_events: *mut [[[u64; PERF_COUNT_HW_CACHE_RESULT_MAX as usize]; PERF_COUNT_HW_CACHE_OP_MAX as usize]; PERF_COUNT_HW_CACHE_MAX as usize],
    pub n_blacklist_ev: ::core::ffi::c_int,
    pub blacklist_ev: *mut ::core::ffi::c_int,
    /* BHRB entries in the PMU */
    pub bhrb_nr: ::core::ffi::c_int,
    /* set this flag with `PERF_PMU_CAP_EXTENDED_REGS` if the pmu supports extended perf regs capability */
    pub capabilities: ::core::ffi::c_int,
    /* Function to check event code for values which are reserved. */
    pub check_attr_config: Option<unsafe extern "C" fn(ev: *mut perf_event) -> ::core::ffi::c_int>,
}

pub const PPMU_LIMITED_PMC5_6: u32 = 0x00000001;
pub const PPMU_ALT_SIPR: u32 = 0x00000002;
pub const PPMU_NO_SIPR: u32 = 0x00000004;
pub const PPMU_NO_CONT_SAMPLING: u32 = 0x00000008;
pub const PPMU_SIAR_VALID: u32 = 0x00000010;
pub const PPMU_HAS_SSLOT: u32 = 0x00000020;
pub const PPMU_HAS_SIER: u32 = 0x00000040;
pub const PPMU_ARCH_207S: u32 = 0x00000080;
pub const PPMU_NO_SIAR: u32 = 0x00000100;
pub const PPMU_ARCH_31: u32 = 0x00000200;
pub const PPMU_P10_DD1: u32 = 0x00000400;
pub const PPMU_P10: u32 = 0x00000800;
pub const PPMU_HAS_ATTR_CONFIG1: u32 = 0x00001000;

pub const PPMU_LIMITED_PMC_OK: u32 = 1;
pub const PPMU_LIMITED_PMC_REQD: u32 = 2;
pub const PPMU_ONLY_COUNT_RUN: u32 = 4;

unsafe extern "C" {
    pub fn register_power_pmu(pmu: *mut power_pmu) -> ::core::ffi::c_int;
    pub fn perf_arch_misc_flags(regs: *mut pt_regs) -> ::core::ffi::c_ulong;
    pub fn perf_arch_instruction_pointer(regs: *mut pt_regs) -> ::core::ffi::c_ulong;
    pub fn read_bhrb(n: ::core::ffi::c_int) -> ::core::ffi::c_ulong;
    pub fn power_events_sysfs_show(dev: *mut device, attr: *mut device_attribute, page: *mut ::core::ffi::c_char) -> isize;
}

pub struct pt_regs;
pub struct union_perf_mem_data_src;
pub struct attribute_group;
pub struct device;
pub struct device_attribute;

/* EVENT_VAR(), EVENT_PTR(), and the EVENT_ATTR family are represented by
 * declarative macros; PMU_EVENT_ATTR remains supplied by the dependencies. */
#[macro_export]
macro_rules! EVENT_VAR { ($id:ident, $suffix:ident) => { event_attr_$id$suffix }; }
#[macro_export]
macro_rules! GENERIC_EVENT_ATTR { ($name:expr, $id:ident) => { EVENT_ATTR!($name, $id, _g) }; }
#[macro_export]
macro_rules! CACHE_EVENT_ATTR { ($name:expr, $id:ident) => { EVENT_ATTR!($name, $id, _c) }; }
#[macro_export]
macro_rules! POWER_EVENT_ATTR { ($name:expr, $id:ident) => { EVENT_ATTR!($name, $id, _p) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
