// SPDX-License-Identifier: GPL-2.0

// C includes translated as external dependency intent:
// <vmlinux.h>
// <bpf/bpf_tracing.h>
// "bpf_misc.h"
// "../test_kmods/bpf_testmod.h"
// "../test_kmods/bpf_testmod_kfunc.h"

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" {
    #[link_name = "bpf_kfunc_multi_st_ops_test_1"]
    fn bpf_kfunc_multi_st_ops_test_1_with_id(args: *mut st_ops_args, st_ops_id: ::core::ffi::c_int)
        -> ::core::ffi::c_int;

    fn bpf_get_current_task_btf() -> *mut task_struct;
}

#[inline(always)]
unsafe fn bpf_kfunc_multi_st_ops_test_1(args: *mut st_ops_args) -> ::core::ffi::c_int {
    unsafe { bpf_kfunc_multi_st_ops_test_1_with_id(args, st_ops_id) }
}

#[unsafe(no_mangle)]
pub static mut st_ops_id: ::core::ffi::c_int = 0;

#[unsafe(no_mangle)]
pub static mut test_pid: ::core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut test_err: ::core::ffi::c_int = 0;

pub const MAP1_MAGIC: ::core::ffi::c_int = 1234;

// External type supplied by vmlinux.h / bpf_testmod headers.
#[repr(C)]
pub struct st_ops_args {
    _unused: [u8; 0],
}

// External type supplied by vmlinux.h. Only the referenced prefix field is
// represented here for this source-level translation.
#[repr(C)]
pub struct task_struct {
    pub pid: ::core::ffi::c_int,
}

// External type supplied by vmlinux.h.
#[repr(C)]
pub struct pt_regs {
    _unused: [u8; 0],
}

// External type supplied by ../test_kmods/bpf_testmod.h.
#[repr(C)]
pub struct bpf_testmod_multi_st_ops {
    pub test_1: *mut ::core::ffi::c_void,
}

unsafe impl Sync for bpf_testmod_multi_st_ops {}

#[unsafe(link_section = "struct_ops")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_1(args: *mut st_ops_args) -> ::core::ffi::c_int {
    MAP1_MAGIC
}

#[unsafe(link_section = "tp_btf/sys_enter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter(
    regs: *mut pt_regs,
    id: ::core::ffi::c_long,
) -> ::core::ffi::c_int {
    let mut args: st_ops_args = unsafe { ::core::mem::zeroed() };
    let task: *mut task_struct;
    let ret: ::core::ffi::c_int;

    task = unsafe { bpf_get_current_task_btf() };
    if unsafe { test_pid == 0 || (*task).pid != test_pid } {
        return 0;
    }

    ret = unsafe { bpf_kfunc_multi_st_ops_test_1(&mut args) };
    if ret != MAP1_MAGIC {
        unsafe {
            test_err += 1;
        }
    }

    0
}

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_prog(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut args: st_ops_args = unsafe { ::core::mem::zeroed() };
    let ret: ::core::ffi::c_int;

    ret = unsafe { bpf_kfunc_multi_st_ops_test_1(&mut args) };
    if ret != MAP1_MAGIC {
        unsafe {
            test_err += 1;
        }
    }

    0
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut st_ops_map: bpf_testmod_multi_st_ops = bpf_testmod_multi_st_ops {
    test_1: test_1 as *mut ::core::ffi::c_void,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
