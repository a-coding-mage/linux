// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_ulong};

/*
 * The 'struct evsel_config_term' is used to pass event
 * specific configuration data to evsel__config routine.
 * It is allocated within event parsing and attached to
 * evsel::config_terms list head.
*/
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum evsel_term_type {
    EVSEL__CONFIG_TERM_PERIOD,
    EVSEL__CONFIG_TERM_FREQ,
    EVSEL__CONFIG_TERM_TIME,
    EVSEL__CONFIG_TERM_CALLGRAPH,
    EVSEL__CONFIG_TERM_STACK_USER,
    EVSEL__CONFIG_TERM_INHERIT,
    EVSEL__CONFIG_TERM_MAX_STACK,
    EVSEL__CONFIG_TERM_MAX_EVENTS,
    EVSEL__CONFIG_TERM_OVERWRITE,
    EVSEL__CONFIG_TERM_DRV_CFG,
    EVSEL__CONFIG_TERM_BRANCH,
    EVSEL__CONFIG_TERM_PERCORE,
    EVSEL__CONFIG_TERM_AUX_OUTPUT,
    EVSEL__CONFIG_TERM_AUX_ACTION,
    EVSEL__CONFIG_TERM_AUX_SAMPLE_SIZE,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG1,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG2,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG3,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG4,
    EVSEL__CONFIG_TERM_RATIO_TO_PREV,
}

#[repr(C)]
pub union evsel_config_term_val {
    pub period: u64,
    pub freq: u64,
    pub time: bool,
    pub stack_user: u64,
    pub max_stack: c_int,
    pub inherit: bool,
    pub overwrite: bool,
    pub max_events: c_ulong,
    pub percore: bool,
    pub aux_output: bool,
    pub aux_sample_size: u32,
    pub cfg_chg: u64,
    pub str: *mut c_char,
    pub cpu: c_int,
    pub val: u64,
}

#[repr(C)]
pub struct evsel_config_term {
    pub list: list_head,
    pub type_: evsel_term_type,
    pub free_str: bool,
    pub val: evsel_config_term_val,
    pub weak: bool,
}

pub enum evsel {}

unsafe extern "C" {
    pub fn __evsel__get_config_term(
        evsel: *mut evsel,
        type_: evsel_term_type,
    ) -> *mut evsel_config_term;
}

macro_rules! evsel__get_config_term {
    ($evsel:expr, PERIOD) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_PERIOD)
    };
    ($evsel:expr, FREQ) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_FREQ)
    };
    ($evsel:expr, TIME) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_TIME)
    };
    ($evsel:expr, CALLGRAPH) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_CALLGRAPH)
    };
    ($evsel:expr, STACK_USER) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_STACK_USER)
    };
    ($evsel:expr, INHERIT) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_INHERIT)
    };
    ($evsel:expr, MAX_STACK) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_MAX_STACK)
    };
    ($evsel:expr, MAX_EVENTS) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_MAX_EVENTS)
    };
    ($evsel:expr, OVERWRITE) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_OVERWRITE)
    };
    ($evsel:expr, DRV_CFG) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_DRV_CFG)
    };
    ($evsel:expr, BRANCH) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_BRANCH)
    };
    ($evsel:expr, PERCORE) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_PERCORE)
    };
    ($evsel:expr, AUX_OUTPUT) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_AUX_OUTPUT)
    };
    ($evsel:expr, AUX_ACTION) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_AUX_ACTION)
    };
    ($evsel:expr, AUX_SAMPLE_SIZE) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_AUX_SAMPLE_SIZE)
    };
    ($evsel:expr, USR_CHG_CONFIG) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_USR_CHG_CONFIG)
    };
    ($evsel:expr, USR_CHG_CONFIG1) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_USR_CHG_CONFIG1)
    };
    ($evsel:expr, USR_CHG_CONFIG2) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_USR_CHG_CONFIG2)
    };
    ($evsel:expr, USR_CHG_CONFIG3) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_USR_CHG_CONFIG3)
    };
    ($evsel:expr, USR_CHG_CONFIG4) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_USR_CHG_CONFIG4)
    };
    ($evsel:expr, RATIO_TO_PREV) => {
        __evsel__get_config_term($evsel, evsel_term_type::EVSEL__CONFIG_TERM_RATIO_TO_PREV)
    };
}

pub(crate) use evsel__get_config_term;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
