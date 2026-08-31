/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

use std::os::raw::{c_char, c_int, c_long, c_void};

// Dependencies from the original header:
// #include "pmu.h"
// struct list_head;
// struct perf_thread_map;

/**
 * enum hwmon_type:
 *
 * As described in Documentation/hwmon/sysfs-interface.rst hwmon events are
 * defined over multiple files of the form <type><num>_<item>. This enum
 * captures potential <type> values.
 *
 * This enum is exposed for testing.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hwmon_type {
    HWMON_TYPE_NONE,

    HWMON_TYPE_CPU,
    HWMON_TYPE_CURR,
    HWMON_TYPE_ENERGY,
    HWMON_TYPE_FAN,
    HWMON_TYPE_HUMIDITY,
    HWMON_TYPE_IN,
    HWMON_TYPE_INTRUSION,
    HWMON_TYPE_POWER,
    HWMON_TYPE_PWM,
    HWMON_TYPE_TEMP,

    HWMON_TYPE_MAX,
}

/**
 * enum hwmon_item:
 *
 * Similar to enum hwmon_type but describes the item part of a sysfs filename.
 *
 * This enum is exposed for testing.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hwmon_item {
    HWMON_ITEM_NONE,

    HWMON_ITEM_ACCURACY,
    HWMON_ITEM_ALARM,
    HWMON_ITEM_AUTO_CHANNELS_TEMP,
    HWMON_ITEM_AVERAGE,
    HWMON_ITEM_AVERAGE_HIGHEST,
    HWMON_ITEM_AVERAGE_INTERVAL,
    HWMON_ITEM_AVERAGE_INTERVAL_MAX,
    HWMON_ITEM_AVERAGE_INTERVAL_MIN,
    HWMON_ITEM_AVERAGE_LOWEST,
    HWMON_ITEM_AVERAGE_MAX,
    HWMON_ITEM_AVERAGE_MIN,
    HWMON_ITEM_BEEP,
    HWMON_ITEM_CAP,
    HWMON_ITEM_CAP_HYST,
    HWMON_ITEM_CAP_MAX,
    HWMON_ITEM_CAP_MIN,
    HWMON_ITEM_CRIT,
    HWMON_ITEM_CRIT_HYST,
    HWMON_ITEM_DIV,
    HWMON_ITEM_EMERGENCY,
    HWMON_ITEM_EMERGENCY_HIST,
    HWMON_ITEM_ENABLE,
    HWMON_ITEM_FAULT,
    HWMON_ITEM_FREQ,
    HWMON_ITEM_HIGHEST,
    HWMON_ITEM_INPUT,
    HWMON_ITEM_LABEL,
    HWMON_ITEM_LCRIT,
    HWMON_ITEM_LCRIT_HYST,
    HWMON_ITEM_LOWEST,
    HWMON_ITEM_MAX,
    HWMON_ITEM_MAX_HYST,
    HWMON_ITEM_MIN,
    HWMON_ITEM_MIN_HYST,
    HWMON_ITEM_MOD,
    HWMON_ITEM_OFFSET,
    HWMON_ITEM_PULSES,
    HWMON_ITEM_RATED_MAX,
    HWMON_ITEM_RATED_MIN,
    HWMON_ITEM_RESET_HISTORY,
    HWMON_ITEM_TARGET,
    HWMON_ITEM_TYPE,
    HWMON_ITEM_VID,

    HWMON_ITEM__MAX,
}

/**
 * union hwmon_pmu_event_key: Key for hwmon_pmu->events as such each key
 * represents an event.
 * union is exposed for testing to ensure problems are avoided on big
 * endian machines.
 *
 * Related hwmon files start <type><number> that this key represents.
 */
#[repr(C)]
pub union hwmon_pmu_event_key {
    pub type_and_num: c_long,
    // Original C anonymous struct bitfields:
    // struct {
    //     int num :16;
    //     enum hwmon_type type :8;
    // };
    // Rust has no direct stable C bitfield equivalent in a union field.
    pub bits: hwmon_pmu_event_key_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwmon_pmu_event_key_bits {
    pub num: c_int,
    pub type_: hwmon_type,
}

unsafe extern "C" {
    pub fn perf_pmu__is_hwmon(pmu: *const perf_pmu) -> bool;
    pub fn evsel__is_hwmon(evsel: *const evsel) -> bool;

    /**
     * parse_hwmon_filename() - Parse filename into constituent parts.
     *
     * @filename: To be parsed, of the form <type><number>_<item>.
     * @type: The type defined from the parsed file name.
     * @number: The number of the type, for example there may be more than 1 fan.
     * @item: A hwmon <type><number> may have multiple associated items.
     * @alarm: Is the filename for an alarm value?
     *
     * An example of a hwmon filename is "temp1_input". The type is temp for a
     * temperature value. The number is 1. The item within the file is an input
     * value - the temperature itself. This file doesn't contain an alarm value.
     *
     * Exposed for testing.
     */
    pub fn parse_hwmon_filename(
        filename: *const c_char,
        type_: *mut hwmon_type,
        number: *mut c_int,
        item: *mut hwmon_item,
        alarm: *mut bool,
    ) -> bool;

    /**
     * hwmon_pmu__new() - Allocate and construct a hwmon PMU.
     *
     * @pmus: The list of PMUs to be added to.
     * @hwmon_dir: The path to a hwmon directory.
     * @sysfs_name: Name of the hwmon sysfs directory like hwmon0.
     * @name: The contents of the "name" file in the hwmon directory.
     *
     * Exposed for testing. Regular construction should happen via
     * perf_pmus__read_hwmon_pmus.
     */
    pub fn hwmon_pmu__new(
        pmus: *mut list_head,
        hwmon_dir: *const c_char,
        sysfs_name: *const c_char,
        name: *const c_char,
    ) -> *mut perf_pmu;
    pub fn hwmon_pmu__exit(pmu: *mut perf_pmu);

    pub fn hwmon_pmu__for_each_event(
        pmu: *mut perf_pmu,
        state: *mut c_void,
        cb: pmu_event_callback,
    ) -> c_int;
    pub fn hwmon_pmu__num_events(pmu: *mut perf_pmu) -> usize;
    pub fn hwmon_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool;
    pub fn hwmon_pmu__config_terms(
        pmu: *const perf_pmu,
        attr: *mut perf_event_attr,
        terms: *mut parse_events_terms,
        err: *mut parse_events_error,
    ) -> c_int;
    pub fn hwmon_pmu__check_alias(
        terms: *mut parse_events_terms,
        info: *mut perf_pmu_info,
        err: *mut parse_events_error,
    ) -> c_int;

    pub fn perf_pmus__read_hwmon_pmus(pmus: *mut list_head) -> c_int;

    pub fn evsel__hwmon_pmu_open(
        evsel: *mut evsel,
        threads: *mut perf_thread_map,
        start_cpu_map_idx: c_int,
        end_cpu_map_idx: c_int,
    ) -> c_int;
    pub fn evsel__hwmon_pmu_read(
        evsel: *mut evsel,
        cpu_map_idx: c_int,
        thread: c_int,
    ) -> c_int;
}
