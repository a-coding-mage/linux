/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Common functions for in-kernel torture tests.
 *
 * Copyright IBM Corporation, 2014
 *
 * Author: Paul E. McKenney <paulmck@linux.ibm.com>
 */

/* C header dependencies are supplied by the surrounding kernel translation. */

/* Definitions for a non-string torture-test module parameter. */
macro_rules! torture_param {
    ($type:ty, $name:ident, $init:expr, $msg:expr) => {
        static mut $name: $type = $init;
        module_param!($name, $type, 0o444);
        module_param_desc!($name, $msg);
    };
}

const TORTURE_FLAG: &str = "-torture:";

macro_rules! TOROUT_STRING {
    ($s:expr) => {
        pr_alert!("{}{} {}\n", torture_type, TORTURE_FLAG, $s)
    };
}

macro_rules! VERBOSE_TOROUT_STRING {
    ($s:expr) => {{
        if verbose {
            unsafe { verbose_torout_sleep(); }
            pr_alert!("{}{} {}\n", torture_type, TORTURE_FLAG, $s);
        }
    }};
}

macro_rules! TOROUT_ERRSTRING {
    ($s:expr) => {
        pr_alert!("{}{}!!! {}\n", torture_type, TORTURE_FLAG, $s)
    };
}

extern "C" {
    pub fn verbose_torout_sleep();
}

macro_rules! torture_init_error {
    ($firsterr:expr) => {{
        let ___firsterr: i32 = $firsterr;
        WARN_ONCE!(
            !IS_MODULE!(CONFIG_RCU_TORTURE_TEST) && ___firsterr < 0,
            "Torture-test initialization failed with error code %d\n",
            ___firsterr
        );
        ___firsterr < 0
    }};
}

/* Definitions for online/offline exerciser. */
/* CONFIG_HOTPLUG_CPU selects the external implementation when enabled. */
#[cfg(CONFIG_HOTPLUG_CPU)]
extern "C" {
    pub fn torture_num_online_cpus() -> i32;
}

#[cfg(not(CONFIG_HOTPLUG_CPU))]
#[inline]
pub fn torture_num_online_cpus() -> i32 { 1 }

pub type torture_ofl_func = unsafe extern "C" fn();

extern "C" {
    pub fn torture_offline(
        cpu: i32,
        n_onl_attempts: *mut kernel::ffi::c_long,
        n_onl_successes: *mut kernel::ffi::c_long,
        sum_offl: *mut kernel::ffi::c_ulong,
        min_onl: *mut i32,
        max_onl: *mut i32,
    ) -> bool;
    pub fn torture_online(
        cpu: i32,
        n_onl_attempts: *mut kernel::ffi::c_long,
        n_onl_successes: *mut kernel::ffi::c_long,
        sum_onl: *mut kernel::ffi::c_ulong,
        min_onl: *mut i32,
        max_onl: *mut i32,
    ) -> bool;
    pub fn torture_onoff_init(ooholdoff: kernel::ffi::c_long, oointerval: kernel::ffi::c_long, f: torture_ofl_func) -> i32;
    pub fn torture_onoff_stats();
    pub fn torture_onoff_failures() -> bool;

    pub fn torture_random(trsp: *mut torture_random_state) -> kernel::ffi::c_ulong;
    pub fn torture_hrtimeout_ns(baset_ns: ktime_t, fuzzt_ns: u32, mode: hrtimer_mode, trsp: *mut torture_random_state) -> i32;
    pub fn torture_hrtimeout_us(baset_us: u32, fuzzt_ns: u32, trsp: *mut torture_random_state) -> i32;
    pub fn torture_hrtimeout_ms(baset_ms: u32, fuzzt_us: u32, trsp: *mut torture_random_state) -> i32;
    pub fn torture_hrtimeout_jiffies(baset_j: u32, trsp: *mut torture_random_state) -> i32;
    pub fn torture_hrtimeout_s(baset_s: u32, fuzzt_ms: u32, trsp: *mut torture_random_state) -> i32;

    pub fn torture_shuffle_task_register(tp: *mut task_struct);
    pub fn torture_shuffle_init(shuffint: kernel::ffi::c_long) -> i32;
    pub fn torture_shutdown_absorb(title: *const kernel::ffi::c_char);
    pub fn torture_shutdown_init(ssecs: i32, cleanup: Option<unsafe extern "C" fn()>) -> i32;
    pub fn stutter_wait(title: *const kernel::ffi::c_char) -> bool;
    pub fn torture_stutter_init(s: i32, sgap: i32) -> i32;
    pub fn torture_init_begin(ttype: *mut kernel::ffi::c_char, v: i32) -> bool;
    pub fn torture_init_end();
    pub fn get_torture_init_jiffies() -> kernel::ffi::c_ulong;
    pub fn torture_cleanup_begin() -> bool;
    pub fn torture_cleanup_end();
    pub fn torture_must_stop() -> bool;
    pub fn torture_must_stop_irq() -> bool;
    pub fn torture_kthread_stopping(title: *mut kernel::ffi::c_char);
    pub fn _torture_create_kthread(fn_: Option<unsafe extern "C" fn(*mut kernel::ffi::c_void) -> i32>, arg: *mut kernel::ffi::c_void, s: *mut kernel::ffi::c_char, m: *mut kernel::ffi::c_char, f: *mut kernel::ffi::c_char, tp: *mut *mut task_struct, cbf: Option<unsafe extern "C" fn(*mut task_struct)>) -> i32;
    pub fn _torture_stop_kthread(m: *mut kernel::ffi::c_char, tp: *mut *mut task_struct);
    pub fn torture_sched_set_normal(t: *mut task_struct, nice: i32);
}

