/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::c_int;

#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct target {
    _unused: [u8; 0],
}

/* Original C condition: #ifdef HAVE_BPF_SKEL */
#[cfg(HAVE_BPF_SKEL)]
pub type bpf_counter_evsel_op = Option<unsafe extern "C" fn(evsel: *mut evsel) -> c_int>;

#[cfg(HAVE_BPF_SKEL)]
pub type bpf_counter_evsel_target_op =
    Option<unsafe extern "C" fn(evsel: *mut evsel, target: *mut target) -> c_int>;

#[cfg(HAVE_BPF_SKEL)]
pub type bpf_counter_evsel_install_pe_op =
    Option<unsafe extern "C" fn(evsel: *mut evsel, cpu_map_idx: c_int, fd: c_int) -> c_int>;

/* Shared ops between bpf_counter, bpf_counter_cgroup, etc. */
#[cfg(HAVE_BPF_SKEL)]
#[repr(C)]
pub struct bpf_counter_ops {
    pub load: bpf_counter_evsel_target_op,
    pub enable: bpf_counter_evsel_op,
    pub disable: bpf_counter_evsel_op,
    pub read: bpf_counter_evsel_op,
    pub destroy: bpf_counter_evsel_op,
    pub install_pe: bpf_counter_evsel_install_pe_op,
}

#[cfg(HAVE_BPF_SKEL)]
unsafe extern "C" {
    pub fn bpf_counter__load(evsel: *mut evsel, target: *mut target) -> c_int;
    pub fn bpf_counter__enable(evsel: *mut evsel) -> c_int;
    pub fn bpf_counter__disable(evsel: *mut evsel) -> c_int;
    pub fn bpf_counter__read(evsel: *mut evsel) -> c_int;
    pub fn bpf_counter__destroy(evsel: *mut evsel);
    pub fn bpf_counter__install_pe(evsel: *mut evsel, cpu_map_idx: c_int, fd: c_int) -> c_int;

    pub fn bperf_trigger_reading(prog_fd: c_int, cpu: c_int) -> c_int;
    pub fn set_max_rlimit();
}

/* Original C condition: #else  // HAVE_BPF_SKEL */
/* Original C dependency in this branch: #include <linux/err.h> */

#[cfg(not(HAVE_BPF_SKEL))]
pub unsafe fn bpf_counter__load(_evsel: *mut evsel, _target: *mut target) -> c_int {
    0
}

#[cfg(not(HAVE_BPF_SKEL))]
pub unsafe fn bpf_counter__enable(_evsel: *mut evsel) -> c_int {
    0
}

#[cfg(not(HAVE_BPF_SKEL))]
pub unsafe fn bpf_counter__disable(_evsel: *mut evsel) -> c_int {
    0
}

#[cfg(not(HAVE_BPF_SKEL))]
pub unsafe fn bpf_counter__read(_evsel: *mut evsel) -> c_int {
    -EAGAIN
}

#[cfg(not(HAVE_BPF_SKEL))]
pub unsafe fn bpf_counter__destroy(_evsel: *mut evsel) {}

#[cfg(not(HAVE_BPF_SKEL))]
pub unsafe fn bpf_counter__install_pe(
    _evsel: *mut evsel,
    _cpu: c_int,
    _fd: c_int,
) -> c_int {
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
