// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// "vmlinux.h", <errno.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

extern "C" {
    static mut CONFIG_PREEMPTION: bool;

    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_storage_get(
        map: *mut task_storage_map,
        task: *mut task_struct,
        value: *mut i32,
        flags: u64,
    ) -> *mut i32;
    fn bpf_task_storage_delete(map: *mut task_storage_map, task: *mut task_struct) -> i32;
}

pub enum socket {}
pub enum task_struct {}

#[repr(C)]
pub struct task_storage_map {
    _private: [u8; 0],
}

pub const EDEADLK: i32 = 35;
pub const ETIMEDOUT: i32 = 110;
pub const BPF_MAP_TYPE_TASK_STORAGE: u32 = 29;
pub const BPF_F_NO_PREALLOC: u32 = 1;
pub const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 1;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut nr_get_errs: i32 = 0;

#[no_mangle]
pub static mut nr_del_errs: i32 = 0;

// Original C BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, int);
// } task_storage SEC(".maps");
#[no_mangle]
#[link_section = ".maps"]
pub static mut task_storage: task_storage_map = task_storage_map { _private: [] };

// SEC("lsm.s/socket_post_create")
#[no_mangle]
#[link_section = "lsm.s/socket_post_create"]
pub unsafe extern "C" fn socket_post_create(
    sock: *mut socket,
    family: i32,
    type_: i32,
    protocol: i32,
    kern: i32,
) -> i32 {
    let mut task: *mut task_struct;
    let ret: i32;
    let mut zero: i32 = 0;
    let value: *mut i32;

    let _ = sock;
    let _ = family;
    let _ = type_;
    let _ = protocol;
    let _ = kern;

    if !CONFIG_PREEMPTION {
        return 0;
    }

    task = bpf_get_current_task_btf();
    value = bpf_task_storage_get(
        &mut task_storage,
        task,
        &mut zero,
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if value.is_null() {
        core::intrinsics::atomic_xadd_seqcst(&mut nr_get_errs, 1);
    }

    ret = bpf_task_storage_delete(&mut task_storage, bpf_get_current_task_btf());
    if ret == -EDEADLK || ret == -ETIMEDOUT {
        core::intrinsics::atomic_xadd_seqcst(&mut nr_del_errs, 1);
    }

    0
}
