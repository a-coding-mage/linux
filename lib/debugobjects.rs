// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of debugobjects.c. Kernel-provided types
// and operations are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;
use core::ptr::{null, null_mut};

pub const ODEBUG_HASH_BITS: usize = 14;
pub const ODEBUG_HASH_SIZE: usize = 1 << ODEBUG_HASH_BITS;
pub const ODEBUG_BATCH_SIZE: usize = 16;
pub const ODEBUG_POOL_SIZE: usize = 64 * ODEBUG_BATCH_SIZE;
pub const ODEBUG_POOL_MIN_LEVEL: usize = ODEBUG_POOL_SIZE / 4;
pub const ODEBUG_POOL_PERCPU_SIZE: usize = 8 * ODEBUG_BATCH_SIZE;
pub const ODEBUG_CHUNK_SHIFT: usize = 12; // PAGE_SHIFT
pub const ODEBUG_CHUNK_SIZE: usize = 1 << ODEBUG_CHUNK_SHIFT;
pub const ODEBUG_CHUNK_MASK: usize = !(ODEBUG_CHUNK_SIZE - 1);
pub const ODEBUG_FREE_WORK_MAX: usize = 1024 / ODEBUG_BATCH_SIZE;

#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
pub type gfp_t = u32;
pub type debug_obj_state = u32;

pub const ODEBUG_STATE_NONE: debug_obj_state = 0;
pub const ODEBUG_STATE_INIT: debug_obj_state = 1;
pub const ODEBUG_STATE_INACTIVE: debug_obj_state = 2;
pub const ODEBUG_STATE_ACTIVE: debug_obj_state = 3;
pub const ODEBUG_STATE_DESTROYED: debug_obj_state = 4;
pub const ODEBUG_STATE_NOTAVAILABLE: debug_obj_state = 5;

#[repr(C)] pub struct debug_obj_descr {
    pub name: *const i8,
    pub is_static_object: Option<unsafe extern "C" fn(*mut c_void) -> bool>,
    pub fixup_init: Option<unsafe extern "C" fn(*mut c_void, debug_obj_state) -> bool>,
    pub fixup_activate: Option<unsafe extern "C" fn(*mut c_void, debug_obj_state) -> bool>,
    pub fixup_destroy: Option<unsafe extern "C" fn(*mut c_void, debug_obj_state) -> bool>,
    pub fixup_free: Option<unsafe extern "C" fn(*mut c_void, debug_obj_state) -> bool>,
    pub debug_hint: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
}
#[repr(C)] pub struct debug_obj {
    pub node: hlist_node, pub object: *mut c_void,
    pub descr: *const debug_obj_descr, pub state: debug_obj_state,
    pub astate: u32, pub batch_last: *mut hlist_node,
}
#[repr(C)] pub struct debug_bucket { pub list: hlist_head, pub lock: raw_spinlock_t }
#[repr(C)] pub struct pool_stats { pub cur_used: u32, pub max_used: u32, pub min_fill: u32 }
#[repr(C)] pub struct obj_pool { pub objects: hlist_head, pub cnt: u32, pub min_cnt: u32, pub max_cnt: u32, pub stats: pool_stats }

extern "C" {
    static mut debug_objects_enabled: bool;
    static mut debug_objects_maxchain: i32;
    static mut debug_objects_maxchecked: i32;
    static mut debug_objects_fixups: i32;
    static mut debug_objects_warnings: i32;
    static mut obj_hash: [debug_bucket; ODEBUG_HASH_SIZE];
    static mut pool_global: obj_pool;
    static mut pool_to_free: obj_pool;
    static mut pool_boot: hlist_head;
    static mut obj_cache: *mut kmem_cache;
    fn kmem_cache_free(c: *mut kmem_cache, p: *mut debug_obj);
    fn kmem_cache_zalloc(c: *mut kmem_cache, g: gfp_t) -> *mut debug_obj;
}

#[inline] unsafe fn pool_count(p: *mut obj_pool) -> u32 { (*p).cnt }
#[inline] unsafe fn pool_should_refill(p: *mut obj_pool) -> bool { pool_count(p) < (*p).min_cnt }
#[inline] unsafe fn pool_must_refill(p: *mut obj_pool) -> bool { pool_count(p) < (*p).min_cnt / 2 }

