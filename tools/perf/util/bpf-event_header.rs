/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_void};

pub type __u64 = u64;
pub type u32 = u32;
pub type FILE = c_void;

#[repr(C)]
pub struct bpf_prog_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_metadata {
    pub event: *mut perf_event,
    pub prog_names: *mut *mut c_char,
    pub nr_prog_names: __u64,
}

#[repr(C)]
pub struct bpf_prog_info_node {
    pub info_linear: *mut perf_bpil,
    pub metadata: *mut bpf_metadata,
    pub rb_node: rb_node,
}

#[repr(C)]
pub struct btf_node {
    pub rb_node: rb_node,
    pub id: u32,
    pub data_size: u32,
    pub data: [c_char; 0],
}

// C conditional: #ifdef HAVE_LIBBPF_SUPPORT
#[cfg(HAVE_LIBBPF_SUPPORT)]
unsafe extern "C" {
    pub fn machine__process_bpf(
        machine: *mut machine,
        event: *mut perf_event,
        sample: *mut perf_sample,
    ) -> c_int;
    pub fn evlist__add_bpf_sb_event(evlist: *mut evlist, env: *mut perf_env) -> c_int;
    pub fn __bpf_event__print_bpf_prog_info(
        info_linear: *mut perf_bpil,
        env: *mut perf_env,
        fp: *mut FILE,
    );
    pub fn bpf_metadata_free(metadata: *mut bpf_metadata);
}

#[cfg(not(HAVE_LIBBPF_SUPPORT))]
#[inline]
pub unsafe fn machine__process_bpf(
    machine: *mut machine,
    event: *mut perf_event,
    sample: *mut perf_sample,
) -> c_int {
    let _ = machine;
    let _ = event;
    let _ = sample;
    0
}

#[cfg(not(HAVE_LIBBPF_SUPPORT))]
#[inline]
pub unsafe fn evlist__add_bpf_sb_event(evlist: *mut evlist, env: *mut perf_env) -> c_int {
    let _ = evlist;
    let _ = env;
    0
}

#[cfg(not(HAVE_LIBBPF_SUPPORT))]
#[inline]
pub unsafe fn __bpf_event__print_bpf_prog_info(
    info_linear: *mut perf_bpil,
    env: *mut perf_env,
    fp: *mut FILE,
) {
    let _ = info_linear;
    let _ = env;
    let _ = fp;
}

#[cfg(not(HAVE_LIBBPF_SUPPORT))]
#[inline]
pub unsafe fn bpf_metadata_free(metadata: *mut bpf_metadata) {
    let _ = metadata;
}
