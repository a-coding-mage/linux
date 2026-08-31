// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Google */
/* Dependencies from the original C source:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct bpf_testmod_btf_type_tag_1 {
    pub a: i32,
}

#[repr(C)]
pub struct bpf_testmod_btf_type_tag_2 {
    pub p: *mut bpf_testmod_btf_type_tag_1,
}

#[no_mangle]
pub static mut g: __u64 = 0;

/* These declarations are supplied by vmlinux.h in the original program.  Only
 * the accessed fields from this source file are represented here.
 */
#[repr(C)]
pub struct css_rstat_cpu {
    pub updated_children: *mut cgroup_subsys_state,
}

#[repr(C)]
pub struct cgroup_subsys_state {
    pub rstat_cpu: *mut css_rstat_cpu,
}

#[repr(C)]
pub struct cgroup {
    pub self_: cgroup_subsys_state,
}

extern "C" {
    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_per_cpu_ptr(percpu_ptr: *mut css_rstat_cpu, cpu: __u32) -> *mut core::ffi::c_void;
}

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_btf_type_tag_percpu_1"]
pub unsafe extern "C" fn test_percpu1(arg: *mut bpf_testmod_btf_type_tag_1) -> i32 {
    g = (*arg).a as __u64;
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_btf_type_tag_percpu_2"]
pub unsafe extern "C" fn test_percpu2(arg: *mut bpf_testmod_btf_type_tag_2) -> i32 {
    g = (*(*arg).p).a as __u64;
    0
}

/* trace_cgroup_mkdir(struct cgroup *cgrp, const char *path)
 *
 * struct css_rstat_cpu {
 *   ...
 *   struct cgroup_subsys_state *updated_children;
 *   ...
 * };
 *
 * struct cgroup_subsys_state {
 *   ...
 *   struct css_rstat_cpu __percpu *rstat_cpu;
 *   ...
 * };
 *
 * struct cgroup {
 *   struct cgroup_subsys_state self;
 *   ...
 * };
 */
#[no_mangle]
#[link_section = "tp_btf/cgroup_mkdir"]
pub unsafe extern "C" fn test_percpu_load(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let _ = path;
    g = (*(*cgrp).self_.rstat_cpu).updated_children as __u64;
    0
}

#[no_mangle]
#[link_section = "tp_btf/cgroup_mkdir"]
pub unsafe extern "C" fn test_percpu_helper(
    cgrp: *mut cgroup,
    path: *const core::ffi::c_char,
) -> i32 {
    let _ = path;
    let mut rstat: *mut css_rstat_cpu;
    let cpu: __u32;

    cpu = bpf_get_smp_processor_id();
    rstat = bpf_per_cpu_ptr((*cgrp).self_.rstat_cpu, cpu) as *mut css_rstat_cpu;
    if !rstat.is_null() {
        /* READ_ONCE */
        core::ptr::read_volatile(rstat as *const core::ffi::c_long);
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
