// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"
// #include "../test_kmods/bpf_testmod.h"
// #include "../test_kmods/bpf_testmod_kfunc.h"

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

// #define bpf_kfunc_multi_st_ops_test_1(args) bpf_kfunc_multi_st_ops_test_1(args, st_ops_id)
#[unsafe(no_mangle)]
pub static mut st_ops_id: i32 = 0;

#[unsafe(no_mangle)]
pub static mut test_pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut test_err: i32 = 0;

const MAP2_MAGIC: i32 = 4567;

#[repr(C)]
pub struct st_ops_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub pid: i32,
}

#[repr(C)]
pub struct bpf_testmod_multi_st_ops {
    pub test_1: *mut core::ffi::c_void,
}

unsafe extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    #[link_name = "bpf_kfunc_multi_st_ops_test_1"]
    fn bpf_kfunc_multi_st_ops_test_1_with_id(args: *mut st_ops_args, st_ops_id: i32) -> i32;
}

#[inline(always)]
unsafe fn bpf_kfunc_multi_st_ops_test_1(args: *mut st_ops_args) -> i32 {
    unsafe { bpf_kfunc_multi_st_ops_test_1_with_id(args, st_ops_id) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "struct_ops")]
pub unsafe extern "C" fn test_1(args: *mut st_ops_args) -> i32 {
    let _ = args;
    MAP2_MAGIC
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/sys_enter")]
pub unsafe extern "C" fn sys_enter(regs: *mut pt_regs, id: isize) -> i32 {
    let _ = regs;
    let _ = id;
    let mut args: st_ops_args = unsafe { core::mem::zeroed() };
    let task: *mut task_struct;
    let ret: i32;

    task = unsafe { bpf_get_current_task_btf() };
    if unsafe { test_pid == 0 || (*task).pid != test_pid } {
        return 0;
    }

    ret = unsafe { bpf_kfunc_multi_st_ops_test_1(&mut args) };
    if ret != MAP2_MAGIC {
        unsafe {
            test_err += 1;
        }
    }

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn syscall_prog(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let mut args: st_ops_args = unsafe { core::mem::zeroed() };
    let ret: i32;

    ret = unsafe { bpf_kfunc_multi_st_ops_test_1(&mut args) };
    if ret != MAP2_MAGIC {
        unsafe {
            test_err += 1;
        }
    }

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".struct_ops.link")]
pub static mut st_ops_map: bpf_testmod_multi_st_ops = bpf_testmod_multi_st_ops {
    test_1: test_1 as *mut core::ffi::c_void,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
