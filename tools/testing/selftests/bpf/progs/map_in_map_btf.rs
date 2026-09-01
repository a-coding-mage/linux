// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */

// Dependencies from the original C source:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// "bpf_misc.h", and "bpf_experimental.h".

#[repr(C)]
pub struct bpf_list_node {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct bpf_list_head {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct node_data {
    pub data: u64,
    pub node: bpf_list_node,
}

#[repr(C)]
pub struct map_value {
    // Original C annotation: __contains(node_data, node)
    pub head: bpf_list_head,
    pub lock: bpf_spin_lock,
}

#[repr(C)]
pub struct inner_array_type {
    // Original BPF map definition:
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __type(key, int);
    // __type(value, struct map_value);
    // __uint(max_entries, 1);
    _private: [u8; 0],
}

// Original C: struct inner_array_type inner_array SEC(".maps");
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut inner_array: inner_array_type = inner_array_type { _private: [] };

#[repr(C)]
pub struct outer_array_type {
    // Original BPF map definition:
    // __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    // __uint(key_size, 4);
    // __uint(value_size, 4);
    // __uint(max_entries, 1);
    // __array(values, struct inner_array_type);
    pub values: [*mut inner_array_type; 1],
}

// Original C: outer_array SEC(".maps") = { .values = { [0] = &inner_array } };
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut outer_array: outer_array_type = outer_array_type {
    values: [core::ptr::addr_of_mut!(inner_array)],
};

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;

#[unsafe(no_mangle)]
pub static mut done: bool = false;

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_obj_new_node_data() -> *mut node_data;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_list_push_back(head: *mut bpf_list_head, node: *mut bpf_list_node);
}

// Original section: SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[unsafe(link_section = "fentry/sys_nanosleep")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_to_list_in_inner_array(ctx: *mut core::ffi::c_void) -> i32 {
    let mut value: *mut map_value;
    let mut new: *mut node_data;
    let mut map: *mut bpf_map;
    let zero: i32 = 0;

    let _ = ctx;

    if done || (bpf_get_current_pid_tgid() as u32) != pid as u32 {
        return 0;
    }

    map = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(outer_array) as *mut core::ffi::c_void,
        core::ptr::addr_of!(zero) as *const core::ffi::c_void,
    ) as *mut bpf_map;
    if map.is_null() {
        return 0;
    }

    value = bpf_map_lookup_elem(
        map as *mut core::ffi::c_void,
        core::ptr::addr_of!(zero) as *const core::ffi::c_void,
    ) as *mut map_value;
    if value.is_null() {
        return 0;
    }

    // Original C: bpf_obj_new(typeof(*new))
    new = bpf_obj_new_node_data();
    if new.is_null() {
        return 0;
    }

    bpf_spin_lock(core::ptr::addr_of_mut!((*value).lock));
    bpf_list_push_back(
        core::ptr::addr_of_mut!((*value).head),
        core::ptr::addr_of_mut!((*new).node),
    );
    bpf_spin_unlock(core::ptr::addr_of_mut!((*value).lock));
    done = true;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
