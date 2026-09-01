// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external symbols/types:
// <vmlinux.h>, <bpf/bpf_tracing.h>, "bpf_misc.h",
// "../test_kmods/bpf_testmod.h", "../test_kmods/bpf_testmod_kfunc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;

#[repr(C)]
pub struct st_ops_args {
    pub a: i32,
}

#[repr(C)]
pub struct bpf_testmod_st_ops {
    pub test_pro_epilogue: *mut c_void,
}

extern "C" {
    fn bpf_kfunc_st_ops_inc10(args: *mut st_ops_args) -> i32;
    fn bpf_kfunc_st_ops_test_pro_epilogue(args: *mut st_ops_args) -> i32;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub unsafe extern "C" fn __kfunc_btf_root() {
    bpf_kfunc_st_ops_inc10(core::ptr::null_mut());
}

#[inline(never)]
#[used]
static subprog_used: unsafe extern "C" fn(*mut st_ops_args) -> i32 = subprog;

unsafe extern "C" fn subprog(args: *mut st_ops_args) -> i32 {
    (*args).a += 1;
    (*args).a
}

// __success
/* prologue */
// __xlated("0: r8 = r1")
// __xlated("1: r1 = 0")
// __xlated("2: call kernel-function")
// __xlated("3: if r0 != 0x0 goto pc+5")
// __xlated("4: r6 = *(u64 *)(r8 +0)")
// __xlated("5: r7 = *(u64 *)(r6 +0)")
// __xlated("6: r7 += 1000")
// __xlated("7: *(u64 *)(r6 +0) = r7")
// __xlated("8: goto pc+2")
// __xlated("9: r1 = r0")
// __xlated("10: call kernel-function")
// __xlated("11: r1 = r8")
/* save __u64 *ctx to stack */
// __xlated("12: *(u64 *)(r10 -8) = r1")
/* main prog */
// __xlated("13: r1 = *(u64 *)(r1 +0)")
// __xlated("14: r6 = r1")
// __xlated("15: call kernel-function")
// __xlated("16: r1 = r6")
// __xlated("17: call pc+")
/* epilogue */
// __xlated("18: r1 = 0")
// __xlated("19: r6 = 0")
// __xlated("20: call kernel-function")
// __xlated("21: if r0 != 0x0 goto pc+6")
// __xlated("22: r1 = *(u64 *)(r10 -8)")
// __xlated("23: r1 = *(u64 *)(r1 +0)")
// __xlated("24: r6 = *(u64 *)(r1 +0)")
// __xlated("25: r6 += 10000")
// __xlated("26: *(u64 *)(r1 +0) = r6")
// __xlated("27: goto pc+2")
// __xlated("28: r1 = r0")
// __xlated("29: call kernel-function")
// __xlated("30: r0 = r6")
// __xlated("31: r0 *= 2")
// __xlated("32: exit")
#[link_section = "struct_ops/test_pro_epilogue"]
#[no_mangle]
pub unsafe extern "C" fn test_kfunc_pro_epilogue() -> i32 {
    asm!(
        "r1 = *(u64 *)(r1 +0);",
        "r6 = r1;",
        "call {bpf_kfunc_st_ops_inc10};",
        "r1 = r6;",
        "call subprog;",
        "exit;",
        bpf_kfunc_st_ops_inc10 = sym bpf_kfunc_st_ops_inc10,
        options(noreturn)
    );
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn syscall_pro_epilogue(_ctx: *mut c_void) -> i32 {
    // __retval(22022)
    // (PROLOGUE_A [1000] + KFUNC_INC10 + SUBPROG_A [1] + EPILOGUE_A [10000]) * 2
    let mut args: st_ops_args = core::mem::zeroed();

    bpf_kfunc_st_ops_test_pro_epilogue(&mut args)
}

#[link_section = ".struct_ops.link"]
#[no_mangle]
pub static mut pro_epilogue_with_kfunc: bpf_testmod_st_ops = bpf_testmod_st_ops {
    test_pro_epilogue: test_kfunc_pro_epilogue as *mut c_void,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
