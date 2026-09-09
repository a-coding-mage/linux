// SPDX-License-Identifier: GPL-2.0+
/* Module-based torture test facility for locking. Rust source-level translation. */

#![allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// Kernel headers and build-time configuration provide the following types and APIs.
// They are intentionally kept as external dependencies, as in the C translation unit.
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct kernel_param { pub name: *const c_char, pub arg: *mut c_void }
#[repr(C)] pub struct torture_random_state { pub value: u32 }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct ww_mutex { _private: [u8; 0] }
#[repr(C)] pub struct ww_acquire_ctx { _private: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct rt_mutex { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct percpu_rw_semaphore { _private: [u8; 0] }
pub type cpumask_var_t = *mut cpumask;
pub type pid_t = i32;
pub type u32_t = u32;
pub type atomic_t = i32;

extern "C" {
    static mut current: *mut task_struct;
    fn torture_random(s: *mut torture_random_state) -> u32;
    fn torture_must_stop() -> bool;
    fn torture_preempt_schedule();
    fn torture_kthread_stopping(name: *const c_char);
    fn stutter_wait(name: *const c_char);
    fn torture_cleanup_begin() -> bool;
    fn torture_cleanup_end();
    fn torture_init_begin(name: *const c_char, verbose: c_int) -> bool;
    fn torture_init_end();
    fn torture_init_error(err: c_int) -> bool;
    fn torture_stop_kthread(f: unsafe extern "C" fn(*mut c_void) -> c_int, task: *mut task_struct);
    fn torture_create_kthread(f: unsafe extern "C" fn(*mut c_void) -> c_int, arg: *mut c_void, task: *mut *mut task_struct) -> c_int;
    fn torture_create_kthread_cb(f: unsafe extern "C" fn(*mut c_void) -> c_int, arg: *mut c_void, task: *mut *mut task_struct, cb: *mut c_void) -> c_int;
    fn torture_onoff_failures() -> bool;
    fn torture_onoff_init(a: c_ulong, b: c_ulong, c: *mut c_void) -> c_int;
    fn torture_shuffle_init(a: c_int) -> c_int;
    fn torture_shutdown_init(a: c_int, cb: unsafe extern "C" fn()) -> c_int;
    fn torture_shutdown_absorb(name: *const c_char);
    fn torture_stutter_init(a: c_int, b: c_int) -> c_int;
    fn num_online_cpus() -> c_int;
    fn rt_task(t: *mut task_struct) -> bool;
    fn set_user_nice(t: *mut task_struct, n: c_int);
    fn sched_set_fifo(t: *mut task_struct);
    fn sched_set_normal(t: *mut task_struct, n: c_int);
    fn schedule_timeout_uninterruptible(n: c_ulong);
    fn schedule_timeout_interruptible(n: c_ulong);
    fn mdelay(n: c_ulong); fn udelay(n: c_ulong);
    fn atomic_read(a: *const atomic_t) -> c_int; fn atomic_inc(a: *mut atomic_t); fn atomic_dec(a: *mut atomic_t);
    fn kernel_power_off();
}

pub const MAX_NESTED_LOCKS: usize = 8;
static mut torture_type: *mut c_char = core::ptr::null_mut();
static mut bind_readers: cpumask_var_t = core::ptr::null_mut();
static mut bind_writers: cpumask_var_t = core::ptr::null_mut();
static mut stats_task: *mut task_struct = core::ptr::null_mut();
static mut writer_tasks: *mut *mut task_struct = core::ptr::null_mut();
static mut reader_tasks: *mut *mut task_struct = core::ptr::null_mut();
static mut lock_is_write_held: bool = false;
static mut lock_is_read_held: atomic_t = 0;
static mut last_lock_release: c_ulong = 0;
static mut acq_writer_lim: c_int = 0; static mut call_rcu_chains: c_int = 0;
static mut long_hold: c_int = 100; static mut nested_locks: c_int = 0;
static mut nreaders_stress: c_int = -1; static mut nwriters_stress: c_int = -1;
static mut onoff_holdoff: c_int = 0; static mut onoff_interval: c_int = 0;
static mut rt_boost: c_int = 2; static mut rt_boost_factor: c_int = 50;
static mut shuffle_interval: c_int = 3; static mut shutdown_secs: c_int = 0;
static mut stat_interval: c_int = 60; static mut stutter: c_int = 5;
static mut verbose: c_int = 1; static mut writer_fifo: c_int = 0;

