// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* Dependencies in the original C source:
 * <vmlinux.h>
 * <bpf/bpf_tracing.h>
 * <bpf/bpf_helpers.h>
 * <bpf/bpf_core_read.h>
 * "../bpf_experimental.h"
 * "../test_kmods/bpf_testmod_kfunc.h"
 */

#[repr(C)]
pub struct plain_local {
    pub key: i64,
    pub data: i64,
}

#[repr(C)]
pub struct bpf_rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_refcount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct prog_test_ref_kfunc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct node_data {
    pub key: i64,
    pub data: i64,
    pub stashed_in_local_kptr: *mut plain_local,
    pub node: bpf_rb_node,
}

#[repr(C)]
pub struct refcounted_node {
    pub data: i64,
    pub rb_node: bpf_rb_node,
    pub refcount: bpf_refcount,
}

#[repr(C)]
pub struct stash {
    pub l: bpf_spin_lock,
    pub stashed: *mut refcounted_node,
}

/* Original C map definition:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_ARRAY);
 *     __type(key, int);
 *     __type(value, struct stash);
 *     __uint(max_entries, 10);
 * } refcounted_node_stash SEC(".maps");
 */
#[repr(C)]
pub struct refcounted_node_stash_map {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut refcounted_node_stash: refcounted_node_stash_map =
    refcounted_node_stash_map { _private: [] };

#[repr(C)]
pub struct local_with_root {
    pub key: i64,
    pub l: bpf_spin_lock,
    /* struct bpf_rb_root r __contains(node_data, node); */
    pub r: bpf_rb_root,
}

#[repr(C)]
pub struct map_value {
    pub not_kptr: *mut prog_test_ref_kfunc,
    pub val: *mut prog_test_ref_kfunc,
    pub node: *mut node_data,
    pub plain: *mut plain_local,
    pub local_root: *mut local_with_root,
}

/* This is necessary so that LLVM generates BTF for node_data struct
 * If it's not included, a fwd reference for node_data will be generated but
 * no struct. Example BTF of "node" field in map_value when not included:
 *
 * [10] PTR '(anon)' type_id=35
 * [34] FWD 'node_data' fwd_kind=struct
 * [35] TYPE_TAG 'kptr_ref' type_id=34
 *
 * (with no node_data struct defined)
 * Had to do the same w/ bpf_kfunc_call_test_release below
 */
#[no_mangle]
pub static mut just_here_because_btf_bug: *mut node_data = core::ptr::null_mut();
#[no_mangle]
pub static mut just_here_because_btf_bug2: *mut refcounted_node = core::ptr::null_mut();

/* Original C map definition:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_ARRAY);
 *     __type(key, int);
 *     __type(value, struct map_value);
 *     __uint(max_entries, 2);
 * } some_nodes SEC(".maps");
 */
#[repr(C)]
pub struct some_nodes_map {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut some_nodes: some_nodes_map = some_nodes_map { _private: [] };

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_obj_new_impl(size: usize) -> *mut core::ffi::c_void;
    fn bpf_obj_drop_impl(ptr: *mut core::ffi::c_void);
    fn bpf_kptr_xchg_impl(
        map_value: *mut *mut core::ffi::c_void,
        ptr: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_rbtree_add(
        root: *mut bpf_rb_root,
        node: *mut bpf_rb_node,
        less: unsafe extern "C" fn(*mut bpf_rb_node, *const bpf_rb_node) -> bool,
    );
    fn bpf_refcount_acquire(ptr: *mut refcounted_node) -> *mut refcounted_node;
    fn bpf_kfunc_call_test_release(ptr: *mut prog_test_ref_kfunc);
}

unsafe fn bpf_obj_new<T>() -> *mut T {
    bpf_obj_new_impl(core::mem::size_of::<T>()) as *mut T
}

unsafe fn bpf_obj_drop<T>(ptr: *mut T) {
    bpf_obj_drop_impl(ptr as *mut core::ffi::c_void);
}

unsafe fn bpf_kptr_xchg<T>(map_value: *mut *mut T, ptr: *mut T) -> *mut T {
    bpf_kptr_xchg_impl(
        map_value as *mut *mut core::ffi::c_void,
        ptr as *mut core::ffi::c_void,
    ) as *mut T
}

unsafe extern "C" fn less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let node_a: *mut node_data = (a as *mut u8).sub(core::mem::offset_of!(node_data, node))
        as *mut node_data;
    let node_b: *mut node_data = (b as *mut u8).sub(core::mem::offset_of!(node_data, node))
        as *mut node_data;

