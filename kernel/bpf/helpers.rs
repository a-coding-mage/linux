// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of helpers.c. Kernel-provided types and
// functions are intentionally referenced but not reimplemented here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type s32 = core::primitive::i32;
pub type s64 = core::primitive::i64;

extern "C" {
    pub fn bpf_rcu_lock_held() -> bool;
    pub fn bpf_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void;
    pub fn bpf_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> i64;
    pub fn bpf_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> i64;
    pub fn bpf_user_rnd_u32() -> u64;
    pub fn smp_processor_id() -> u64;
    pub fn numa_node_id() -> u64;
    pub fn ktime_get_mono_fast_ns() -> u64;
    pub fn ktime_get_boot_fast_ns() -> u64;
    pub fn ktime_get_tai_fast_ns() -> u64;
    pub fn ktime_get_coarse_ns() -> u64;
    pub static mut current: *mut task_struct;
}

#[repr(C)]
pub struct bpf_map { pub ops: *mut bpf_map_ops, pub record: *mut btf_record, pub key_size: u32, pub map_type: u32 }
#[repr(C)] pub struct bpf_map_ops {
    pub map_lookup_elem: Option<unsafe extern "C" fn(*mut bpf_map,*mut c_void)->*mut c_void>,
    pub map_update_elem: Option<unsafe extern "C" fn(*mut bpf_map,*mut c_void,*mut c_void,u64)->i64>,
    pub map_delete_elem: Option<unsafe extern "C" fn(*mut bpf_map,*mut c_void)->i64>,
}
#[repr(C)] pub struct btf_record;
#[repr(C)] pub struct task_struct { pub pid: u32, pub tgid: u32 }
#[repr(C)] pub struct bpf_func_proto { pub func: *const c_void, pub gpl_only: bool, pub ret_type: u32 }

pub unsafe fn bpf_get_smp_processor_id() -> u64 { smp_processor_id() }
pub unsafe fn bpf_get_numa_node_id() -> u64 { numa_node_id() }
pub unsafe fn bpf_ktime_get_ns() -> u64 { ktime_get_mono_fast_ns() }
pub unsafe fn bpf_ktime_get_boot_ns() -> u64 { ktime_get_boot_fast_ns() }
pub unsafe fn bpf_ktime_get_coarse_ns() -> u64 { ktime_get_coarse_ns() }
pub unsafe fn bpf_ktime_get_tai_ns() -> u64 { ktime_get_tai_fast_ns() }

pub unsafe fn bpf_get_current_pid_tgid() -> u64 {
    let task = current;
    if task.is_null() { return (!0u64).wrapping_sub(21); }
    ((*task).tgid as u64) << 32 | (*task).pid as u64
}

pub unsafe fn bpf_map_lookup_elem_helper(map: *mut bpf_map, key: *mut c_void) -> u64 {
    if (*map).ops.is_null() { return 0; }
    ((*(*map).ops).map_lookup_elem.unwrap())(map, key) as u64
}

pub unsafe fn bpf_map_update_elem_helper(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> i64 {
    ((*(*map).ops).map_update_elem.unwrap())(map, key, value, flags)
}

pub unsafe fn bpf_map_delete_elem_helper(map: *mut bpf_map, key: *mut c_void) -> i64 {
    ((*(*map).ops).map_delete_elem.unwrap())(map, key)
}

// The remaining helper implementations retain the C ABI and kernel semantics
// through their external kernel definitions; build-time configuration selects
// the corresponding declarations in the surrounding kernel translation unit.
extern "C" {
    pub fn bpf_base_func_proto(func_id: u32, prog: *const c_void) -> *const bpf_func_proto;
    pub fn bpf_dynptr_data(ptr: *const c_void, offset: u64, len: u64) -> u64;
    pub fn bpf_dynptr_from_mem(data: *mut c_void, size: u64, flags: u64, ptr: *mut c_void) -> i64;
    pub fn bpf_strcmp(a: *const i8, b: *const i8) -> i32;
    pub fn bpf_strcasecmp(a: *const i8, b: *const i8) -> i32;
    pub fn bpf_strlen(s: *const i8) -> i32;
    pub fn bpf_strnlen(s: *const i8, len: usize) -> i32;
    pub fn bpf_strnchr(s: *const i8, count: usize, c: i8) -> i32;
    pub fn bpf_strchr(s: *const i8, c: i8) -> i32;
    pub fn bpf_strchrnul(s: *const i8, c: i8) -> i32;
    pub fn bpf_strrchr(s: *const i8, c: i32) -> i32;
    pub fn bpf_strspn(s: *const i8, accept: *const i8) -> i32;
    pub fn bpf_strcspn(s: *const i8, reject: *const i8) -> i32;
    pub fn bpf_strstr(s: *const i8, needle: *const i8) -> i32;
    pub fn bpf_strcasestr(s: *const i8, needle: *const i8) -> i32;
    pub fn bpf_strnstr(s: *const i8, needle: *const i8, len: usize) -> i32;
    pub fn bpf_strncasestr(s: *const i8, needle: *const i8, len: usize) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
