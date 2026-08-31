// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
 *
 *  Based on Len Brown's <lenb@kernel.org> turbostat tool.
 */

/* Original C source was compiled only for defined(__i386__) || defined(__x86_64__). */

use core::ffi::{c_char, c_double, c_int, c_uint, c_ulonglong, c_void};

const MSR_PKG_C3_RESIDENCY: c_int = 0x3F8;
const MSR_PKG_C6_RESIDENCY: c_int = 0x3F9;
const MSR_CORE_C3_RESIDENCY: c_int = 0x3FC;
const MSR_CORE_C6_RESIDENCY: c_int = 0x3FD;

const MSR_TSC: c_int = 0x10;

const NHM_CSTATE_COUNT: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum intel_nhm_id {
    C3 = 0,
    C6,
    PC3,
    PC6,
    TSC = 0xFFFF,
}

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
#[derive(Copy, Clone)]
pub struct cpuidle_monitor_flags {
    pub needs_root: c_int,
}

#[repr(C)]
pub struct cpuidle_monitor {
    pub name: *const c_char,
    pub name_len: usize,
    pub hw_states_num: c_uint,
    pub hw_states: *mut cstate_t,
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
    pub caps: c_uint,
}

unsafe extern "C" {
    static mut base_cpu: c_uint;
    static mut cpu_count: c_int;
    static mut cpupower_cpu_info: cpupower_cpu_info_t;

    static X86_VENDOR_INTEL: c_int;
    static CPUPOWER_CAP_INV_TSC: c_uint;
    static CPUPOWER_CAP_APERF: c_uint;
    static RANGE_CORE: c_int;
    static RANGE_PACKAGE: c_int;

    fn read_msr(cpu: c_uint, msr: c_int, val: *mut c_ulonglong) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn dprint(fmt: *const c_char, ...);
}

unsafe extern "C" fn nhm_get_count_percent(
    self_id: c_uint,
    percent: *mut c_double,
    cpu: c_uint,
) -> c_int {
    let _ = self_id;
    nhm_get_count_percent_impl(self_id, percent, cpu)
}

static mut nhm_cstates: [cstate_t; NHM_CSTATE_COUNT] = [
    cstate_t {
        name: b"C3\0".as_ptr() as *const c_char,
        desc: b"Processor Core C3\0".as_ptr() as *const c_char,
        id: intel_nhm_id::C3 as c_uint,
        range: unsafe { RANGE_CORE },
        get_count_percent: Some(nhm_get_count_percent),
    },
    cstate_t {
        name: b"C6\0".as_ptr() as *const c_char,
        desc: b"Processor Core C6\0".as_ptr() as *const c_char,
        id: intel_nhm_id::C6 as c_uint,
        range: unsafe { RANGE_CORE },
        get_count_percent: Some(nhm_get_count_percent),
    },
    cstate_t {
        name: b"PC3\0".as_ptr() as *const c_char,
        desc: b"Processor Package C3\0".as_ptr() as *const c_char,
        id: intel_nhm_id::PC3 as c_uint,
        range: unsafe { RANGE_PACKAGE },
        get_count_percent: Some(nhm_get_count_percent),
    },
    cstate_t {
        name: b"PC6\0".as_ptr() as *const c_char,
        desc: b"Processor Package C6\0".as_ptr() as *const c_char,
        id: intel_nhm_id::PC6 as c_uint,
        range: unsafe { RANGE_PACKAGE },
        get_count_percent: Some(nhm_get_count_percent),
    },
];

static mut tsc_at_measure_start: c_ulonglong = 0;
static mut tsc_at_measure_end: c_ulonglong = 0;
static mut previous_count: [*mut c_ulonglong; NHM_CSTATE_COUNT] =
    [core::ptr::null_mut(); NHM_CSTATE_COUNT];
static mut current_count: [*mut c_ulonglong; NHM_CSTATE_COUNT] =
    [core::ptr::null_mut(); NHM_CSTATE_COUNT];
/* valid flag for all CPUs. If a MSR read failed it will be zero */
static mut is_valid: *mut c_int = core::ptr::null_mut();

unsafe fn nhm_get_count(id: intel_nhm_id, val: *mut c_ulonglong, cpu: c_uint) -> c_int {
    let msr: c_int;

    match id {
        intel_nhm_id::C3 => {
            msr = MSR_CORE_C3_RESIDENCY;
        }
        intel_nhm_id::C6 => {
            msr = MSR_CORE_C6_RESIDENCY;
        }
        intel_nhm_id::PC3 => {
            msr = MSR_PKG_C3_RESIDENCY;
        }
        intel_nhm_id::PC6 => {
            msr = MSR_PKG_C6_RESIDENCY;
        }
        intel_nhm_id::TSC => {
            msr = MSR_TSC;
        }
    }
    if read_msr(cpu, msr, val) != 0 {
        return -1;
    }

    0
}

unsafe fn nhm_get_count_from_uint(
    id: c_uint,
    val: *mut c_ulonglong,
    cpu: c_uint,
) -> c_int {
    match id {
        0 => nhm_get_count(intel_nhm_id::C3, val, cpu),
        1 => nhm_get_count(intel_nhm_id::C6, val, cpu),
        2 => nhm_get_count(intel_nhm_id::PC3, val, cpu),
        3 => nhm_get_count(intel_nhm_id::PC6, val, cpu),
        0xFFFF => nhm_get_count(intel_nhm_id::TSC, val, cpu),
        _ => -1,
    }
}

