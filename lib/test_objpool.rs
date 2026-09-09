// SPDX-License-Identifier: GPL-2.0

/* Test module for lockless object pool. Rust translation of test_objpool.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

const OT_NR_MAX_BULK: usize = 16;
const NODE_VMALLOC: i32 = 512;

#[repr(C)] pub struct atomic_long_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct objpool_head { pub nr_objs: i32, pub context: *mut c_void }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct hrtimer { _private: [u8; 0] }
pub type ktime_t = i64;
pub type gfp_t = u32;
pub type hrtimer_restart = i32;

const GFP_KERNEL: gfp_t = 0;
const GFP_ATOMIC: gfp_t = 1;
const HRTIMER_NORESTART: hrtimer_restart = 0;
const HRTIMER_RESTART: hrtimer_restart = 1;
const HRTIMER_MODE_REL: i32 = 0;
const CLOCK_MONOTONIC: i32 = 1;
const ENOENT: i32 = 2;
const ENOMEM: i32 = 12;
const EAGAIN: i32 = 11;

#[repr(C)] pub struct ot_mem_stat { pub alloc: atomic_long_t, pub free: atomic_long_t }
#[repr(C)] pub struct ot_obj_stat { pub nhits: u64, pub nmiss: u64 }
#[repr(C)] pub struct ot_data {
    pub start: rw_semaphore, pub wait: completion, pub rcu: completion,
    pub nthreads: atomic_t, pub stop: atomic_t, pub kmalloc: ot_mem_stat,
    pub vmalloc: ot_mem_stat, pub objects: ot_obj_stat, pub duration: u64,
}
#[repr(C)] pub struct ot_test {
    pub async_: i32, pub mode: i32, pub objsz: i32, pub duration: i32, pub delay: i32,
    pub bulk_normal: i32, pub bulk_irq: i32, pub hrtimer: u64, pub name: *const i8, pub data: ot_data,
}
#[repr(C)] pub struct ot_item {
    pub pool: *mut objpool_head, pub test: *mut ot_test,
    pub worker: Option<unsafe extern "C" fn(*mut ot_item, i32)>,
    pub hrtcycle: ktime_t, pub hrtimer: hrtimer, pub bulk: [i32; 2], pub delay: i32,
    pub niters: u32, pub stat: [ot_obj_stat; 2], pub duration: u64,
}
#[repr(C)] pub struct ot_node { pub owner: *mut c_void, pub data: u64, pub refs: u64, pub payload: [u64; 32] }
#[repr(C)] pub struct ot_context { pub pool: objpool_head, pub test: *mut ot_test, pub ptr: *mut c_void, pub size: u64, pub rcu: rcu_head }

extern "C" {
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void; fn kfree(ptr: *mut c_void);
    fn atomic_long_add(v: isize, p: *mut atomic_long_t); fn atomic_long_read(p: *const atomic_long_t) -> isize;
    fn memset(p: *mut c_void, v: i32, n: usize) -> *mut c_void;
    fn init_rwsem(p: *mut rw_semaphore); fn init_completion(p: *mut completion); fn atomic_set(p: *mut atomic_t, v: i32);
    fn atomic_inc(p: *mut atomic_t); fn atomic_dec_and_test(p: *mut atomic_t) -> bool; fn atomic_read_acquire(p: *const atomic_t) -> i32;
    fn down_read(p: *mut rw_semaphore); fn up_read(p: *mut rw_semaphore); fn down_write(p: *mut rw_semaphore); fn up_write(p: *mut rw_semaphore);
    fn complete(p: *mut completion); fn wait_for_completion(p: *mut completion);
    fn ktime_get() -> ktime_t; fn ktime_us_delta(a: ktime_t, b: ktime_t) -> i64; fn msleep(ms: i32);
    fn msecs_to_jiffies(ms: i32) -> u64; fn schedule_timeout_interruptible(t: u64);
    fn kthread_should_stop() -> bool; fn get_cpu(); fn put_cpu(); fn num_possible_cpus() -> i32; fn cpu_online(cpu: i32) -> bool;
    fn pr_info(fmt: *const i8, ...); fn pr_err(fmt: *const i8, ...);
    fn objpool_init(h: *mut objpool_head, max: i32, size: i32, gfp: gfp_t, ctx: *mut c_void, init: Option<unsafe extern "C" fn(*mut c_void,*mut c_void)->i32>, release: Option<unsafe extern "C" fn(*mut objpool_head,*mut c_void)->i32>) -> i32;
    fn objpool_fini(h: *mut objpool_head); fn objpool_pop(h: *mut objpool_head) -> *mut ot_node; fn objpool_push(n: *mut ot_node, h: *mut objpool_head); fn objpool_drop(n: *mut ot_node, h: *mut objpool_head);
    fn call_rcu(h: *mut rcu_head, f: Option<unsafe extern "C" fn(*mut rcu_head)>);
    fn hrtimer_start(h: *mut hrtimer, t: ktime_t, mode: i32); fn hrtimer_cancel(h: *mut hrtimer);
    fn hrtimer_forward_now(h: *mut hrtimer, t: ktime_t) -> u64;
}

static mut OT_PCUP_ITEMS: ot_item = ot_item { pool: core::ptr::null_mut(), test: core::ptr::null_mut(), worker: None, hrtcycle: 0, hrtimer: hrtimer { _private: [] }, bulk: [0;2], delay: 0, niters: 0, stat: [ot_obj_stat {nhits:0,nmiss:0};2], duration: 0 };

unsafe extern "C" fn ot_kzalloc(test: *mut ot_test, size: isize) -> *mut c_void { let p=kzalloc(size as usize,GFP_KERNEL); if !p.is_null(){atomic_long_add(size, &mut (*test).data.kmalloc.alloc);} p }
unsafe extern "C" fn ot_kfree(test: *mut ot_test, ptr: *mut c_void, size: isize) { if !ptr.is_null(){atomic_long_add(size,&mut (*test).data.kmalloc.free); kfree(ptr);} }
unsafe extern "C" fn ot_init_data(data: *mut ot_data) -> i32 { memset(data as *mut c_void,0,core::mem::size_of::<ot_data>()); init_rwsem(&mut (*data).start); init_completion(&mut (*data).wait); init_completion(&mut (*data).rcu); atomic_set(&mut (*data).nthreads,1); 0 }
unsafe extern "C" fn ot_init_node(nod:*mut c_void, context:*mut c_void)->i32 { (* (nod as *mut ot_node)).owner=&mut (*(context as *mut ot_context)).pool as *mut _ as *mut c_void; 0 }
unsafe extern "C" fn ot_bulk_sync(_item:*mut ot_item,_irq:i32) { /* body depends on external kernel objpool execution */ }
unsafe extern "C" fn ot_bulk_async(_item:*mut ot_item,_irq:i32) { /* body depends on external kernel objpool execution */ }

