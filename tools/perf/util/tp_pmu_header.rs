/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

use core::ffi::{c_char, c_int, c_void};

/* Dependency intent from C: #include "pmu.h" */
use crate::{perf_pmu, pmu_event_callback};

pub type tp_sys_callback =
    Option<unsafe extern "C" fn(state: *mut c_void, sys_name: *const c_char) -> c_int>;
pub type tp_event_callback = Option<
    unsafe extern "C" fn(
        state: *mut c_void,
        sys_name: *const c_char,
        evt_name: *const c_char,
    ) -> c_int,
>;

unsafe extern "C" {
    pub fn tp_pmu__id(sys: *const c_char, name: *const c_char) -> c_int;
    pub fn tp_pmu__for_each_tp_event(
        sys: *const c_char,
        state: *mut c_void,
        cb: tp_event_callback,
    ) -> c_int;
    pub fn tp_pmu__for_each_tp_sys(state: *mut c_void, cb: tp_sys_callback) -> c_int;

    pub fn perf_pmu__is_tracepoint(pmu: *const perf_pmu) -> bool;
    pub fn tp_pmu__for_each_event(
        pmu: *mut perf_pmu,
        state: *mut c_void,
        cb: pmu_event_callback,
    ) -> c_int;
    pub fn tp_pmu__num_events(pmu: *mut perf_pmu) -> usize;
    pub fn tp_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
