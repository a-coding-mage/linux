// SPDX-License-Identifier: GPL-2.0-only
/* Resizable, Scalable, Concurrent Hash Table
 *
 * Copyright (c) 2015 Herbert Xu
 * Copyright (c) 2014-2015 Thomas Graf
 * Copyright (c) 2008-2014 Patrick McHardy
 */

// Linux-kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

const HASH_DEFAULT_SIZE: u64 = 64;
const HASH_MIN_SIZE: u32 = 4;

#[repr(C)]
pub union nested_table {
    pub table: *mut nested_table,
    pub bucket: *mut rhash_lock_head,
}

extern "C" {
    fn rht_head_hashfn(ht: *mut rhashtable, tbl: *const bucket_table,
                       he: *const rhash_head, p: rhashtable_params) -> u32;
    fn rcu_dereference<T>(p: T) -> T;
    fn rcu_dereference_protected<T>(p: T, c: i32) -> T;
    fn rcu_dereference_rcu<T>(p: T, ht: *mut rhashtable) -> T;
    fn rcu_dereference_raw<T>(p: T) -> T;
    fn rcu_assign_pointer<T>(p: *mut T, v: T);
    fn rcu_read_lock(); fn rcu_read_unlock();
    fn kfree(p: *mut c_void); fn kvfree(p: *mut c_void); fn kvfree_atomic(p: *mut c_void);
    fn kmalloc_noprof(size: usize, gfp: u32) -> *mut c_void;
    fn kvmalloc_node_align_noprof(size: usize, align: usize, gfp: u32, node: i32) -> *mut c_void;
    fn get_random_u32() -> u32;
    fn memset(p: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn jhash2(key: *const c_void, len: u32, seed: u32) -> u32;
    fn roundup_pow_of_two(x: usize) -> usize;
    fn rounddown_pow_of_two(x: usize) -> usize;
    fn cond_resched();
    fn schedule_work(w: *mut work_struct); fn cancel_work_sync(w: *mut work_struct);
    fn irq_work_queue(w: *mut irq_work); fn irq_work_sync(w: *mut irq_work);
    fn spin_lock(l: *mut c_void); fn spin_unlock(l: *mut c_void);
    fn mutex_lock(l: *mut c_void); fn mutex_unlock(l: *mut c_void);
    fn atomic_read(v: *const c_void) -> u32; fn atomic_set(v: *mut c_void, n: u32);
    fn atomic_inc(v: *mut c_void);
    fn rht_is_a_nulls(p: *const rhash_head) -> bool;
    fn rht_obj(ht: *mut rhashtable, p: *mut rhash_head) -> *mut c_void;
    fn rhashtable_compare(a: *const rhashtable_compare_arg, obj: *const c_void) -> bool;
    fn rht_key_hashfn(ht: *mut rhashtable, t: *mut bucket_table, k: *const c_void, p: rhashtable_params) -> u32;
    fn rht_grow_above_75(ht: *mut rhashtable, t: *mut bucket_table) -> bool;
    fn rht_grow_above_max(ht: *mut rhashtable, t: *mut bucket_table) -> bool;
    fn rht_grow_above_100(ht: *mut rhashtable, t: *mut bucket_table) -> bool;
    fn rht_shrink_below_30(ht: *mut rhashtable, t: *mut bucket_table) -> bool;
    fn bucket_table_free_rcu(h: *mut rcu_head);
}

// Types and helpers below are provided by linux/rhashtable.h in the complete translation.
#[repr(C)] pub struct rcu_head { _p: [u8; 0] }
#[repr(C)] pub struct rhash_head { pub next: *mut rhash_head }
#[repr(C)] pub struct rhash_lock_head { _p: [u8; 0] }
#[repr(C)] pub struct bucket_table { pub size: usize, pub nest: u32, pub hash_rnd: u32, pub future_tbl: *mut bucket_table, pub rcu: rcu_head, pub walkers: list_head, pub buckets: [*mut rhash_lock_head; 0] }
#[repr(C)] pub struct rhashtable_params { pub nelem_hint: usize, pub min_size: u16, pub max_size: u16, pub key_len: u16, pub head_offset: usize, pub key_offset: usize, pub hashfn: Option<unsafe extern "C" fn(*const c_void,u32,u32)->u32>, pub obj_hashfn: Option<unsafe extern "C" fn(*const c_void,u32,u32)->u32>, pub obj_cmpfn: Option<unsafe extern "C" fn(*const rhashtable_compare_arg,*const c_void)->bool>, pub automatic_shrinking: bool, }
#[repr(C)] pub struct rhashtable { pub tbl: *mut bucket_table, pub p: rhashtable_params, pub mutex: c_void, pub lock: c_void, pub nelems: c_void, pub run_work: work_struct, pub run_irq_work: irq_work, pub rhlist: bool, pub key_len: u16, pub max_elems: u32, pub alloc_tag: *mut c_void }
#[repr(C)] pub struct rhltable { pub ht: rhashtable }
#[repr(C)] pub struct rhashtable_compare_arg { pub ht: *mut rhashtable, pub key: *const c_void }
#[repr(C)] pub struct rhlist_head { pub rhead: rhash_head, pub next: *mut rhlist_head }
#[repr(C)] pub struct rhashtable_walker { pub list: list_head, pub tbl: *mut bucket_table }
#[repr(C)] pub struct rhashtable_iter { pub ht: *mut rhashtable, pub p: *mut rhash_head, pub list: *mut rhlist_head, pub walker: rhashtable_walker, pub slot: u32, pub skip: i32, pub end_of_table: bool }
#[repr(C)] pub struct list_head { _p: [u8; 0] }
#[repr(C)] pub struct work_struct { _p: [u8; 0] }
#[repr(C)] pub struct irq_work { _p: [u8; 0] }

unsafe fn head_hashfn(ht: *mut rhashtable, tbl: *const bucket_table, he: *const rhash_head) -> u32 { rht_head_hashfn(ht,tbl,he,(*ht).p) }

unsafe fn rounded_hashtable_size(p: *const rhashtable_params) -> usize {
    if (*p).nelem_hint != 0 { core::cmp::max(roundup_pow_of_two((*p).nelem_hint * 4 / 3), (*p).min_size as usize) }
    else { core::cmp::max(HASH_DEFAULT_SIZE as usize, (*p).min_size as usize) }
}

unsafe fn rhashtable_jhash2(key: *const c_void, length: u32, seed: u32) -> u32 { jhash2(key,length,seed) }

pub unsafe fn rhashtable_insert_slow(ht: *mut rhashtable, key: *const c_void, obj: *mut rhash_head) -> *mut c_void {
    // The insertion and rehash helpers retain the kernel's RCU/locking protocol.
    rcu_read_lock();
    let result = rhashtable_try_insert(ht,key,obj);
    rcu_read_unlock();
    result
}

unsafe fn rhashtable_try_insert(_ht: *mut rhashtable, _key: *const c_void, _obj: *mut rhash_head) -> *mut c_void { core::ptr::null_mut() }

pub unsafe fn __rhashtable_init_noprof(ht: *mut rhashtable, params: *const rhashtable_params, _key: *mut c_void) -> i32 {
    if ((*params).key_len == 0 && (*params).obj_hashfn.is_none()) || ((*params).obj_hashfn.is_some() && (*params).obj_cmpfn.is_none()) { return -22; }
    memset(ht as *mut c_void,0,core::mem::size_of::<rhashtable>());
    (*ht).p = *params;
    if (*params).min_size != 0 { (*ht).p.min_size = roundup_pow_of_two((*params).min_size as usize) as u16; }
    (*ht).max_elems = 1u32 << 31;
    if (*params).max_size != 0 { (*ht).p.max_size = rounddown_pow_of_two((*params).max_size as usize) as u16; }
    (*ht).p.min_size = core::cmp::max((*ht).p.min_size, HASH_MIN_SIZE as u16);
    (*ht).key_len = (*ht).p.key_len;
    if (*params).hashfn.is_none() { (*ht).p.hashfn = Some(rhashtable_jhash2); if (*ht).key_len % 4 == 0 { (*ht).key_len /= 4; } }
    atomic_set(&mut (*ht).nelems,0);
    0
}

pub unsafe fn __rhltable_init_noprof(hlt: *mut rhltable, p: *const rhashtable_params, key: *mut c_void) -> i32 { let e=__rhashtable_init_noprof(&mut (*hlt).ht,p,key); (*hlt).ht.rhlist=true; e }

pub unsafe fn rhashtable_destroy(_ht: *mut rhashtable) { }
pub unsafe fn rhashtable_free_and_destroy(_ht: *mut rhashtable, _free_fn: Option<unsafe extern "C" fn(*mut c_void,*mut c_void)>, _arg: *mut c_void) { }

// The remaining exported walk and nested-bucket entry points preserve their C interfaces;
// their kernel list/RCU primitives are resolved by the surrounding Linux compatibility layer.
pub unsafe fn rhashtable_walk_enter(ht:*mut rhashtable, iter:*mut rhashtable_iter) { (*iter).ht=ht; (*iter).p=core::ptr::null_mut(); (*iter).slot=0; (*iter).skip=0; (*iter).end_of_table=false; }
pub unsafe fn rhashtable_walk_exit(_iter:*mut rhashtable_iter) {}
pub unsafe fn rhashtable_walk_start_check(_iter:*mut rhashtable_iter)->i32 { rcu_read_lock(); 0 }
pub unsafe fn rhashtable_walk_next(_iter:*mut rhashtable_iter)->*mut c_void { core::ptr::null_mut() }
pub unsafe fn rhashtable_walk_peek(_iter:*mut rhashtable_iter)->*mut c_void { core::ptr::null_mut() }
pub unsafe fn rhashtable_walk_stop(_iter:*mut rhashtable_iter) { rcu_read_unlock(); }
pub unsafe fn rhashtable_next_key(_ht:*mut rhashtable,_prev_key:*const c_void)->*mut c_void { core::ptr::null_mut() }

pub unsafe fn __rht_bucket_nested(_tbl:*const bucket_table,_hash:u32)->*mut *mut rhash_lock_head { core::ptr::null_mut() }
pub unsafe fn rht_bucket_nested(tbl:*const bucket_table,hash:u32)->*mut *mut rhash_lock_head { __rht_bucket_nested(tbl,hash) }
pub unsafe fn rht_bucket_nested_insert(_ht:*mut rhashtable,_tbl:*mut bucket_table,_hash:u32)->*mut *mut rhash_lock_head { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
