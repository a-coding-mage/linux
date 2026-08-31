// SPDX-License-Identifier: GPL-2.0
//
// Translated from C source using vmlinux.h, bpf/bpf_tracing.h, and
// bpf/bpf_helpers.h dependencies.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

#[repr(C)]
pub struct task_struct {
    pub pid: i32,
}

#[repr(C)]
pub struct map_value {
    pub ptr: *mut task_struct,
}

#[repr(C)]
pub struct lru_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

pub const BPF_MAP_TYPE_LRU_HASH: u32 = 9;

#[link_section = ".maps"]
#[no_mangle]
pub static mut lru_map: lru_map_def = lru_map_def {
    type_: BPF_MAP_TYPE_LRU_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
};

#[no_mangle]
pub static mut pid: i32 = 0;

#[no_mangle]
pub static mut result: i32 = 1;

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_map_update_elem(
        map: *mut lru_map_def,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> i64;
    fn bpf_map_lookup_elem(map: *mut lru_map_def, key: *const c_void) -> *mut c_void;
    fn bpf_map_delete_elem(map: *mut lru_map_def, key: *const c_void) -> i64;
    fn bpf_ktime_get_ns() -> u64;
}

#[link_section = "fentry/bpf_ktime_get_ns"]
#[no_mangle]
pub unsafe extern "C" fn printk(ctx: *mut c_void) -> i32 {
    let v: map_value = map_value {
        ptr: core::ptr::null_mut(),
    };

    if pid == (*bpf_get_current_task_btf()).pid {
        let key: i32 = 0;
        bpf_map_update_elem(
            &mut lru_map,
            &key as *const i32 as *const c_void,
            &v as *const map_value as *const c_void,
            0,
        );
    }
    0
}

#[link_section = "fentry/do_nanosleep"]
#[no_mangle]
pub unsafe extern "C" fn nanosleep(ctx: *mut c_void) -> i32 {
    let val: map_value = map_value {
        ptr: core::ptr::null_mut(),
    };
    let mut v: *mut map_value;
    let current: *mut task_struct;

    let key: i32 = 0;
    bpf_map_update_elem(
        &mut lru_map,
        &key as *const i32 as *const c_void,
        &val as *const map_value as *const c_void,
        0,
    );
    v = bpf_map_lookup_elem(&mut lru_map, &key as *const i32 as *const c_void) as *mut map_value;
    if v.is_null() {
        return 0;
    }
    bpf_map_delete_elem(&mut lru_map, &key as *const i32 as *const c_void);
    current = bpf_get_current_task_btf();
    (*v).ptr = current;
    pid = (*current).pid;
    bpf_ktime_get_ns();
    result = ((*v).ptr.is_null()) as i32;
    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
