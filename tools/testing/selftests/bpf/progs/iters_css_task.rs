// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 Chuyi Zhou <zhouchuyi@bytedance.com> */

/*
 * Translated from C. The original file depends on vmlinux.h, errno.h,
 * bpf_helpers.h, bpf_tracing.h, bpf_misc.h, and bpf_experimental.h for BPF
 * types, section attributes, helper macros, iterator macros, and ksyms.
 */

type u64 = u64;
type pid_t = i32;

const EPERM: i32 = 1;
const CSS_TASK_ITER_PROCS: u32 = 0;

#[repr(C)]
pub struct kernfs_node {
    pub id: u64,
}

#[repr(C)]
pub struct cgroup_subsys_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    pub self_: cgroup_subsys_state,
    pub kn: *mut kernfs_node,
}

#[repr(C)]
pub struct task_struct {
    pub pid: pid_t,
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
    pub seq_num: u64,
}

#[repr(C)]
pub struct bpf_iter__cgroup {
    pub meta: *mut bpf_iter_meta,
    pub cgroup: *mut cgroup,
}

extern "C" {
    #[link_name = "bpf_cgroup_acquire"]
    fn bpf_cgroup_acquire(p: *mut cgroup) -> *mut cgroup;
    #[link_name = "bpf_cgroup_from_id"]
    fn bpf_cgroup_from_id(cgid: u64) -> *mut cgroup;
    #[link_name = "bpf_cgroup_release"]
    fn bpf_cgroup_release(p: *mut cgroup);

    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_get_current_cgroup_id() -> u64;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut target_pid: pid_t = 0;
#[no_mangle]
pub static mut css_task_cnt: i32 = 0;
#[no_mangle]
pub static mut cg_id: u64 = 0;

// SEC("lsm/file_mprotect")
#[no_mangle]
pub unsafe extern "C" fn iter_css_task_for_each(
    vma: *mut vm_area_struct,
    reqprot: core::ffi::c_ulong,
    prot: core::ffi::c_ulong,
    ret: i32,
) -> i32 {
    let cur_task: *mut task_struct = bpf_get_current_task_btf();
    let mut css: *mut cgroup_subsys_state;
    let mut task: *mut task_struct;
    let cgrp: *mut cgroup;

    let _ = vma;
    let _ = reqprot;
    let _ = prot;

    if (*cur_task).pid != target_pid {
        return ret;
    }

    cgrp = bpf_cgroup_from_id(cg_id);

    if cgrp.is_null() {
        return -EPERM;
    }

    css = &mut (*cgrp).self_;
    css_task_cnt = 0;

    bpf_for_each!(css_task, task, css, CSS_TASK_ITER_PROCS, {
        if (*task).pid == target_pid {
            css_task_cnt += 1;
        }
    });

    bpf_cgroup_release(cgrp);

    -EPERM
}

#[inline]
unsafe fn cgroup_id(cgrp: *mut cgroup) -> u64 {
    (*(*cgrp).kn).id
}

// SEC("?iter/cgroup")
#[no_mangle]
pub unsafe extern "C" fn cgroup_id_printer(ctx: *mut bpf_iter__cgroup) -> i32 {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let cgrp: *mut cgroup = (*ctx).cgroup;
    let mut css: *mut cgroup_subsys_state;
    let mut task: *mut task_struct;

    /* epilogue */
    if cgrp.is_null() {
        BPF_SEQ_PRINTF!(seq, "epilogue\n");
        return 0;
    }

    /* prologue */
    if (*(*ctx).meta).seq_num == 0 {
        BPF_SEQ_PRINTF!(seq, "prologue\n");
    }

    BPF_SEQ_PRINTF!(seq, "%8llu\n", cgroup_id(cgrp));

    css = &mut (*cgrp).self_;
    css_task_cnt = 0;
    bpf_for_each!(css_task, task, css, CSS_TASK_ITER_PROCS, {
        if (*task).pid == target_pid {
            css_task_cnt += 1;
        }
    });

    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn iter_css_task_for_each_sleep() -> i32 {
    let cgrp_id: u64 = bpf_get_current_cgroup_id();
    let cgrp: *mut cgroup = bpf_cgroup_from_id(cgrp_id);
    let mut css: *mut cgroup_subsys_state;
    let mut task: *mut task_struct;

    if cgrp.is_null() {
        return 0;
    }
    css = &mut (*cgrp).self_;

    bpf_for_each!(css_task, task, css, CSS_TASK_ITER_PROCS, {});
    bpf_cgroup_release(cgrp);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
