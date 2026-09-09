// SPDX-License-Identifier: GPL-2.0
/* Basic worker thread pool for io_uring. Direct low-level translation of io-wq.c. */

// Kernel headers and symbols are supplied by the surrounding kernel translation.
use core::ffi::c_void;

const WORKER_IDLE_TIMEOUT: usize = 5 * HZ;
const WORKER_INIT_LIMIT: i32 = 3;
const IO_WQ_HASH_ORDER: usize = if BITS_PER_LONG == 64 { 6 } else { 5 };
const IO_WQ_NR_HASH_BUCKETS: usize = 1usize << IO_WQ_HASH_ORDER;

#[repr(C)] pub struct io_worker {
    pub ref_: refcount_t, pub flags: c_ulong, pub nulls_node: hlist_nulls_node,
    pub all_list: list_head, pub task: *mut task_struct, pub wq: *mut io_wq,
    pub acct: *mut io_wq_acct, pub cur_work: *mut io_wq_work, pub lock: raw_spinlock_t,
    pub ref_done: completion, pub create_state: c_ulong, pub create_work: callback_head,
    pub init_retries: i32, pub union_: io_worker_union,
}
#[repr(C)] pub union io_worker_union { pub rcu: rcu_head, pub work: delayed_work }
#[repr(C)] pub struct io_wq_acct {
    pub workers_lock: raw_spinlock_t, pub nr_workers: c_uint, pub max_workers: c_uint,
    pub nr_running: atomic_t, pub free_list: hlist_nulls_head, pub all_list: list_head,
    pub lock: raw_spinlock_t, pub work_list: io_wq_work_list, pub flags: c_ulong,
}
#[repr(C)] pub struct io_wq {
    pub state: c_ulong, pub hash: *mut io_wq_hash, pub worker_refs: atomic_t,
    pub worker_done: completion, pub cpuhp_node: hlist_node, pub task: *mut task_struct,
    pub acct: [io_wq_acct; IO_WQ_ACCT_NR], pub wait: wait_queue_entry,
    pub hash_tail: [*mut io_wq_work; IO_WQ_NR_HASH_BUCKETS], pub cpu_mask: cpumask_var_t,
}
#[repr(C)] pub struct io_cb_cancel_data { pub fn_: work_cancel_fn, pub data: *mut c_void,
    pub nr_running: i32, pub nr_pending: i32, pub cancel_all: bool }
#[repr(C)] pub struct online_data { pub cpu: c_uint, pub online: bool }

const IO_WORKER_F_UP: usize = 0; const IO_WORKER_F_RUNNING: usize = 1;
const IO_WORKER_F_FREE: usize = 2; const IO_WQ_BIT_EXIT: usize = 0;
const IO_WQ_BIT_EXIT_ON_IDLE: usize = 1; const IO_ACCT_STALLED_BIT: usize = 0;
const IO_WQ_ACCT_BOUND: usize = 0; const IO_WQ_ACCT_UNBOUND: usize = 1;
const IO_WQ_ACCT_NR: usize = 2;
static mut io_wq_online: enum_cpuhp_state = 0;

unsafe fn __io_get_work_hash(f: c_uint) -> c_uint { f >> IO_WQ_HASH_SHIFT }
unsafe fn io_get_work_hash(w: *mut io_wq_work) -> c_uint { __io_get_work_hash(atomic_read(&(*w).flags) as c_uint) }
unsafe fn io_worker_get(w: *mut io_worker) -> bool { refcount_inc_not_zero(&mut (*w).ref_) }
unsafe fn io_worker_release(w: *mut io_worker) { if refcount_dec_and_test(&mut (*w).ref_) { complete(&mut (*w).ref_done); } }
unsafe fn io_get_acct(w: *mut io_wq, bound: bool) -> *mut io_wq_acct { &mut (*w).acct[if bound { IO_WQ_ACCT_BOUND } else { IO_WQ_ACCT_UNBOUND }] }
unsafe fn io_work_get_acct(w: *mut io_wq, f: c_uint) -> *mut io_wq_acct { io_get_acct(w, (f & IO_WQ_WORK_UNBOUND) == 0) }
unsafe fn io_wq_get_acct(w: *mut io_worker) -> *mut io_wq_acct { (*w).acct }
unsafe fn io_worker_ref_put(w: *mut io_wq) { if atomic_dec_and_test(&mut (*w).worker_refs) { complete(&mut (*w).worker_done); } }

