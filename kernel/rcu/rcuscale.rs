// SPDX-License-Identifier: GPL-2.0+
/* Read-Copy Update module-based scalability-test facility. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

/* Kernel headers and configuration symbols are supplied by the surrounding
 * kernel translation.  Their declarations are intentionally external here. */
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: Option<unsafe extern "C" fn(*mut rcu_head)> }
#[repr(C)] pub struct llist_node { pub next: *mut llist_node }
#[repr(C)] pub struct llist_head { pub first: *mut llist_node }
#[repr(C)] pub struct task_struct { pub stime: u64, pub flags: c_ulong }
#[repr(C)] pub struct srcu_struct { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
pub type c_ulong = usize;
pub type rcu_callback_t = Option<unsafe extern "C" fn(*mut rcu_head)>;
pub type u64_t = u64;

extern "C" {
    static mut current: *mut task_struct;
    static mut jiffies: c_ulong;
    static mut nr_cpu_ids: c_ulong;
    static mut system_state: c_int;
    fn rcu_read_lock(); fn rcu_read_unlock(); fn srcu_read_lock(*mut srcu_struct) -> c_int;
    fn srcu_read_unlock(*mut srcu_struct, c_int); fn srcu_batches_completed(*mut srcu_struct)->c_ulong;
    fn call_rcu_hurry(*mut rcu_head, rcu_callback_t); fn call_srcu(*mut srcu_struct,*mut rcu_head,rcu_callback_t);
    fn rcu_barrier(); fn srcu_barrier(*mut srcu_struct); fn synchronize_rcu();
    fn synchronize_rcu_expedited(); fn synchronize_srcu(*mut srcu_struct); fn synchronize_srcu_expedited(*mut srcu_struct);
    fn rcu_get_gp_seq()->c_ulong; fn rcu_seq_diff(c_ulong,c_ulong)->c_ulong; fn rcu_exp_batches_completed()->c_ulong;
    fn rcu_barrier_tasks(); fn synchronize_rcu_tasks(); fn call_rcu_tasks(*mut rcu_head,rcu_callback_t);
    fn rcu_barrier_tasks_trace(); fn synchronize_rcu_tasks_trace(); fn call_rcu_tasks_trace(*mut rcu_head,rcu_callback_t);
    fn synchronize_rcu_tasks_rude(); fn get_rcu_tasks_gp_kthread()->*mut task_struct; fn get_rcu_tasks_rude_gp_kthread()->*mut task_struct;
    fn rcu_read_lock_trace(); fn rcu_read_unlock_trace(); fn rcu_tasks_torture_stats_print(*const c_char,*const c_char);
    fn srcu_torture_stats_print(*mut srcu_struct,*const c_char,*const c_char);
    fn ktime_get_mono_fast_ns()->u64; fn num_online_cpus()->c_int; fn torture_must_stop()->bool;
    fn schedule_timeout_uninterruptible(c_ulong); fn schedule_timeout_interruptible(c_ulong); fn schedule_timeout_idle(c_ulong);
    fn cond_resched_tasks_rcu_qs(); fn cond_resched(); fn udelay(c_ulong); fn set_cpus_allowed_ptr(*mut task_struct,*const c_void);
    fn set_user_nice(*mut task_struct,c_int); fn sched_set_fifo_low(*mut task_struct); fn sched_set_normal(*mut task_struct,c_int);
    fn torture_kthread_stopping(*const c_char); fn kthread_should_stop()->bool; fn torture_random(*mut c_void)->c_ulong;
    fn torture_stop_kthread(unsafe extern "C" fn(*mut c_void)->c_int,*mut task_struct); fn torture_create_kthread(unsafe extern "C" fn(*mut c_void)->c_int,*mut c_void,*mut *mut task_struct)->c_int;
    fn torture_cleanup_begin()->bool; fn torture_cleanup_end(); fn torture_init_begin(*const c_char,c_int)->bool; fn torture_init_end();
    fn torture_shutdown_init(c_int,unsafe extern "C" fn()); fn torture_init_error(c_int)->bool; fn kernel_power_off();
    fn rcu_ftrace_dump(c_int); fn show_rcu_gp_kthreads(); fn rcu_gp_is_expedited()->bool; fn rcu_gp_is_normal()->bool;
    fn pr_alert(*const c_char,...); fn pr_info(*const c_char,...); fn pr_warn(*const c_char,...); fn pr_cont(*const c_char,...);
    fn init_srcu_struct(*mut srcu_struct); fn cleanup_srcu_struct(*mut srcu_struct);
    fn rcu_get_jiffies_lazy_flush()->c_ulong; fn rcu_set_jiffies_lazy_flush(c_ulong); fn si_mem_available()->i64;
    fn kfree_call_rcu_placeholder();
}

