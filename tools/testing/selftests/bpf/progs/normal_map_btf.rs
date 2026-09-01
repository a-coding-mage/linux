// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */
// C dependencies translated as external Rust dependencies:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// "bpf_misc.h", and "bpf_experimental.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u64 = u64;

#[repr(C)]
pub struct bpf_list_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct node_data {
    pub data: __u64,
    pub node: bpf_list_node,
}

#[repr(C)]
pub struct map_value {
    // C field annotation: __contains(node_data, node)
    pub head: bpf_list_head,
    pub lock: bpf_spin_lock,
}

// Original C map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __type(key, int);
//     __type(value, struct map_value);
//     __uint(max_entries, 1);
// } array SEC(".maps");
#[repr(C)]
pub struct array_map {
    _private: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut array: array_map = array_map { _private: [] };

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut pid: i32 = 0;

#[no_mangle]
pub static mut done: bool = false;

extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_map_lookup_elem(map: *mut array_map, key: *const i32) -> *mut map_value;
    fn bpf_obj_new_node_data() -> *mut node_data;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_list_push_back(head: *mut bpf_list_head, node: *mut bpf_list_node);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
}

// SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[link_section = "fentry/sys_nanosleep"]
#[no_mangle]
pub unsafe extern "C" fn add_to_list_in_array(ctx: *mut core::ffi::c_void) -> i32 {
    let mut value: *mut map_value;
    let mut new: *mut node_data;
    let zero: i32 = 0;

    let _ = ctx;

    if done || bpf_get_current_pid_tgid() as i32 != pid {
        return 0;
    }

    value = bpf_map_lookup_elem(&mut array, &zero);
    if value.is_null() {
        return 0;
    }

    // C source used: bpf_obj_new(typeof(*new))
    new = bpf_obj_new_node_data();
    if new.is_null() {
        return 0;
    }

    bpf_spin_lock(&mut (*value).lock);
    bpf_list_push_back(&mut (*value).head, &mut (*new).node);
    bpf_spin_unlock(&mut (*value).lock);
    done = true;

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
