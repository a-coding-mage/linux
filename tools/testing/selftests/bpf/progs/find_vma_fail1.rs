// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
// Dependencies from vmlinux.h and <bpf/bpf_helpers.h> are expected to be
// provided by the surrounding BPF build environment.
// #define vm_flags vm_start

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_start: u64,
}

#[repr(C)]
pub struct callback_ctx {
    pub dummy: i32,
}

unsafe extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_find_vma(
        task: *mut task_struct,
        addr: u64,
        callback_fn: unsafe extern "C" fn(
            task: *mut task_struct,
            vma: *mut vm_area_struct,
            data: *mut callback_ctx,
        ) -> i64,
        callback_ctx: *mut callback_ctx,
        flags: u64,
    ) -> i64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" fn write_vma(
    task: *mut task_struct,
    vma: *mut vm_area_struct,
    data: *mut callback_ctx,
) -> i64 {
    let _ = task;
    let _ = data;

    /* writing to vma, which is illegal */
    unsafe {
        (*vma).vm_start = 0xffffffffff600000;
    }

    0
}

#[unsafe(link_section = "raw_tp/sys_enter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_getpid() -> i32 {
    let task: *mut task_struct = unsafe { bpf_get_current_task_btf() };
    let mut data: callback_ctx = callback_ctx { dummy: 0 };

    unsafe {
        bpf_find_vma(task, 0, write_vma, &mut data, 0);
    }
    0
}
