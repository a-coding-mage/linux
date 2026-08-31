// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 Chuyi Zhou <zhouchuyi@bytedance.com> */

// Translated from includes:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, "bpf_misc.h",
// and "bpf_experimental.h".

type pid_t = i32;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut target_pid: pid_t = 0;
#[no_mangle]
pub static mut root_cg_id: u64 = 0;
#[no_mangle]
pub static mut leaf_cg_id: u64 = 0;
#[no_mangle]
pub static mut first_cg_id: u64 = 0;
#[no_mangle]
pub static mut last_cg_id: u64 = 0;
#[no_mangle]
pub static mut pre_order_cnt: i32 = 0;
#[no_mangle]
pub static mut post_order_cnt: i32 = 0;
#[no_mangle]
pub static mut children_cnt: i32 = 0;
#[no_mangle]
pub static mut tree_high: i32 = 0;

#[repr(C)]
pub struct kernfs_node {
    pub id: u64,
}

#[repr(C)]
pub struct cgroup {
    pub self_: cgroup_subsys_state,
    pub kn: *mut kernfs_node,
}

#[repr(C)]
pub struct cgroup_subsys_state {
    pub cgroup: *mut cgroup,
}

#[repr(C)]
pub struct task_struct {
    pub pid: pid_t,
}

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;

    #[link_name = "bpf_cgroup_from_id"]
    fn bpf_cgroup_from_id(cgid: u64) -> *mut cgroup;
    #[link_name = "bpf_cgroup_release"]
    fn bpf_cgroup_release(p: *mut cgroup);
    #[link_name = "bpf_rcu_read_lock"]
    fn bpf_rcu_read_lock();
    #[link_name = "bpf_rcu_read_unlock"]
    fn bpf_rcu_read_unlock();

    // Rust-side equivalents for bpf_for_each(css, ...). These are external
    // iterator dependencies provided by the BPF build environment.
    fn bpf_for_each_css_next(
        pos: *mut *mut cgroup_subsys_state,
        root: *mut cgroup_subsys_state,
        flags: i32,
    ) -> bool;
}

const BPF_CGROUP_ITER_DESCENDANTS_POST: i32 = 0;
const BPF_CGROUP_ITER_DESCENDANTS_PRE: i32 = 1;
const BPF_CGROUP_ITER_CHILDREN: i32 = 2;
const BPF_CGROUP_ITER_ANCESTORS_UP: i32 = 3;

// Section name in C: SEC("fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
#[link_section = "fentry.s/sys_getpgid"]
pub unsafe extern "C" fn iter_css_for_each(ctx: *const core::ffi::c_void) -> i32 {
    let cur_task: *mut task_struct = bpf_get_current_task_btf();
    let mut root_css: *mut cgroup_subsys_state;
    let mut leaf_css: *mut cgroup_subsys_state;
    let mut pos: *mut cgroup_subsys_state = core::ptr::null_mut();
    let root_cgrp: *mut cgroup;
    let leaf_cgrp: *mut cgroup;
    let mut cur_cgrp: *mut cgroup;

    let _ = ctx;

    if (*cur_task).pid != target_pid {
        return 0;
    }

    root_cgrp = bpf_cgroup_from_id(root_cg_id);

    if root_cgrp.is_null() {
        return 0;
    }

    leaf_cgrp = bpf_cgroup_from_id(leaf_cg_id);

    if leaf_cgrp.is_null() {
        bpf_cgroup_release(root_cgrp);
        return 0;
    }
    root_css = &mut (*root_cgrp).self_;
    leaf_css = &mut (*leaf_cgrp).self_;
    tree_high = 0;
    children_cnt = tree_high;
    post_order_cnt = children_cnt;
    pre_order_cnt = post_order_cnt;
    last_cg_id = 0;
    first_cg_id = last_cg_id;

    bpf_rcu_read_lock();
    pos = core::ptr::null_mut();
    while bpf_for_each_css_next(
        &mut pos,
        root_css,
        BPF_CGROUP_ITER_DESCENDANTS_POST,
    ) {
        cur_cgrp = (*pos).cgroup;
        post_order_cnt += 1;
        last_cg_id = (*(*cur_cgrp).kn).id;
    }

    pos = core::ptr::null_mut();
    while bpf_for_each_css_next(
        &mut pos,
        root_css,
        BPF_CGROUP_ITER_DESCENDANTS_PRE,
    ) {
        cur_cgrp = (*pos).cgroup;
        pre_order_cnt += 1;
        if first_cg_id == 0 {
            first_cg_id = (*(*cur_cgrp).kn).id;
        }
    }

    pos = core::ptr::null_mut();
    while bpf_for_each_css_next(&mut pos, root_css, BPF_CGROUP_ITER_CHILDREN) {
        children_cnt += 1;
    }

    pos = core::ptr::null_mut();
    while bpf_for_each_css_next(&mut pos, leaf_css, BPF_CGROUP_ITER_ANCESTORS_UP) {
        tree_high += 1;
    }

    pos = core::ptr::null_mut();
    while bpf_for_each_css_next(&mut pos, root_css, BPF_CGROUP_ITER_ANCESTORS_UP) {
        tree_high -= 1;
    }
    bpf_rcu_read_unlock();
    bpf_cgroup_release(root_cgrp);
    bpf_cgroup_release(leaf_cgrp);
    0
}
