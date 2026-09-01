// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
 *
 *  Based on Len Brown's <lenb@kernel.org> turbostat tool.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use core::ffi::{c_char, c_double, c_int, c_uint, c_ulonglong, c_void};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MSR_PKG_C2_RESIDENCY: c_int = 0x60D;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MSR_PKG_C7_RESIDENCY: c_int = 0x3FA;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MSR_CORE_C7_RESIDENCY: c_int = 0x3FE;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MSR_TSC: c_int = 0x10;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const C7: c_uint = 0;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const PC2: c_uint = 1;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const PC7: c_uint = 2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const SNB_CSTATE_COUNT: usize = 3;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const TSC: c_uint = 0xFFFF;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
extern "C" {
    static mut cpu_count: c_int;
    static mut base_cpu: c_uint;
    static mut cpupower_cpu_info: cpupower_cpu_info_t;

    fn read_msr(cpu: c_uint, msr: c_int, val: *mut c_ulonglong) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const X86_VENDOR_INTEL: c_int = 1;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const RANGE_CORE: c_int = 0;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const RANGE_PACKAGE: c_int = 1;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct cpupower_cpu_info_t {
    pub vendor: c_int,
    pub family: c_uint,
    pub model: c_uint,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct cstate_t {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub id: c_uint,
    pub range: c_int,
    pub get_count_percent:
        Option<unsafe extern "C" fn(c_uint, *mut c_double, c_uint) -> c_int>,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct cpuidle_monitor_flags {
    pub needs_root: c_uint,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct cpuidle_monitor {
    pub name: *const c_char,
    pub name_len: usize,
    pub hw_states: *mut cstate_t,
    pub hw_states_num: c_uint,
    pub start: Option<unsafe extern "C" fn() -> c_int>,
    pub stop: Option<unsafe extern "C" fn() -> c_int>,
    pub do_register: Option<unsafe extern "C" fn() -> *mut cpuidle_monitor>,
    pub unregister: Option<unsafe extern "C" fn()>,
    pub flags: cpuidle_monitor_flags,
    pub overflow_s: c_uint,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! dprint {
    ($($arg:tt)*) => {};
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static C7_NAME: &[u8] = b"C7\0";
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static C7_DESC: &[u8] = b"Processor Core C7\0";
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static PC2_NAME: &[u8] = b"PC2\0";
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static PC2_DESC: &[u8] = b"Processor Package C2\0";
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static PC7_NAME: &[u8] = b"PC7\0";
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static PC7_DESC: &[u8] = b"Processor Package C7\0";
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static MONITOR_NAME: &[u8] = b"SandyBridge\0";

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn snb_get_count_percent(
    self_id: c_uint,
    percent: *mut c_double,
    cpu: c_uint,
) -> c_int {
    unsafe {
        *percent = 0.0;

        if *is_valid.add(cpu as usize) == 0 {
            return -1;
        }

        *percent = (100.0
            * (*(*current_count.as_mut_ptr().add(self_id as usize)).add(cpu as usize)
                - *(*previous_count.as_mut_ptr().add(self_id as usize)).add(cpu as usize))
                as c_double)
            / (tsc_at_measure_end - tsc_at_measure_start) as c_double;

        dprint!(
            "%s: previous: %llu - current: %llu - (%u)\n",
            snb_cstates[self_id as usize].name,
            *(*previous_count.as_mut_ptr().add(self_id as usize)).add(cpu as usize),
            *(*current_count.as_mut_ptr().add(self_id as usize)).add(cpu as usize),
            cpu
        );

        dprint!(
            "%s: tsc_diff: %llu - count_diff: %llu - percent: %2.f (%u)\n",
            snb_cstates[self_id as usize].name,
            tsc_at_measure_end - tsc_at_measure_start,
            *(*current_count.as_mut_ptr().add(self_id as usize)).add(cpu as usize)
                - *(*previous_count.as_mut_ptr().add(self_id as usize)).add(cpu as usize),
            *percent,
            cpu
        );

        0
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut snb_cstates: [cstate_t; SNB_CSTATE_COUNT] = [
    cstate_t {
        name: C7_NAME.as_ptr() as *const c_char,
        desc: C7_DESC.as_ptr() as *const c_char,
        id: C7,
        range: RANGE_CORE,
        get_count_percent: Some(snb_get_count_percent),
    },
    cstate_t {
        name: PC2_NAME.as_ptr() as *const c_char,
        desc: PC2_DESC.as_ptr() as *const c_char,
        id: PC2,
        range: RANGE_PACKAGE,
        get_count_percent: Some(snb_get_count_percent),
    },
    cstate_t {
        name: PC7_NAME.as_ptr() as *const c_char,
        desc: PC7_DESC.as_ptr() as *const c_char,
        id: PC7,
        range: RANGE_PACKAGE,
        get_count_percent: Some(snb_get_count_percent),
    },
];

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut tsc_at_measure_start: c_ulonglong = 0;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut tsc_at_measure_end: c_ulonglong = 0;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut previous_count: [*mut c_ulonglong; SNB_CSTATE_COUNT] =
    [core::ptr::null_mut(); SNB_CSTATE_COUNT];
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut current_count: [*mut c_ulonglong; SNB_CSTATE_COUNT] =
    [core::ptr::null_mut(); SNB_CSTATE_COUNT];
/* valid flag for all CPUs. If a MSR read failed it will be zero */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut is_valid: *mut c_int = core::ptr::null_mut();

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn snb_get_count(id: c_uint, val: *mut c_ulonglong, cpu: c_uint) -> c_int {
    let msr: c_int;

    match id {
        C7 => {
            msr = MSR_CORE_C7_RESIDENCY;
        }
        PC2 => {
            msr = MSR_PKG_C2_RESIDENCY;
        }
        PC7 => {
            msr = MSR_PKG_C7_RESIDENCY;
        }
        TSC => {
            msr = MSR_TSC;
        }
        _ => {
            return -1;
        }
    }
    unsafe {
        if read_msr(cpu, msr, val) != 0 {
            return -1;
        }
    }
    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn snb_start() -> c_int {
    let mut num: c_int;
    let mut cpu: c_int;
    let mut val: c_ulonglong = 0;

    unsafe {
        num = 0;
        while num < SNB_CSTATE_COUNT as c_int {
            cpu = 0;
            while cpu < cpu_count {
                *is_valid.add(cpu as usize) =
                    (snb_get_count(num as c_uint, &mut val, cpu as c_uint) == 0) as c_int;
                *(*previous_count.as_mut_ptr().add(num as usize)).add(cpu as usize) = val;
                cpu += 1;
            }
            num += 1;
        }
        snb_get_count(TSC, &mut tsc_at_measure_start, base_cpu);
    }
    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn snb_stop() -> c_int {
    let mut val: c_ulonglong = 0;
    let mut num: c_int;
    let mut cpu: c_int;

    unsafe {
        snb_get_count(TSC, &mut tsc_at_measure_end, base_cpu);

        num = 0;
        while num < SNB_CSTATE_COUNT as c_int {
            cpu = 0;
            while cpu < cpu_count {
                *is_valid.add(cpu as usize) |=
                    (snb_get_count(num as c_uint, &mut val, cpu as c_uint) == 0) as c_int;
                *(*current_count.as_mut_ptr().add(num as usize)).add(cpu as usize) = val;
                cpu += 1;
            }
            num += 1;
        }
    }
    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn snb_register() -> *mut cpuidle_monitor {
    let mut num: c_int;

    unsafe {
        if cpupower_cpu_info.vendor != X86_VENDOR_INTEL || cpupower_cpu_info.family != 6 {
            return core::ptr::null_mut();
        }

        match cpupower_cpu_info.model {
            0x2A => {} /* SNB */
            0x2D => {} /* SNB Xeon */
            0x3A => {} /* IVB */
            0x3E => {} /* IVB Xeon */
            0x3C => {} /* HSW */
            0x3F => {} /* HSW */
            0x45 => {} /* HSW */
            0x46 => {} /* HSW */
            _ => {
                return core::ptr::null_mut();
            }
        }

        is_valid = calloc(cpu_count as usize, core::mem::size_of::<c_int>()) as *mut c_int;
        num = 0;
        while num < SNB_CSTATE_COUNT as c_int {
            *previous_count.as_mut_ptr().add(num as usize) = calloc(
                cpu_count as usize,
                core::mem::size_of::<c_ulonglong>(),
            ) as *mut c_ulonglong;
            *current_count.as_mut_ptr().add(num as usize) = calloc(
                cpu_count as usize,
                core::mem::size_of::<c_ulonglong>(),
            ) as *mut c_ulonglong;
            num += 1;
        }
        intel_snb_monitor.name_len = strlen(intel_snb_monitor.name);
        &mut intel_snb_monitor
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn snb_unregister() {
    let mut num: c_int;
    unsafe {
        free(is_valid as *mut c_void);
        num = 0;
        while num < SNB_CSTATE_COUNT as c_int {
            free(*previous_count.as_mut_ptr().add(num as usize) as *mut c_void);
            free(*current_count.as_mut_ptr().add(num as usize) as *mut c_void);
            num += 1;
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[no_mangle]
pub static mut intel_snb_monitor: cpuidle_monitor = cpuidle_monitor {
    name: MONITOR_NAME.as_ptr() as *const c_char,
    hw_states: unsafe { snb_cstates.as_mut_ptr() },
    hw_states_num: SNB_CSTATE_COUNT as c_uint,
    start: Some(snb_start),
    stop: Some(snb_stop),
    do_register: Some(snb_register),
    unregister: Some(snb_unregister),
    flags: cpuidle_monitor_flags { needs_root: 1 },
    overflow_s: 922000000, /* 922337203 seconds TSC overflow
                            * at 20GHz */
    name_len: 0,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
