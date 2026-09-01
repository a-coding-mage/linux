/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <stdio.h> supplies FILE.
use libc::FILE;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum trace_summary_mode {
    SUMMARY__NONE = 0,
    SUMMARY__BY_TOTAL,
    SUMMARY__BY_THREAD,
    SUMMARY__BY_CGROUP,
}

// C conditional: HAVE_BPF_SKEL
#[cfg(HAVE_BPF_SKEL)]
unsafe extern "C" {
    pub fn trace_prepare_bpf_summary(mode: trace_summary_mode) -> ::std::os::raw::c_int;
    pub fn trace_start_bpf_summary();
    pub fn trace_end_bpf_summary();
    pub fn trace_print_bpf_summary(
        fp: *mut FILE,
        max_summary: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn trace_cleanup_bpf_summary();
}

// C conditional: !HAVE_BPF_SKEL
#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn trace_prepare_bpf_summary(
    _mode: trace_summary_mode,
) -> ::std::os::raw::c_int {
    -1
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn trace_start_bpf_summary() {}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn trace_end_bpf_summary() {}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn trace_print_bpf_summary(
    _fp: *mut FILE,
    _max_summary: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn trace_cleanup_bpf_summary() {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