    (*node_a).key < (*node_b).key
}

unsafe fn create_and_stash(idx: i32, val: i32) -> i32 {
    let mut inner_local_kptr: *mut plain_local;
    let mapval: *mut map_value;
    let mut res: *mut node_data;

    mapval = bpf_map_lookup_elem(
        &raw mut some_nodes as *mut core::ffi::c_void,
        &idx as *const _ as *const core::ffi::c_void,
    ) as *mut map_value;
    if mapval.is_null() {
        return 1;
    }

    inner_local_kptr = bpf_obj_new::<plain_local>();
    if inner_local_kptr.is_null() {
        return 2;
    }

    res = bpf_obj_new::<node_data>();
    if res.is_null() {
        bpf_obj_drop(inner_local_kptr);
        return 3;
    }
    (*res).key = val as i64;

    inner_local_kptr = bpf_kptr_xchg(&mut (*res).stashed_in_local_kptr, inner_local_kptr);
    if !inner_local_kptr.is_null() {
        /* Should never happen, we just obj_new'd res */
        bpf_obj_drop(inner_local_kptr);
        bpf_obj_drop(res);
        return 4;
    }

    res = bpf_kptr_xchg(&mut (*mapval).node, res);
    if !res.is_null() {
        bpf_obj_drop(res);
    }
    0
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn stash_rb_nodes(_ctx: *mut core::ffi::c_void) -> i64 {
    let ret = create_and_stash(0, 41);
    if ret != 0 {
        ret as i64
    } else {
        create_and_stash(1, 42) as i64
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn stash_plain(_ctx: *mut core::ffi::c_void) -> i64 {
    let mapval: *mut map_value;
    let mut res: *mut plain_local;
    let idx: i32 = 0;

    mapval = bpf_map_lookup_elem(
        &raw mut some_nodes as *mut core::ffi::c_void,
        &idx as *const _ as *const core::ffi::c_void,
    ) as *mut map_value;
    if mapval.is_null() {
        return 1;
    }

    res = bpf_obj_new::<plain_local>();
    if res.is_null() {
        return 1;
    }
    (*res).key = 41;

    res = bpf_kptr_xchg(&mut (*mapval).plain, res);
    if !res.is_null() {
        bpf_obj_drop(res);
    }
    0
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn stash_local_with_root(_ctx: *mut core::ffi::c_void) -> i64 {
    let mut res: *mut local_with_root;
    let mapval: *mut map_value;
    let n: *mut node_data;
    let idx: i32 = 0;

    mapval = bpf_map_lookup_elem(
        &raw mut some_nodes as *mut core::ffi::c_void,
        &idx as *const _ as *const core::ffi::c_void,
    ) as *mut map_value;
    if mapval.is_null() {
        return 1;
    }

    res = bpf_obj_new::<local_with_root>();
    if res.is_null() {
        return 2;
    }
    (*res).key = 41;

    n = bpf_obj_new::<node_data>();
    if n.is_null() {
        bpf_obj_drop(res);
        return 3;
    }

    bpf_spin_lock(&mut (*res).l);
    bpf_rbtree_add(&mut (*res).r, &mut (*n).node, less);
    bpf_spin_unlock(&mut (*res).l);

    res = bpf_kptr_xchg(&mut (*mapval).local_root, res);
    if !res.is_null() {
        bpf_obj_drop(res);
        return 4;
    }
    0
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn unstash_rb_node(_ctx: *mut core::ffi::c_void) -> i64 {
    let mut inner_local_kptr: *mut plain_local = core::ptr::null_mut();
    let mapval: *mut map_value;
    let res: *mut node_data;
    let retval: i64;
    let key: i32 = 1;

    mapval = bpf_map_lookup_elem(
        &raw mut some_nodes as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut map_value;
    if mapval.is_null() {
        return 1;
    }

    res = bpf_kptr_xchg(&mut (*mapval).node, core::ptr::null_mut());
    if !res.is_null() {
        inner_local_kptr =
            bpf_kptr_xchg(&mut (*res).stashed_in_local_kptr, inner_local_kptr);
        if inner_local_kptr.is_null() {
            bpf_obj_drop(res);
            return 1;
        }
        bpf_obj_drop(inner_local_kptr);

        retval = (*res).key;
        bpf_obj_drop(res);
        return retval;
    }
    1
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn stash_test_ref_kfunc(_ctx: *mut core::ffi::c_void) -> i64 {
    let res: *mut prog_test_ref_kfunc;
    let mapval: *mut map_value;
    let key: i32 = 0;

    mapval = bpf_map_lookup_elem(
        &raw mut some_nodes as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut map_value;
    if mapval.is_null() {
        return 1;
    }

    res = bpf_kptr_xchg(&mut (*mapval).val, core::ptr::null_mut());
    if !res.is_null() {
        bpf_kfunc_call_test_release(res);
    }
    0
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn refcount_acquire_without_unstash(
    _ctx: *mut core::ffi::c_void,
) -> i64 {
    let p: *mut refcounted_node;
    let s: *mut stash;
    let mut ret: i32 = 0;

    s = bpf_map_lookup_elem(
        &raw mut refcounted_node_stash as *mut core::ffi::c_void,
        &ret as *const _ as *const core::ffi::c_void,
    ) as *mut stash;
    if s.is_null() {
        return 1;
    }

    if (*s).stashed.is_null() {
        /* refcount_acquire failure is expected when no refcounted_node
         * has been stashed before this program executes
         */
        return 2;
    }

    p = bpf_refcount_acquire((*s).stashed);
    if p.is_null() {
        return 3;
    }

    ret = if !(*s).stashed.is_null() {
        (*(*s).stashed).data as i32
    } else {
        -1
    };
    bpf_obj_drop(p);
    ret as i64
}

/* Helper for refcount_acquire_without_unstash test */
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn stash_refcounted_node(_ctx: *mut core::ffi::c_void) -> i64 {
    let mut p: *mut refcounted_node;
    let s: *mut stash;
    let key: i32 = 0;

    s = bpf_map_lookup_elem(
        &raw mut refcounted_node_stash as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut stash;
    if s.is_null() {
        return 1;
    }

    p = bpf_obj_new::<refcounted_node>();
    if p.is_null() {
        return 2;
    }
    (*p).data = 42;

    p = bpf_kptr_xchg(&mut (*s).stashed, p);
    if !p.is_null() {
        bpf_obj_drop(p);
        return 3;
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