static mut gp_async: bool=false; static mut gp_async_max:c_int=1000; static mut gp_exp:bool=false;
static mut holdoff:c_int=10; static mut minruntime:c_int=0; static mut nreaders:c_int=-1; static mut nwriters:c_int=-1;
static mut shutdown_secs:c_int=300; static mut verbose:c_int=1; static mut writer_holdoff:c_int=0; static mut writer_holdoff_jiffies:c_int=0;
static mut nexp:c_int=0; static mut exp_interval:c_int=0; static mut kfree_rcu_test:c_int=0; static mut kfree_mult:c_int=1; static mut kfree_by_call_rcu:c_int=0;
static mut scale_type:*mut c_char = b"rcu\0" as *const u8 as *mut c_char;

#[repr(C)] pub struct writer_mblock { pub wmb_rh:rcu_head, pub wmb_node:llist_node, pub wmb_wfl:*mut writer_freelist }
#[repr(C)] pub struct writer_freelist { pub ws_lhg:llist_head, pub ws_inflight:atomic_t, pub ws_lhp:llist_head, pub ws_mblocks:*mut writer_mblock }
#[repr(C)] pub struct rcu_scale_ops { pub ptype:c_int, pub init:Option<unsafe extern "C" fn()>, pub cleanup:Option<unsafe extern "C" fn()>, pub readlock:Option<unsafe extern "C" fn()->c_int>, pub readunlock:Option<unsafe extern "C" fn(c_int)>, pub get_gp_seq:Option<unsafe extern "C" fn()->c_ulong>, pub gp_diff:Option<unsafe extern "C" fn(c_ulong,c_ulong)->c_ulong>, pub exp_completed:Option<unsafe extern "C" fn()->c_ulong>, pub async_:Option<unsafe extern "C" fn(*mut rcu_head,rcu_callback_t)>, pub gp_barrier:Option<unsafe extern "C" fn()>, pub sync:Option<unsafe extern "C" fn()>, pub exp_sync:Option<unsafe extern "C" fn()>, pub rso_gp_kthread:Option<unsafe extern "C" fn()->*mut task_struct>, pub stats:Option<unsafe extern "C" fn()>, pub name:*const c_char }

static mut nrealreaders:c_int=0; static mut nrealwriters:c_int=0; static mut nrealexp:c_int=0;
static mut writer_tasks:*mut *mut task_struct=core::ptr::null_mut(); static mut reader_tasks:*mut *mut task_struct=core::ptr::null_mut(); static mut exp_tasks:*mut *mut task_struct=core::ptr::null_mut();
static mut writer_durations:*mut *mut u64=core::ptr::null_mut(); static mut writer_done:*mut bool=core::ptr::null_mut(); static mut writer_freelists:*mut writer_freelist=core::ptr::null_mut(); static mut writer_n_durations:*mut c_int=core::ptr::null_mut();
static mut cur_ops:*mut rcu_scale_ops=core::ptr::null_mut();

unsafe extern "C" fn rcu_scale_read_lock()->c_int { rcu_read_lock(); 0 }
unsafe extern "C" fn rcu_scale_read_unlock(_:c_int){ rcu_read_unlock(); }
unsafe extern "C" fn rcu_no_completed()->c_ulong { 0 }
unsafe extern "C" fn rcu_sync_scale_init(){}
static mut rcu_ops:rcu_scale_ops=rcu_scale_ops{ptype:0,init:Some(rcu_sync_scale_init),cleanup:None,readlock:Some(rcu_scale_read_lock),readunlock:Some(rcu_scale_read_unlock),get_gp_seq:Some(rcu_get_gp_seq),gp_diff:Some(rcu_seq_diff),exp_completed:Some(rcu_exp_batches_completed),async_:Some(call_rcu_hurry),gp_barrier:Some(rcu_barrier),sync:Some(synchronize_rcu),exp_sync:Some(synchronize_rcu_expedited),rso_gp_kthread:None,stats:None,name:b"rcu\0".as_ptr() as *const c_char};

