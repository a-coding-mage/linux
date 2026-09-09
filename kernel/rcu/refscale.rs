// SPDX-License-Identifier: GPL-2.0+
// Scalability test comparing RCU vs other mechanisms for acquiring references on objects.
// Copyright (C) Google, 2020.
// Author: Joel Fernandes <joel@joelfernandes.org>

// Kernel headers and rcu.h are supplied by the surrounding kernel translation.

const SCALE_FLAG: &str = "-ref-scale: ";

#[repr(C)]
struct ReaderTask { task: *mut task_struct, start_reader: i32, wq: wait_queue_head_t, last_duration_ns: u64 }

#[repr(C)]
struct RefScaleOps {
    init: Option<unsafe extern "C" fn() -> bool>,
    cleanup: Option<unsafe extern "C" fn()>,
    readsection: Option<unsafe extern "C" fn(i32)>,
    delaysection: Option<unsafe extern "C" fn(i32, i32, i32)>,
    enable_irqs: bool,
    name: *const u8,
}

extern "C" {
    type task_struct; type wait_queue_head_t; type srcu_struct; type srcu_ctr;
    type kmem_cache; type rw_semaphore; type spinlock_t; type seqlock_t; type atomic_t;
    static mut scale_type: *mut u8; static mut verbose: i32; static mut verbose_batched: i32;
    static mut guest_os_delay: i64; static mut holdoff: i32; static mut lookup_instances: i64;
    static mut loops: i32; static mut nreaders: i32; static mut nruns: i32;
    static mut readdelay: i32; static mut shutdown_secs: i32; static mut nr_cpu_ids: i64;
    static mut jiffies: u64; static mut cur_ops: *const RefScaleOps;
    static mut reader_tasks: *mut ReaderTask; static mut main_task: *mut task_struct;
    static mut main_wq: wait_queue_head_t; static mut nreaders_exp: atomic_t;
    static mut n_init: atomic_t; static mut n_started: atomic_t; static mut n_warmedup: atomic_t;
    static mut n_cooleddown: atomic_t; static mut exp_idx: i32;
    static mut refcnt: atomic_t; static mut test_acqrel: u64; static mut stopopts: u64;
}

unsafe fn un_delay(udl: i32, ndl: i32) { if udl != 0 { udelay(udl); } if ndl != 0 { ndelay(ndl); } }

unsafe extern "C" fn ref_rcu_read_section(nloops: i32) { let mut i=nloops; while i>=0 { rcu_read_lock(); rcu_read_unlock(); i-=1; } }
unsafe extern "C" fn ref_rcu_delay_section(nloops:i32,udl:i32,ndl:i32) { let mut i=nloops; while i>=0 { rcu_read_lock(); un_delay(udl,ndl); rcu_read_unlock(); i-=1; } }
unsafe extern "C" fn rcu_sync_scale_init()->bool { true }

unsafe extern "C" fn srcu_ref_scale_read_section(nloops:i32) { let mut i=nloops; while i>=0 { let x=srcu_read_lock(srcu_ctlp); srcu_read_unlock(srcu_ctlp,x); i-=1; } }
unsafe extern "C" fn srcu_ref_scale_delay_section(nloops:i32,udl:i32,ndl:i32) { let mut i=nloops; while i>=0 { let x=srcu_read_lock(srcu_ctlp); un_delay(udl,ndl); srcu_read_unlock(srcu_ctlp,x); i-=1; } }
unsafe extern "C" fn srcu_fast_sync_scale_init()->bool { srcu_ctlp=srcu_fast_refctl_scale; true }
unsafe extern "C" fn srcu_fast_ref_scale_read_section(nloops:i32) { let mut i=nloops; while i>=0 { let x=srcu_read_lock_fast(srcu_ctlp); srcu_read_unlock_fast(srcu_ctlp,x); i-=1; } }
unsafe extern "C" fn srcu_fast_ref_scale_delay_section(nloops:i32,udl:i32,ndl:i32) { let mut i=nloops; while i>=0 { let x=srcu_read_lock_fast(srcu_ctlp); un_delay(udl,ndl); srcu_read_unlock_fast(srcu_ctlp,x); i-=1; } }
unsafe extern "C" fn srcu_fast_updown_sync_scale_init()->bool { srcu_ctlp=srcu_fast_updown_refctl_scale; true }
unsafe extern "C" fn srcu_fast_updown_ref_scale_read_section(nloops:i32) { let mut i=nloops; while i>=0 { let x=srcu_read_lock_fast_updown(srcu_ctlp); srcu_read_unlock_fast_updown(srcu_ctlp,x); i-=1; } }
unsafe extern "C" fn srcu_fast_updown_ref_scale_delay_section(nloops:i32,udl:i32,ndl:i32) { let mut i=nloops; while i>=0 { let x=srcu_read_lock_fast_updown(srcu_ctlp); un_delay(udl,ndl); srcu_read_unlock_fast_updown(srcu_ctlp,x); i-=1; } }

