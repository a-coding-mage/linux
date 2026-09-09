// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 Facebook
 * Copyright 2020 Google LLC.
 */

// Translated from bpf_inode_storage.c. Linux headers and externally supplied
// symbols are intentionally left as dependencies of the surrounding tree.

DEFINE_BPF_STORAGE_CACHE!(inode_cache);

unsafe fn inode_storage_ptr(owner: *mut core::ffi::c_void) -> *mut *mut bpf_local_storage {
    let inode = owner as *mut inode;
    let bsb: *mut bpf_storage_blob = bpf_inode(inode);
    if bsb.is_null() {
        return core::ptr::null_mut();
    }
    &mut (*bsb).storage
}

unsafe fn inode_storage_lookup(
    inode: *mut inode,
    map: *mut bpf_map,
    cacheit_lockit: bool,
) -> *mut bpf_local_storage_data {
    let bsb: *mut bpf_storage_blob = bpf_inode(inode);
    if bsb.is_null() {
        return core::ptr::null_mut();
    }

    let inode_storage = rcu_dereference_check((*bsb).storage, bpf_rcu_lock_held());
    if inode_storage.is_null() {
        return core::ptr::null_mut();
    }

    let smap = map as *mut bpf_local_storage_map;
    bpf_local_storage_lookup(inode_storage, smap, cacheit_lockit)
}

unsafe fn bpf_inode_storage_free(inode: *mut inode) {
    let bsb: *mut bpf_storage_blob = bpf_inode(inode);
    if bsb.is_null() {
        return;
    }

    rcu_read_lock_dont_migrate();
    let local_storage = rcu_dereference((*bsb).storage);
    if !local_storage.is_null() {
        bpf_local_storage_destroy(local_storage);
    }
    rcu_read_unlock_migrate();
}

unsafe fn bpf_fd_inode_storage_lookup_elem(
    map: *mut bpf_map,
    key: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let f = fd_raw_new(*(key as *mut i32));
    if fd_empty(f) {
        return ERR_PTR(-EBADF);
    }

    let sdata = inode_storage_lookup(file_inode(fd_file(f)), map, true);
    if sdata.is_null() { core::ptr::null_mut() } else { (*sdata).data }
}

unsafe fn bpf_fd_inode_storage_update_elem(
    map: *mut bpf_map,
    key: *mut core::ffi::c_void,
    value: *mut core::ffi::c_void,
    map_flags: u64,
) -> i64 {
    let f = fd_raw_new(*(key as *mut i32));
    if fd_empty(f) { return -EBADF as i64; }
    if inode_storage_ptr(file_inode(fd_file(f)) as *mut core::ffi::c_void).is_null() {
        return -EBADF as i64;
    }

    let sdata = bpf_local_storage_update(
        file_inode(fd_file(f)), map as *mut bpf_local_storage_map, value, map_flags, false,
    );
    PTR_ERR_OR_ZERO(sdata)
}

unsafe fn inode_storage_delete(inode: *mut inode, map: *mut bpf_map) -> i32 {
    let sdata = inode_storage_lookup(inode, map, false);
    if sdata.is_null() { return -ENOENT; }
    bpf_selem_unlink(SELEM(sdata))
}

unsafe fn bpf_fd_inode_storage_delete_elem(
    map: *mut bpf_map,
    key: *mut core::ffi::c_void,
) -> i64 {
    let f = fd_raw_new(*(key as *mut i32));
    if fd_empty(f) { return -EBADF as i64; }
    inode_storage_delete(file_inode(fd_file(f)), map) as i64
}

unsafe fn bpf_inode_storage_get(
    map: *mut bpf_map,
    inode: *mut inode,
    value: *mut core::ffi::c_void,
    flags: u64,
) -> usize {
    WARN_ON_ONCE(!bpf_rcu_lock_held());
    if flags & !BPF_LOCAL_STORAGE_GET_F_CREATE != 0 { return 0; }
    if inode.is_null() || inode_storage_ptr(inode as *mut core::ffi::c_void).is_null() { return 0; }

    let sdata = inode_storage_lookup(inode, map, true);
    if !sdata.is_null() { return (*sdata).data as usize; }

    if flags & BPF_LOCAL_STORAGE_GET_F_CREATE != 0 {
        let sdata = bpf_local_storage_update(
            inode, map as *mut bpf_local_storage_map, value, BPF_NOEXIST, false,
        );
        return if IS_ERR(sdata) { 0 } else { (*sdata).data as usize };
    }
    0
}

unsafe fn bpf_inode_storage_delete(map: *mut bpf_map, inode: *mut inode) -> i64 {
    WARN_ON_ONCE(!bpf_rcu_lock_held());
    if inode.is_null() { return -EINVAL as i64; }
    inode_storage_delete(inode, map) as i64
}

unsafe fn notsupp_get_next_key(
    _map: *mut bpf_map,
    _key: *mut core::ffi::c_void,
    _next_key: *mut core::ffi::c_void,
) -> i32 { -ENOTSUPP }

unsafe fn inode_storage_map_alloc(attr: *mut bpf_attr) -> *mut bpf_map {
    if !bpf_lsm_initialized { return ERR_PTR(-EOPNOTSUPP); }
    bpf_local_storage_map_alloc(attr, &inode_cache)
}

unsafe fn inode_storage_map_free(map: *mut bpf_map) {
    bpf_local_storage_map_free(map, &inode_cache);
}

pub static inode_storage_map_ops: bpf_map_ops = bpf_map_ops {
    map_meta_equal: Some(bpf_map_meta_equal),
    map_alloc_check: Some(bpf_local_storage_map_alloc_check),
    map_alloc: Some(inode_storage_map_alloc),
    map_free: Some(inode_storage_map_free),
    map_get_next_key: Some(notsupp_get_next_key),
    map_lookup_elem: Some(bpf_fd_inode_storage_lookup_elem),
    map_update_elem: Some(bpf_fd_inode_storage_update_elem),
    map_delete_elem: Some(bpf_fd_inode_storage_delete_elem),
    map_check_btf: Some(bpf_local_storage_map_check_btf),
    map_mem_usage: Some(bpf_local_storage_map_mem_usage),
    map_btf_id: &bpf_local_storage_map_btf_id[0],
    map_owner_storage_ptr: Some(inode_storage_ptr),
};

BTF_ID_LIST_SINGLE!(bpf_inode_storage_btf_ids, struct, inode);

pub static bpf_inode_storage_get_proto: bpf_func_proto = bpf_func_proto {
    func: Some(bpf_inode_storage_get), gpl_only: false,
    ret_type: RET_PTR_TO_MAP_VALUE_OR_NULL, arg1_type: ARG_CONST_MAP_PTR,
    arg2_type: ARG_PTR_TO_BTF_ID_OR_NULL, arg2_btf_id: &bpf_inode_storage_btf_ids[0],
    arg3_type: ARG_PTR_TO_MAP_VALUE_OR_NULL, arg4_type: ARG_ANYTHING,
};

pub static bpf_inode_storage_delete_proto: bpf_func_proto = bpf_func_proto {
    func: Some(bpf_inode_storage_delete), gpl_only: false,
    ret_type: RET_INTEGER, arg1_type: ARG_CONST_MAP_PTR,
    arg2_type: ARG_PTR_TO_BTF_ID_OR_NULL, arg2_btf_id: &bpf_inode_storage_btf_ids[0],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
