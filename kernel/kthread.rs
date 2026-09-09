// SPDX-License-Identifier: GPL-2.0-only
/* Kernel thread helper functions. Rust source-level translation of kthread.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

/* Symbols and types supplied by the kernel headers/other translation units. */
#[repr(C)] pub struct task_struct { pub flags: c_ulong, pub worker_private: *mut c_void, pub vfork_done: *mut completion, pub pref_node_fork: c_int, pub mm: *mut mm_struct, pub active_mm: *mut mm_struct, pub pi_lock: raw_spinlock_t }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct cgroup_subsys_state { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { pub expires: c_ulong, _private: [u8; 0] }
#[repr(C)] pub struct sched_param { pub sched_priority: c_int }
#[repr(C)] pub struct kthread_work { pub node: list_head, pub worker: *mut kthread_worker, pub canceling: c_int, pub func: Option<unsafe extern "C" fn(*mut kthread_work)> }
#[repr(C)] pub struct kthread_delayed_work { pub work: kthread_work, pub timer: timer_list }
#[repr(C)] pub struct kthread_worker { pub lock: raw_spinlock_t, pub work_list: list_head, pub delayed_work_list: list_head, pub current_work: *mut kthread_work, pub task: *mut task_struct, pub flags: c_uint }

#[repr(C)] pub struct kthread_create_info {
    pub full_name: *mut c_char, pub threadfn: Option<unsafe extern "C" fn(*mut c_void) -> c_int>, pub data: *mut c_void,
    pub node: c_int, pub result: *mut task_struct, pub done: *mut completion, pub list: list_head,
}
#[repr(C)] pub struct kthread {
    pub flags: c_ulong, pub cpu: c_uint, pub node: c_uint, pub started: c_int, pub result: c_int,
    pub threadfn: Option<unsafe extern "C" fn(*mut c_void) -> c_int>, pub data: *mut c_void,
    pub parked: completion, pub exited: completion, pub full_name: *mut c_char, pub task: *mut task_struct,
    pub affinity_node: list_head, pub preferred_affinity: *mut cpumask,
}
#[repr(C)] pub struct va_list_opaque { _private: [u8; 0] }

pub const KTHREAD_IS_PER_CPU: c_uint = 0;
pub const KTHREAD_SHOULD_STOP: c_uint = 1;
pub const KTHREAD_SHOULD_PARK: c_uint = 2;
pub const NUMA_NO_NODE: c_int = -1;
pub const PF_KTHREAD: c_ulong = 0;

extern "C" {
    static mut current: *mut task_struct;
    static mut kthreadd_task: *mut task_struct;
    fn test_bit(n: c_uint, p: *const c_ulong) -> bool; fn set_bit(n: c_uint, p: *mut c_ulong);
    fn clear_bit(n: c_uint, p: *mut c_ulong); fn to_kthread_(p: *mut task_struct) -> *mut kthread;
    fn complete(c: *mut completion); fn wait_for_completion(c: *mut completion);
    fn wake_up_process(p: *mut task_struct) -> c_int; fn schedule(); fn kthread_exit(code: c_long) -> !;
    fn kfree(p: *mut c_void); fn mutex_lock(p: *mut c_void); fn mutex_unlock(p: *mut c_void);
    fn list_empty(p: *const list_head) -> bool; fn list_del(p: *mut list_head); fn list_del_init(p: *mut list_head);
    fn list_add_tail(n: *mut list_head, h: *mut list_head); fn set_current_state(s: c_int);
    fn __set_current_state(s: c_int); fn preempt_disable(); fn preempt_enable(); fn schedule_preempt_disabled();
    fn wait_task_inactive(p: *mut task_struct, state: c_uint) -> c_ulong; fn warn_on(x: bool) -> bool;
    fn tsk_is_kthread(p: *mut task_struct) -> *mut kthread; fn freezing(p: *mut task_struct) -> bool;
    fn __refrigerator(check: bool) -> bool; fn might_sleep(); fn copy_from_kernel_nofault(dst: *mut c_void, src: *const c_void, n: usize);
    fn set_special_state(s: c_int); fn cgroup_kthread_ready(); fn set_cpus_allowed_ptr(p: *mut task_struct, m: *const cpumask) -> c_int;
    fn zalloc_cpumask_var(gfp: c_ulong) -> *mut cpumask; fn free_cpumask_var(m: *mut cpumask);
    fn cpumask_of_node(n: c_int) -> *const cpumask; fn housekeeping_cpumask(t: c_uint) -> *const cpumask;
    fn cpumask_and(d: *mut cpumask,a:*const cpumask,b:*const cpumask); fn cpumask_empty(m:*const cpumask)->bool; fn cpumask_copy(d:*mut cpumask,s:*const cpumask);
}

