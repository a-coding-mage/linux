// Translated from perf/util/trace_augment.h.
// C includes removed: <linux/compiler.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ptr;

pub type pid_t = i32;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

// C conditional: HAVE_BPF_SKEL.
#[cfg(feature = "HAVE_BPF_SKEL")]
unsafe extern "C" {
    pub fn augmented_syscalls__prepare() -> i32;
    pub fn augmented_syscalls__create_bpf_output(evlist: *mut evlist) -> i32;
    pub fn augmented_syscalls__setup_bpf_output();
    pub fn augmented_syscalls__set_filter_pids(nr: u32, pids: *mut pid_t) -> i32;
    pub fn augmented_syscalls__get_map_fds(
        enter_fd: *mut i32,
        exit_fd: *mut i32,
        beauty_fd: *mut i32,
    ) -> i32;
    pub fn augmented_syscalls__find_by_title(name: *const core::ffi::c_char) -> *mut bpf_program;
    pub fn augmented_syscalls__unaugmented() -> *mut bpf_program;
    pub fn augmented_syscalls__cleanup();
}

// C conditional: !HAVE_BPF_SKEL.
#[cfg(not(feature = "HAVE_BPF_SKEL"))]
#[inline]
pub unsafe fn augmented_syscalls__prepare() -> i32 {
    -1
}

#[cfg(not(feature = "HAVE_BPF_SKEL"))]
#[inline]
pub unsafe fn augmented_syscalls__create_bpf_output(_evlist: *mut evlist) -> i32 {
    -1
}

#[cfg(not(feature = "HAVE_BPF_SKEL"))]
#[inline]
pub unsafe fn augmented_syscalls__setup_bpf_output() {}

#[cfg(not(feature = "HAVE_BPF_SKEL"))]
#[inline]
pub unsafe fn augmented_syscalls__set_filter_pids(_nr: u32, _pids: *mut pid_t) -> i32 {
    0
}

#[cfg(not(feature = "HAVE_BPF_SKEL"))]
#[inline]
pub unsafe fn augmented_syscalls__get_map_fds(
    _enter_fd: *mut i32,
    _exit_fd: *mut i32,
    _beauty_fd: *mut i32,
) -> i32 {
    -1
}

#[cfg(not(feature = "HAVE_BPF_SKEL"))]
#[inline]
pub unsafe fn augmented_syscalls__find_by_title(
    _name: *const core::ffi::c_char,
) -> *mut bpf_program {
    ptr::null_mut()
}

#[cfg(not(feature = "HAVE_BPF_SKEL"))]
#[inline]
pub unsafe fn augmented_syscalls__unaugmented() -> *mut bpf_program {
    ptr::null_mut()
}

#[cfg(not(feature = "HAVE_BPF_SKEL"))]
#[inline]
pub unsafe fn augmented_syscalls__cleanup() {}
