// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
 *
 *  Based on SandyBridge monitor. Implements the new package C-states
 *  (PC8, PC9, PC10) coming with a specific Haswell (family 0x45) CPU.
 */

/* C source was guarded by: defined(__i386__) || defined(__x86_64__) */

use core::ffi::{c_char, c_double, c_int, c_uint, c_ulonglong, c_void};

const MSR_PKG_C8_RESIDENCY: c_int = 0x00000630;
const MSR_PKG_C9_RESIDENCY: c_int = 0x00000631;
const MSR_PKG_C10_RESIDENCY: c_int = 0x00000632;

const MSR_TSC: c_int = 0x10;

const PC8: c_uint = 0;
const PC9: c_uint = 1;
const PC10: c_uint = 2;
const HSW_EXT_CSTATE_COUNT: usize = 3;
const TSC: c_uint = 0xFFFF;

const RANGE_PACKAGE: c_int = 0;
const X86_VENDOR_INTEL: c_int = 1;

#[repr(C)]
pub struct cstate_t {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub id: c_uint,
    pub range: c_int,
    pub get_count_percent:
        Option<unsafe extern "C" fn(c_uint, *mut c_double, c_uint) -> c_int>,
}

#[repr(C)]
pub struct cpuidle_monitor_flags {
    pub needs_root: c_int,
}

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

#[repr(C)]
pub struct cpupower_cpu_info_t {
    pub vendor: c_int,
    pub family: c_int,
    pub model: c_int,
}

unsafe extern "C" {
    static mut cpu_count: c_int;
    static mut base_cpu: c_uint;
    static mut cpupower_cpu_info: cpupower_cpu_info_t;

    fn read_msr(cpu: c_uint, msr: c_int, val: *mut c_ulonglong) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn dprint(fmt: *const c_char, ...);
}

static mut HSW_EXT_CSTATES: [cstate_t; HSW_EXT_CSTATE_COUNT] = [
    cstate_t {
        name: b"PC8\0".as_ptr() as *const c_char,
        desc: b"Processor Package C8\0".as_ptr() as *const c_char,
        id: PC8,
        range: RANGE_PACKAGE,
        get_count_percent: Some(hsw_ext_get_count_percent),
    },
    cstate_t {
        name: b"PC9\0".as_ptr() as *const c_char,
        desc: b"Processor Package C9\0".as_ptr() as *const c_char,
        id: PC9,
        range: RANGE_PACKAGE,
        get_count_percent: Some(hsw_ext_get_count_percent),
    },
    cstate_t {
        name: b"PC10\0".as_ptr() as *const c_char,
        desc: b"Processor Package C10\0".as_ptr() as *const c_char,
        id: PC10,
        range: RANGE_PACKAGE,
        get_count_percent: Some(hsw_ext_get_count_percent),
    },
];

static mut TSC_AT_MEASURE_START: c_ulonglong = 0;
static mut TSC_AT_MEASURE_END: c_ulonglong = 0;
static mut PREVIOUS_COUNT: [*mut c_ulonglong; HSW_EXT_CSTATE_COUNT] =
    [core::ptr::null_mut(); HSW_EXT_CSTATE_COUNT];
static mut CURRENT_COUNT: [*mut c_ulonglong; HSW_EXT_CSTATE_COUNT] =
    [core::ptr::null_mut(); HSW_EXT_CSTATE_COUNT];
/* valid flag for all CPUs. If a MSR read failed it will be zero */
static mut IS_VALID: *mut c_int = core::ptr::null_mut();

unsafe fn hsw_ext_get_count(
    id: c_uint,
    val: *mut c_ulonglong,
    cpu: c_uint,
) -> c_int {
    let msr: c_int;

    match id {
        PC8 => {
            msr = MSR_PKG_C8_RESIDENCY;
        }
        PC9 => {
            msr = MSR_PKG_C9_RESIDENCY;
        }
        PC10 => {
            msr = MSR_PKG_C10_RESIDENCY;
        }
        TSC => {
            msr = MSR_TSC;
        }
        _ => {
            return -1;
        }
    }
    if read_msr(cpu, msr, val) != 0 {
        return -1;
    }
    0
}

