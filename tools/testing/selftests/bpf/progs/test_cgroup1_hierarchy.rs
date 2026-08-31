// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 Yafang Shao <laoar.shao@gmail.com> */

// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_core_read.h>

pub static mut target_ancestor_level: __u32 = 0;
pub static mut target_ancestor_cgid: __u64 = 0;
pub static mut target_pid: ::core::ffi::c_int = 0;
pub static mut target_hid: ::core::ffi::c_int = 0;

unsafe extern "C" {
    #[link_name = "bpf_task_get_cgroup1"]
    fn bpf_task_get_cgroup1(
        task: *mut task_struct,
        hierarchy_id: ::core::ffi::c_int,
    ) -> *mut cgroup;

    #[link_name = "bpf_cgroup_ancestor"]
    fn bpf_cgroup_ancestor(cgrp: *mut cgroup, level: ::core::ffi::c_int) -> *mut cgroup;

    #[link_name = "bpf_cgroup_release"]
    fn bpf_cgroup_release(cgrp: *mut cgroup);

    fn bpf_get_current_task_btf() -> *mut task_struct;
}

unsafe fn bpf_link_create_verify(cmd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let cgrp: *mut cgroup;
    let ancestor: *mut cgroup;
    let task: *mut task_struct;
    let mut ret: ::core::ffi::c_int = 0;

    if cmd != BPF_LINK_CREATE as ::core::ffi::c_int {
        return 0;
    }

    task = bpf_get_current_task_btf();

    /* Then it can run in parallel with others */
    if (*task).pid != target_pid {
        return 0;
    }

    cgrp = bpf_task_get_cgroup1(task, target_hid);
    if cgrp.is_null() {
        return 0;
    }

    /* Refuse it if its cgid or its ancestor's cgid is the target cgid */
    if (*(*cgrp).kn).id == target_ancestor_cgid {
        ret = -1;
    }

    ancestor = bpf_cgroup_ancestor(cgrp, target_ancestor_level as ::core::ffi::c_int);
    if ancestor.is_null() {
        bpf_cgroup_release(cgrp);
        return ret;
    }

    if (*(*ancestor).kn).id == target_ancestor_cgid {
        ret = -1;
    }
    bpf_cgroup_release(ancestor);

    bpf_cgroup_release(cgrp);
    ret
}

// SEC("lsm/bpf")
// int BPF_PROG(lsm_run, int cmd, union bpf_attr *attr, unsigned int size, bool kernel)
#[no_mangle]
pub unsafe extern "C" fn lsm_run(
    cmd: ::core::ffi::c_int,
    attr: *mut bpf_attr,
    size: ::core::ffi::c_uint,
    kernel: bool,
) -> ::core::ffi::c_int {
    let _ = attr;
    let _ = size;
    let _ = kernel;
    bpf_link_create_verify(cmd)
}

// SEC("lsm.s/bpf")
// int BPF_PROG(lsm_s_run, int cmd, union bpf_attr *attr, unsigned int size, bool kernel)
#[no_mangle]
pub unsafe extern "C" fn lsm_s_run(
    cmd: ::core::ffi::c_int,
    attr: *mut bpf_attr,
    size: ::core::ffi::c_uint,
    kernel: bool,
) -> ::core::ffi::c_int {
    let _ = attr;
    let _ = size;
    let _ = kernel;
    bpf_link_create_verify(cmd)
}

// SEC("fentry")
// int BPF_PROG(fentry_run)
#[no_mangle]
pub extern "C" fn fentry_run() -> ::core::ffi::c_int {
    0
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