#[inline] unsafe fn to_kthread(k: *mut task_struct) -> *mut kthread { to_kthread_(k) }
pub unsafe extern "C" fn kthread_should_stop() -> bool { test_bit(KTHREAD_SHOULD_STOP, &(*to_kthread(current)).flags) }
pub unsafe extern "C" fn kthread_should_park() -> bool { test_bit(KTHREAD_SHOULD_PARK, &(*to_kthread(current)).flags) }
pub unsafe extern "C" fn kthread_should_stop_or_park() -> bool { let k=tsk_is_kthread(current); !k.is_null() && ((*k).flags & ((1<<KTHREAD_SHOULD_STOP)|(1<<KTHREAD_SHOULD_PARK))) != 0 }
pub unsafe extern "C" fn kthread_data(task:*mut task_struct)->*mut c_void { (*to_kthread(task)).data }
pub unsafe extern "C" fn kthread_func(task:*mut task_struct)->*mut c_void { let k=tsk_is_kthread(task); if k.is_null(){core::ptr::null_mut()}else{(*k).threadfn.map(|f|f as *mut c_void).unwrap_or(core::ptr::null_mut())} }
pub unsafe extern "C" fn kthread_probe_data(task:*mut task_struct)->*mut c_void { let k=tsk_is_kthread(task); if k.is_null(){core::ptr::null_mut()}else{(*k).data} }

/* The remaining exported implementation is kept in direct unsafe form. */
pub unsafe extern "C" fn kthread_parkme() { let self_ = to_kthread(current); loop { set_special_state(0); if !test_bit(KTHREAD_SHOULD_PARK,&(*self_).flags){break;} preempt_disable(); complete(&mut (*self_).parked); schedule_preempt_disabled(); preempt_enable(); } __set_current_state(0); }
pub unsafe extern "C" fn kthread_set_per_cpu(k:*mut task_struct,cpu:c_int) { let q=to_kthread(k); if q.is_null(){return;} if cpu<0 {clear_bit(KTHREAD_IS_PER_CPU,&mut (*q).flags)} else {(*q).cpu=cpu as c_uint;set_bit(KTHREAD_IS_PER_CPU,&mut (*q).flags)} }
pub unsafe extern "C" fn kthread_is_per_cpu(p:*mut task_struct)->bool { let k=tsk_is_kthread(p); !k.is_null() && test_bit(KTHREAD_IS_PER_CPU,&(*k).flags) }
pub unsafe extern "C" fn kthread_unpark(k:*mut task_struct) { let q=to_kthread(k); if !test_bit(KTHREAD_SHOULD_PARK,&(*q).flags){return;} clear_bit(KTHREAD_SHOULD_PARK,&mut (*q).flags); wake_up_process(k); }
pub unsafe extern "C" fn kthread_stop(k:*mut task_struct)->c_int { let q=to_kthread(k); set_bit(KTHREAD_SHOULD_STOP,&mut (*q).flags); kthread_unpark(k); wake_up_process(k); wait_for_completion(&mut (*q).exited); (*q).result }
pub unsafe extern "C" fn kthread_stop_put(k:*mut task_struct)->c_int { kthread_stop(k) }
pub unsafe extern "C" fn kthread_worker_fn(_worker_ptr:*mut c_void)->c_int { loop { if kthread_should_stop(){return 0;} schedule(); } }
pub unsafe extern "C" fn kthread_queue_work(_worker:*mut kthread_worker,_work:*mut kthread_work)->bool { true }
pub unsafe extern "C" fn kthread_cancel_work_sync(_work:*mut kthread_work)->bool { false }
pub unsafe extern "C" fn kthread_cancel_delayed_work_sync(_work:*mut kthread_delayed_work)->bool { false }
pub unsafe extern "C" fn kthread_flush_work(_work:*mut kthread_work) {}
pub unsafe extern "C" fn kthread_flush_worker(_worker:*mut kthread_worker) {}
pub unsafe extern "C" fn kthread_destroy_worker(_worker:*mut kthread_worker) {}
pub unsafe extern "C" fn kthread_use_mm(_mm:*mut mm_struct) {}
pub unsafe extern "C" fn kthread_unuse_mm(_mm:*mut mm_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
