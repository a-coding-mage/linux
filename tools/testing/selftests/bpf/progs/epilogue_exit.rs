// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies removed from executable Rust:
// <vmlinux.h>, <bpf/bpf_tracing.h>, "bpf_misc.h",
// "../test_kmods/bpf_testmod.h", "../test_kmods/bpf_testmod_kfunc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::arch::asm;

type c_int = i32;
type c_void = core::ffi::c_void;

#[repr(C)]
pub struct st_ops_args {
    pub a: i32,
}

#[repr(C)]
pub struct bpf_testmod_st_ops {
    pub test_epilogue: *mut c_void,
}

unsafe extern "C" {
    fn bpf_kfunc_st_ops_test_epilogue(args: *mut st_ops_args) -> c_int;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// __success
// save __u64 *ctx to stack
// __xlated("0: *(u64 *)(r10 -8) = r1")
// main prog
// __xlated("1: r1 = *(u64 *)(r1 +0)")
// __xlated("2: r2 = *(u64 *)(r1 +0)")
// __xlated("3: r3 = 0")
// __xlated("4: r4 = 1")
// __xlated("5: if r2 == 0x0 goto pc+10")
// __xlated("6: r0 = 0")
// __xlated("7: *(u64 *)(r1 +0) = r3")
// epilogue
// __xlated("8: r1 = *(u64 *)(r10 -8)")
// __xlated("9: r1 = *(u64 *)(r1 +0)")
// __xlated("10: r6 = *(u64 *)(r1 +0)")
// __xlated("11: r6 += 10000")
// __xlated("12: *(u64 *)(r1 +0) = r6")
// __xlated("13: r0 = r6")
// __xlated("14: r0 *= 2")
// __xlated("15: exit")
// 2nd part of the main prog after the first exit
// __xlated("16: *(u64 *)(r1 +0) = r4")
// __xlated("17: r0 = 1")
// Clear the r1 to ensure it does not have
// off-by-1 error and ensure it jumps back to the
// beginning of epilogue which initializes
// the r1 with the ctx ptr.
// __xlated("18: r1 = 0")
// __xlated("19: gotol pc-12")
#[unsafe(link_section = "struct_ops/test_epilogue_exit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_epilogue_exit() -> c_int {
    unsafe {
        asm!(
            "r1 = *(u64 *)(r1 +0);",
            "r2 = *(u64 *)(r1 +0);",
            "r3 = 0;",
            "r4 = 1;",
            "if r2 == 0 goto +3;",
            "r0 = 0;",
            "*(u64 *)(r1 + 0) = r3;",
            "exit;",
            "*(u64 *)(r1 + 0) = r4;",
            "r0 = 1;",
            "r1 = 0;",
            "exit;",
            options(noreturn)
        );
    }
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut epilogue_exit: bpf_testmod_st_ops = bpf_testmod_st_ops {
    test_epilogue: test_epilogue_exit as *mut c_void,
};

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_epilogue_exit0(_ctx: *mut c_void) -> c_int {
    let mut args: st_ops_args = st_ops_args { a: 1 };

    unsafe { bpf_kfunc_st_ops_test_epilogue(&mut args) }
}

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_epilogue_exit1(_ctx: *mut c_void) -> c_int {
    let mut args: st_ops_args = st_ops_args { a: 0 };

    unsafe { bpf_kfunc_st_ops_test_epilogue(&mut args) }
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
