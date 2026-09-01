// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022 Google LLC.
 */
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// and <bpf/bpf_core_read.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u64 = u64;
type uint64_t = u64;

const BPF_MAP_TYPE_PERCPU_HASH: u32 = 5;
const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_NOEXIST: u64 = 1;

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[repr(C)]
pub struct kernfs_node {
    pub id: __u64,
}

#[repr(C)]
pub struct cgroup_subsys_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    pub kn: *mut kernfs_node,
    pub self_: cgroup_subsys_state,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct percpu_attach_counter {
    /* Previous percpu state, to figure out if we have new updates */
    pub prev: __u64,
    /* Current percpu state */
    pub state: __u64,
}

#[repr(C)]
pub struct attach_counter {
    /* State propagated through children, pending aggregation */
    pub pending: __u64,
    /* Total state, including all cpus and all children */
    pub state: __u64,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut percpu_attach_counters: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_HASH,
    max_entries: 1024,
    key_size: core::mem::size_of::<__u64>() as u32,
    value_size: core::mem::size_of::<percpu_attach_counter>() as u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut attach_counters: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1024,
    key_size: core::mem::size_of::<__u64>() as u32,
    value_size: core::mem::size_of::<attach_counter>() as u32,
};

extern "C" {
    #[link_name = "css_rstat_updated"]
    fn css_rstat_updated(css: *mut cgroup_subsys_state, cpu: i32);
    #[link_name = "css_rstat_flush"]
    fn css_rstat_flush(css: *mut cgroup_subsys_state);

    fn bpf_map_update_elem(
        map: *mut bpf_map_def,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i32;
    fn bpf_map_lookup_elem(
        map: *mut bpf_map_def,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_map_lookup_percpu_elem(
        map: *mut bpf_map_def,
        key: *const core::ffi::c_void,
        cpu: i32,
    ) -> *mut core::ffi::c_void;
    fn bpf_get_smp_processor_id() -> i32;
    fn bpf_seq_printf(seq: *mut seq_file, fmt: *const u8, fmt_size: u32, ...) -> i32;
}

unsafe fn cgroup_id(cgrp: *mut cgroup) -> uint64_t {
    (*(*cgrp).kn).id
}

unsafe fn create_percpu_attach_counter(cg_id: __u64, state: __u64) -> i32 {
    let pcpu_init = percpu_attach_counter { prev: 0, state };

    bpf_map_update_elem(
        &raw mut percpu_attach_counters,
        &cg_id as *const __u64 as *const core::ffi::c_void,
        &pcpu_init as *const percpu_attach_counter as *const core::ffi::c_void,
        BPF_NOEXIST,
    )
}

unsafe fn create_attach_counter(cg_id: __u64, state: __u64, pending: __u64) -> i32 {
    let init = attach_counter { pending, state };

    bpf_map_update_elem(
        &raw mut attach_counters,
        &cg_id as *const __u64 as *const core::ffi::c_void,
        &init as *const attach_counter as *const core::ffi::c_void,
        BPF_NOEXIST,
    )
}

#[no_mangle]
#[link_section = "tp_btf/cgroup_attach_task"]
pub unsafe extern "C" fn counter(
    dst_cgrp: *mut cgroup,
    _path: *const i8,
    _task: *mut task_struct,
    _threadgroup: bool,
) -> i32 {
    let cg_id: __u64 = cgroup_id(dst_cgrp);
    let pcpu_counter = bpf_map_lookup_elem(
        &raw mut percpu_attach_counters,
        &cg_id as *const __u64 as *const core::ffi::c_void,
    ) as *mut percpu_attach_counter;

    if !pcpu_counter.is_null() {
        (*pcpu_counter).state = (*pcpu_counter).state.wrapping_add(1);
    } else if create_percpu_attach_counter(cg_id, 1) != 0 {
        return 0;
    }

    css_rstat_updated(&raw mut (*dst_cgrp).self_, bpf_get_smp_processor_id());
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_rstat_flush"]
pub unsafe extern "C" fn flusher(cgrp: *mut cgroup, parent: *mut cgroup, cpu: i32) -> i32 {
    let mut pcpu_counter: *mut percpu_attach_counter;
    let mut total_counter: *mut attach_counter;
    let parent_counter: *mut attach_counter;
    let cg_id: __u64 = cgroup_id(cgrp);
    let parent_cg_id: __u64 = if !parent.is_null() {
        cgroup_id(parent)
    } else {
        0
    };
    let state: __u64;
    let mut delta: __u64 = 0;

    /* Add CPU changes on this level since the last flush */
    pcpu_counter = bpf_map_lookup_percpu_elem(
        &raw mut percpu_attach_counters,
        &cg_id as *const __u64 as *const core::ffi::c_void,
        cpu,
    ) as *mut percpu_attach_counter;
    if !pcpu_counter.is_null() {
        state = (*pcpu_counter).state;
        delta = delta.wrapping_add(state.wrapping_sub((*pcpu_counter).prev));
        (*pcpu_counter).prev = state;
    }

    total_counter = bpf_map_lookup_elem(
        &raw mut attach_counters,
        &cg_id as *const __u64 as *const core::ffi::c_void,
    ) as *mut attach_counter;
    if total_counter.is_null() {
        if create_attach_counter(cg_id, delta, 0) != 0 {
            return 0;
        }
    } else {
        /* Collect pending stats from subtree */
        if (*total_counter).pending != 0 {
            delta = delta.wrapping_add((*total_counter).pending);
            (*total_counter).pending = 0;
        }

        /* Propagate changes to this cgroup's total */
        (*total_counter).state = (*total_counter).state.wrapping_add(delta);
    }

    /* Skip if there are no changes to propagate, or no parent */
    if delta == 0 || parent_cg_id == 0 {
        return 0;
    }

    /* Propagate changes to cgroup's parent */
    parent_counter = bpf_map_lookup_elem(
        &raw mut attach_counters,
        &parent_cg_id as *const __u64 as *const core::ffi::c_void,
    ) as *mut attach_counter;
    if !parent_counter.is_null() {
        (*parent_counter).pending = (*parent_counter).pending.wrapping_add(delta);
    } else {
        create_attach_counter(parent_cg_id, 0, delta);
    }
    0
}

#[no_mangle]
#[link_section = "iter.s/cgroup"]
pub unsafe extern "C" fn dumper(meta: *mut bpf_iter_meta, cgrp: *mut cgroup) -> i32 {
    let seq: *mut seq_file = (*meta).seq;
    let total_counter: *mut attach_counter;
    let cg_id: __u64 = if !cgrp.is_null() { cgroup_id(cgrp) } else { 0 };

    /* Do nothing for the terminal call */
    if cg_id == 0 {
        return 1;
    }

    /* Flush the stats to make sure we get the most updated numbers */
    css_rstat_flush(&raw mut (*cgrp).self_);

    total_counter = bpf_map_lookup_elem(
        &raw mut attach_counters,
        &cg_id as *const __u64 as *const core::ffi::c_void,
    ) as *mut attach_counter;
    if total_counter.is_null() {
        bpf_seq_printf(
            seq,
            b"cg_id: %llu, attach_counter: 0\n\0".as_ptr(),
            34,
            cg_id,
        );
    } else {
        bpf_seq_printf(
            seq,
            b"cg_id: %llu, attach_counter: %llu\n\0".as_ptr(),
            36,
            cg_id,
            (*total_counter).state,
        );
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