unsafe fn nhm_get_count_percent_impl(
    id: c_uint,
    percent: *mut c_double,
    cpu: c_uint,
) -> c_int {
    *percent = 0.0;

    if *is_valid.add(cpu as usize) == 0 {
        return -1;
    }

    *percent = (100.0
        * (*current_count[id as usize].add(cpu as usize)
            - *previous_count[id as usize].add(cpu as usize)) as c_double)
        / (tsc_at_measure_end - tsc_at_measure_start) as c_double;

    dprint(
        b"%s: previous: %llu - current: %llu - (%u)\n\0".as_ptr() as *const c_char,
        nhm_cstates[id as usize].name,
        *previous_count[id as usize].add(cpu as usize),
        *current_count[id as usize].add(cpu as usize),
        cpu,
    );

    dprint(
        b"%s: tsc_diff: %llu - count_diff: %llu - percent: %2.f (%u)\n\0".as_ptr()
            as *const c_char,
        nhm_cstates[id as usize].name,
        (tsc_at_measure_end as c_ulonglong).wrapping_sub(tsc_at_measure_start),
        (*current_count[id as usize].add(cpu as usize))
            .wrapping_sub(*previous_count[id as usize].add(cpu as usize)),
        *percent,
        cpu,
    );

    0
}

unsafe extern "C" fn nhm_start() -> c_int {
    let mut num: c_int;
    let mut cpu: c_int;
    let mut dbg: c_ulonglong = 0;
    let mut val: c_ulonglong = 0;

    nhm_get_count(
        intel_nhm_id::TSC,
        core::ptr::addr_of_mut!(tsc_at_measure_start),
        base_cpu,
    );

    num = 0;
    while num < NHM_CSTATE_COUNT as c_int {
        cpu = 0;
        while cpu < cpu_count {
            *is_valid.add(cpu as usize) =
                (nhm_get_count_from_uint(num as c_uint, &mut val, cpu as c_uint) == 0) as c_int;
            *previous_count[num as usize].add(cpu as usize) = val;
            cpu += 1;
        }
        num += 1;
    }
    nhm_get_count(intel_nhm_id::TSC, &mut dbg, base_cpu);
    dprint(
        b"TSC diff: %llu\n\0".as_ptr() as *const c_char,
        dbg.wrapping_sub(tsc_at_measure_start),
    );
    0
}

unsafe extern "C" fn nhm_stop() -> c_int {
    let mut val: c_ulonglong = 0;
    let mut dbg: c_ulonglong = 0;
    let mut num: c_int;
    let mut cpu: c_int;

    nhm_get_count(
        intel_nhm_id::TSC,
        core::ptr::addr_of_mut!(tsc_at_measure_end),
        base_cpu,
    );

    num = 0;
    while num < NHM_CSTATE_COUNT as c_int {
        cpu = 0;
        while cpu < cpu_count {
            *is_valid.add(cpu as usize) |=
                (nhm_get_count_from_uint(num as c_uint, &mut val, cpu as c_uint) == 0) as c_int;
            *current_count[num as usize].add(cpu as usize) = val;
            cpu += 1;
        }
        num += 1;
    }
    nhm_get_count(intel_nhm_id::TSC, &mut dbg, base_cpu);
    dprint(
        b"TSC diff: %llu\n\0".as_ptr() as *const c_char,
        dbg.wrapping_sub(tsc_at_measure_end),
    );

    0
}

#[no_mangle]
pub static mut intel_nhm_monitor: cpuidle_monitor = cpuidle_monitor {
    name: b"Nehalem\0".as_ptr() as *const c_char,
    name_len: 0,
    hw_states_num: NHM_CSTATE_COUNT as c_uint,
    hw_states: unsafe { core::ptr::addr_of_mut!(nhm_cstates) as *mut cstate_t },
    start: Some(nhm_start),
    stop: Some(nhm_stop),
    do_register: Some(intel_nhm_register),
    unregister: Some(intel_nhm_unregister),
    flags: cpuidle_monitor_flags { needs_root: 1 },
    overflow_s: 922000000, /* 922337203 seconds TSC overflow
                            * at 20GHz */
};

#[no_mangle]
pub unsafe extern "C" fn intel_nhm_register() -> *mut cpuidle_monitor {
    let mut num: c_int;

    if cpupower_cpu_info.vendor != X86_VENDOR_INTEL {
        return core::ptr::null_mut();
    }

    if (cpupower_cpu_info.caps & CPUPOWER_CAP_INV_TSC) == 0 {
        return core::ptr::null_mut();
    }

    if (cpupower_cpu_info.caps & CPUPOWER_CAP_APERF) == 0 {
        return core::ptr::null_mut();
    }

    /* Free this at program termination */
    is_valid = calloc(cpu_count as usize, core::mem::size_of::<c_int>()) as *mut c_int;
    num = 0;
    while num < NHM_CSTATE_COUNT as c_int {
        previous_count[num as usize] = calloc(
            cpu_count as usize,
            core::mem::size_of::<c_ulonglong>(),
        ) as *mut c_ulonglong;
        current_count[num as usize] = calloc(
            cpu_count as usize,
            core::mem::size_of::<c_ulonglong>(),
        ) as *mut c_ulonglong;
        num += 1;
    }

    intel_nhm_monitor.name_len = strlen(intel_nhm_monitor.name);
    core::ptr::addr_of_mut!(intel_nhm_monitor)
}

#[no_mangle]
pub unsafe extern "C" fn intel_nhm_unregister() {
    let mut num: c_int;

    num = 0;
    while num < NHM_CSTATE_COUNT as c_int {
        free(previous_count[num as usize] as *mut c_void);
        free(current_count[num as usize] as *mut c_void);
        num += 1;
    }
    free(is_valid as *mut c_void);
}
