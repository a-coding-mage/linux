/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/rhashtable.h.  Types and primitives supplied by
 * the surrounding kernel translation are intentionally left external. */

use core::{mem, ptr};

pub const RHT_ELASTICITY: u32 = 16;

#[repr(C)]
pub struct rhash_lock_head { _private: [u8; 0] }

#[repr(C)]
pub struct bucket_table {
    pub size: u32,
    pub nest: u32,
    pub hash_rnd: u32,
    pub walkers: list_head,
    pub rcu: rcu_head,
    pub future_tbl: *mut bucket_table,
    pub dep_map: lockdep_map,
    pub buckets: [*mut rhash_lock_head; 0],
}

extern "C" {
    pub fn NULLS_MARKER(x: usize) -> *mut core::ffi::c_void;
    pub fn jhash(key: *const core::ffi::c_void, len: u32, init: u32) -> u32;
    pub fn jhash2(key: *const u32, len: u32, init: u32) -> u32;
    pub fn likely(x: bool) -> bool;
    pub fn unlikely(x: bool) -> bool;
    pub fn atomic_read(x: *const atomic_t) -> i32;
    pub fn atomic_inc(x: *mut atomic_t);
    pub fn atomic_dec(x: *mut atomic_t);
    pub fn memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, n: usize) -> i32;
    pub fn rcu_read_lock(); pub fn rcu_read_unlock();
    pub fn rcu_dereference_all<T>(p: T) -> T;
    pub fn rcu_dereference_protected<T>(p: T, c: bool) -> T;
    pub fn rcu_assign_pointer<T>(p: *mut T, v: T);
    pub fn bit_spin_lock(n: u32, p: *mut usize); pub fn bit_spin_unlock(n: u32, p: *mut usize);
    pub fn local_irq_save(flags: *mut usize); pub fn local_irq_restore(flags: usize);
    pub fn lock_map_acquire(p: *mut lockdep_map); pub fn lock_map_release(p: *mut lockdep_map);
    pub fn lock_acquire_exclusive(p: *mut lockdep_map, subclass: u32, trylock: u32, a: *const core::ffi::c_void, ip: usize);
    pub fn preempt_enable(); pub fn irq_work_queue(p: *mut irq_work);
    pub fn rhashtable_insert_slow(ht: *mut rhashtable, key: *const core::ffi::c_void, obj: *mut rhash_head) -> *mut core::ffi::c_void;
    pub fn rhashtable_walk_enter(ht: *mut rhashtable, iter: *mut rhashtable_iter);
    pub fn rhashtable_walk_exit(iter: *mut rhashtable_iter);
    pub fn rhashtable_walk_start_check(iter: *mut rhashtable_iter) -> i32;
    pub fn rhashtable_walk_next(iter: *mut rhashtable_iter) -> *mut core::ffi::c_void;
    pub fn rhashtable_walk_peek(iter: *mut rhashtable_iter) -> *mut core::ffi::c_void;
    pub fn rhashtable_walk_stop(iter: *mut rhashtable_iter);
    pub fn rhashtable_free_and_destroy(ht: *mut rhashtable, f: Option<unsafe extern "C" fn(*mut core::ffi::c_void,*mut core::ffi::c_void)>, arg: *mut core::ffi::c_void);
    pub fn rhashtable_destroy(ht: *mut rhashtable);
    pub fn rhashtable_next_key(ht: *mut rhashtable, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

/* External kernel structures. */
#[allow(non_camel_case_types)] pub type u32_t = u32;
#[allow(non_camel_case_types)] pub type atomic_t = i32;
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: Option<unsafe extern "C" fn(*mut rcu_head)> }
#[repr(C)] pub struct lockdep_map { _private: [u8; 0] }
#[repr(C)] pub struct irq_work { _private: [u8; 0] }
#[repr(C)] pub struct rhash_head { pub next: *mut rhash_head }
#[repr(C)] pub struct rhashtable_params { pub key_len: u32, pub key_offset: usize, pub head_offset: usize, pub max_size: u32, pub min_size: u32, pub insecure_elasticity: bool, pub automatic_shrinking: bool, pub hashfn: Option<unsafe extern "C" fn(*const core::ffi::c_void,u32,u32)->u32>, pub obj_hashfn: Option<unsafe extern "C" fn(*const core::ffi::c_void,u32,u32)->u32>, pub obj_cmpfn: Option<unsafe extern "C" fn(*mut rhashtable_compare_arg,*const core::ffi::c_void)->bool> }
#[repr(C)] pub struct rhashtable { pub p: rhashtable_params, pub tbl: *mut bucket_table, pub nelems: atomic_t, pub key_len: u32, pub max_elems: u32, pub run_irq_work: irq_work }
#[repr(C)] pub struct rhltable { pub ht: rhashtable }
#[repr(C)] pub struct rhlist_head { pub rhead: rhash_head, pub next: *mut rhlist_head }
#[repr(C)] pub struct rhashtable_iter { _private: [u8; 0] }
#[repr(C)] pub struct rhashtable_compare_arg { pub ht: *mut rhashtable, pub key: *const core::ffi::c_void }

