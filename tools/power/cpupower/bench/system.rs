// SPDX-License-Identifier: GPL-2.0-or-later
/*  cpufreq-bench CPUFreq microbenchmark
 *
 *  Copyright (C) 2008 Christian Kornacker <ckornacker@suse.de>
 */

// C dependencies translated from:
// <stdio.h>, <time.h>, <sys/time.h>, <sys/types.h>, <unistd.h>, <sched.h>,
// <cpufreq.h>, <cpupower.h>, "config.h", and "system.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct sched_param {
    pub sched_priority: c_int,
}

#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [usize; 16],
}

#[repr(C)]
pub struct config {
    pub sleep: c_uint,
    pub load: c_uint,
    pub sleep_step: c_uint,
    pub load_step: c_uint,
    pub cycles: c_uint,
    pub rounds: c_uint,
    pub verbose: c_int,
    pub output: *mut FILE,
    pub cpu: c_uint,
    pub prio: c_int,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;

    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
    fn sched_setscheduler(pid: c_int, policy: c_int, param: *const sched_param) -> c_int;

    fn cpupower_is_cpu_online(cpu: c_uint) -> c_int;
    fn cpufreq_modify_policy_governor(cpu: c_uint, governor: *mut c_char) -> c_int;
    fn dprintf(format: *const c_char, ...) -> c_int;
}

// Macro/constants supplied by "config.h" and "system.h" in the original C.
unsafe extern "C" {
    static SCHEDULER: c_int;
    static SCHED_HIGH: c_int;
    static SCHED_LOW: c_int;
    static PRIORITY_HIGH: c_int;
    static PRIORITY_LOW: c_int;
    static PRIORITY_DEFAULT: c_int;
}

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    ptr::write_bytes(set as *mut u8, 0, mem::size_of::<cpu_set_t>());
}

unsafe fn CPU_SET(cpu: c_uint, set: *mut cpu_set_t) {
    let bits_per_word = 8 * mem::size_of::<usize>();
    let idx = cpu as usize / bits_per_word;
    let bit = cpu as usize % bits_per_word;

    if idx < (*set).__bits.len() {
        (*set).__bits[idx] |= 1usize << bit;
    }
}

/**
 * returns time since epoch in us
 *
 * @retval time
 **/
#[no_mangle]
pub unsafe extern "C" fn get_time() -> i64 {
    let mut now: timeval = mem::zeroed();

    gettimeofday(&mut now, ptr::null_mut());

    (now.tv_sec * 1000000i64 as c_long + now.tv_usec) as i64
}

/**
 * sets the cpufreq governor
 *
 * @param governor cpufreq governor name
 * @param cpu cpu for which the governor should be set
 *
 * @retval 0 on success
 * @retval -1 when failed
 **/
#[no_mangle]
pub unsafe extern "C" fn set_cpufreq_governor(governor: *mut c_char, cpu: c_uint) -> c_int {
    dprintf(
        c"set %s as cpufreq governor\n".as_ptr(),
        governor,
    );

    if cpupower_is_cpu_online(cpu) != 1 {
        perror(c"cpufreq_cpu_exists".as_ptr());
        fprintf(
            stderr,
            c"error: cpu %u does not exist\n".as_ptr(),
            cpu,
        );
        return -1;
    }

    if cpufreq_modify_policy_governor(cpu, governor) != 0 {
        perror(c"cpufreq_modify_policy_governor".as_ptr());
        fprintf(
            stderr,
            c"error: unable to set %s governor\n".as_ptr(),
            governor,
        );
        return -1;
    }

    0
}

/**
 * sets cpu affinity for the process
 *
 * @param cpu cpu# to which the affinity should be set
 *
 * @retval 0 on success
 * @retval -1 when setting the affinity failed
 **/
#[no_mangle]
pub unsafe extern "C" fn set_cpu_affinity(cpu: c_uint) -> c_int {
    let mut cpuset: cpu_set_t = mem::zeroed();

    CPU_ZERO(&mut cpuset);
    CPU_SET(cpu, &mut cpuset);

    dprintf(c"set affinity to cpu #%u\n".as_ptr(), cpu);

    if sched_setaffinity(getpid(), mem::size_of::<cpu_set_t>(), &cpuset) < 0 {
        perror(c"sched_setaffinity".as_ptr());
        fprintf(stderr, c"warning: unable to set cpu affinity\n".as_ptr());
        return -1;
    }

    0
}

/**
 * sets the process priority parameter
 *
 * @param priority priority value
 *
 * @retval 0 on success
 * @retval -1 when setting the priority failed
 **/
#[no_mangle]
pub unsafe extern "C" fn set_process_priority(priority: c_int) -> c_int {
    let mut param: sched_param = mem::zeroed();

    dprintf(c"set scheduler priority to %i\n".as_ptr(), priority);

    param.sched_priority = priority;

    if sched_setscheduler(0, SCHEDULER, &param) < 0 {
        perror(c"sched_setscheduler".as_ptr());
        fprintf(
            stderr,
            c"warning: unable to set scheduler priority\n".as_ptr(),
        );
        return -1;
    }

    0
}

/**
 * notifies the user that the benchmark may run some time
 *
 * @param config benchmark config values
 *
 **/
#[no_mangle]
pub unsafe extern "C" fn prepare_user(config: *const config) {
    let mut sleep_time: c_long = 0;
    let mut load_time: c_long = 0;
    let mut round: c_uint;

    round = 0;
    while round < (*config).rounds {
        sleep_time += (2 * (*config).cycles * ((*config).sleep + (*config).sleep_step * round)) as c_long;
        load_time += (2 * (*config).cycles * ((*config).load + (*config).load_step * round)
            + ((*config).load + (*config).load_step * round * 4)) as c_long;
        round += 1;
    }

    if (*config).verbose != 0 || (*config).output != stdout {
        printf(
            c"approx. test duration: %im\n".as_ptr(),
            ((sleep_time + load_time) / 60000000) as c_int,
        );
    }
}

/**
 * sets up the cpu affinity and scheduler priority
 *
 * @param config benchmark config values
 *
 **/
#[no_mangle]
pub unsafe extern "C" fn prepare_system(config: *const config) {
    if (*config).verbose != 0 {
        printf(c"set cpu affinity to cpu #%u\n".as_ptr(), (*config).cpu);
    }

    set_cpu_affinity((*config).cpu);

    if (*config).prio == SCHED_HIGH {
        if (*config).verbose != 0 {
            printf(c"high priority condition requested\n".as_ptr());
        }

        set_process_priority(PRIORITY_HIGH);
    } else if (*config).prio == SCHED_LOW {
        if (*config).verbose != 0 {
            printf(c"low priority condition requested\n".as_ptr());
        }

        set_process_priority(PRIORITY_LOW);
    } else {
        if (*config).verbose != 0 {
            printf(c"default priority condition requested\n".as_ptr());
        }

        set_process_priority(PRIORITY_DEFAULT);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
