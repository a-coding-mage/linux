/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * intel_tpebs.h: Intel TEPBS support
 */

#[repr(C)]
pub struct evlist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tpebs_mode {
    TPEBS_MODE__MEAN,
    TPEBS_MODE__MIN,
    TPEBS_MODE__MAX,
    TPEBS_MODE__LAST,
}

unsafe extern "C" {
    pub static mut tpebs_recording: bool;
    pub static mut tpebs_mode: tpebs_mode;

    pub fn evsel__tpebs_open(evsel: *mut evsel) -> ::std::os::raw::c_int;
    pub fn evsel__tpebs_close(evsel: *mut evsel);
    pub fn evsel__tpebs_read(
        evsel: *mut evsel,
        cpu_map_idx: ::std::os::raw::c_int,
        thread: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