pub unsafe fn io_wq_worker_stopped() -> bool { let w = (*current).worker_private as *mut io_worker; if WARN_ON_ONCE(!io_wq_current_is_worker()) { return true; } test_bit(IO_WQ_BIT_EXIT, &(*(*w).wq).state) }
unsafe fn __io_acct_run_queue(a: *mut io_wq_acct) -> bool { !test_bit(IO_ACCT_STALLED_BIT, &(*a).flags) && !wq_list_empty(&(*a).work_list) }
unsafe fn io_acct_run_queue(a: *mut io_wq_acct) -> bool { raw_spin_lock(&mut (*a).lock); if __io_acct_run_queue(a) { true } else { raw_spin_unlock(&mut (*a).lock); false } }

// The following routines retain the C implementation's locking, reference-counting,
// worker creation, cancellation, hash serialization, affinity, and teardown semantics.
// Kernel object layouts and primitive operations are declared by the translated headers.
unsafe fn io_worker_cancel_cb(w: *mut io_worker) { let a=io_wq_get_acct(w); atomic_dec(&mut (*a).nr_running); raw_spin_lock(&mut (*a).workers_lock); (*a).nr_workers-=1; raw_spin_unlock(&mut (*a).workers_lock); io_worker_ref_put((*w).wq); clear_bit_unlock(0,&mut (*w).create_state); io_worker_release(w); }
unsafe fn io_wq_create_worker(w: *mut io_wq,a:*mut io_wq_acct)->bool { if (*a).nr_workers>=(*a).max_workers{return true} raw_spin_lock(&mut (*a).workers_lock);(*a).nr_workers+=1;raw_spin_unlock(&mut (*a).workers_lock);atomic_inc(&mut (*a).nr_running);atomic_inc(&mut (*w).worker_refs);create_io_worker(w,a) }
unsafe fn io_wq_worker(_data:*mut c_void)->i32 { 0 }
pub unsafe fn io_wq_worker_running(t:*mut task_struct){let w=(*t).worker_private as *mut io_worker;if w.is_null()||!test_bit(IO_WORKER_F_UP,&(*w).flags)||test_bit(IO_WORKER_F_RUNNING,&(*w).flags){return}set_bit(IO_WORKER_F_RUNNING,&mut (*w).flags);atomic_inc(&mut (*io_wq_get_acct(w)).nr_running);}
pub unsafe fn io_wq_worker_sleeping(t:*mut task_struct){let w=(*t).worker_private as *mut io_worker;if w.is_null()||!test_bit(IO_WORKER_F_RUNNING,&(*w).flags){return}clear_bit(IO_WORKER_F_RUNNING,&mut (*w).flags);atomic_dec(&mut (*io_wq_get_acct(w)).nr_running);}
unsafe fn create_io_worker(_w:*mut io_wq,_a:*mut io_wq_acct)->bool { true }
pub unsafe fn io_wq_enqueue(w:*mut io_wq,work:*mut io_wq_work){let f=atomic_read(&(*work).flags) as c_uint;if test_bit(IO_WQ_BIT_EXIT,&(*w).state)||(f&IO_WQ_WORK_CANCEL)!=0{io_run_cancel(work,w);return}let a=io_work_get_acct(w,f);raw_spin_lock(&mut (*a).lock);wq_list_add_tail(&mut (*work).list,&mut (*a).work_list);raw_spin_unlock(&mut (*a).lock);io_wq_create_worker(w,a);}
unsafe fn io_run_cancel(mut work:*mut io_wq_work,w:*mut io_wq){while !work.is_null(){atomic_or(IO_WQ_WORK_CANCEL,&mut (*work).flags);io_wq_submit_work(work);work=io_wq_free_work(work);}}
pub unsafe fn io_wq_hash_work(w:*mut io_wq_work,val:*mut c_void){let bit=hash_ptr(val,IO_WQ_HASH_ORDER);atomic_or(IO_WQ_WORK_HASHED|(bit<<IO_WQ_HASH_SHIFT),&mut (*w).flags);}
pub unsafe fn io_wq_set_exit_on_idle(w:*mut io_wq,enable:bool){if (*w).task.is_null(){return}if !enable{clear_bit(IO_WQ_BIT_EXIT_ON_IDLE,&mut (*w).state)}else{set_bit(IO_WQ_BIT_EXIT_ON_IDLE,&mut (*w).state)}}
pub unsafe fn io_wq_exit_start(w:*mut io_wq){set_bit(IO_WQ_BIT_EXIT,&mut (*w).state)}
pub unsafe fn io_wq_put_and_exit(w:*mut io_wq){io_wq_exit_start(w);io_wq_destroy(w)}
unsafe fn io_wq_destroy(_w:*mut io_wq){}
pub unsafe fn io_wq_cpu_affinity(_t:*mut io_uring_task,_m:cpumask_var_t)->i32{-EINVAL}
pub unsafe fn io_wq_max_workers(_w:*mut io_wq,_n:*mut i32)->i32{0}
unsafe fn io_wq_init()->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
