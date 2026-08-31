// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
// Dependencies from C source: "vmlinux.h" and <bpf/bpf_helpers.h>.

#[repr(C)]
pub struct task_struct {
    pub mm: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct callback_ctx {
    pub dummy: i32,
}

unsafe extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_find_vma(
        task: *mut task_struct,
        start: u64,
        callback: unsafe extern "C" fn(
            task: *mut task_struct,
            vma: *mut vm_area_struct,
            data: *mut callback_ctx,
        ) -> i64,
        callback_ctx: *mut callback_ctx,
        flags: u64,
    ) -> i64;
}

unsafe extern "C" fn write_task(
    task: *mut task_struct,
    _vma: *mut vm_area_struct,
    _data: *mut callback_ctx,
) -> i64 {
    /* writing to task, which is illegal */
    unsafe {
        (*task).mm = core::ptr::null_mut();
    }

    0
}

#[unsafe(link_section = "raw_tp/sys_enter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_getpid() -> i32 {
    let task: *mut task_struct = unsafe { bpf_get_current_task_btf() };
    let mut data: callback_ctx = callback_ctx { dummy: 0 };

    unsafe {
        bpf_find_vma(task, 0, write_task, &mut data, 0);
    }
    0
}
