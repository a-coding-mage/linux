/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn topdown_sys_has_perf_metrics() -> bool;
    pub fn arch_is_topdown_slots(evsel: *const evsel) -> bool;
    pub fn arch_is_topdown_metrics(evsel: *const evsel) -> bool;
    pub fn topdown_insert_slots_event(
        list: *mut list_head,
        idx: ::std::os::raw::c_int,
        metric_event: *mut evsel,
    ) -> ::std::os::raw::c_int;
}
