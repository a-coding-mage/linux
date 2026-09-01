/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
 */

/* Translated from the C header cpupower-monitor.h.
 * C include dependencies preserved as Rust external references:
 * <stdarg.h>, <time.h>, <sched.h>, <sys/types.h>, <unistd.h>,
 * and "idle_monitor/idle_monitors.h".
 */

use core::ffi::{c_char, c_double, c_int, c_longlong, c_uint, c_ulonglong};

pub const MONITORS_MAX: usize = 20;
pub const MONITOR_NAME_LEN: usize = 20;

/* CSTATE_NAME_LEN is limited by header field width defined
 * in cpupower-monitor.c. Header field width is defined to be
 * sum of percent width and two spaces for padding.
 *
 * C source used:
 *   #ifdef __powerpc__
 *   #define CSTATE_NAME_LEN 7
 *   #else
 *   #define CSTATE_NAME_LEN 5
 *   #endif
 */
#[cfg(target_arch = "powerpc")]
pub const CSTATE_NAME_LEN: usize = 7;
#[cfg(not(target_arch = "powerpc"))]
pub const CSTATE_NAME_LEN: usize = 5;

pub const CSTATE_DESC_LEN: usize = 60;

unsafe extern "C" {
    pub static mut cpu_count: c_int;
}

/* Hard to define the right names ...: */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum power_range_e {
    RANGE_THREAD = 0,  /* Lowest in topology hierarcy, AMD: core, Intel: thread
                        * kernel sysfs: cpu */
    RANGE_CORE = 1,    /* AMD: unit, Intel: core, kernel_sysfs: core_id */
    RANGE_PACKAGE = 2, /* Package, processor socket */
    RANGE_MACHINE = 3, /* Machine, platform wide */
    RANGE_MAX = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstate {
    pub id: c_int,
    pub range: power_range_e,
    pub name: [c_char; CSTATE_NAME_LEN],
    pub desc: [c_char; CSTATE_DESC_LEN],

    /* either provide a percentage or a general count */
    pub get_count_percent:
        Option<unsafe extern "C" fn(self_id: c_uint, percent: *mut c_double, cpu: c_uint) -> c_int>,
    pub get_count:
        Option<unsafe extern "C" fn(self_id: c_uint, count: *mut c_ulonglong, cpu: c_uint) -> c_int>,
}

#[allow(non_camel_case_types)]
pub type cstate_t = cstate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_monitor_flags {
    bitfield: c_uint,
}

impl cpuidle_monitor_flags {
    pub const NEEDS_ROOT_MASK: c_uint = 1 << 0;
    pub const PER_CPU_SCHEDULE_MASK: c_uint = 1 << 1;

    pub fn needs_root(&self) -> c_uint {
        self.bitfield & Self::NEEDS_ROOT_MASK
    }

    pub fn set_needs_root(&mut self, value: c_uint) {
        self.bitfield = (self.bitfield & !Self::NEEDS_ROOT_MASK) | ((value & 1) << 0);
    }

    pub fn per_cpu_schedule(&self) -> c_uint {
        (self.bitfield & Self::PER_CPU_SCHEDULE_MASK) >> 1
    }

    pub fn set_per_cpu_schedule(&mut self, value: c_uint) {
        self.bitfield = (self.bitfield & !Self::PER_CPU_SCHEDULE_MASK) | ((value & 1) << 1);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_monitor {
    /* Name must not contain whitespaces */
    pub name: [c_char; MONITOR_NAME_LEN],
    pub name_len: c_int,
    pub hw_states_num: c_int,
    pub hw_states: *mut cstate_t,
    pub start: Option<unsafe extern "C" fn() -> c_int>,
    pub stop: Option<unsafe extern "C" fn() -> c_int>,
    pub do_register: Option<unsafe extern "C" fn() -> *mut cpuidle_monitor>,
    pub unregister: Option<unsafe extern "C" fn()>,
    pub overflow_s: c_uint,
    pub flags: cpuidle_monitor_flags,
}

unsafe extern "C" {
    pub fn timespec_diff_us(start: libc::timespec, end: libc::timespec) -> c_longlong;

    pub static mut stderr: *mut libc::FILE;
    pub fn fprintf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    pub fn gettext(msgid: *const c_char) -> *mut c_char;
}

#[inline]
pub unsafe fn print_overflow_err(mes: c_uint, ov: c_uint) {
    unsafe {
        fprintf(
            stderr,
            gettext(
                b"Measure took %u seconds, but registers could overflow at %u seconds, results could be inaccurate\n\0"
                    .as_ptr() as *const c_char,
            ),
            mes,
            ov,
        );
    }
}

/* Taken over from x86info project sources  -> return 0 on success */
#[inline]
pub unsafe fn bind_cpu(cpu: c_int) -> c_int {
    unsafe {
        let mut set: libc::cpu_set_t = core::mem::zeroed();

        if libc::sched_getaffinity(libc::getpid(), core::mem::size_of_val(&set), &mut set) == 0 {
            libc::CPU_ZERO(&mut set);
            libc::CPU_SET(cpu as usize, &mut set);
            return libc::sched_setaffinity(libc::getpid(), core::mem::size_of_val(&set), &set);
        }
        1
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
