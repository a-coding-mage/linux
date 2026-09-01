/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* C dependencies removed from executable Rust:
 * <errno.h>, <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>
 */

#[repr(C)]
pub struct __cgrps_kfunc_map_value {
    pub cgrp: *mut cgroup,
}

/* Original BPF map declaration:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_HASH);
 *     __type(key, int);
 *     __type(value, struct __cgrps_kfunc_map_value);
 *     __uint(max_entries, 1);
 * } __cgrps_kfunc_map SEC(".maps");
 */
#[link_section = ".maps"]
#[no_mangle]
pub static mut __cgrps_kfunc_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<::core::ffi::c_int>() as u32,
    value_size: core::mem::size_of::<__cgrps_kfunc_map_value>() as u32,
    max_entries: 1,
    map_flags: 0,
};

extern "C" {
    pub fn bpf_cgroup_acquire(p: *mut cgroup) -> *mut cgroup;
    pub fn bpf_cgroup_release(p: *mut cgroup);
    pub fn bpf_cgroup_ancestor(cgrp: *mut cgroup, level: ::core::ffi::c_int) -> *mut cgroup;
    pub fn bpf_cgroup_from_id(cgid: u64) -> *mut cgroup;
    pub fn bpf_rcu_read_lock();
    pub fn bpf_rcu_read_unlock();
}

#[inline]
pub unsafe fn cgrps_kfunc_map_value_lookup(
    cgrp: *mut cgroup,
) -> *mut __cgrps_kfunc_map_value {
    let mut id: s32 = 0;
    let mut status: ::core::ffi::c_long;

    status = bpf_probe_read_kernel(
        &mut id as *mut s32 as *mut ::core::ffi::c_void,
        core::mem::size_of_val(&id) as u32,
        &mut (*cgrp).self_.id as *mut _ as *const ::core::ffi::c_void,
    );
    if status != 0 {
        return core::ptr::null_mut();
    }

    bpf_map_lookup_elem(
        &mut __cgrps_kfunc_map as *mut bpf_map_def as *mut ::core::ffi::c_void,
        &mut id as *mut s32 as *const ::core::ffi::c_void,
    ) as *mut __cgrps_kfunc_map_value
}

#[inline]
pub unsafe fn cgrps_kfunc_map_insert(cgrp: *mut cgroup) -> ::core::ffi::c_int {
    let mut local: __cgrps_kfunc_map_value = __cgrps_kfunc_map_value {
        cgrp: core::ptr::null_mut(),
    };
    let mut v: *mut __cgrps_kfunc_map_value;
    let mut status: ::core::ffi::c_long;
    let mut acquired: *mut cgroup;
    let mut old: *mut cgroup;
    let mut id: s32 = 0;

    status = bpf_probe_read_kernel(
        &mut id as *mut s32 as *mut ::core::ffi::c_void,
        core::mem::size_of_val(&id) as u32,
        &mut (*cgrp).self_.id as *mut _ as *const ::core::ffi::c_void,
    );
    if status != 0 {
        return status as ::core::ffi::c_int;
    }

    local.cgrp = core::ptr::null_mut();
    status = bpf_map_update_elem(
        &mut __cgrps_kfunc_map as *mut bpf_map_def as *mut ::core::ffi::c_void,
        &mut id as *mut s32 as *const ::core::ffi::c_void,
        &mut local as *mut __cgrps_kfunc_map_value as *const ::core::ffi::c_void,
        BPF_NOEXIST,
    );
    if status != 0 {
        return status as ::core::ffi::c_int;
    }

    v = bpf_map_lookup_elem(
        &mut __cgrps_kfunc_map as *mut bpf_map_def as *mut ::core::ffi::c_void,
        &mut id as *mut s32 as *const ::core::ffi::c_void,
    ) as *mut __cgrps_kfunc_map_value;
    if v.is_null() {
        bpf_map_delete_elem(
            &mut __cgrps_kfunc_map as *mut bpf_map_def as *mut ::core::ffi::c_void,
            &mut id as *mut s32 as *const ::core::ffi::c_void,
        );
        return -ENOENT;
    }

    acquired = bpf_cgroup_acquire(cgrp);
    if acquired.is_null() {
        bpf_map_delete_elem(
            &mut __cgrps_kfunc_map as *mut bpf_map_def as *mut ::core::ffi::c_void,
            &mut id as *mut s32 as *const ::core::ffi::c_void,
        );
        return -ENOENT;
    }

    old = bpf_kptr_xchg(
        &mut (*v).cgrp as *mut *mut cgroup as *mut ::core::ffi::c_void,
        acquired as *mut ::core::ffi::c_void,
    ) as *mut cgroup;
    if !old.is_null() {
        bpf_cgroup_release(old);
        return -EEXIST;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
