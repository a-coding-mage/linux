/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/target.h. */

use std::os::raw::{c_char, c_int};

pub type uid_t = u32;
pub type size_t = usize;

#[repr(C)]
pub struct target {
    pub pid: *const c_char,
    pub tid: *const c_char,
    pub cpu_list: *const c_char,
    pub bpf_str: *const c_char,
    pub system_wide: bool,
    pub uses_mmap: bool,
    pub default_per_cpu: bool,
    pub per_thread: bool,
    pub use_bpf: bool,
    pub inherit: bool,
    pub initial_delay: c_int,
    pub attr_map: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum target_errno {
    TARGET_ERRNO__SUCCESS = 0,

    /*
     * Choose an arbitrary negative big number not to clash with standard
     * errno since SUS requires the errno has distinct positive values.
     * See 'Issue 6' in the link below.
     *
     * http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/errno.h.html
     */
    __TARGET_ERRNO__START = -10000,

    /* for target__validate() */
    TARGET_ERRNO__PID_OVERRIDE_CPU = -10000,
    TARGET_ERRNO__PID_OVERRIDE_SYSTEM = -9999,
    TARGET_ERRNO__SYSTEM_OVERRIDE_THREAD = -9998,
    TARGET_ERRNO__BPF_OVERRIDE_CPU = -9997,
    TARGET_ERRNO__BPF_OVERRIDE_PID = -9996,
    TARGET_ERRNO__BPF_OVERRIDE_THREAD = -9995,

    __TARGET_ERRNO__END = -9994,
}

extern "C" {
    pub fn target__validate(target: *mut target) -> target_errno;

    pub fn parse_uid(str: *const c_char) -> uid_t;

    pub fn target__strerror(
        target: *mut target,
        errnum: c_int,
        buf: *mut c_char,
        buflen: size_t,
    ) -> c_int;
}

#[inline]
pub unsafe fn target__has_task(target: *const target) -> bool {
    unsafe { !(*target).tid.is_null() || !(*target).pid.is_null() }
}

#[inline]
pub unsafe fn target__has_cpu(target: *const target) -> bool {
    unsafe { (*target).system_wide || !(*target).cpu_list.is_null() }
}

#[inline]
pub unsafe fn target__none(target: *const target) -> bool {
    unsafe { !target__has_task(target) && !target__has_cpu(target) }
}

#[inline]
pub unsafe fn target__enable_on_exec(target: *const target) -> bool {
    /*
     * Normally enable_on_exec should be set if:
     *  1) The tracee process is forked (not attaching to existed task or cpu).
     *  2) And initial_delay is not configured.
     * Otherwise, we enable tracee events manually.
     */
    unsafe { target__none(target) && (*target).initial_delay == 0 }
}

#[inline]
pub unsafe fn target__has_per_thread(target: *const target) -> bool {
    unsafe { (*target).system_wide && (*target).per_thread }
}

#[inline]
pub unsafe fn target__uses_dummy_map(target: *const target) -> bool {
    let mut use_dummy = false;

    unsafe {
        if (*target).default_per_cpu {
            use_dummy = if (*target).per_thread { true } else { false };
        } else if target__has_task(target)
            || (!target__has_cpu(target) && !(*target).uses_mmap)
        {
            use_dummy = true;
        } else if target__has_per_thread(target) {
            use_dummy = true;
        }
    }

    use_dummy
}