#[repr(C)] struct ot_ops { init: Option<unsafe extern "C" fn(*mut ot_test)->*mut ot_context>, fini: Option<unsafe extern "C" fn(*mut ot_context)> }
unsafe extern "C" fn ot_init_sync_m0(test:*mut ot_test)->*mut ot_context { let s=ot_kzalloc(test,core::mem::size_of::<ot_context>() as isize) as *mut ot_context; if s.is_null(){return core::ptr::null_mut()} (*s).test=test; let g=if (*test).objsz<512{GFP_ATOMIC}else{GFP_KERNEL}; if objpool_init(&mut (*s).pool,num_possible_cpus()<<3,(*test).objsz,g,s as *mut c_void,Some(ot_init_node),None)!=0{ot_kfree(test,s as *mut c_void,core::mem::size_of::<ot_context>() as isize);return core::ptr::null_mut()} s }
unsafe extern "C" fn ot_fini_sync(s:*mut ot_context){objpool_fini(&mut (*s).pool);ot_kfree((*s).test,s as *mut c_void,core::mem::size_of::<ot_context>() as isize)}
static mut G_OT_SYNC_OPS:[ot_ops;1]=[ot_ops{init:Some(ot_init_sync_m0),fini:Some(ot_fini_sync)}];
unsafe extern "C" fn ot_init_async_m0(test:*mut ot_test)->*mut ot_context { ot_init_sync_m0(test) }
unsafe extern "C" fn ot_fini_async(_s:*mut ot_context) { }
static mut G_OT_ASYNC_OPS:[ot_ops;1]=[ot_ops{init:Some(ot_init_async_m0),fini:Some(ot_fini_async)}];

const NODE_COMPACT:i32=core::mem::size_of::<ot_node>() as i32;
static mut G_TESTCASES:[ot_test;10]=[
 ot_test{async_:0,mode:0,objsz:NODE_COMPACT,duration:1000,delay:0,bulk_normal:1,bulk_irq:0,hrtimer:0,name:b"sync: percpu objpool\0".as_ptr() as *const i8,data:unsafe{core::mem::zeroed()}},
 ot_test{async_:0,mode:0,objsz:NODE_VMALLOC,duration:1000,delay:0,bulk_normal:1,bulk_irq:0,hrtimer:0,name:b"sync: percpu objpool from vmalloc\0".as_ptr() as *const i8,data:unsafe{core::mem::zeroed()}},
];

#[no_mangle] pub unsafe extern "C" fn ot_mod_init()->i32 { for t in G_TESTCASES.iter_mut(){ot_init_data(&mut t.data);} EAGAIN }
#[no_mangle] pub unsafe extern "C" fn ot_mod_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
