/* SPDX-License-Identifier: GPL-2.0 */
/* Simple interface for creating and stopping kernel threads without mess. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

/* External types supplied by other headers. */
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }

/* opaque kthread data */
#[repr(C)] pub struct kthread { _private: [u8; 0] }
#[repr(C)] pub struct cgroup_subsys_state { _private: [u8; 0] }

pub const PF_KTHREAD: c_ulong = 0; // supplied by linux/sched.h
pub const NUMA_NO_NODE: c_int = -1; // supplied by linux/numa.h
pub const KTW_FREEZABLE: c_uint = 1 << 0; /* freeze during suspend */

/* Return NULL for any task that is not a kthread. */
pub unsafe fn tsk_is_kthread(p: *mut task_struct) -> *mut kthread {
    if (*(p as *mut TaskStructLayout)).flags & PF_KTHREAD != 0 {
        (*(p as *mut TaskStructLayout)).worker_private as *mut kthread
    } else { core::ptr::null_mut() }
}

#[repr(C)] struct TaskStructLayout { flags: c_ulong, worker_private: *mut c_void }

extern "C" {
    pub fn kthread_create_on_node(threadfn: Option<unsafe extern "C" fn(*mut c_void) -> c_int>, data: *mut c_void, node: c_int, namefmt: *const c_char, ...) -> *mut task_struct;
    pub fn kthread_create_on_cpu(threadfn: Option<unsafe extern "C" fn(*mut c_void) -> c_int>, data: *mut c_void, cpu: c_uint, namefmt: *const c_char) -> *mut task_struct;
    pub fn get_kthread_comm(buf: *mut c_char, buf_size: usize, tsk: *mut task_struct);
    pub fn set_kthread_struct(p: *mut task_struct) -> bool;
    pub fn kthread_set_per_cpu(k: *mut task_struct, cpu: c_int);
    pub fn kthread_is_per_cpu(k: *mut task_struct) -> bool;
    pub fn free_kthread_struct(k: *mut task_struct);
    pub fn kthread_bind(k: *mut task_struct, cpu: c_uint);
    pub fn kthread_bind_mask(k: *mut task_struct, mask: *const cpumask);
    pub fn kthread_affine_preferred(p: *mut task_struct, mask: *const cpumask) -> c_int;
    pub fn kthread_stop(k: *mut task_struct) -> c_int;
    pub fn kthread_stop_put(k: *mut task_struct) -> c_int;
    pub fn kthread_should_stop() -> bool;
    pub fn kthread_should_park() -> bool;
    pub fn kthread_should_stop_or_park() -> bool;
    pub fn kthread_freezable_should_stop(was_frozen: *mut bool) -> bool;
    pub fn kthread_func(k: *mut task_struct) -> *mut c_void;
    pub fn kthread_data(k: *mut task_struct) -> *mut c_void;
    pub fn kthread_probe_data(k: *mut task_struct) -> *mut c_void;
    pub fn kthread_park(k: *mut task_struct) -> c_int;
    pub fn kthread_unpark(k: *mut task_struct);
    pub fn kthread_parkme();
    pub fn do_exit(result: c_int) -> !;
    pub fn kthread_complete_and_exit(c: *mut completion, code: c_long) -> !;
    pub fn kthreads_update_housekeeping() -> c_int;
    pub fn kthread_do_exit(k: *mut kthread, code: c_long);
    pub fn kthreadd(unused: *mut c_void) -> c_int;
    pub static mut kthreadd_task: *mut task_struct;
    pub fn tsk_fork_get_node(tsk: *mut task_struct) -> c_int;
    pub fn kthread_delayed_work_timer_fn(t: *mut timer_list);
    pub fn __kthread_init_worker(worker: *mut kthread_worker, name: *const c_char, key: *mut lock_class_key);
    pub fn kthread_worker_fn(worker_ptr: *mut c_void) -> c_int;
    pub fn kthread_create_worker_on_node(flags: c_uint, node: c_int, namefmt: *const c_char, ...) -> *mut kthread_worker;
    pub fn kthread_create_worker_on_cpu(cpu: c_int, flags: c_uint, namefmt: *const c_char) -> *mut kthread_worker;
    pub fn kthread_queue_work(worker: *mut kthread_worker, work: *mut kthread_work) -> bool;
    pub fn kthread_queue_delayed_work(worker: *mut kthread_worker, dwork: *mut kthread_delayed_work, delay: c_ulong) -> bool;
    pub fn kthread_mod_delayed_work(worker: *mut kthread_worker, dwork: *mut kthread_delayed_work, delay: c_ulong) -> bool;
    pub fn kthread_flush_work(work: *mut kthread_work);
    pub fn kthread_flush_worker(worker: *mut kthread_worker);
    pub fn kthread_cancel_work_sync(work: *mut kthread_work) -> bool;
    pub fn kthread_cancel_delayed_work_sync(work: *mut kthread_delayed_work) -> bool;
    pub fn kthread_destroy_worker(worker: *mut kthread_worker);
    pub fn kthread_use_mm(mm: *mut mm_struct);
    pub fn kthread_unuse_mm(mm: *mut mm_struct);
    #[cfg(CONFIG_BLK_CGROUP)]
    pub fn kthread_associate_blkcg(css: *mut cgroup_subsys_state);
    pub fn kthread_blkcg() -> *mut cgroup_subsys_state;
}

pub type kthread_work_func_t = Option<unsafe extern "C" fn(*mut kthread_work)>;

#[repr(C)] pub struct kthread_worker {
    pub flags: c_uint,
    pub lock: raw_spinlock_t,
    pub work_list: list_head,
    pub delayed_work_list: list_head,
    pub task: *mut task_struct,
    pub current_work: *mut kthread_work,
}
#[repr(C)] pub struct kthread_work {
    pub node: list_head,
    pub func: kthread_work_func_t,
    pub worker: *mut kthread_worker,
    /* Number of canceling calls that are running at the moment. */
    pub canceling: c_int,
}
#[repr(C)] pub struct kthread_delayed_work { pub work: kthread_work, pub timer: timer_list }

pub unsafe fn kthread_run_on_cpu(threadfn: Option<unsafe extern "C" fn(*mut c_void) -> c_int>, data: *mut c_void, cpu: c_uint, namefmt: *const c_char) -> *mut task_struct {
    let p = kthread_create_on_cpu(threadfn, data, cpu, namefmt);
    if !p.is_null() { wake_up_process(p); }
    p
}
pub unsafe fn kthread_run_worker_on_cpu(cpu: c_int, flags: c_uint, namefmt: *const c_char) -> *mut kthread_worker {
    let kw = kthread_create_worker_on_cpu(cpu, flags, namefmt);
    if !kw.is_null() { wake_up_process((*kw).task); }
    kw
}

extern "C" { pub fn wake_up_process(p: *mut task_struct) -> c_int; }

#[cfg(not(CONFIG_BLK_CGROUP))]
pub unsafe fn kthread_associate_blkcg(_css: *mut cgroup_subsys_state) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