#[inline] pub unsafe fn rht_is_a_nulls(p: *const rhash_head) -> bool { (p as usize & 1) != 0 }
#[inline] pub unsafe fn rht_obj(ht: *const rhashtable, he: *const rhash_head) -> *mut u8 { (he as *mut u8).sub((*ht).p.head_offset) }
#[inline] pub unsafe fn rht_bucket_index(tbl: *const bucket_table, hash: u32) -> u32 { hash & ((*tbl).size - 1) }
#[inline] pub unsafe fn rht_key_get_hash(ht: *mut rhashtable, key: *const core::ffi::c_void, p: rhashtable_params, rnd: u32) -> u32 { if let Some(f)=p.hashfn { f(key, if p.key_len!=0 {p.key_len} else {(*ht).p.key_len}, rnd) } else if p.key_len!=0 { jhash(key,p.key_len,rnd) } else { jhash(key,(*ht).p.key_len,rnd) } }
#[inline] pub unsafe fn rht_key_hashfn(ht:*mut rhashtable,tbl:*const bucket_table,key:*const core::ffi::c_void,p:rhashtable_params)->u32 { rht_bucket_index(tbl,rht_key_get_hash(ht,key,p,(*tbl).hash_rnd)) }
#[inline] pub unsafe fn rht_grow_above_75(ht:*const rhashtable,tbl:*const bucket_table)->bool { atomic_read(&(*ht).nelems) > ((*tbl).size/4*3) as i32 && ((*ht).p.max_size==0 || (*tbl).size<(*ht).p.max_size) }
#[inline] pub unsafe fn rht_shrink_below_30(ht:*const rhashtable,tbl:*const bucket_table)->bool { atomic_read(&(*ht).nelems) < ((*tbl).size*3/10) as i32 && (*tbl).size>(*ht).p.min_size }
#[inline] pub unsafe fn rht_grow_above_100(ht:*const rhashtable,tbl:*const bucket_table)->bool { atomic_read(&(*ht).nelems) > (*tbl).size as i32 && ((*ht).p.max_size==0 || (*tbl).size<(*ht).p.max_size) }
#[inline] pub unsafe fn rht_grow_above_max(ht:*const rhashtable,tbl:*const bucket_table)->bool { atomic_read(&(*ht).nelems) >= (*ht).max_elems as i32 }

extern "C" { pub fn rht_bucket_nested(t:*const bucket_table,h:u32)->*mut *mut rhash_lock_head; pub fn __rht_bucket_nested(t:*const bucket_table,h:u32)->*mut *mut rhash_lock_head; pub fn rht_bucket_nested_insert(ht:*mut rhashtable,t:*mut bucket_table,h:u32)->*mut *mut rhash_lock_head; }
#[inline] pub unsafe fn rht_bucket(tbl:*const bucket_table,hash:u32)->*mut *mut rhash_lock_head { if (*tbl).nest!=0 {rht_bucket_nested(tbl,hash)} else {(*tbl).buckets.as_ptr().add(hash as usize) as *mut _} }
#[inline] pub unsafe fn rht_bucket_var(tbl:*mut bucket_table,hash:u32)->*mut *mut rhash_lock_head { rht_bucket(tbl,hash) }
#[inline] pub unsafe fn rht_ptr(b:*mut *mut rhash_lock_head)->*mut rhash_head { ((*b as usize)&!1) as *mut rhash_head }
#[inline] pub unsafe fn rhashtable_compare(a:*mut rhashtable_compare_arg,obj:*const core::ffi::c_void)->i32 { memcmp((obj as *const u8).add((*(*a).ht).p.key_offset) as _,(*a).key,(*(*a).ht).p.key_len as usize) }

/* C iteration macros are represented by their direct Rust loop idiom; callers
 * should use rht_ptr/rht_is_a_nulls and dereference next under RCU. */
#[inline] pub unsafe fn rhashtable_lookup(ht:*mut rhashtable,key:*const core::ffi::c_void,p:rhashtable_params)->*mut core::ffi::c_void { let mut t=(*ht).tbl; loop { let h=rht_key_hashfn(ht,t,key,p); let mut n=rht_ptr(rht_bucket(t,h)); while !rht_is_a_nulls(n) { let o=rht_obj(ht,n); let a=rhashtable_compare(&mut rhashtable_compare_arg{ht,key},o); if a==0{return o as _} n=(*n).next; } if (*t).future_tbl.is_null(){return ptr::null_mut()} t=(*t).future_tbl; } }
#[inline] pub unsafe fn rhashtable_lookup_fast(ht:*mut rhashtable,key:*const core::ffi::c_void,p:rhashtable_params)->*mut core::ffi::c_void { rcu_read_lock(); let r=rhashtable_lookup(ht,key,p); rcu_read_unlock(); r }
#[inline] pub unsafe fn rhltable_lookup(h:*mut rhltable,k:*const core::ffi::c_void,p:rhashtable_params)->*mut rhlist_head { rhashtable_lookup(&mut (*h).ht,k,p) as *mut rhash_head as *mut rhlist_head }
#[inline] pub unsafe fn rhltable_walk_enter(h:*mut rhltable,i:*mut rhashtable_iter){rhashtable_walk_enter(&mut (*h).ht,i)}
#[inline] pub unsafe fn rhltable_free_and_destroy(h:*mut rhltable,f:Option<unsafe extern "C" fn(*mut core::ffi::c_void,*mut core::ffi::c_void)>,a:*mut core::ffi::c_void){rhashtable_free_and_destroy(&mut (*h).ht,f,a)}
#[inline] pub unsafe fn rhltable_destroy(h:*mut rhltable){rhltable_free_and_destroy(h,None,ptr::null_mut())}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
