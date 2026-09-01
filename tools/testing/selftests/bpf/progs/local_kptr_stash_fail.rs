// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* C dependencies removed from executable Rust:
 * <vmlinux.h>
 * <bpf/bpf_helpers.h>
 * <bpf/bpf_tracing.h>
 * <bpf/bpf_core_read.h>
 * "../bpf_experimental.h"
 * "bpf_misc.h"
 */

#[repr(C)]
pub struct bpf_rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct node_data {
    pub key: core::ffi::c_long,
    pub data: core::ffi::c_long,
    pub node: bpf_rb_node,
}

#[repr(C)]
pub struct map_value {
    /* __kptr */
    pub node: *mut node_data,
}

#[repr(C)]
pub struct node_data2 {
    pub key: [core::ffi::c_long; 4],
}

/* This is necessary so that LLVM generates BTF for node_data struct
 * If it's not included, a fwd reference for node_data will be generated but
 * no struct. Example BTF of "node" field in map_value when not included:
 *
 * [10] PTR '(anon)' type_id=35
 * [34] FWD 'node_data' fwd_kind=struct
 * [35] TYPE_TAG 'kptr_ref' type_id=34
 */
#[no_mangle]
pub static mut just_here_because_btf_bug: *mut node_data = core::ptr::null_mut();

/* Original C BPF map definition:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_ARRAY);
 *     __type(key, int);
 *     __type(value, struct map_value);
 *     __uint(max_entries, 2);
 * } some_nodes SEC(".maps");
 */
#[repr(C)]
pub struct some_nodes_map_def {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut some_nodes: some_nodes_map_def = some_nodes_map_def { _private: [] };

extern "C" {
    fn bpf_map_lookup_elem(
        map: *mut some_nodes_map_def,
        key: *const core::ffi::c_int,
    ) -> *mut map_value;
    fn bpf_obj_new_node_data2() -> *mut node_data2;
    fn bpf_obj_new_node_data() -> *mut node_data;
    fn bpf_kptr_xchg_node_data2(
        kptr: *mut *mut node_data,
        ptr: *mut node_data2,
    ) -> *mut node_data2;
    fn bpf_obj_drop_node_data2(ptr: *mut node_data2);
    fn bpf_obj_drop_bpf_rb_node(ptr: *mut bpf_rb_node);
}

/* SEC("tc")
 * __failure __msg("invalid kptr access, R2 type=ptr_node_data2 expected=ptr_node_data")
 */
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn stash_rb_nodes(ctx: *mut core::ffi::c_void) -> core::ffi::c_long {
    let mut mapval: *mut map_value;
    let mut res: *mut node_data2;
    let mut idx: core::ffi::c_int = 0;

    let _ = ctx;

    mapval = bpf_map_lookup_elem(&mut some_nodes, &idx);
    if mapval.is_null() {
        return 1;
    }

    res = bpf_obj_new_node_data2();
    if res.is_null() {
        return 1;
    }
    (*res).key[0] = 40;

    res = bpf_kptr_xchg_node_data2(&mut (*mapval).node, res);
    if !res.is_null() {
        bpf_obj_drop_node_data2(res);
    }
    0
}

/* SEC("tc")
 * __failure __msg("R1 must have zero offset when passed to release func")
 */
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn drop_rb_node_off(ctx: *mut core::ffi::c_void) -> core::ffi::c_long {
    let mut mapval: *mut map_value;
    let mut res: *mut node_data;
    let mut idx: core::ffi::c_int = 0;

    let _ = ctx;

    mapval = bpf_map_lookup_elem(&mut some_nodes, &idx);
    if mapval.is_null() {
        return 1;
    }

    res = bpf_obj_new_node_data();
    if res.is_null() {
        return 1;
    }
    /* Try releasing with graph node offset */
    bpf_obj_drop_bpf_rb_node(&mut (*res).node);
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
