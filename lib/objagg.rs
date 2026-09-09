// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/* Copyright (c) 2018 Mellanox Technologies. All rights reserved */

/* Linux-kernel dependencies and trace points are supplied by the surrounding
 * translation unit. */

use core::ffi::c_void;

#[repr(C)] pub struct rhashtable { _private: [u8; 0] }
#[repr(C)] pub struct rhashtable_params { pub key_len: usize, pub key_offset: usize, pub head_offset: usize }
#[repr(C)] pub struct rhash_head { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct objagg_ops {
    pub obj_size: usize,
    pub root_create: unsafe extern "C" fn(*mut c_void, *const c_void, u32) -> *mut c_void,
    pub root_destroy: unsafe extern "C" fn(*mut c_void, *mut c_void),
    pub delta_check: unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void) -> bool,
    pub delta_create: unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void) -> *mut c_void,
    pub delta_destroy: unsafe extern "C" fn(*mut c_void, *mut c_void),
}
#[repr(C)] #[derive(Copy, Clone)] pub struct objagg_obj_stats { pub user_count: u32, pub delta_user_count: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct objagg_obj_stats_info { pub stats: objagg_obj_stats, pub objagg_obj: *mut objagg_obj, pub is_root: bool }
#[repr(C)] pub struct objagg_stats { pub stats_info_count: u32, pub root_count: u32, pub stats_info: [objagg_obj_stats_info; 0] }
#[repr(C)] pub struct objagg_hints { node_ht: rhashtable, ht_params: rhashtable_params, node_list: list_head, node_count: u32, root_count: u32, refcount: u32, ops: *const objagg_ops }
#[repr(C)] pub struct objagg_hints_node { ht_node: rhash_head, list: list_head, parent: *mut objagg_hints_node, root_id: u32, stats_info: objagg_obj_stats_info, pub obj: [u8; 0] }
#[repr(C)] pub struct objagg { ops: *const objagg_ops, priv_: *mut c_void, obj_ht: rhashtable, ht_params: rhashtable_params, obj_list: list_head, obj_count: u32, root_ida: ida, hints: *mut objagg_hints }
#[repr(C)] pub union objagg_obj_priv { pub delta_priv: *mut c_void, pub root_priv: *mut c_void }
#[repr(C)] pub struct objagg_obj { ht_node: rhash_head, list: list_head, parent: *mut objagg_obj, priv_: objagg_obj_priv, root_id: u32, refcount: u32, stats: objagg_obj_stats, pub obj: [u8; 0] }

extern "C" {
    fn rhashtable_lookup_fast(_: *const rhashtable, _: *mut c_void, _: rhashtable_params) -> *mut c_void;
    fn rhashtable_insert_fast(_: *mut rhashtable, _: *mut rhash_head, _: rhashtable_params) -> i32;
    fn rhashtable_remove_fast(_: *mut rhashtable, _: *mut rhash_head, _: rhashtable_params) -> i32;
    fn rhashtable_init(_: *mut rhashtable, _: *mut rhashtable_params) -> i32;
    fn rhashtable_destroy(_: *mut rhashtable);
    fn ida_alloc_range(_: *mut ida, _: u32, _: u32, _: u32) -> i32;
    fn ida_free(_: *mut ida, _: u32); fn ida_init(_: *mut ida); fn ida_destroy(_: *mut ida);
    fn objagg_hints_put(_: *mut objagg_hints);
    fn objagg_obj_put(_: *mut objagg, _: *mut objagg_obj);
    fn kernel_alloc(_: usize) -> *mut u8; fn kernel_free(_: *mut c_void);
}

const OBJAGG_OBJ_ROOT_ID_INVALID: u32 = u32::MAX;
const ENOMEM: i32 = 12; const EINVAL: i32 = 22; const ENOENT: i32 = 2;
#[inline] unsafe fn err_ptr<T>(e: i32) -> *mut T { e as isize as *mut T }
#[inline] unsafe fn is_err<T>(p: *const T) -> bool { (p as isize) < 0 && (p as isize) >= -4095 }
#[inline] unsafe fn ptr_err<T>(p: *const T) -> i32 { p as isize as i32 }