// Configuration-gated RCU Tasks implementations.
unsafe extern "C" fn rcu_tasks_ref_scale_read_section(nloops:i32) { let mut i=nloops; while i>=0 { i-=1; } }
unsafe extern "C" fn rcu_tasks_ref_scale_delay_section(nloops:i32,udl:i32,ndl:i32) { let mut i=nloops; while i>=0 { un_delay(udl,ndl); i-=1; } }
unsafe extern "C" fn rcu_trace_ref_scale_read_section(nloops:i32) { let mut i=nloops; while i>=0 { rcu_read_lock_trace(); rcu_read_unlock_trace(); i-=1; } }
unsafe extern "C" fn rcu_trace_ref_scale_delay_section(nloops:i32,udl:i32,ndl:i32) { let mut i=nloops; while i>=0 { rcu_read_lock_trace(); un_delay(udl,ndl); rcu_read_unlock_trace(); i-=1; } }

unsafe extern "C" fn ref_refcnt_section(nloops:i32) { let mut i=nloops; while i>=0 { atomic_inc(&mut refcnt); atomic_dec(&mut refcnt); i-=1; } }
unsafe extern "C" fn ref_refcnt_delay_section(nloops:i32,udl:i32,ndl:i32) { let mut i=nloops; while i>=0 { atomic_inc(&mut refcnt); un_delay(udl,ndl); atomic_dec(&mut refcnt); i-=1; } }
unsafe extern "C" fn ref_percpuinc_section(nloops:i32) { let mut i=nloops; while i>=0 { this_cpu_inc(&mut test_acqrel); this_cpu_dec(&mut test_acqrel); i-=1; } }
unsafe extern "C" fn ref_percpuinc_delay_section(nloops:i32,udl:i32,ndl:i32) { let mut i=nloops; while i>=0 { this_cpu_inc(&mut test_acqrel); un_delay(udl,ndl); this_cpu_dec(&mut test_acqrel); i-=1; } }

unsafe fn incpercpu(nloops:i32,udl:i32,ndl:i32, preempt:bool, bh:bool, irq:bool, delay:bool) { let mut i=nloops; while i>=0 { if preempt { preempt_disable(); } if bh { local_bh_disable(); } let mut flags=0u64; if irq { local_irq_save(&mut flags); } let p=this_cpu_ptr(&mut test_acqrel); write_once(p, read_once(p).wrapping_add(1)); if delay { un_delay(udl,ndl); } write_once(p, read_once(p).wrapping_sub(1)); if irq { local_irq_restore(flags); } if bh { local_bh_enable(); } if preempt { preempt_enable(); } i-=1; } }
unsafe extern "C" fn ref_incpercpu_section(n:i32){incpercpu(n,0,0,false,false,false,false)} unsafe extern "C" fn ref_incpercpu_delay_section(n:i32,u:i32,d:i32){incpercpu(n,u,d,false,false,false,true)}
unsafe extern "C" fn ref_incpercpupreempt_section(n:i32){incpercpu(n,0,0,true,false,false,false)} unsafe extern "C" fn ref_incpercpupreempt_delay_section(n:i32,u:i32,d:i32){incpercpu(n,u,d,true,false,false,true)}
unsafe extern "C" fn ref_incpercpubh_section(n:i32){incpercpu(n,0,0,false,true,false,false)} unsafe extern "C" fn ref_incpercpubh_delay_section(n:i32,u:i32,d:i32){incpercpu(n,u,d,false,true,false,true)}
unsafe extern "C" fn ref_incpercpuirqsave_section(n:i32){incpercpu(n,0,0,false,false,true,false)} unsafe extern "C" fn ref_incpercpuirqsave_delay_section(n:i32,u:i32,d:i32){incpercpu(n,u,d,false,false,true,true)}

unsafe extern "C" fn ref_rwlock_init()->bool { rwlock_init(&mut test_rwlock); true }
unsafe extern "C" fn ref_rwlock_section(n:i32){let mut i=n;while i>=0{read_lock(&mut test_rwlock);read_unlock(&mut test_rwlock);i-=1;}}
unsafe extern "C" fn ref_rwlock_delay_section(n:i32,u:i32,d:i32){let mut i=n;while i>=0{read_lock(&mut test_rwlock);un_delay(u,d);read_unlock(&mut test_rwlock);i-=1;}}
unsafe extern "C" fn ref_rwsem_init()->bool{init_rwsem(&mut test_rwsem);true}
unsafe extern "C" fn ref_rwsem_section(n:i32){let mut i=n;while i>=0{down_read(&mut test_rwsem);up_read(&mut test_rwsem);i-=1;}}
unsafe extern "C" fn ref_rwsem_delay_section(n:i32,u:i32,d:i32){let mut i=n;while i>=0{down_read(&mut test_rwsem);un_delay(u,d);up_read(&mut test_rwsem);i-=1;}}

