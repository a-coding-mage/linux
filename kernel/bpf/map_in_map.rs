// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2017 Facebook
 */
// Dependencies supplied by the surrounding kernel translation unit.

pub unsafe fn bpf_map_meta_alloc(inner_map_ufd: ::core::ffi::c_int) -> *mut bpf_map {
    let (inner_map, mut inner_map_meta): (*mut bpf_map, *mut bpf_map);
    let mut inner_map_meta_size: u32;
    let f = FdClass::new(inner_map_ufd);

    inner_map = __bpf_map_get(f);
    if IS_ERR(inner_map) {
        return inner_map;
    }

    /* Does not support >1 level map-in-map */
    if !(*inner_map).inner_map_meta.is_null() {
        return ERR_PTR(-EINVAL);
    }
    if !(*inner_map).excl_prog_sha.is_null() {
        return ERR_PTR(-ENOTSUPP);
    }
    if (*(*inner_map).ops).map_meta_equal.is_none() {
        return ERR_PTR(-ENOTSUPP);
    }

    inner_map_meta_size = core::mem::size_of::<bpf_map>() as u32;
    /* In some cases verifier needs to access beyond just base map. */
    if (*inner_map).ops == &array_map_ops || (*inner_map).ops == &percpu_array_map_ops {
        inner_map_meta_size = core::mem::size_of::<bpf_array>() as u32;
    }

    inner_map_meta = kzalloc(inner_map_meta_size as usize, GFP_USER);
    if inner_map_meta.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*inner_map_meta).map_type = (*inner_map).map_type;
    (*inner_map_meta).key_size = (*inner_map).key_size;
    (*inner_map_meta).value_size = (*inner_map).value_size;
    (*inner_map_meta).map_flags = (*inner_map).map_flags;
    (*inner_map_meta).max_entries = (*inner_map).max_entries;

    (*inner_map_meta).record = btf_record_dup((*inner_map).record);
    if IS_ERR((*inner_map_meta).record) {
        /* btf_record_dup returns NULL or valid pointer in case of
         * invalid/empty/valid, but ERR_PTR in case of errors. During
         * equality NULL or IS_ERR is equivalent.
         */
        let ret: *mut bpf_map = ERR_CAST((*inner_map_meta).record);
        kfree(inner_map_meta as *mut core::ffi::c_void);
        return ret;
    }
    /* Note: We must use the same BTF, as we also used btf_record_dup above
     * which relies on BTF being same for both maps, as some members like
     * record->fields.list_head have pointers like value_rec pointing into
     * inner_map->btf.
     */
    if !(*inner_map).btf.is_null() {
        btf_get((*inner_map).btf);
        (*inner_map_meta).btf = (*inner_map).btf;
    }

    /* Misc members not needed in bpf_map_meta_equal() check. */
    (*inner_map_meta).ops = (*inner_map).ops;
    if (*inner_map).ops == &array_map_ops || (*inner_map).ops == &percpu_array_map_ops {
        let inner_array_meta = container_of!(inner_map_meta, bpf_array, map);
        let inner_array = container_of!(inner_map, bpf_array, map);

        (*inner_array_meta).index_mask = (*inner_array).index_mask;
        (*inner_array_meta).elem_size = (*inner_array).elem_size;
        (*inner_map_meta).bypass_spec_v1 = (*inner_map).bypass_spec_v1;
    }
    inner_map_meta
}

pub unsafe fn bpf_map_meta_free(map_meta: *mut bpf_map) {
    bpf_map_free_record(map_meta);
    btf_put((*map_meta).btf);
    kfree(map_meta as *mut core::ffi::c_void);
}

pub unsafe fn bpf_map_meta_equal(meta0: *const bpf_map, meta1: *const bpf_map) -> bool {
    /* No need to compare ops because it is covered by map_type */
    (*meta0).map_type == (*meta1).map_type
        && (*meta0).key_size == (*meta1).key_size
        && (*meta0).value_size == (*meta1).value_size
        && (*meta0).map_flags == (*meta1).map_flags
        && btf_record_equal((*meta0).record, (*meta1).record)
}

pub unsafe fn bpf_map_fd_get_ptr(
    map: *mut bpf_map,
    _map_file: *mut file, /* not used */
    ufd: ::core::ffi::c_int,
) -> *mut core::ffi::c_void {
    let mut inner_map: *mut bpf_map;
    let inner_map_meta: *mut bpf_map;
    let f = FdClass::new(ufd);

    inner_map = __bpf_map_get(f);
    if IS_ERR(inner_map) {
        return inner_map as *mut core::ffi::c_void;
    }
    if !(*inner_map).excl_prog_sha.is_null() {
        return ERR_PTR(-ENOTSUPP) as *mut core::ffi::c_void;
    }

    inner_map_meta = (*map).inner_map_meta;
    if ((*(*inner_map_meta).ops).map_meta_equal.unwrap())(inner_map_meta, inner_map) {
        bpf_map_inc(inner_map);
    } else {
        inner_map = ERR_PTR(-EINVAL);
    }

    inner_map as *mut core::ffi::c_void
}

pub unsafe fn bpf_map_fd_put_ptr(map: *mut bpf_map, ptr: *mut core::ffi::c_void, need_defer: bool) {
    let inner_map = ptr as *mut bpf_map;

    /* Defer the freeing of inner map according to the sleepable attribute
     * of bpf program which owns the outer map, so unnecessary waiting for
     * RCU tasks trace grace period can be avoided.
     */
    if need_defer {
        if atomic64_read(&(*map).sleepable_refcnt) != 0 {
            WRITE_ONCE!((*inner_map).free_after_mult_rcu_gp, true);
        } else {
            WRITE_ONCE!((*inner_map).free_after_rcu_gp, true);
        }
    }
    bpf_map_put(inner_map);
}

pub unsafe fn bpf_map_fd_sys_lookup_elem(ptr: *mut core::ffi::c_void) -> u32 {
    (*(ptr as *mut bpf_map)).id
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