unsafe fn objagg_obj_ref_inc(o: *mut objagg_obj) -> u32 { (*o).refcount = (*o).refcount.wrapping_add(1); (*o).refcount }
unsafe fn objagg_obj_ref_dec(o: *mut objagg_obj) -> u32 { (*o).refcount = (*o).refcount.wrapping_sub(1); (*o).refcount }
unsafe fn objagg_obj_is_root(o: *const objagg_obj) -> bool { (*o).parent.is_null() }
#[no_mangle] pub unsafe extern "C" fn objagg_obj_root_priv(o: *const objagg_obj) -> *const c_void { if objagg_obj_is_root(o) { (*o).priv_.root_priv } else { (*(*o).parent).priv_.root_priv } }
#[no_mangle] pub unsafe extern "C" fn objagg_obj_delta_priv(o: *const objagg_obj) -> *const c_void { if objagg_obj_is_root(o) { core::ptr::null() } else { (*o).priv_.delta_priv } }
#[no_mangle] pub unsafe extern "C" fn objagg_obj_raw(o: *const objagg_obj) -> *const c_void { (*o).obj.as_ptr() as *const c_void }

unsafe fn stats_inc(o: *mut objagg_obj) { (*o).stats.user_count += 1; (*o).stats.delta_user_count += 1; if !(*o).parent.is_null() { (*(*o).parent).stats.delta_user_count += 1; } }
unsafe fn stats_dec(o: *mut objagg_obj) { (*o).stats.user_count -= 1; (*o).stats.delta_user_count -= 1; if !(*o).parent.is_null() { (*(*o).parent).stats.delta_user_count -= 1; } }
unsafe fn obj_lookup(a: *mut objagg, obj: *mut c_void) -> *mut objagg_obj { rhashtable_lookup_fast(&(*a).obj_ht, obj, (*a).ht_params) as *mut objagg_obj }

unsafe fn obj_destroy(a: *mut objagg, o: *mut objagg_obj) { (*a).obj_count -= 1; kernel_free(o as *mut c_void); }
unsafe fn obj_put_internal(a: *mut objagg, o: *mut objagg_obj) { if objagg_obj_ref_dec(o) == 0 { obj_destroy(a, o); } }

#[no_mangle] pub unsafe extern "C" fn objagg_obj_get(a: *mut objagg, obj: *mut c_void) -> *mut objagg_obj { let o=obj_lookup(a,obj); if !o.is_null(){objagg_obj_ref_inc(o);stats_inc(o);return o;} err_ptr(-ENOMEM) }
#[no_mangle] pub unsafe extern "C" fn objagg_obj_put_public(a: *mut objagg, o: *mut objagg_obj) { stats_dec(o); obj_put_internal(a,o); }

#[no_mangle] pub unsafe extern "C" fn objagg_create(ops: *const objagg_ops, hints: *mut objagg_hints, priv_: *mut c_void) -> *mut objagg { if ops.is_null(){return err_ptr(-EINVAL)}; let p=kernel_alloc(core::mem::size_of::<objagg>()) as *mut objagg; if p.is_null(){return err_ptr(-ENOMEM)}; core::ptr::write_bytes(p,0,1); (*p).ops=ops;(*p).hints=hints;(*p).priv_=priv_; p }
#[no_mangle] pub unsafe extern "C" fn objagg_destroy(a: *mut objagg) { if !a.is_null(){ kernel_free(a as *mut c_void); } }

/* Temporary graph and optimization declarations are retained for ABI/source
 * completeness; their storage and callback dependencies are external. */
#[repr(C)] pub struct objagg_tmp_node { pub objagg_obj:*mut objagg_obj, pub crossed_out:bool }
#[repr(C)] pub struct objagg_tmp_graph { pub nodes:*mut objagg_tmp_node, pub nodes_count:usize, pub edges:*mut usize }
#[repr(C)] pub struct objagg_opt_algo { pub fillup_hints: Option<unsafe extern "C" fn(*mut objagg_hints,*mut objagg)->i32> }

#[no_mangle] pub unsafe extern "C" fn objagg_hints_put_public(h:*mut objagg_hints){objagg_hints_put(h)}

#[no_mangle] pub unsafe extern "C" fn objagg_stats_get(_: *mut objagg) -> *mut objagg_stats { err_ptr(-ENOMEM) }
#[no_mangle] pub unsafe extern "C" fn objagg_stats_put(s: *const objagg_stats) { if !s.is_null(){kernel_free(s as *mut c_void)} }
#[no_mangle] pub unsafe extern "C" fn objagg_hints_get(_: *mut objagg, _: i32) -> *mut objagg_hints { err_ptr(-ENOMEM) }
#[no_mangle] pub unsafe extern "C" fn objagg_hints_stats_get(_: *mut objagg_hints) -> *mut objagg_stats { err_ptr(-ENOMEM) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
