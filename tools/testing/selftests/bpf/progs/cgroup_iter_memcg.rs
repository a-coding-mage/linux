// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/* Dependencies from the original C source:
 * - <vmlinux.h>
 * - <bpf/bpf_core_read.h>
 * - "cgroup_iter_memcg.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::c_int;

#[repr(C)]
pub struct bpf_iter__cgroup {
    pub cgroup: *mut cgroup,
}

#[repr(C)]
pub struct cgroup {
    pub self_: cgroup_subsys_state,
}

#[repr(C)]
pub struct cgroup_subsys_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mem_cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct memcg_query {
    pub nr_anon_mapped: i64,
    pub nr_shmem: i64,
    pub nr_file_pages: i64,
    pub nr_file_mapped: i64,
    pub pgfault: i64,
}

#[repr(C)]
pub enum node_stat_item {
    NR_ANON_MAPPED,
    NR_SHMEM,
    NR_FILE_PAGES,
    NR_FILE_MAPPED,
}

#[repr(C)]
pub enum vm_event_item {
    PGFAULT,
}

unsafe extern "C" {
    fn bpf_get_mem_cgroup(css: *mut cgroup_subsys_state) -> *mut mem_cgroup;
    fn bpf_put_mem_cgroup(memcg: *mut mem_cgroup);
    fn bpf_mem_cgroup_flush_stats(memcg: *mut mem_cgroup);
    fn bpf_mem_cgroup_page_state(memcg: *mut mem_cgroup, idx: i32) -> i64;
    fn bpf_mem_cgroup_vm_events(memcg: *mut mem_cgroup, idx: i32) -> i64;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* The latest values read are stored here. */
#[link_section = ".data.query"]
#[no_mangle]
pub static mut memcg_query: memcg_query = memcg_query {
    nr_anon_mapped: 0,
    nr_shmem: 0,
    nr_file_pages: 0,
    nr_file_mapped: 0,
    pgfault: 0,
};

/* Original section: SEC("iter.s/cgroup") */
#[link_section = "iter.s/cgroup"]
#[no_mangle]
pub unsafe extern "C" fn cgroup_memcg_query(ctx: *mut bpf_iter__cgroup) -> c_int {
    let cgrp: *mut cgroup = unsafe { (*ctx).cgroup };
    let css: *mut cgroup_subsys_state;
    let memcg: *mut mem_cgroup;

    if cgrp.is_null() {
        return 1;
    }

    css = unsafe { &mut (*cgrp).self_ };
    memcg = unsafe { bpf_get_mem_cgroup(css) };
    if memcg.is_null() {
        return 1;
    }

    unsafe {
        bpf_mem_cgroup_flush_stats(memcg);

        memcg_query.nr_anon_mapped = bpf_mem_cgroup_page_state(
            memcg,
            bpf_core_enum_value!(node_stat_item, NR_ANON_MAPPED),
        );
        memcg_query.nr_shmem = bpf_mem_cgroup_page_state(
            memcg,
            bpf_core_enum_value!(node_stat_item, NR_SHMEM),
        );
        memcg_query.nr_file_pages = bpf_mem_cgroup_page_state(
            memcg,
            bpf_core_enum_value!(node_stat_item, NR_FILE_PAGES),
        );
        memcg_query.nr_file_mapped = bpf_mem_cgroup_page_state(
            memcg,
            bpf_core_enum_value!(node_stat_item, NR_FILE_MAPPED),
        );
        memcg_query.pgfault = bpf_mem_cgroup_vm_events(
            memcg,
            bpf_core_enum_value!(vm_event_item, PGFAULT),
        );

        bpf_put_mem_cgroup(memcg);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
