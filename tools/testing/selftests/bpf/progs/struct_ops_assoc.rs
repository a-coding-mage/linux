// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external/file-provided symbols:
// <vmlinux.h>, <bpf/bpf_tracing.h>, "bpf_misc.h",
// "../test_kmods/bpf_testmod.h", "../test_kmods/bpf_testmod_kfunc.h"

#[repr(C)]
pub struct st_ops_args {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _unused: [u8; 0],
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
    fn bpf_kfunc_multi_st_ops_test_1_assoc(args: *mut st_ops_args) -> i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut test_pid: i32 = 0;

/* Programs associated with st_ops_map_a */

pub const MAP_A_MAGIC: i32 = 1234;

#[unsafe(no_mangle)]
pub static mut test_err_a: i32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "struct_ops")]
pub unsafe extern "C" fn test_1_a(_args: *mut st_ops_args) -> i32 {
    return MAP_A_MAGIC;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/sys_enter")]
pub unsafe extern "C" fn sys_enter_prog_a(_regs: *mut pt_regs, _id: i64) -> i32 {
    let mut args: st_ops_args = core::mem::zeroed();
    let task: *mut task_struct;
    let ret: i32;

    task = bpf_get_current_task_btf();
    if test_pid == 0 || (*task).pid != test_pid {
        return 0;
    }

    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&mut args);
    if ret != MAP_A_MAGIC {
        test_err_a += 1;
    }

    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn syscall_prog_a(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut args: st_ops_args = core::mem::zeroed();
    let ret: i32;

    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&mut args);
    if ret != MAP_A_MAGIC {
        test_err_a += 1;
    }

    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".struct_ops.link")]
pub static mut st_ops_map_a: bpf_testmod_multi_st_ops = bpf_testmod_multi_st_ops {
    test_1: test_1_a as *mut core::ffi::c_void,
};

/* Programs associated with st_ops_map_b */

pub const MAP_B_MAGIC: i32 = 5678;

#[unsafe(no_mangle)]
pub static mut test_err_b: i32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "struct_ops")]
pub unsafe extern "C" fn test_1_b(_args: *mut st_ops_args) -> i32 {
    return MAP_B_MAGIC;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/sys_enter")]
pub unsafe extern "C" fn sys_enter_prog_b(_regs: *mut pt_regs, _id: i64) -> i32 {
    let mut args: st_ops_args = core::mem::zeroed();
    let task: *mut task_struct;
    let ret: i32;

    task = bpf_get_current_task_btf();
    if test_pid == 0 || (*task).pid != test_pid {
        return 0;
    }

    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&mut args);
    if ret != MAP_B_MAGIC {
        test_err_b += 1;
    }

    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn syscall_prog_b(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut args: st_ops_args = core::mem::zeroed();
    let ret: i32;

    ret = bpf_kfunc_multi_st_ops_test_1_assoc(&mut args);
    if ret != MAP_B_MAGIC {
        test_err_b += 1;
    }

    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".struct_ops.link")]
pub static mut st_ops_map_b: bpf_testmod_multi_st_ops = bpf_testmod_multi_st_ops {
    test_1: test_1_b as *mut core::ffi::c_void,
};
