// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"
// #include "../test_kmods/bpf_testmod.h"
// #include "../test_kmods/bpf_testmod_kfunc.h"

use core::ffi::c_void;

#[repr(C)]
pub struct st_ops_args {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_testmod_multi_st_ops {
    pub test_1: *mut c_void,
}

unsafe extern "C" {
    fn bpf_kfunc_multi_st_ops_test_1_assoc(args: *mut st_ops_args) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

pub const MAP_A_MAGIC: i32 = 1234;

#[unsafe(no_mangle)]
pub static mut test_err_a: i32 = 0;

#[unsafe(no_mangle)]
pub static mut recur: i32 = 0;

/*
 * test_1_a is reused. The kfunc should not be able to get the associated
 * struct_ops and call test_1 recursively as it is ambiguous.
 */
#[unsafe(link_section = "struct_ops")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_1_a(args: *mut st_ops_args) -> i32 {
    let ret: i32;

    if unsafe { recur } == 0 {
        unsafe {
            recur += 1;
        }
        ret = unsafe { bpf_kfunc_multi_st_ops_test_1_assoc(args) };
        if ret != -1 {
            unsafe {
                test_err_a += 1;
            }
        }
        unsafe {
            recur -= 1;
        }
    }

    MAP_A_MAGIC
}

/* Programs associated with st_ops_map_a */

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_prog_a(ctx: *mut c_void) -> i32 {
    let mut args: st_ops_args = unsafe { core::mem::zeroed() };
    let ret: i32;

    let _ = ctx;
    ret = unsafe { bpf_kfunc_multi_st_ops_test_1_assoc(&mut args) };
    if ret != MAP_A_MAGIC {
        unsafe {
            test_err_a += 1;
        }
    }

    0
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut st_ops_map_a: bpf_testmod_multi_st_ops = bpf_testmod_multi_st_ops {
    test_1: test_1_a as *mut c_void,
};

/* Programs associated with st_ops_map_b */

#[unsafe(no_mangle)]
pub static mut test_err_b: i32 = 0;

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_prog_b(ctx: *mut c_void) -> i32 {
    let mut args: st_ops_args = unsafe { core::mem::zeroed() };
    let ret: i32;

    let _ = ctx;
    ret = unsafe { bpf_kfunc_multi_st_ops_test_1_assoc(&mut args) };
    if ret != MAP_A_MAGIC {
        unsafe {
            test_err_b += 1;
        }
    }

    0
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut st_ops_map_b: bpf_testmod_multi_st_ops = bpf_testmod_multi_st_ops {
    test_1: test_1_a as *mut c_void,
};
