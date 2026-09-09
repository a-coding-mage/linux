// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 */

// Kernel dependency declarations and macro-generated interfaces are supplied by
// the surrounding BPF implementation.

static mut CGROUP_CACHE: bpf_local_storage_cache = bpf_local_storage_cache { _private: [] };

unsafe fn cgroup_storage_ptr(owner: *mut core::ffi::c_void) -> *mut *mut bpf_local_storage {
    let cg = owner as *mut cgroup;
    &mut (*cg).bpf_cgrp_storage
}

pub unsafe fn bpf_cgrp_storage_free(cgroup: *mut cgroup) {
    rcu_read_lock();
    let local_storage = rcu_dereference((*cgroup).bpf_cgrp_storage);
    if !local_storage.is_null() {
        bpf_local_storage_destroy(local_storage);
    }
    rcu_read_unlock();
}

unsafe fn cgroup_storage_lookup(
    cgroup: *mut cgroup,
    map: *mut bpf_map,
    cacheit_lockit: bool,
) -> *mut bpf_local_storage_data {
    let cgroup_storage = rcu_dereference_check(
        (*cgroup).bpf_cgrp_storage,
        bpf_rcu_lock_held(),
    );
    if cgroup_storage.is_null() {
        return core::ptr::null_mut();
    }
    let smap = map as *mut bpf_local_storage_map;
    bpf_local_storage_lookup(cgroup_storage, smap, cacheit_lockit)
}

unsafe fn bpf_cgrp_storage_lookup_elem(map: *mut bpf_map, key: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let fd = *(key as *mut i32);
    let cgroup = cgroup_v1v2_get_from_fd(fd);
    if is_err(cgroup) {
        return err_cast(cgroup);
    }
    let sdata = cgroup_storage_lookup(cgroup, map, true);
    cgroup_put(cgroup);
    if sdata.is_null() { core::ptr::null_mut() } else { (*sdata).data }
}

unsafe fn bpf_cgrp_storage_update_elem(
    map: *mut bpf_map, key: *mut core::ffi::c_void,
    value: *mut core::ffi::c_void, map_flags: u64,
) -> i64 {
    let fd = *(key as *mut i32);
    let cgroup = cgroup_v1v2_get_from_fd(fd);
    if is_err(cgroup) { return ptr_err(cgroup); }
    let sdata = bpf_local_storage_update(cgroup, map as *mut bpf_local_storage_map,
                                         value, map_flags, false);
    cgroup_put(cgroup);
    ptr_err_or_zero(sdata)
}

unsafe fn cgroup_storage_delete(cgroup: *mut cgroup, map: *mut bpf_map) -> i32 {
    let sdata = cgroup_storage_lookup(cgroup, map, false);
    if sdata.is_null() { return -ENOENT; }
    bpf_selem_unlink(selem(sdata))
}

unsafe fn bpf_cgrp_storage_delete_elem(map: *mut bpf_map, key: *mut core::ffi::c_void) -> i64 {
    let cgroup = cgroup_v1v2_get_from_fd(*(key as *mut i32));
    if is_err(cgroup) { return ptr_err(cgroup) as i64; }
    let err = cgroup_storage_delete(cgroup, map);
    cgroup_put(cgroup);
    err as i64
}

unsafe fn notsupp_get_next_key(_: *mut bpf_map, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32 {
    -ENOTSUPP
}

unsafe fn cgroup_storage_map_alloc(attr: *mut bpf_attr) -> *mut bpf_map {
    bpf_local_storage_map_alloc(attr, &raw mut CGROUP_CACHE)
}

unsafe fn cgroup_storage_map_free(map: *mut bpf_map) {
    bpf_local_storage_map_free(map, &raw mut CGROUP_CACHE);
}

pub unsafe fn bpf_cgrp_storage_get(
    map: *mut bpf_map, cgroup: *mut cgroup,
    value: *mut core::ffi::c_void, flags: u64,
) -> u64 {
    warn_on_once(!bpf_rcu_lock_held());
    if flags & !BPF_LOCAL_STORAGE_GET_F_CREATE != 0 || cgroup.is_null() {
        return 0;
    }
    let mut sdata = cgroup_storage_lookup(cgroup, map, true);
    if sdata.is_null() && !percpu_ref_is_dying(&(*cgroup).self_.refcnt)
        && flags & BPF_LOCAL_STORAGE_GET_F_CREATE != 0 {
        sdata = bpf_local_storage_update(cgroup, map as *mut bpf_local_storage_map,
                                         value, BPF_NOEXIST, false);
    }
    if is_err_or_null(sdata) { 0 } else { (*sdata).data as u64 }
}

pub unsafe fn bpf_cgrp_storage_delete(map: *mut bpf_map, cgroup: *mut cgroup) -> i64 {
    warn_on_once(!bpf_rcu_lock_held());
    if cgroup.is_null() { return -EINVAL as i64; }
    cgroup_storage_delete(cgroup, map) as i64
}

pub static cgrp_storage_map_ops: bpf_map_ops = bpf_map_ops {
    map_meta_equal: Some(bpf_map_meta_equal), map_alloc_check: Some(bpf_local_storage_map_alloc_check),
    map_alloc: Some(cgroup_storage_map_alloc), map_free: Some(cgroup_storage_map_free),
    map_get_next_key: Some(notsupp_get_next_key), map_lookup_elem: Some(bpf_cgrp_storage_lookup_elem),
    map_update_elem: Some(bpf_cgrp_storage_update_elem), map_delete_elem: Some(bpf_cgrp_storage_delete_elem),
    map_check_btf: Some(bpf_local_storage_map_check_btf), map_mem_usage: Some(bpf_local_storage_map_mem_usage),
    map_btf_id: &raw const bpf_local_storage_map_btf_id[0], map_owner_storage_ptr: Some(cgroup_storage_ptr),
};

pub static bpf_cgrp_storage_get_proto: bpf_func_proto = bpf_func_proto {
    func: Some(bpf_cgrp_storage_get), gpl_only: false, ret_type: RET_PTR_TO_MAP_VALUE_OR_NULL,
    arg1_type: ARG_CONST_MAP_PTR, arg2_type: ARG_PTR_TO_BTF_ID_OR_NULL,
    arg2_btf_id: &raw const bpf_cgroup_btf_id[0], arg3_type: ARG_PTR_TO_MAP_VALUE_OR_NULL,
    arg4_type: ARG_ANYTHING,
};

pub static bpf_cgrp_storage_delete_proto: bpf_func_proto = bpf_func_proto {
    func: Some(bpf_cgrp_storage_delete), gpl_only: false, ret_type: RET_INTEGER,
    arg1_type: ARG_CONST_MAP_PTR, arg2_type: ARG_PTR_TO_BTF_ID_OR_NULL,
    arg2_btf_id: &raw const bpf_cgroup_btf_id[0],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
