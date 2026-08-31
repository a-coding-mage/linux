// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external Rust dependencies:
// <vmlinux.h>, <bpf/bpf_tracing.h>, "bpf_misc.h",
// "../test_kmods/bpf_testmod.h", "../test_kmods/bpf_testmod_kfunc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::{asm, naked_asm};

#[repr(C)]
pub struct st_ops_args {
    pub a: u64,
}

#[repr(C)]
pub struct bpf_testmod_st_ops {
    pub test_prologue: *mut core::ffi::c_void,
    pub test_epilogue: *mut core::ffi::c_void,
    pub test_pro_epilogue: *mut core::ffi::c_void,
}

unsafe extern "C" {
    fn bpf_kfunc_st_ops_inc10(args: *mut st_ops_args) -> i32;
    fn bpf_kfunc_st_ops_test_prologue(args: *mut st_ops_args) -> i32;
    fn bpf_kfunc_st_ops_test_epilogue(args: *mut st_ops_args) -> i32;
    fn bpf_kfunc_st_ops_test_pro_epilogue(args: *mut st_ops_args) -> i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __kfunc_btf_root() {
    unsafe {
        bpf_kfunc_st_ops_inc10(core::ptr::null_mut());
    }
}

#[inline(never)]
unsafe extern "C" fn subprog(args: *mut st_ops_args) -> i32 {
    unsafe {
        (*args).a = (*args).a.wrapping_add(1);
        (*args).a as i32
    }
}

// __success
// prologue
// __xlated("0: r6 = *(u64 *)(r1 +0)")
// __xlated("1: r7 = *(u64 *)(r6 +0)")
// __xlated("2: r7 += 1000")
// __xlated("3: *(u64 *)(r6 +0) = r7")
// main prog
// __xlated("4: r1 = *(u64 *)(r1 +0)")
// __xlated("5: r6 = r1")
// __xlated("6: call kernel-function")
// __xlated("7: r1 = r6")
// __xlated("8: call pc+1")
// __xlated("9: exit")
#[unsafe(no_mangle)]
#[unsafe(link_section = "struct_ops/test_prologue")]
#[unsafe(naked)]
pub unsafe extern "C" fn test_prologue() -> i32 {
    naked_asm!(
        "r1 = *(u64 *)(r1 +0);",
        "r6 = r1;",
        "call {bpf_kfunc_st_ops_inc10};",
        "r1 = r6;",
        "call {subprog};",
        "exit;",
        bpf_kfunc_st_ops_inc10 = sym bpf_kfunc_st_ops_inc10,
        subprog = sym subprog,
    );
}

// __success
// save __u64 *ctx to stack
// __xlated("0: *(u64 *)(r10 -8) = r1")
// main prog
// __xlated("1: r1 = *(u64 *)(r1 +0)")
// __xlated("2: r6 = r1")
// __xlated("3: call kernel-function")
// __xlated("4: r1 = r6")
// __xlated("5: call pc+")
// epilogue
// __xlated("6: r1 = *(u64 *)(r10 -8)")
// __xlated("7: r1 = *(u64 *)(r1 +0)")
// __xlated("8: r6 = *(u64 *)(r1 +0)")
// __xlated("9: r6 += 10000")
// __xlated("10: *(u64 *)(r1 +0) = r6")
// __xlated("11: r0 = r6")
// __xlated("12: r0 *= 2")
// __xlated("13: exit")
#[unsafe(no_mangle)]
#[unsafe(link_section = "struct_ops/test_epilogue")]
#[unsafe(naked)]
pub unsafe extern "C" fn test_epilogue() -> i32 {
    naked_asm!(
        "r1 = *(u64 *)(r1 +0);",
        "r6 = r1;",
        "call {bpf_kfunc_st_ops_inc10};",
        "r1 = r6;",
        "call {subprog};",
        "exit;",
        bpf_kfunc_st_ops_inc10 = sym bpf_kfunc_st_ops_inc10,
        subprog = sym subprog,
    );
}

// __success
// prologue
// __xlated("0: r6 = *(u64 *)(r1 +0)")
// __xlated("1: r7 = *(u64 *)(r6 +0)")
// __xlated("2: r7 += 1000")
// __xlated("3: *(u64 *)(r6 +0) = r7")
// save __u64 *ctx to stack
// __xlated("4: *(u64 *)(r10 -8) = r1")
// main prog
// __xlated("5: r1 = *(u64 *)(r1 +0)")
// __xlated("6: r6 = r1")
// __xlated("7: call kernel-function")
// __xlated("8: r1 = r6")
// __xlated("9: call pc+")
// epilogue
// __xlated("10: r1 = *(u64 *)(r10 -8)")
// __xlated("11: r1 = *(u64 *)(r1 +0)")
// __xlated("12: r6 = *(u64 *)(r1 +0)")
// __xlated("13: r6 += 10000")
// __xlated("14: *(u64 *)(r1 +0) = r6")
// __xlated("15: r0 = r6")
// __xlated("16: r0 *= 2")
// __xlated("17: exit")
#[unsafe(no_mangle)]
#[unsafe(link_section = "struct_ops/test_pro_epilogue")]
#[unsafe(naked)]
pub unsafe extern "C" fn test_pro_epilogue() -> i32 {
    naked_asm!(
        "r1 = *(u64 *)(r1 +0);",
        "r6 = r1;",
        "call {bpf_kfunc_st_ops_inc10};",
        "r1 = r6;",
        "call {subprog};",
        "exit;",
        bpf_kfunc_st_ops_inc10 = sym bpf_kfunc_st_ops_inc10,
        subprog = sym subprog,
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn syscall_prologue(_ctx: *mut core::ffi::c_void) -> i32 {
    // __retval(1011) /* PROLOGUE_A [1000] + KFUNC_INC10 + SUBPROG_A [1] */
    let mut args = st_ops_args { a: 0 };

    unsafe { bpf_kfunc_st_ops_test_prologue(&mut args) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn syscall_epilogue(_ctx: *mut core::ffi::c_void) -> i32 {
    // __retval(20022) /* (KFUNC_INC10 + SUBPROG_A [1] + EPILOGUE_A [10000]) * 2 */
    let mut args = st_ops_args { a: 0 };

    unsafe { bpf_kfunc_st_ops_test_epilogue(&mut args) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
pub unsafe extern "C" fn syscall_pro_epilogue(_ctx: *mut core::ffi::c_void) -> i32 {
    // __retval(22022) /* (PROLOGUE_A [1000] + KFUNC_INC10 + SUBPROG_A [1] + EPILOGUE_A [10000]) * 2 */
    let mut args = st_ops_args { a: 0 };

    unsafe { bpf_kfunc_st_ops_test_pro_epilogue(&mut args) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".struct_ops.link")]
pub static mut pro_epilogue: bpf_testmod_st_ops = bpf_testmod_st_ops {
    test_prologue: test_prologue as *mut core::ffi::c_void,
    test_epilogue: test_epilogue as *mut core::ffi::c_void,
    test_pro_epilogue: test_pro_epilogue as *mut core::ffi::c_void,
};