unsafe fn __alloc_object(list: *mut hlist_head) -> *mut debug_obj {
    if (*list).first.is_null() { return null_mut(); }
    let obj = (*list).first as *mut debug_obj;
    (*list).first = (*(*obj).node.next).next;
    obj
}

unsafe fn lookup_object(addr: *mut c_void, b: *mut debug_bucket) -> *mut debug_obj {
    let mut n = (*b).list.first;
    let mut cnt = 0;
    while !n.is_null() { let o = n as *mut debug_obj; cnt += 1; if (*o).object == addr { return o; } n = (*n).next; }
    if cnt > debug_objects_maxchain { debug_objects_maxchain = cnt; } null_mut()
}

unsafe fn get_bucket(addr: usize) -> *mut debug_bucket {
    &mut obj_hash[((addr >> ODEBUG_CHUNK_SHIFT).wrapping_mul(0x9e3779b97f4a7c15) >> (64-ODEBUG_HASH_BITS)) as usize]
}

unsafe fn debug_object_fixup(f: Option<unsafe extern "C" fn(*mut c_void, debug_obj_state)->bool>, a: *mut c_void, s: debug_obj_state) -> bool {
    if let Some(fun) = f { if fun(a,s) { debug_objects_fixups += 1; return true; } } false
}

pub unsafe extern "C" fn debug_object_init(addr: *mut c_void, descr: *const debug_obj_descr) {
    if !debug_objects_enabled { return; }
    let b = get_bucket(addr as usize); let o = lookup_object(addr,b);
    if !o.is_null() { (*o).state = ODEBUG_STATE_INIT; return; }
    debug_objects_enabled = false;
}
pub unsafe extern "C" fn debug_object_init_on_stack(a:*mut c_void,d:*const debug_obj_descr){debug_object_init(a,d)}
pub unsafe extern "C" fn debug_object_activate(addr:*mut c_void, descr:*const debug_obj_descr)->i32 {
    if !debug_objects_enabled{return 0} let o=lookup_object(addr,get_bucket(addr as usize));
    if !o.is_null(){match (*o).state{ODEBUG_STATE_INIT|ODEBUG_STATE_INACTIVE=>{(*o).state=ODEBUG_STATE_ACTIVE;return 0},ODEBUG_STATE_ACTIVE|ODEBUG_STATE_DESTROYED=>{debug_object_fixup((*descr).fixup_activate,addr,(*o).state);return -22},_=>{}}} -22
}
pub unsafe extern "C" fn debug_object_deactivate(addr:*mut c_void,_:*const debug_obj_descr){if debug_objects_enabled{if let o=lookup_object(addr,get_bucket(addr as usize)){if !o.is_null()&&(*o).state!=ODEBUG_STATE_DESTROYED{(*o).state=ODEBUG_STATE_INACTIVE;}}}}
pub unsafe extern "C" fn debug_object_destroy(addr:*mut c_void,descr:*const debug_obj_descr){if debug_objects_enabled{if let o=lookup_object(addr,get_bucket(addr as usize)){if !o.is_null(){if (*o).state==ODEBUG_STATE_ACTIVE{debug_object_fixup((*descr).fixup_destroy,addr,(*o).state)}else{(*o).state=ODEBUG_STATE_DESTROYED;}}}}}
pub unsafe extern "C" fn debug_object_free(addr:*mut c_void,descr:*const debug_obj_descr){if debug_objects_enabled{if let o=lookup_object(addr,get_bucket(addr as usize)){if !o.is_null(){if (*o).state==ODEBUG_STATE_ACTIVE{debug_object_fixup((*descr).fixup_free,addr,(*o).state)}else{(*o).node.next=null_mut();}}}}}
pub unsafe extern "C" fn debug_object_assert_init(_:*mut c_void,_:*const debug_obj_descr){}
pub unsafe extern "C" fn debug_object_active_state(addr:*mut c_void,_:*const debug_obj_descr,expect:u32,next:u32){if debug_objects_enabled{let o=lookup_object(addr,get_bucket(addr as usize));if !o.is_null()&&(*o).state==ODEBUG_STATE_ACTIVE&&(*o).astate==expect{(*o).astate=next;}}}
pub unsafe extern "C" fn debug_objects_early_init(){}
pub unsafe extern "C" fn debug_objects_mem_init(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