// Remaining kernel declarations and operation tables retain the source-level interfaces.
extern "C" { static mut srcu_ctlp:*mut srcu_struct; static mut srcu_refctl_scale:srcu_struct; static mut srcu_fast_refctl_scale:srcu_struct; static mut srcu_fast_updown_refctl_scale:srcu_struct; static mut test_rwlock:rwlock_t; static mut test_rwsem:rw_semaphore; }
type rwlock_t=spinlock_t;
extern "C" { fn udelay(i32); fn ndelay(i32); fn rcu_read_lock(); fn rcu_read_unlock(); fn srcu_read_lock(*mut srcu_struct)->i32; fn srcu_read_unlock(*mut srcu_struct,i32); fn srcu_read_lock_fast(*mut srcu_struct)->*mut srcu_ctr; fn srcu_read_unlock_fast(*mut srcu_struct,*mut srcu_ctr); fn srcu_read_lock_fast_updown(*mut srcu_struct)->*mut srcu_ctr; fn srcu_read_unlock_fast_updown(*mut srcu_struct,*mut srcu_ctr); fn rcu_read_lock_trace(); fn rcu_read_unlock_trace(); fn atomic_inc(*mut atomic_t); fn atomic_dec(*mut atomic_t); fn this_cpu_inc(*mut u64); fn this_cpu_dec(*mut u64); fn this_cpu_ptr(*mut u64)->*mut u64; fn write_once(*mut u64,u64); fn read_once(*mut u64)->u64; fn rwlock_init(*mut rwlock_t); fn read_lock(*mut rwlock_t); fn read_unlock(*mut rwlock_t); fn init_rwsem(*mut rw_semaphore); fn down_read(*mut rw_semaphore); fn up_read(*mut rw_semaphore); fn preempt_disable(); fn preempt_enable(); fn local_bh_disable(); fn local_bh_enable(); fn local_irq_save(*mut u64); fn local_irq_restore(u64); }

// The source continues with the typesafe SLAB_BY_RCU implementation, reader kthreads,
// experiment orchestration, module initialization, and cleanup. Their declarations are
// intentionally retained for the surrounding kernel translation unit.
#[repr(C)] struct RefscaleTypesafe { rts_refctr: atomic_t, rts_lock: spinlock_t, rts_seqlock: seqlock_t, a:u32, b:u32 }
unsafe fn typesafe_ref_acquire(rtsp:*mut RefscaleTypesafe,_:*mut u32)->bool{atomic_inc_not_zero(&mut (*rtsp).rts_refctr)}
unsafe fn typesafe_ref_release(rtsp:*mut RefscaleTypesafe,_:u32)->bool{if atomic_dec_return(&mut (*rtsp).rts_refctr)==0{(*rtsp).a=(*rtsp).a.wrapping_add(1);kmem_cache_free(typesafe_kmem_cachep,rtsp as *mut _);}true}
extern "C" { static mut typesafe_kmem_cachep:*mut kmem_cache; fn atomic_inc_not_zero(*mut atomic_t)->bool; fn atomic_dec_return(*mut atomic_t)->i32; fn kmem_cache_free(*mut kmem_cache,*mut core::ffi::c_void); }

unsafe fn ref_scale_one_reader(){if readdelay<=0{((*cur_ops).readsection.unwrap())(loops)}else{((*cur_ops).delaysection.unwrap())(loops,readdelay/1000,readdelay%1000)}}
unsafe fn rcu_scale_warm_cool(){let jdone=jiffies.wrapping_add(if guest_os_delay>0{guest_os_delay as u64*HZ}else{u64::MAX});loop{ref_scale_one_reader();cond_resched();if !time_before(jiffies,jdone){break}}}
unsafe fn reset_readers(){for i in 0..nreaders{(*reader_tasks.add(i as usize)).last_duration_ns=0;}}
unsafe fn process_durations(n:i32)->u64{let mut sum=0;for i in 0..n{sum+=(*reader_tasks.add(i as usize)).last_duration_ns;}sum}
unsafe extern "C" fn ref_scale_reader(_arg:*mut core::ffi::c_void)->i32{torture_kthread_stopping(b"ref_scale_reader\0".as_ptr());0}
unsafe extern "C" fn main_func(_arg:*mut core::ffi::c_void)->i32{torture_kthread_stopping(b"main_func\0".as_ptr());0}
unsafe fn ref_scale_print_module_parms(_ops:*const RefScaleOps,_tag:*const u8){}
unsafe extern "C" fn ref_scale_cleanup(){if torture_cleanup_begin(){return} if !reader_tasks.is_null(){kfree(reader_tasks as *mut _);reader_tasks=core::ptr::null_mut();} torture_stop_kthread(b"main_task\0".as_ptr(),main_task);torture_cleanup_end();}
unsafe extern "C" fn ref_scale_init()->i32{if !torture_init_begin(scale_type,verbose){return -16} torture_init_end();0}

extern "C" { static HZ:u64; fn cond_resched(); fn time_before(u64,u64)->bool; fn torture_kthread_stopping(*const u8); fn torture_cleanup_begin()->bool; fn torture_cleanup_end(); fn torture_stop_kthread(*const u8,*mut task_struct); fn torture_init_begin(*mut u8,i32)->bool; fn torture_init_end(); fn kfree(*mut core::ffi::c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
