// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/jit.h.
// Original dependency: <data.h>

#[repr(C)]
pub struct perf_session {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_data {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _unused: [u8; 0],
}

#[allow(non_camel_case_types)]
pub type pid_t = i32;

unsafe extern "C" {
    pub fn jit_process(
        session: *mut perf_session,
        output: *mut perf_data,
        machine: *mut machine,
        filename: *const ::std::os::raw::c_char,
        pid: pid_t,
        tid: pid_t,
        nbytes: *mut u64,
    ) -> ::std::os::raw::c_int;

    pub fn jit_inject_record(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
