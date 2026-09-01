// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

const PT_REGS_SIZE: usize = core::mem::size_of::<pt_regs>();

/*
 * The kernel struct pt_regs isn't exported in its entirety to userspace.
 * Pass it as an array to task_pt_regs.c
 */
#[no_mangle]
pub static mut current_regs: [u8; PT_REGS_SIZE] = [0; PT_REGS_SIZE];
#[no_mangle]
pub static mut ctx_regs: [u8; PT_REGS_SIZE] = [0; PT_REGS_SIZE];
#[no_mangle]
pub static mut uprobe_res: i32 = 0;

#[no_mangle]
#[link_section = "uprobe"]
pub unsafe extern "C" fn handle_uprobe(ctx: *mut pt_regs) -> i32 {
    let current: *mut task_struct;
    let regs: *mut pt_regs;

    current = bpf_get_current_task_btf();
    regs = bpf_task_pt_regs(current) as *mut pt_regs;
    if bpf_probe_read_kernel(
        current_regs.as_mut_ptr() as *mut core::ffi::c_void,
        PT_REGS_SIZE as u32,
        regs as *const core::ffi::c_void,
    ) != 0
    {
        return 0;
    }
    if bpf_probe_read_kernel(
        ctx_regs.as_mut_ptr() as *mut core::ffi::c_void,
        PT_REGS_SIZE as u32,
        ctx as *const core::ffi::c_void,
    ) != 0
    {
        return 0;
    }

    /* Prove that uprobe was run */
    uprobe_res = 1;

    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
