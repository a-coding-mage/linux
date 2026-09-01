// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc
 */

use core::ffi::{c_char, c_double, c_int, c_uint, c_ulonglong, c_void};

const CPUIDLE_STATES_MAX: usize = 10;

const CLOCK_REALTIME: c_int = 0;
const UINT_MAX: c_uint = c_uint::MAX;

extern "C" {
    static cpu_count: c_int;

    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn cpuidle_state_time(cpu: c_int, state: c_int) -> c_ulonglong;
    fn cpuidle_state_count(cpu: c_int) -> c_int;
    fn cpuidle_state_name(cpu: c_int, state: c_int) -> *mut c_char;
    fn cpuidle_state_desc(cpu: c_int, state: c_int) -> *mut c_char;
    fn dprint(fmt: *const c_char, ...);
    fn free(ptr: *mut c_void);
    fn malloc(size: usize) -> *mut c_void;
    fn sched_getcpu() -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn timespec_diff_us(start: timespec, end: timespec) -> c_ulonglong;
}

extern "C" {
    static CSTATE_NAME_LEN: usize;
    static CSTATE_DESC_LEN: usize;
    static RANGE_THREAD: c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: isize,
    pub tv_nsec: isize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstate_t {
    pub name: [c_char; CSTATE_NAME_LEN],
    pub desc: [c_char; CSTATE_DESC_LEN],
    pub range: c_int,
    pub id: c_int,
    pub get_count_percent:
        Option<unsafe extern "C" fn(id: c_uint, percent: *mut c_double, cpu: c_uint) -> c_int>,
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
    pub hw_states_num: c_int,
    pub hw_states: *mut cstate_t,
    pub start: Option<unsafe extern "C" fn() -> c_int>,
    pub stop: Option<unsafe extern "C" fn() -> c_int>,
    pub do_register: Option<unsafe extern "C" fn() -> *mut cpuidle_monitor>,
    pub unregister: Option<unsafe extern "C" fn()>,
    pub flags: cpuidle_monitor_flags,
    pub overflow_s: c_uint,
}

static mut cpuidle_cstates: [cstate_t; CPUIDLE_STATES_MAX] = [cstate_t {
    name: [0; CSTATE_NAME_LEN],
    desc: [0; CSTATE_DESC_LEN],
    range: 0,
    id: 0,
    get_count_percent: None,
}; CPUIDLE_STATES_MAX];

static mut previous_count: *mut *mut c_ulonglong = core::ptr::null_mut();
static mut current_count: *mut *mut c_ulonglong = core::ptr::null_mut();
static mut start_time: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut timediff: c_ulonglong = 0;

unsafe extern "C" fn cpuidle_get_count_percent(
    id: c_uint,
    percent: *mut c_double,
    cpu: c_uint,
) -> c_int {
    let statediff = *(*current_count.add(cpu as usize)).add(id as usize)
        - *(*previous_count.add(cpu as usize)).add(id as usize);
    dprint(
        b"%s: - diff: %llu - percent: %f (%u)\n\0".as_ptr() as *const c_char,
        cpuidle_cstates[id as usize].name.as_ptr(),
        timediff,
        *percent,
        cpu,
    );

    if timediff == 0 {
        *percent = 0.0;
    } else {
        *percent = (100.0 * statediff as c_double) / timediff as c_double;
    }

    dprint(
        b"%s: - timediff: %llu - statediff: %llu - percent: %f (%u)\n\0".as_ptr()
            as *const c_char,
        cpuidle_cstates[id as usize].name.as_ptr(),
        timediff,
        statediff,
        *percent,
        cpu,
    );

    0
}

unsafe extern "C" fn cpuidle_start() -> c_int {
    let mut cpu: c_int;
    let mut state: c_int;

    clock_gettime(CLOCK_REALTIME, core::ptr::addr_of_mut!(start_time));
    cpu = 0;
    while cpu < cpu_count {
        state = 0;
        while state < cpuidle_sysfs_monitor.hw_states_num {
            *(*previous_count.add(cpu as usize)).add(state as usize) =
                cpuidle_state_time(cpu, state);
            dprint(
                b"CPU %d - State: %d - Val: %llu\n\0".as_ptr() as *const c_char,
                cpu,
                state,
                *(*previous_count.add(cpu as usize)).add(state as usize),
            );
            state += 1;
        }
        cpu += 1;
    }
    0
}

unsafe extern "C" fn cpuidle_stop() -> c_int {
    let mut cpu: c_int;
    let mut state: c_int;
    let mut end_time = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    clock_gettime(CLOCK_REALTIME, &mut end_time);
    timediff = timespec_diff_us(start_time, end_time);

    cpu = 0;
    while cpu < cpu_count {
        state = 0;
        while state < cpuidle_sysfs_monitor.hw_states_num {
            *(*current_count.add(cpu as usize)).add(state as usize) =
                cpuidle_state_time(cpu, state);
            dprint(
                b"CPU %d - State: %d - Val: %llu\n\0".as_ptr() as *const c_char,
                cpu,
                state,
                *(*current_count.add(cpu as usize)).add(state as usize),
            );
            state += 1;
        }
        cpu += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn fix_up_intel_idle_driver_name(tmp: *mut c_char, num: c_int) {
    /* fix up cpuidle name for intel idle driver */
    if strncmp(tmp, b"NHM-\0".as_ptr() as *const c_char, 4) == 0 {
        match num {
            1 => {
                strcpy(tmp, b"C1\0".as_ptr() as *const c_char);
            }
            2 => {
                strcpy(tmp, b"C3\0".as_ptr() as *const c_char);
            }
            3 => {
                strcpy(tmp, b"C6\0".as_ptr() as *const c_char);
            }
            _ => {}
        }
    } else if strncmp(tmp, b"SNB-\0".as_ptr() as *const c_char, 4) == 0 {
        match num {
            1 => {
                strcpy(tmp, b"C1\0".as_ptr() as *const c_char);
            }
            2 => {
                strcpy(tmp, b"C3\0".as_ptr() as *const c_char);
            }
            3 => {
                strcpy(tmp, b"C6\0".as_ptr() as *const c_char);
            }
            4 => {
                strcpy(tmp, b"C7\0".as_ptr() as *const c_char);
            }
            _ => {}
        }
    } else if strncmp(tmp, b"ATM-\0".as_ptr() as *const c_char, 4) == 0 {
        match num {
            1 => {
                strcpy(tmp, b"C1\0".as_ptr() as *const c_char);
            }
            2 => {
                strcpy(tmp, b"C2\0".as_ptr() as *const c_char);
            }
            3 => {
                strcpy(tmp, b"C4\0".as_ptr() as *const c_char);
            }
            4 => {
                strcpy(tmp, b"C6\0".as_ptr() as *const c_char);
            }
            _ => {}
        }
    }
}

/*
 * Original C conditional:
 * #ifdef __powerpc__
 */
#[cfg(target_arch = "powerpc")]
#[no_mangle]
pub unsafe extern "C" fn map_power_idle_state_name(tmp: *mut c_char) {
    if strncmp(tmp, b"stop0_lite\0".as_ptr() as *const c_char, CSTATE_NAME_LEN) == 0 {
        strcpy(tmp, b"stop0L\0".as_ptr() as *const c_char);
    } else if strncmp(
        tmp,
        b"stop1_lite\0".as_ptr() as *const c_char,
        CSTATE_NAME_LEN,
    ) == 0
    {
        strcpy(tmp, b"stop1L\0".as_ptr() as *const c_char);
    } else if strncmp(
        tmp,
        b"stop2_lite\0".as_ptr() as *const c_char,
        CSTATE_NAME_LEN,
    ) == 0
    {
        strcpy(tmp, b"stop2L\0".as_ptr() as *const c_char);
    }
}

#[cfg(not(target_arch = "powerpc"))]
#[no_mangle]
pub unsafe extern "C" fn map_power_idle_state_name(_tmp: *mut c_char) {}

unsafe extern "C" fn cpuidle_register() -> *mut cpuidle_monitor {
    let mut num: c_int;
    let mut tmp: *mut c_char;
    let this_cpu: c_int;

    this_cpu = sched_getcpu();

    /* Assume idle state count is the same for all CPUs */
    cpuidle_sysfs_monitor.hw_states_num = cpuidle_state_count(this_cpu);

    if cpuidle_sysfs_monitor.hw_states_num <= 0 {
        return core::ptr::null_mut();
    }

    num = 0;
    while num < cpuidle_sysfs_monitor.hw_states_num {
        tmp = cpuidle_state_name(this_cpu, num);
        if tmp.is_null() {
            num += 1;
            continue;
        }

        map_power_idle_state_name(tmp);
        fix_up_intel_idle_driver_name(tmp, num);
        strncpy(
            cpuidle_cstates[num as usize].name.as_mut_ptr(),
            tmp,
            CSTATE_NAME_LEN - 1,
        );
        free(tmp as *mut c_void);

        tmp = cpuidle_state_desc(this_cpu, num);
        if tmp.is_null() {
            num += 1;
            continue;
        }
        strncpy(
            cpuidle_cstates[num as usize].desc.as_mut_ptr(),
            tmp,
            CSTATE_DESC_LEN - 1,
        );
        free(tmp as *mut c_void);

        cpuidle_cstates[num as usize].range = RANGE_THREAD;
        cpuidle_cstates[num as usize].id = num;
        cpuidle_cstates[num as usize].get_count_percent = Some(cpuidle_get_count_percent);

        num += 1;
    }

    /* Free this at program termination */
    previous_count = malloc(core::mem::size_of::<*mut c_ulonglong>() * cpu_count as usize)
        as *mut *mut c_ulonglong;
    current_count = malloc(core::mem::size_of::<*mut c_ulonglong>() * cpu_count as usize)
        as *mut *mut c_ulonglong;
    num = 0;
    while num < cpu_count {
        *previous_count.add(num as usize) = malloc(
            core::mem::size_of::<c_ulonglong>() * cpuidle_sysfs_monitor.hw_states_num as usize,
        ) as *mut c_ulonglong;
        *current_count.add(num as usize) = malloc(
            core::mem::size_of::<c_ulonglong>() * cpuidle_sysfs_monitor.hw_states_num as usize,
        ) as *mut c_ulonglong;
        num += 1;
    }

    cpuidle_sysfs_monitor.name_len = strlen(cpuidle_sysfs_monitor.name);
    core::ptr::addr_of_mut!(cpuidle_sysfs_monitor)
}

#[no_mangle]
pub unsafe extern "C" fn cpuidle_unregister() {
    let mut num: c_int;

    num = 0;
    while num < cpu_count {
        free(*previous_count.add(num as usize) as *mut c_void);
        free(*current_count.add(num as usize) as *mut c_void);
        num += 1;
    }
    free(previous_count as *mut c_void);
    free(current_count as *mut c_void);
}

#[no_mangle]
pub static mut cpuidle_sysfs_monitor: cpuidle_monitor = cpuidle_monitor {
    name: b"Idle_Stats\0".as_ptr() as *const c_char,
    hw_states: core::ptr::addr_of_mut!(cpuidle_cstates) as *mut cstate_t,
    start: Some(cpuidle_start),
    stop: Some(cpuidle_stop),
    do_register: Some(cpuidle_register),
    unregister: Some(cpuidle_unregister),
    flags: cpuidle_monitor_flags { needs_root: 0 },
    overflow_s: UINT_MAX,
    name_len: 0,
    hw_states_num: 0,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