#[repr(C)]
pub struct torture_random_state {
    pub trs_state: kernel::ffi::c_ulong,
    pub trs_count: kernel::ffi::c_long,
}

macro_rules! DEFINE_TORTURE_RANDOM {
    ($name:ident) => { let mut $name = torture_random_state { trs_state: 0, trs_count: 0 }; };
}
macro_rules! DEFINE_TORTURE_RANDOM_PERCPU {
    ($name:ident) => { DEFINE_PER_CPU!(torture_random_state, $name); };
}

#[inline]
pub unsafe fn torture_random_init(trsp: *mut torture_random_state) {
    (*trsp).trs_state = 0;
    (*trsp).trs_count = 0;
}

macro_rules! torture_create_kthread {
    ($n:ident, $arg:expr, $tp:ident) => {
        _torture_create_kthread(Some($n), $arg, concat!(stringify!($n), "\0").as_ptr() as *mut _, concat!("Creating ", stringify!($n), " task").as_ptr() as *mut _, concat!("Failed to create ", stringify!($n)).as_ptr() as *mut _, &mut $tp, None)
    };
}
macro_rules! torture_create_kthread_cb { ($n:ident, $arg:expr, $tp:ident, $cbf:expr) => { _torture_create_kthread(Some($n), $arg, concat!(stringify!($n), "\0").as_ptr() as *mut _, concat!("Creating ", stringify!($n), " task").as_ptr() as *mut _, concat!("Failed to create ", stringify!($n)).as_ptr() as *mut _, &mut $tp, $cbf) }; }
macro_rules! torture_stop_kthread { ($n:ident, $tp:ident) => { _torture_stop_kthread(concat!("Stopping ", stringify!($n), " task").as_ptr() as *mut _, &mut $tp) }; }

/* Scheduler-related definitions. */
/* CONFIG_PREEMPTION selects __preempt_schedule; otherwise this is a no-op. */
#[cfg(CONFIG_PREEMPTION)]
macro_rules! torture_preempt_schedule { () => { __preempt_schedule!() }; }
#[cfg(not(CONFIG_PREEMPTION))]
macro_rules! torture_preempt_schedule { () => {{}}; }

#[cfg(any(CONFIG_RCU_TORTURE_TEST, CONFIG_LOCK_TORTURE_TEST))]
extern "C" {
    pub fn torture_sched_setaffinity(pid: pid_t, in_mask: *const cpumask, dowarn: bool) -> kernel::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
