// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/*
 * C includes translated as external dependencies:
 * - <vmlinux.h>
 * - <bpf/bpf_tracing.h>
 * - "bpf_misc.h"
 * - "../test_kmods/bpf_testmod.h"
 * - "../test_kmods/bpf_testmod_kfunc.h"
 */

pub type __u32 = u32;

pub const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct st_ops_args {
    pub a: i32,
}

#[repr(C)]
pub struct bpf_testmod_st_ops {
    pub test_epilogue: *mut core::ffi::c_void,
}

unsafe extern "C" {
    pub fn bpf_tail_call(
        ctx: *mut core::ffi::c_void,
        map: *mut core::ffi::c_void,
        index: __u32,
    );
    pub fn bpf_kfunc_st_ops_test_epilogue(args: *mut st_ops_args) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[inline(never)]
unsafe fn subprog(args: *mut st_ops_args) -> i32 {
    unsafe {
        (*args).a += 1;
        (*args).a
    }
}

#[unsafe(link_section = "struct_ops/test_epilogue_subprog")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_epilogue_subprog(args: *mut st_ops_args) -> i32 {
    unsafe {
        subprog(args);
        (*args).a
    }
}

#[repr(C)]
pub struct epilogue_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub values: [*mut core::ffi::c_void; 1],
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut epilogue_map: epilogue_map_def = epilogue_map_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
    values: [test_epilogue_subprog as *mut core::ffi::c_void],
};

#[unsafe(link_section = "struct_ops/test_epilogue_tailcall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_epilogue_tailcall(ctx: *mut u64) -> i32 {
    unsafe {
        bpf_tail_call(
            ctx as *mut core::ffi::c_void,
            core::ptr::addr_of_mut!(epilogue_map) as *mut core::ffi::c_void,
            0,
        );
    }
    0
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut epilogue_tailcall: bpf_testmod_st_ops = bpf_testmod_st_ops {
    test_epilogue: test_epilogue_tailcall as *mut core::ffi::c_void,
};

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut epilogue_subprog: bpf_testmod_st_ops = bpf_testmod_st_ops {
    test_epilogue: test_epilogue_subprog as *mut core::ffi::c_void,
};

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_epilogue_tailcall(args: *mut st_ops_args) -> i32 {
    unsafe { bpf_kfunc_st_ops_test_epilogue(args) }
}
