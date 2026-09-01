/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct perf_event_attr {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct print_callbacks {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn pmu_name_len_no_suffix(str_: *const c_char) -> usize;

    /* Exposed for testing only. */
    pub fn pmu_name_cmp(lhs_pmu_name: *const c_char, rhs_pmu_name: *const c_char) -> c_int;

    pub fn perf_pmus__destroy();

    pub fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    pub fn perf_pmus__find_by_type(type_: c_uint) -> *mut perf_pmu;
    pub fn perf_pmus__find_by_attr(attr: *const perf_event_attr) -> *mut perf_pmu;

    pub fn perf_pmus__scan(pmu: *mut perf_pmu) -> *mut perf_pmu;
    pub fn perf_pmus__scan_core(pmu: *mut perf_pmu) -> *mut perf_pmu;
    pub fn perf_pmus__scan_for_event(pmu: *mut perf_pmu, event: *const c_char) -> *mut perf_pmu;
    pub fn perf_pmus__scan_matching_wildcard(
        pmu: *mut perf_pmu,
        wildcard: *const c_char,
    ) -> *mut perf_pmu;
    pub fn perf_pmus__scan_for_uncore_id(
        pmu: *mut perf_pmu,
        compat: *const c_char,
    ) -> *mut perf_pmu;

    pub fn perf_pmus__pmu_for_pmu_filter(str_: *const c_char) -> *const perf_pmu;

    pub fn perf_pmus__print_pmu_events(
        print_cb: *const print_callbacks,
        print_state: *mut c_void,
    );
    pub fn perf_pmus__print_raw_pmu_events(
        print_cb: *const print_callbacks,
        print_state: *mut c_void,
    );
    pub fn perf_pmus__have_event(pname: *const c_char, name: *const c_char) -> bool;
    pub fn perf_pmus__num_core_pmus() -> c_int;
    pub fn perf_pmus__supports_extended_type() -> bool;

    pub fn perf_pmus__add_test_pmu(test_sysfs_dirfd: c_int, name: *const c_char) -> *mut perf_pmu;
    pub fn perf_pmus__add_test_hwmon_pmu(
        hwmon_dir: *const c_char,
        sysfs_name: *const c_char,
        name: *const c_char,
    ) -> *mut perf_pmu;
    pub fn perf_pmus__fake_pmu() -> *mut perf_pmu;
    pub fn perf_pmus__find_core_pmu() -> *mut perf_pmu;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
