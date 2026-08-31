// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// Dependencies originally supplied by:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

type pid_t = i32;
type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct callback_ctx {
    pub dummy: i32,
}

const VM_EXEC: __u64 = 0x00000004;
const DNAME_INLINE_LEN: usize = 32;

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_probe_read_kernel_str(dst: *mut ::core::ffi::c_void, size: __u32, unsafe_ptr: *const ::core::ffi::c_void) -> i64;
    fn bpf_find_vma(
        task: *mut task_struct,
        addr: __u64,
        callback_fn: Option<
            unsafe extern "C" fn(
                task: *mut task_struct,
                vma: *mut vm_area_struct,
                data: *mut callback_ctx,
            ) -> i64,
        >,
        callback_ctx: *mut callback_ctx,
        flags: __u64,
    ) -> i64;
}

extern "C" {
    pub type task_struct;
    pub type vm_area_struct;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut target_pid: pid_t = 0;

#[no_mangle]
pub static mut d_iname: [u8; DNAME_INLINE_LEN] = [0; DNAME_INLINE_LEN];

#[no_mangle]
pub static mut found_vm_exec: __u32 = 0;

#[no_mangle]
pub static mut addr: __u64 = 0;

#[no_mangle]
pub static mut find_zero_ret: i32 = -1;

#[no_mangle]
pub static mut find_addr_ret: i32 = -1;

unsafe extern "C" fn check_vma(
    _task: *mut task_struct,
    vma: *mut vm_area_struct,
    _data: *mut callback_ctx,
) -> i64 {
    if !(*vma).vm_file.is_null() {
        bpf_probe_read_kernel_str(
            d_iname.as_mut_ptr() as *mut ::core::ffi::c_void,
            (DNAME_INLINE_LEN - 1) as __u32,
            (*(*(*vma).vm_file).f_path.dentry)
                .d_shortname
                .string
                .as_ptr() as *const ::core::ffi::c_void,
        );
    }

    /* check for VM_EXEC */
    if (*vma).vm_flags & VM_EXEC != 0 {
        found_vm_exec = 1;
    }

    0
}

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn handle_getpid() -> i32 {
    let task: *mut task_struct = bpf_get_current_task_btf();
    let mut data: callback_ctx = callback_ctx { dummy: 0 };

    if (*task).pid != target_pid {
        return 0;
    }

    find_addr_ret = bpf_find_vma(task, addr, Some(check_vma), &mut data, 0) as i32;

    /* this should return -ENOENT */
    find_zero_ret = bpf_find_vma(task, 0, Some(check_vma), &mut data, 0) as i32;
    0
}

#[no_mangle]
#[link_section = "perf_event"]
pub unsafe extern "C" fn handle_pe() -> i32 {
    let task: *mut task_struct = bpf_get_current_task_btf();
    let mut data: callback_ctx = callback_ctx { dummy: 0 };

    if (*task).pid != target_pid {
        return 0;
    }

    find_addr_ret = bpf_find_vma(task, addr, Some(check_vma), &mut data, 0) as i32;

    /* In NMI, this should return -EBUSY, as the previous call is using
     * the irq_work.
     */
    find_zero_ret = bpf_find_vma(task, 0, Some(check_vma), &mut data, 0) as i32;
    0
}
