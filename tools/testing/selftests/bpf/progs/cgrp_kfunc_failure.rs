// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// Dependencies from the original C file:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// "bpf_misc.h", and "cgrp_kfunc_common.h".

pub const BPF_NOEXIST: u64 = 1;
pub const ENOENT: i32 = 2;

#[repr(C)]
pub struct cgroup_subsys_state {
    pub id: i32,
}

#[repr(C)]
pub struct cgroup {
    pub old_dom_cgrp: *mut cgroup,
    pub self_: cgroup_subsys_state,
}

#[repr(C)]
pub struct __cgrps_kfunc_map_value {
    pub cgrp: *mut cgroup,
}

unsafe extern "C" {
    static mut __cgrps_kfunc_map: core::ffi::c_void;

    fn cgrps_kfunc_map_insert(cgrp: *mut cgroup) -> i32;
    fn cgrps_kfunc_map_value_lookup(cgrp: *mut cgroup) -> *mut __cgrps_kfunc_map_value;
    fn bpf_cgroup_acquire(cgrp: *mut cgroup) -> *mut cgroup;
    fn bpf_cgroup_release(cgrp: *mut cgroup);
    fn bpf_kptr_xchg(map_value: *mut *mut cgroup, ptr: *mut cgroup) -> *mut cgroup;
    fn bpf_rcu_read_lock();
    fn bpf_rcu_read_unlock();
    fn bpf_probe_read_kernel(
        dst: *mut core::ffi::c_void,
        size: usize,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i64;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn __sink(ptr: *mut cgroup);
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Prototype for all of the program trace events below:
 *
 * TRACE_EVENT(cgroup_mkdir,
 *         TP_PROTO(struct cgroup *cgrp, const char *path),
 *         TP_ARGS(cgrp, path)
 */

unsafe fn insert_lookup_cgrp(cgrp: *mut cgroup) -> *mut __cgrps_kfunc_map_value {
    let status: i32;

    status = cgrps_kfunc_map_insert(cgrp);
    if status != 0 {
        return core::ptr::null_mut();
    }

    cgrps_kfunc_map_value_lookup(cgrp)
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("Possibly NULL pointer passed to trusted R1")
pub unsafe extern "C" fn cgrp_kfunc_acquire_untrusted(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let acquired: *mut cgroup;
    let v: *mut __cgrps_kfunc_map_value;

    v = insert_lookup_cgrp(cgrp);
    if v.is_null() {
        return 0;
    }

    /* Can't invoke bpf_cgroup_acquire() on an untrusted pointer. */
    acquired = bpf_cgroup_acquire((*v).cgrp);
    if !acquired.is_null() {
        bpf_cgroup_release(acquired);
    }

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("Possibly NULL pointer passed to trusted R1")
pub unsafe extern "C" fn cgrp_kfunc_acquire_no_null_check(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let acquired: *mut cgroup;

    acquired = bpf_cgroup_acquire(cgrp);
    /*
     * Can't invoke bpf_cgroup_release() without checking the return value
     * of bpf_cgroup_acquire().
     */
    bpf_cgroup_release(acquired);

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("R1 is fp expected STRUCT cgroup")
pub unsafe extern "C" fn cgrp_kfunc_acquire_fp(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let acquired: *mut cgroup;
    let stack_cgrp: *mut cgroup = (&path as *const *const core::ffi::c_char) as *mut cgroup;

    /* Can't invoke bpf_cgroup_acquire() on a random frame pointer. */
    acquired = bpf_cgroup_acquire((&stack_cgrp as *const *mut cgroup) as *mut cgroup);
    if !acquired.is_null() {
        bpf_cgroup_release(acquired);
    }

    0
}

#[unsafe(link_section = "kretprobe/cgroup_destroy_locked")]
#[unsafe(no_mangle)]
// __failure __msg("calling kernel function bpf_cgroup_acquire is not allowed")
pub unsafe extern "C" fn cgrp_kfunc_acquire_unsafe_kretprobe(cgrp: *mut cgroup) -> i32 {
    let acquired: *mut cgroup;

    /* Can't acquire an untrusted struct cgroup * pointer. */
    acquired = bpf_cgroup_acquire(cgrp);
    if !acquired.is_null() {
        bpf_cgroup_release(acquired);
    }

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("cgrp_kfunc_acquire_trusted_walked")
pub unsafe extern "C" fn cgrp_kfunc_acquire_trusted_walked(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let acquired: *mut cgroup;

    /* Can't invoke bpf_cgroup_acquire() on a pointer obtained from walking a trusted cgroup. */
    acquired = bpf_cgroup_acquire((*cgrp).old_dom_cgrp);
    if !acquired.is_null() {
        bpf_cgroup_release(acquired);
    }

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("Possibly NULL pointer passed to trusted R1")
pub unsafe extern "C" fn cgrp_kfunc_acquire_null(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let acquired: *mut cgroup;

    /* Can't invoke bpf_cgroup_acquire() on a NULL pointer. */
    acquired = bpf_cgroup_acquire(core::ptr::null_mut());
    if !acquired.is_null() {
        bpf_cgroup_release(acquired);
    }

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("Unreleased reference")
pub unsafe extern "C" fn cgrp_kfunc_acquire_unreleased(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let acquired: *mut cgroup;

    acquired = bpf_cgroup_acquire(cgrp);

    /* Acquired cgroup is never released. */
    __sink(acquired);

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("Unreleased reference")
pub unsafe extern "C" fn cgrp_kfunc_xchg_unreleased(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let kptr: *mut cgroup;
    let v: *mut __cgrps_kfunc_map_value;

    v = insert_lookup_cgrp(cgrp);
    if v.is_null() {
        return 0;
    }

    kptr = bpf_kptr_xchg(&mut (*v).cgrp, core::ptr::null_mut());
    if kptr.is_null() {
        return 0;
    }

    /* Kptr retrieved from map is never released. */

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("release kfunc bpf_cgroup_release expects referenced PTR_TO_BTF_ID passed to R1")
pub unsafe extern "C" fn cgrp_kfunc_rcu_get_release(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let kptr: *mut cgroup;
    let v: *mut __cgrps_kfunc_map_value;

    v = insert_lookup_cgrp(cgrp);
    if v.is_null() {
        return 0;
    }

    bpf_rcu_read_lock();
    kptr = (*v).cgrp;
    if !kptr.is_null() {
        /* Can't release a cgroup kptr stored in a map. */
        bpf_cgroup_release(kptr);
    }
    bpf_rcu_read_unlock();

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("Possibly NULL pointer passed to trusted R1")
pub unsafe extern "C" fn cgrp_kfunc_release_untrusted(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let v: *mut __cgrps_kfunc_map_value;

    v = insert_lookup_cgrp(cgrp);
    if v.is_null() {
        return 0;
    }

    /* Can't invoke bpf_cgroup_release() on an untrusted pointer. */
    bpf_cgroup_release((*v).cgrp);

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("release kfunc bpf_cgroup_release expects referenced PTR_TO_BTF_ID passed to R1")
pub unsafe extern "C" fn cgrp_kfunc_release_fp(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let acquired: *mut cgroup = (&path as *const *const core::ffi::c_char) as *mut cgroup;

    /* Cannot release random frame pointer. */
    bpf_cgroup_release(acquired);

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("Possibly NULL pointer passed to trusted R1")
pub unsafe extern "C" fn cgrp_kfunc_release_null(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let mut local: __cgrps_kfunc_map_value;
    let mut v: *mut __cgrps_kfunc_map_value;
    let mut status: i64;
    let acquired: *mut cgroup;
    let old: *mut cgroup;
    let mut id: i32 = 0;

    status = bpf_probe_read_kernel(
        (&mut id as *mut i32).cast::<core::ffi::c_void>(),
        core::mem::size_of_val(&id),
        (&(*cgrp).self_.id as *const i32).cast::<core::ffi::c_void>(),
    );
    if status != 0 {
        return 0;
    }

    local.cgrp = core::ptr::null_mut();
    status = bpf_map_update_elem(
        (&mut __cgrps_kfunc_map as *mut core::ffi::c_void),
        (&id as *const i32).cast::<core::ffi::c_void>(),
        (&local as *const __cgrps_kfunc_map_value).cast::<core::ffi::c_void>(),
        BPF_NOEXIST,
    );
    if status != 0 {
        return status as i32;
    }

    v = bpf_map_lookup_elem(
        (&mut __cgrps_kfunc_map as *mut core::ffi::c_void),
        (&id as *const i32).cast::<core::ffi::c_void>(),
    )
    .cast::<__cgrps_kfunc_map_value>();
    if v.is_null() {
        return -ENOENT;
    }

    acquired = bpf_cgroup_acquire(cgrp);
    if acquired.is_null() {
        return -ENOENT;
    }

    old = bpf_kptr_xchg(&mut (*v).cgrp, acquired);

    /* old cannot be passed to bpf_cgroup_release() without a NULL check. */
    bpf_cgroup_release(old);

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
// __failure __msg("release kfunc bpf_cgroup_release expects referenced PTR_TO_BTF_ID passed to R1")
pub unsafe extern "C" fn cgrp_kfunc_release_unacquired(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    /* Cannot release trusted cgroup pointer which was not acquired. */
    bpf_cgroup_release(cgrp);

    0
}