unsafe extern "C" fn hsw_ext_get_count_percent(
    id: c_uint,
    percent: *mut c_double,
    cpu: c_uint,
) -> c_int {
    *percent = 0.0;

    if *IS_VALID.add(cpu as usize) == 0 {
        return -1;
    }

    *percent = (100.0
        * (*CURRENT_COUNT[id as usize].add(cpu as usize)
            - *PREVIOUS_COUNT[id as usize].add(cpu as usize)) as c_double)
        / (TSC_AT_MEASURE_END - TSC_AT_MEASURE_START) as c_double;

    dprint(
        b"%s: previous: %llu - current: %llu - (%u)\n\0".as_ptr() as *const c_char,
        HSW_EXT_CSTATES[id as usize].name,
        *PREVIOUS_COUNT[id as usize].add(cpu as usize),
        *CURRENT_COUNT[id as usize].add(cpu as usize),
        cpu,
    );

    dprint(
        b"%s: tsc_diff: %llu - count_diff: %llu - percent: %2.f (%u)\n\0".as_ptr()
            as *const c_char,
        HSW_EXT_CSTATES[id as usize].name,
        TSC_AT_MEASURE_END - TSC_AT_MEASURE_START,
        *CURRENT_COUNT[id as usize].add(cpu as usize)
            - *PREVIOUS_COUNT[id as usize].add(cpu as usize),
        *percent,
        cpu,
    );

    0
}

unsafe extern "C" fn hsw_ext_start() -> c_int {
    let mut num: c_int;
    let mut cpu: c_int;
    let mut val: c_ulonglong = 0;

    num = 0;
    while num < HSW_EXT_CSTATE_COUNT as c_int {
        cpu = 0;
        while cpu < cpu_count {
            *IS_VALID.add(cpu as usize) =
                (hsw_ext_get_count(num as c_uint, &mut val, cpu as c_uint) == 0) as c_int;
            *PREVIOUS_COUNT[num as usize].add(cpu as usize) = val;
            cpu += 1;
        }
        num += 1;
    }
    hsw_ext_get_count(TSC, &mut TSC_AT_MEASURE_START, base_cpu);
    0
}

unsafe extern "C" fn hsw_ext_stop() -> c_int {
    let mut val: c_ulonglong = 0;
    let mut num: c_int;
    let mut cpu: c_int;

    hsw_ext_get_count(TSC, &mut TSC_AT_MEASURE_END, base_cpu);

    num = 0;
    while num < HSW_EXT_CSTATE_COUNT as c_int {
        cpu = 0;
        while cpu < cpu_count {
            *IS_VALID.add(cpu as usize) |=
                (hsw_ext_get_count(num as c_uint, &mut val, cpu as c_uint) == 0) as c_int;
            *CURRENT_COUNT[num as usize].add(cpu as usize) = val;
            cpu += 1;
        }
        num += 1;
    }
    0
}

unsafe extern "C" fn hsw_ext_register() -> *mut cpuidle_monitor {
    let mut num: c_int;

    if cpupower_cpu_info.vendor != X86_VENDOR_INTEL || cpupower_cpu_info.family != 6 {
        return core::ptr::null_mut();
    }

    match cpupower_cpu_info.model {
        0x45 => {
            /* HSW */
        }
        _ => {
            return core::ptr::null_mut();
        }
    }

    IS_VALID = calloc(cpu_count as usize, core::mem::size_of::<c_int>()) as *mut c_int;
    num = 0;
    while num < HSW_EXT_CSTATE_COUNT as c_int {
        PREVIOUS_COUNT[num as usize] =
            calloc(cpu_count as usize, core::mem::size_of::<c_ulonglong>()) as *mut c_ulonglong;
        CURRENT_COUNT[num as usize] =
            calloc(cpu_count as usize, core::mem::size_of::<c_ulonglong>()) as *mut c_ulonglong;
        num += 1;
    }
    INTEL_HSW_EXT_MONITOR.name_len = strlen(INTEL_HSW_EXT_MONITOR.name);
    &mut INTEL_HSW_EXT_MONITOR
}

#[no_mangle]
pub unsafe extern "C" fn hsw_ext_unregister() {
    let mut num: c_int;
    free(IS_VALID as *mut c_void);
    num = 0;
    while num < HSW_EXT_CSTATE_COUNT as c_int {
        free(PREVIOUS_COUNT[num as usize] as *mut c_void);
        free(CURRENT_COUNT[num as usize] as *mut c_void);
        num += 1;
    }
}

#[no_mangle]
pub static mut INTEL_HSW_EXT_MONITOR: cpuidle_monitor = cpuidle_monitor {
    name: b"HaswellExtended\0".as_ptr() as *const c_char,
    name_len: 0,
    hw_states: unsafe { HSW_EXT_CSTATES.as_mut_ptr() },
    hw_states_num: HSW_EXT_CSTATE_COUNT as c_uint,
    start: Some(hsw_ext_start),
    stop: Some(hsw_ext_stop),
    do_register: Some(hsw_ext_register),
    unregister: Some(hsw_ext_unregister),
    flags: cpuidle_monitor_flags { needs_root: 1 },
    overflow_s: 922000000, /* 922337203 seconds TSC overflow
                            * at 20GHz */
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
