/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Intel SOC Telemetry Driver Header File
 * Copyright (C) 2015, Intel Corporation.
 * All Rights Reserved.
 */

// Dependency supplied by the surrounding kernel translation.

pub const TELEM_MAX_EVENTS_SRAM: u32 = 28;
pub const TELEM_MAX_OS_ALLOCATED_EVENTS: u32 = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum telemetry_unit {
    TELEM_PSS = 0,
    TELEM_IOSS,
    TELEM_UNIT_NONE,
}

#[repr(C)]
pub struct telemetry_evtlog {
    pub telem_evtid: u32,
    pub telem_evtlog: u64,
}

#[repr(C)]
pub struct telemetry_evtconfig {
    /* Array of Event-IDs to Enable */
    pub evtmap: *mut u32,

    /* Number of Events (<29) in evtmap */
    pub num_evts: u8,

    /* Sampling period */
    pub period: u8,
}

#[repr(C)]
pub struct telemetry_evtmap {
    pub name: *const core::ffi::c_char,
    pub evt_id: u32,
}

#[repr(C)]
pub struct telemetry_unit_config {
    pub telem_evts: *mut telemetry_evtmap,
    pub regmap: *mut core::ffi::c_void,
    pub ssram_evts_used: u8,
    pub curr_period: u8,
    pub max_period: u8,
    pub min_period: u8,
}

#[repr(C)]
pub struct telemetry_plt_config {
    pub pss_config: telemetry_unit_config,
    pub ioss_config: telemetry_unit_config,
    pub telem_trace_lock: mutex,
    pub telem_lock: mutex,
    pub pmc: *mut intel_pmc_dev,
    pub scu: *mut intel_scu_ipc_dev,
    pub telem_in_use: bool,
}

#[repr(C)]
pub struct telemetry_core_ops {
    pub get_trace_verbosity:
        Option<unsafe extern "C" fn(telem_unit: telemetry_unit, verbosity: *mut u32) -> i32>,
    pub set_trace_verbosity:
        Option<unsafe extern "C" fn(telem_unit: telemetry_unit, verbosity: u32) -> i32>,
    pub raw_read_eventlog: Option<unsafe extern "C" fn(
        telem_unit: telemetry_unit,
        evtlog: *mut telemetry_evtlog,
        len: i32,
        log_all_evts: i32,
    ) -> i32>,
    pub read_eventlog: Option<unsafe extern "C" fn(
        telem_unit: telemetry_unit,
        evtlog: *mut telemetry_evtlog,
        len: i32,
        log_all_evts: i32,
    ) -> i32>,
}

extern "C" {
    pub fn telemetry_set_pltdata(
        ops: *const telemetry_core_ops,
        pltconfig: *mut telemetry_plt_config,
    ) -> i32;

    pub fn telemetry_clear_pltdata() -> i32;

    pub fn telemetry_get_pltdata() -> *mut telemetry_plt_config;

    pub fn telemetry_get_evtname(
        telem_unit: telemetry_unit,
        name: *mut *const core::ffi::c_char,
        len: i32,
    ) -> i32;

    pub fn telemetry_read_events(
        telem_unit: telemetry_unit,
        evtlog: *mut telemetry_evtlog,
        len: i32,
    ) -> i32;

    pub fn telemetry_read_eventlog(
        telem_unit: telemetry_unit,
        evtlog: *mut telemetry_evtlog,
        len: i32,
    ) -> i32;

    pub fn telemetry_raw_read_eventlog(
        telem_unit: telemetry_unit,
        evtlog: *mut telemetry_evtlog,
        len: i32,
    ) -> i32;

    pub fn telemetry_set_trace_verbosity(telem_unit: telemetry_unit, verbosity: u32) -> i32;

    pub fn telemetry_get_trace_verbosity(
        telem_unit: telemetry_unit,
        verbosity: *mut u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
