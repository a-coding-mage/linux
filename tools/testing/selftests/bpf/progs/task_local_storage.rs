// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// Translated from:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// struct {
//      __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
//      __uint(map_flags, BPF_F_NO_PREALLOC);
//      __type(key, int);
//      __type(value, long);
// } enter_id SEC(".maps");
#[repr(C)]
pub struct EnterId {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut enter_id: EnterId = EnterId { _private: [] };

// Translated from: #include "err.h"

pub const MAGIC_VALUE: libc::c_long = 0xabcd1234;

#[no_mangle]
pub static mut target_pid: pid_t = 0;
#[no_mangle]
pub static mut mismatch_cnt: libc::c_int = 0;
#[no_mangle]
pub static mut enter_cnt: libc::c_int = 0;
#[no_mangle]
pub static mut exit_cnt: libc::c_int = 0;
#[no_mangle]
pub static mut update_err: libc::c_long = 0;

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_storage_get(
        map: *mut EnterId,
        task: *mut task_struct,
        value: *mut libc::c_void,
        flags: u64,
    ) -> *mut libc::c_long;
    fn IS_ERR_VALUE(value: *mut bpf_local_storage_data) -> bool;
    fn PTR_ERR(value: *mut bpf_local_storage_data) -> libc::c_long;
}

// External constants supplied by BPF headers.
extern "C" {
    static BPF_LOCAL_STORAGE_GET_F_CREATE: u64;
}

// External types supplied by vmlinux.h / BPF headers.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub pid: pid_t,
}

#[repr(C)]
pub struct bpf_local_storage_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_local_storage_data {
    _private: [u8; 0],
}

pub type pid_t = libc::c_int;
pub type u64 = libc::c_ulonglong;

// SEC("tp_btf/sys_enter")
#[no_mangle]
#[link_section = "tp_btf/sys_enter"]
pub unsafe extern "C" fn on_enter(regs: *mut pt_regs, id: libc::c_long) -> libc::c_int {
    let mut task: *mut task_struct;
    let mut ptr: *mut libc::c_long;

    task = bpf_get_current_task_btf();
    if (*task).pid != target_pid {
        return 0;
    }

    ptr = bpf_task_storage_get(
        &mut enter_id,
        task,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if ptr.is_null() {
        return 0;
    }

    __sync_fetch_and_add(&mut enter_cnt, 1);
    *ptr = MAGIC_VALUE + enter_cnt as libc::c_long;

    return 0;
}

// SEC("tp_btf/sys_exit")
#[no_mangle]
#[link_section = "tp_btf/sys_exit"]
pub unsafe extern "C" fn on_exit(regs: *mut pt_regs, id: libc::c_long) -> libc::c_int {
    let mut task: *mut task_struct;
    let mut ptr: *mut libc::c_long;

    task = bpf_get_current_task_btf();
    if (*task).pid != target_pid {
        return 0;
    }

    ptr = bpf_task_storage_get(
        &mut enter_id,
        task,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if ptr.is_null() {
        return 0;
    }

    __sync_fetch_and_add(&mut exit_cnt, 1);
    if *ptr != MAGIC_VALUE + exit_cnt as libc::c_long {
        __sync_fetch_and_add(&mut mismatch_cnt, 1);
    }
    return 0;
}

// SEC("fexit/bpf_local_storage_update")
#[no_mangle]
#[link_section = "fexit/bpf_local_storage_update"]
pub unsafe extern "C" fn fexit_update(
    owner: *mut libc::c_void,
    smap: *mut bpf_local_storage_map,
    value: *mut libc::c_void,
    map_flags: u64,
    swap_uptrs: bool,
    ret: *mut bpf_local_storage_data,
) -> libc::c_int {
    let task: *mut task_struct = bpf_get_current_task_btf();

    if (*task).pid != target_pid {
        return 0;
    }

    if IS_ERR_VALUE(ret) {
        update_err = PTR_ERR(ret);
    }

    return 0;
}

unsafe fn __sync_fetch_and_add(ptr: *mut libc::c_int, val: libc::c_int) -> libc::c_int {
    core::sync::atomic::AtomicI32::from_ptr(ptr).fetch_add(val, core::sync::atomic::Ordering::SeqCst)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
