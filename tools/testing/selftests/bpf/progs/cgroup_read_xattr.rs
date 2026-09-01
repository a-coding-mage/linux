// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C dependencies: <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>, "bpf_experimental.h", "bpf_misc.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type u64 = u64;

pub const BPF_CGROUP_ITER_ANCESTORS_UP: u32 = 0;

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    pub self_: cgroup_subsys_state,
}

#[repr(C)]
pub struct cgroup_subsys_state {
    pub cgroup: *mut cgroup,
}

unsafe extern "C" {
    fn bpf_dynptr_from_mem(
        data: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_cgroup_read_xattr(
        cgroup: *mut cgroup,
        name: *const core::ffi::c_char,
        value_ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_get_current_cgroup_id() -> u64;
    fn bpf_cgroup_from_id(cgrp_id: u64) -> *mut cgroup;
    fn bpf_cgroup_release(cgrp: *mut cgroup);
    fn bpf_rcu_read_lock();
    fn bpf_rcu_read_unlock();
    fn bpf_cgroup_ancestor(cgrp: *mut cgroup, level: i32) -> *mut cgroup;

    // Rust translation of the C bpf_for_each(css, css, &cgrp->self, flags)
    // macro depends on external BPF iterator support supplied by headers.
    fn bpf_for_each_css(
        css: *mut *mut cgroup_subsys_state,
        start: *mut cgroup_subsys_state,
        flags: u32,
    ) -> bool;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as _, b'P' as _, b'L' as _, 0];

#[unsafe(no_mangle)]
pub static mut value: [core::ffi::c_char; 16] = [0; 16];

#[inline(always)]
unsafe fn read_xattr(cgroup: *mut cgroup) {
    let mut value_ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    unsafe {
        bpf_dynptr_from_mem(
            core::ptr::addr_of_mut!(value) as *mut core::ffi::c_void,
            core::mem::size_of_val(&*core::ptr::addr_of!(value)) as u32,
            0,
            value_ptr.as_mut_ptr(),
        );
        bpf_cgroup_read_xattr(cgroup, c"user.bpf_test".as_ptr(), value_ptr.as_mut_ptr());
    }
}

#[unsafe(link_section = "lsm.s/socket_connect")]
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trusted_cgroup_ptr_sleepable() -> i32 {
    let cgrp_id: u64 = unsafe { bpf_get_current_cgroup_id() };
    let cgrp: *mut cgroup;

    cgrp = unsafe { bpf_cgroup_from_id(cgrp_id) };
    if cgrp.is_null() {
        return 0;
    }

    unsafe {
        read_xattr(cgrp);
        bpf_cgroup_release(cgrp);
    }
    return 0;
}

#[unsafe(link_section = "lsm/socket_connect")]
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trusted_cgroup_ptr_non_sleepable() -> i32 {
    let cgrp_id: u64 = unsafe { bpf_get_current_cgroup_id() };
    let cgrp: *mut cgroup;

    cgrp = unsafe { bpf_cgroup_from_id(cgrp_id) };
    if cgrp.is_null() {
        return 0;
    }

    unsafe {
        read_xattr(cgrp);
        bpf_cgroup_release(cgrp);
    }
    return 0;
}

#[unsafe(link_section = "lsm/socket_connect")]
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_css_iter_non_sleepable() -> i32 {
    let cgrp_id: u64 = unsafe { bpf_get_current_cgroup_id() };
    let mut css: *mut cgroup_subsys_state = core::ptr::null_mut();
    let cgrp: *mut cgroup;

    cgrp = unsafe { bpf_cgroup_from_id(cgrp_id) };
    if cgrp.is_null() {
        return 0;
    }

    while unsafe {
        bpf_for_each_css(
            &mut css,
            core::ptr::addr_of_mut!((*cgrp).self_),
            BPF_CGROUP_ITER_ANCESTORS_UP,
        )
    } {
        unsafe { read_xattr((*css).cgroup) };
    }

    unsafe { bpf_cgroup_release(cgrp) };
    return 0;
}

#[unsafe(link_section = "lsm.s/socket_connect")]
// __failure __msg("kernel func bpf_iter_css_new requires RCU critical section protection")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_css_iter_sleepable_missing_rcu_lock() -> i32 {
    let cgrp_id: u64 = unsafe { bpf_get_current_cgroup_id() };
    let mut css: *mut cgroup_subsys_state = core::ptr::null_mut();
    let cgrp: *mut cgroup;

    cgrp = unsafe { bpf_cgroup_from_id(cgrp_id) };
    if cgrp.is_null() {
        return 0;
    }

    while unsafe {
        bpf_for_each_css(
            &mut css,
            core::ptr::addr_of_mut!((*cgrp).self_),
            BPF_CGROUP_ITER_ANCESTORS_UP,
        )
    } {
        unsafe { read_xattr((*css).cgroup) };
    }

    unsafe { bpf_cgroup_release(cgrp) };
    return 0;
}

#[unsafe(link_section = "lsm.s/socket_connect")]
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_css_iter_sleepable_with_rcu_lock() -> i32 {
    let cgrp_id: u64 = unsafe { bpf_get_current_cgroup_id() };
    let mut css: *mut cgroup_subsys_state = core::ptr::null_mut();
    let cgrp: *mut cgroup;

    unsafe { bpf_rcu_read_lock() };
    cgrp = unsafe { bpf_cgroup_from_id(cgrp_id) };
    if cgrp.is_null() {
        unsafe { bpf_rcu_read_unlock() };
        return 0;
    }

    while unsafe {
        bpf_for_each_css(
            &mut css,
            core::ptr::addr_of_mut!((*cgrp).self_),
            BPF_CGROUP_ITER_ANCESTORS_UP,
        )
    } {
        unsafe { read_xattr((*css).cgroup) };
    }

    unsafe {
        bpf_cgroup_release(cgrp);
        bpf_rcu_read_unlock();
    }
    return 0;
}

#[unsafe(link_section = "lsm/socket_connect")]
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_bpf_cgroup_ancestor() -> i32 {
    let cgrp_id: u64 = unsafe { bpf_get_current_cgroup_id() };
    let cgrp: *mut cgroup;
    let ancestor: *mut cgroup;

    cgrp = unsafe { bpf_cgroup_from_id(cgrp_id) };
    if cgrp.is_null() {
        return 0;
    }

    ancestor = unsafe { bpf_cgroup_ancestor(cgrp, 1) };
    if !ancestor.is_null() {
        unsafe {
            read_xattr(cgrp);
            bpf_cgroup_release(ancestor);
        }
    }

    unsafe { bpf_cgroup_release(cgrp) };
    return 0;
}

#[unsafe(link_section = "cgroup/sendmsg4")]
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cgroup_skb() -> i32 {
    let cgrp_id: u64 = unsafe { bpf_get_current_cgroup_id() };
    let cgrp: *mut cgroup;
    let ancestor: *mut cgroup;

    cgrp = unsafe { bpf_cgroup_from_id(cgrp_id) };
    if cgrp.is_null() {
        return 0;
    }

    ancestor = unsafe { bpf_cgroup_ancestor(cgrp, 1) };
    if !ancestor.is_null() {
        unsafe {
            read_xattr(cgrp);
            bpf_cgroup_release(ancestor);
        }
    }

    unsafe { bpf_cgroup_release(cgrp) };
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
