// SPDX-License-Identifier: GPL-2.0+
/* Common functions for in-kernel torture tests. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Kernel headers, module metadata, and EXPORT_SYMBOL_GPL directives are supplied
// by the surrounding kernel build and are intentionally represented by externals.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type bool_t = bool;
type ktime_t = i64;
type u32_t = u32;
type torture_ofl_func = unsafe extern "C" fn();

#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct torture_random_state { pub trs_state: c_ulong, pub trs_count: c_long }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
type cpumask_var_t = *mut c_void;
type hrtimer_mode = c_int;

const FULLSTOP_DONTSTOP: c_int = 0;
const FULLSTOP_SHUTDOWN: c_int = 1;
const FULLSTOP_RMMOD: c_int = 2;
const TORTURE_RANDOM_MULT: c_ulong = 39916801;
const TORTURE_RANDOM_ADD: c_ulong = 479001701;
const TORTURE_RANDOM_REFRESH: c_long = 10000;

static mut disable_onoff_at_boot: bool = false;
static mut ftrace_dump_at_shutdown: bool = false;
static mut verbose_sleep_frequency: c_int = 0;
static mut verbose_sleep_duration: c_int = 1;
static mut random_shuffle: c_int = 0;
static mut torture_type: *mut c_char = core::ptr::null_mut();
static mut verbose: c_int = 0;
static mut fullstop: c_int = FULLSTOP_RMMOD;
static mut verbose_sleep_counter: c_int = 0;

extern "C" {
    fn atomic_inc_return(v: *mut c_int) -> c_int;
    fn schedule_timeout_uninterruptible(x: c_long) -> c_long;
    fn schedule_timeout_interruptible(x: c_long) -> c_long;
    fn set_current_state(x: c_int);
    fn schedule_hrtimeout(t: *mut ktime_t, mode: hrtimer_mode) -> c_int;
    fn jiffies_to_nsecs(x: c_ulong) -> ktime_t;
    fn ktime_get() -> ktime_t;
    fn ktime_add(x: ktime_t, y: ktime_t) -> ktime_t;
    fn ktime_add_ns(x: ktime_t, y: ktime_t) -> ktime_t;
    fn ktime_set(sec: c_long, nsec: c_long) -> ktime_t;
    fn ktime_before(x: ktime_t, y: ktime_t) -> bool;
    fn torture_shutdown_absorb(x: *const c_char);
    fn torture_kthread_stopping(x: *mut c_char);
    fn kthread_should_stop() -> bool;
    fn local_clock() -> c_ulong;
    fn raw_smp_processor_id() -> c_ulong;
    fn swahw32(x: c_ulong) -> c_ulong;
    fn cond_resched_tasks_rcu_qs();
    fn rcu_ftrace_dump(x: c_int);
    fn kernel_power_off();
    fn register_reboot_notifier(x: *mut notifier_block) -> c_int;
    fn unregister_reboot_notifier(x: *mut notifier_block) -> c_int;
    fn torture_create_kthread(f: unsafe extern "C" fn(*mut c_void) -> c_int, a: *mut c_void, t: *mut *mut task_struct) -> c_int;
    fn kthread_create(f: unsafe extern "C" fn(*mut c_void) -> c_int, a: *mut c_void, fmt: *const c_char, ...) -> *mut task_struct;
    fn wake_up_process(x: *mut task_struct) -> c_int;
    fn kthread_stop(x: *mut task_struct) -> c_int;
    fn sched_set_normal(x: *mut task_struct, nice: c_int);
    fn mutex_lock(x: *mut c_void); fn mutex_unlock(x: *mut c_void);
    fn kmalloc_obj(size: usize) -> *mut shuffle_task;
    fn kfree(x: *mut c_void);
    fn set_cpus_allowed_ptr(t: *mut task_struct, m: cpumask_var_t) -> c_int;
    fn alloc_cpumask_var(m: *mut cpumask_var_t, flags: c_int) -> bool;
    fn free_cpumask_var(m: cpumask_var_t);
    fn cpumask_setall(m: cpumask_var_t); fn cpumask_clear_cpu(c: c_int, m: cpumask_var_t);
    fn cpumask_next(c: c_int, m: cpumask_var_t) -> c_int;
    fn num_online_cpus() -> c_int; fn cpu_online(c: c_int) -> bool; fn cpu_is_hotpluggable(c: c_int) -> bool;
    fn remove_cpu(c: c_int) -> c_int; fn add_cpu(c: c_int) -> c_int;
}

const NSEC_PER_USEC: ktime_t = 1000;
const NSEC_PER_MSEC: ktime_t = 1000000;
const NSEC_PER_SEC: ktime_t = 1000000000;
const HRTIMER_MODE_REL: hrtimer_mode = 0;
const HRTIMER_MODE_ABS: hrtimer_mode = 1;
const TASK_IDLE: c_int = 0; const TASK_INTERRUPTIBLE: c_int = 1;

pub unsafe extern "C" fn verbose_torout_sleep() {
    if verbose_sleep_frequency > 0 && verbose_sleep_duration > 0 &&
       atomic_inc_return(&raw mut verbose_sleep_counter) % verbose_sleep_frequency == 0 {
        schedule_timeout_uninterruptible(verbose_sleep_duration as c_long);
    }
}

pub unsafe extern "C" fn torture_hrtimeout_ns(baset_ns: ktime_t, fuzzt_ns: u32, mode: hrtimer_mode, trsp: *mut torture_random_state) -> c_int {
    let mut hto = baset_ns;
    if !trsp.is_null() && fuzzt_ns != 0 { hto += (torture_random(trsp) % fuzzt_ns as c_ulong) as ktime_t; }
    set_current_state(TASK_IDLE); schedule_hrtimeout(&mut hto, mode)
}
pub unsafe extern "C" fn torture_hrtimeout_us(baset_us: u32, fuzzt_ns: u32, trsp: *mut torture_random_state) -> c_int { torture_hrtimeout_ns(baset_us as ktime_t * NSEC_PER_USEC, fuzzt_ns, HRTIMER_MODE_REL, trsp) }
pub unsafe extern "C" fn torture_hrtimeout_ms(baset_ms: u32, fuzzt_us: u32, trsp: *mut torture_random_state) -> c_int {
    let f = if (u32::MAX as ktime_t / NSEC_PER_USEC) < fuzzt_us as ktime_t { u32::MAX } else { (fuzzt_us as ktime_t * NSEC_PER_USEC) as u32 };
    torture_hrtimeout_ns(baset_ms as ktime_t * NSEC_PER_MSEC, f, HRTIMER_MODE_REL, trsp)
}
pub unsafe extern "C" fn torture_hrtimeout_jiffies(baset_j: u32, trsp: *mut torture_random_state) -> c_int { torture_hrtimeout_ns(jiffies_to_nsecs(baset_j as c_ulong), jiffies_to_nsecs(1) as u32, HRTIMER_MODE_REL, trsp) }
pub unsafe extern "C" fn torture_hrtimeout_s(baset_s: u32, fuzzt_ms: u32, trsp: *mut torture_random_state) -> c_int {
    let f = if (u32::MAX as ktime_t / NSEC_PER_MSEC) < fuzzt_ms as ktime_t { u32::MAX } else { (fuzzt_ms as ktime_t * NSEC_PER_MSEC) as u32 };
    torture_hrtimeout_ns(baset_s as ktime_t * NSEC_PER_SEC, f, HRTIMER_MODE_REL, trsp)
}

#[repr(C)] struct shuffle_task { st_l: list_head, st_t: *mut task_struct }
static mut shuffle_interval: c_long = 0;
static mut shuffler_task: *mut task_struct = core::ptr::null_mut();
static mut shuffle_tmp_mask: cpumask_var_t = core::ptr::null_mut();
static mut shuffle_idle_cpu: c_int = -1;
static mut shutdown_task: *mut task_struct = core::ptr::null_mut();
static mut shutdown_time: ktime_t = 0;
static mut torture_shutdown_hook: Option<unsafe extern "C" fn()> = None;
static mut stutter_task: *mut task_struct = core::ptr::null_mut();
static mut stutter_till_abs_time: ktime_t = 0;
static mut stutter: c_int = 0; static mut stutter_gap: c_int = 0;
static mut torture_init_jiffies: c_ulong = 0;

pub unsafe extern "C" fn torture_random(trsp: *mut torture_random_state) -> c_ulong {
    (*trsp).trs_count -= 1;
    if (*trsp).trs_count < 0 { (*trsp).trs_state = (*trsp).trs_state.wrapping_add(local_clock()).wrapping_add(raw_smp_processor_id()); (*trsp).trs_count = TORTURE_RANDOM_REFRESH; }
    (*trsp).trs_state = (*trsp).trs_state.wrapping_mul(TORTURE_RANDOM_MULT).wrapping_add(TORTURE_RANDOM_ADD); swahw32((*trsp).trs_state)
}

pub unsafe extern "C" fn torture_shutdown_absorb(title: *const c_char) { while fullstop == FULLSTOP_SHUTDOWN { schedule_timeout_uninterruptible(c_long::MAX); } }
pub unsafe extern "C" fn stutter_wait(title: *const c_char) -> bool { cond_resched_tasks_rcu_qs(); let t = stutter_till_abs_time; let mut ret = false; if t != 0 && ktime_before(ktime_get(), t) { torture_hrtimeout_ns(t, 0, HRTIMER_MODE_ABS, core::ptr::null_mut()); ret = true; } torture_shutdown_absorb(title); ret }
pub unsafe extern "C" fn get_torture_init_jiffies() -> c_ulong { torture_init_jiffies }
pub unsafe extern "C" fn torture_must_stop() -> bool { torture_must_stop_irq() || kthread_should_stop() }
pub unsafe extern "C" fn torture_must_stop_irq() -> bool { fullstop != FULLSTOP_DONTSTOP }

pub unsafe extern "C" fn torture_init_begin(ttype: *mut c_char, v: c_int) -> bool { if !torture_type.is_null() { return false; } torture_type = ttype; verbose = v; fullstop = FULLSTOP_DONTSTOP; torture_init_jiffies = 0; true }
pub unsafe extern "C" fn torture_init_end() { register_reboot_notifier(core::ptr::null_mut()); }
pub unsafe extern "C" fn torture_cleanup_begin() -> bool { fullstop = FULLSTOP_RMMOD; false }
pub unsafe extern "C" fn torture_cleanup_end() { torture_type = core::ptr::null_mut(); }

pub unsafe extern "C" fn _torture_stop_kthread(_m: *mut c_char, tp: *mut *mut task_struct) { if !(*tp).is_null() { kthread_stop(*tp); *tp = core::ptr::null_mut(); } }
pub unsafe extern "C" fn torture_sched_set_normal(t: *mut task_struct, nice: c_int) { sched_set_normal(t, nice.clamp(-20, 19)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