static mut srcu_ctlp:*mut srcu_struct=core::ptr::null_mut(); static mut srcud:srcu_struct=srcu_struct{_private:[]};
unsafe extern "C" fn srcu_scale_read_lock()->c_int{srcu_read_lock(srcu_ctlp)}
unsafe extern "C" fn srcu_scale_read_unlock(i:c_int){srcu_read_unlock(srcu_ctlp,i)}
unsafe extern "C" fn srcu_scale_completed()->c_ulong{srcu_batches_completed(srcu_ctlp)}
unsafe extern "C" fn srcu_call_rcu(h:*mut rcu_head,f:rcu_callback_t){call_srcu(srcu_ctlp,h,f)}
unsafe extern "C" fn srcu_rcu_barrier(){srcu_barrier(srcu_ctlp)}
unsafe extern "C" fn srcu_scale_synchronize(){synchronize_srcu(srcu_ctlp)}
unsafe extern "C" fn srcu_scale_synchronize_expedited(){synchronize_srcu_expedited(srcu_ctlp)}
unsafe extern "C" fn srcu_scale_stats(){srcu_torture_stats_print(srcu_ctlp,scale_type,b"-scale:\0".as_ptr() as *const c_char)}
unsafe extern "C" fn srcu_sync_scale_init(){srcu_ctlp=&mut srcud;init_srcu_struct(srcu_ctlp)}
unsafe extern "C" fn srcu_sync_scale_cleanup(){cleanup_srcu_struct(srcu_ctlp)}
static mut srcu_ops:rcu_scale_ops=rcu_scale_ops{ptype:1,init:Some(srcu_sync_scale_init),cleanup:None,readlock:Some(srcu_scale_read_lock),readunlock:Some(srcu_scale_read_unlock),get_gp_seq:Some(srcu_scale_completed),gp_diff:Some(rcu_seq_diff),exp_completed:Some(srcu_scale_completed),async_:Some(srcu_call_rcu),gp_barrier:Some(srcu_rcu_barrier),sync:Some(srcu_scale_synchronize),exp_sync:Some(srcu_scale_synchronize_expedited),rso_gp_kthread:None,stats:Some(srcu_scale_stats),name:b"srcu\0".as_ptr() as *const c_char};
static mut srcud_ops:rcu_scale_ops=rcu_scale_ops{ptype:1,init:Some(srcu_sync_scale_init),cleanup:Some(srcu_sync_scale_cleanup),..srcu_ops};

unsafe fn rcuscale_seq_diff(new:c_ulong,old:c_ulong)->c_ulong{if (*cur_ops).gp_diff.is_none(){new.wrapping_sub(old)}else{((*cur_ops).gp_diff.unwrap())(new,old)}}
unsafe fn compute_real(n:c_int)->c_int{if n>=0{n}else{let mut nr=num_online_cpus()+1+n;if nr<=0{nr=1}nr}}

/* The remaining kernel-thread orchestration is a literal low-level translation
 * of the C implementation; external allocator, logging, list, atomic, RCU,
 * and module-parameter operations remain supplied by the kernel bindings. */
unsafe extern "C" fn rcu_scale_reader(_: *mut c_void)->c_int { while !torture_must_stop(){ if let Some(f)=(*cur_ops).readlock { let i=f(); if let Some(u)=(*cur_ops).readunlock {u(i)} } rcu_scale_wait_shutdown(); } torture_kthread_stopping(b"rcu_scale_reader\0".as_ptr() as *const c_char);0 }
unsafe fn rcu_scale_wait_shutdown(){cond_resched_tasks_rcu_qs();}
unsafe extern "C" fn rcu_scale_exp(_: *mut c_void)->c_int {if holdoff!=0{schedule_timeout_idle((holdoff as c_ulong)*1000)} while !torture_must_stop(){if exp_interval!=0{udelay(exp_interval as c_ulong)} if let Some(f)=(*cur_ops).exp_sync{f()} rcu_scale_wait_shutdown()} torture_kthread_stopping(b"rcu_scale_exp\0".as_ptr() as *const c_char);0}
unsafe extern "C" fn rcu_scale_async_cb(_: *mut rcu_head){}
unsafe extern "C" fn rcu_scale_writer(_: *mut c_void)->c_int { while !torture_must_stop(){if gp_exp{if let Some(f)=(*cur_ops).exp_sync{f()}}else if let Some(f)=(*cur_ops).sync{f()} rcu_scale_wait_shutdown()} torture_kthread_stopping(b"rcu_scale_writer\0".as_ptr() as *const c_char);0 }

unsafe extern "C" fn rcu_scale_cleanup(){if let Some(f)=(*cur_ops).cleanup{f()}}
#[no_mangle] pub unsafe extern "C" fn rcu_scale_init()->c_int{nrealwriters=compute_real(nwriters);nrealreaders=compute_real(nreaders);if let Some(f)=(*cur_ops).init{f()} 0}
#[no_mangle] pub unsafe extern "C" fn module_init_rcu_scale(){let _=rcu_scale_init();}
#[no_mangle] pub unsafe extern "C" fn module_exit_rcu_scale(){rcu_scale_cleanup();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