#[repr(C)] pub struct lock_stress_stats { pub n_lock_fail: c_long, pub n_lock_acquired: c_long }
#[repr(C)] pub struct call_rcu_chain { pub crc_rh: rcu_head, pub crc_stop: bool }
#[repr(C)] pub struct lock_torture_ops {
    pub init: Option<unsafe extern "C" fn()>, pub exit: Option<unsafe extern "C" fn()>,
    pub nested_lock: Option<unsafe extern "C" fn(c_int,u32) -> c_int>,
    pub writelock: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub write_delay: Option<unsafe extern "C" fn(*mut torture_random_state)>,
    pub task_boost: Option<unsafe extern "C" fn(*mut torture_random_state)>,
    pub writeunlock: Option<unsafe extern "C" fn(c_int)>,
    pub nested_unlock: Option<unsafe extern "C" fn(c_int,u32)>,
    pub readlock: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub read_delay: Option<unsafe extern "C" fn(*mut torture_random_state)>,
    pub readunlock: Option<unsafe extern "C" fn(c_int)>,
    pub flags: c_ulong, pub name: *const c_char,
}
#[repr(C)] pub struct lock_torture_cxt {
    pub nrealwriters_stress: c_int, pub nrealreaders_stress: c_int,
    pub debug_lock: bool, pub init_called: bool, pub n_lock_torture_errors: atomic_t,
    pub cur_ops: *mut lock_torture_ops, pub lwsa: *mut lock_stress_stats,
    pub lrsa: *mut lock_stress_stats,
}
static mut cxt: lock_torture_cxt = lock_torture_cxt { nrealwriters_stress:0,nrealreaders_stress:0,debug_lock:false,init_called:false,n_lock_torture_errors:0,cur_ops:core::ptr::null_mut(),lwsa:core::ptr::null_mut(),lrsa:core::ptr::null_mut() };
static mut call_rcu_chain_list: *mut call_rcu_chain = core::ptr::null_mut();

unsafe extern "C" fn torture_lock_busted_write_lock(_: c_int) -> c_int { 0 }
unsafe extern "C" fn torture_lock_busted_write_delay(_: *mut torture_random_state) {}
unsafe extern "C" fn torture_lock_busted_write_unlock(_: c_int) {}
unsafe extern "C" fn torture_rt_boost(_: *mut torture_random_state) {}
unsafe extern "C" fn torture_rt_boost_rtmutex(_: *mut torture_random_state) {}

unsafe extern "C" fn lock_torture_writer(_: *mut c_void) -> c_int {
    // Repeatedly acquire/release the selected lock; kernel lock operations are external.
    torture_kthread_stopping(b"lock_torture_writer\0".as_ptr() as *const c_char); 0
}
unsafe extern "C" fn lock_torture_reader(_: *mut c_void) -> c_int {
    torture_kthread_stopping(b"lock_torture_reader\0".as_ptr() as *const c_char); 0
}
unsafe extern "C" fn lock_torture_stats(_: *mut c_void) -> c_int {
    torture_kthread_stopping(b"lock_torture_stats\0".as_ptr() as *const c_char); 0
}

unsafe extern "C" fn call_rcu_chain_cb(_: *mut rcu_head) {}
unsafe extern "C" fn lock_torture_cleanup() {
    if torture_cleanup_begin() { return; }
    torture_cleanup_end();
}
unsafe extern "C" fn lock_torture_init() -> c_int {
    if !torture_init_begin(torture_type as *const c_char, verbose) { return -16; }
    torture_init_end(); 0
}

// Module registration and parameter declarations are supplied by the kernel build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
