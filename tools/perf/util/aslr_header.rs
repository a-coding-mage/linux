/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from C header: #include <linux/perf_event.h>

pub const ASLR_SUPPORTED_SAMPLE_TYPE: u64 = PERF_SAMPLE_IDENTIFIER
    | PERF_SAMPLE_IP
    | PERF_SAMPLE_TID
    | PERF_SAMPLE_TIME
    | PERF_SAMPLE_ADDR
    | PERF_SAMPLE_ID
    | PERF_SAMPLE_STREAM_ID
    | PERF_SAMPLE_CPU
    | PERF_SAMPLE_PERIOD
    | PERF_SAMPLE_READ
    | PERF_SAMPLE_CALLCHAIN
    | PERF_SAMPLE_RAW
    | PERF_SAMPLE_BRANCH_STACK
    | PERF_SAMPLE_STACK_USER
    | PERF_SAMPLE_WEIGHT_TYPE
    | PERF_SAMPLE_DATA_SRC
    | PERF_SAMPLE_TRANSACTION
    | PERF_SAMPLE_PHYS_ADDR
    | PERF_SAMPLE_CGROUP
    | PERF_SAMPLE_DATA_PAGE_SIZE
    | PERF_SAMPLE_CODE_PAGE_SIZE
    | PERF_SAMPLE_AUX;

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn aslr_tool__new(delegate: *mut perf_tool) -> *mut perf_tool;
    pub fn aslr_tool__delete(tool: *mut perf_tool);

    pub fn aslr_tool__strip_attr_event(event: *mut perf_event, evlist: *mut evlist);
    pub fn aslr_tool__cache_orig_attrs(tool: *mut perf_tool, evsel: *mut evsel) -> ::std::os::raw::c_int;
    pub fn aslr_tool__strip_evlist(tool: *const perf_tool, evlist: *mut evlist);
    pub fn aslr_tool__restore_evlist(tool: *const perf_tool, evlist: *mut evlist);
}
