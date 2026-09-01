// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies: <vmlinux.h>, <bpf/bpf_tracing.h>, "bpf_misc.h",
// "../test_kmods/bpf_testmod.h", "../test_kmods/bpf_testmod_kfunc.h"

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    fn bpf_kfunc_st_ops_test_prologue(args: *mut st_ops_args) -> i32;
    fn bpf_kfunc_st_ops_test_epilogue(args: *mut st_ops_args) -> i32;
    fn bpf_kfunc_st_ops_test_pro_epilogue(args: *mut st_ops_args) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_ops_args {
    // Definition is supplied by "../test_kmods/bpf_testmod.h".
    _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_st_ops {
    pub test_prologue: *mut core::ffi::c_void,
    pub test_epilogue: *mut core::ffi::c_void,
    pub test_pro_epilogue: *mut core::ffi::c_void,
}

// __success
/* prologue */
// __xlated("0: r6 = *(u64 *)(r1 +0)")
// __xlated("1: r7 = *(u64 *)(r6 +0)")
// __xlated("2: r7 += 1000")
// __xlated("3: *(u64 *)(r6 +0) = r7")
/* main prog */
// __xlated("4: if r1 == 0x0 goto pc+5")
// __xlated("5: if r1 == 0x1 goto pc+2")
// __xlated("6: r1 = 1")
// __xlated("7: goto pc-3")
// __xlated("8: r1 = 0")
// __xlated("9: goto pc-6")
// __xlated("10: r0 = 0")
// __xlated("11: exit")
#[link_section = "struct_ops/test_prologue_goto_start"]
#[no_mangle]
pub unsafe extern "C" fn test_prologue_goto_start() -> i32 {
    core::arch::asm!(
        "if r1 == 0 goto +5",
        "if r1 == 1 goto +2",
        "r1 = 1",
        "goto -3",
        "r1 = 0",
        "goto -6",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

// __success
/* save __u64 *ctx to stack */
// __xlated("0: *(u64 *)(r10 -8) = r1")
/* main prog */
// __xlated("1: if r1 == 0x0 goto pc+5")
// __xlated("2: if r1 == 0x1 goto pc+2")
// __xlated("3: r1 = 1")
// __xlated("4: goto pc-3")
// __xlated("5: r1 = 0")
// __xlated("6: goto pc-6")
// __xlated("7: r0 = 0")
/* epilogue */
// __xlated("8: r1 = *(u64 *)(r10 -8)")
// __xlated("9: r1 = *(u64 *)(r1 +0)")
// __xlated("10: r6 = *(u64 *)(r1 +0)")
// __xlated("11: r6 += 10000")
// __xlated("12: *(u64 *)(r1 +0) = r6")
// __xlated("13: r0 = r6")
// __xlated("14: r0 *= 2")
// __xlated("15: exit")
#[link_section = "struct_ops/test_epilogue_goto_start"]
#[no_mangle]
pub unsafe extern "C" fn test_epilogue_goto_start() -> i32 {
    core::arch::asm!(
        "if r1 == 0 goto +5",
        "if r1 == 1 goto +2",
        "r1 = 1",
        "goto -3",
        "r1 = 0",
        "goto -6",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

// __success
/* prologue */
// __xlated("0: r6 = *(u64 *)(r1 +0)")
// __xlated("1: r7 = *(u64 *)(r6 +0)")
// __xlated("2: r7 += 1000")
// __xlated("3: *(u64 *)(r6 +0) = r7")
/* save __u64 *ctx to stack */
// __xlated("4: *(u64 *)(r10 -8) = r1")
/* main prog */
// __xlated("5: if r1 == 0x0 goto pc+5")
// __xlated("6: if r1 == 0x1 goto pc+2")
// __xlated("7: r1 = 1")
// __xlated("8: goto pc-3")
// __xlated("9: r1 = 0")
// __xlated("10: goto pc-6")
// __xlated("11: r0 = 0")
/* epilogue */
// __xlated("12: r1 = *(u64 *)(r10 -8)")
// __xlated("13: r1 = *(u64 *)(r1 +0)")
// __xlated("14: r6 = *(u64 *)(r1 +0)")
// __xlated("15: r6 += 10000")
// __xlated("16: *(u64 *)(r1 +0) = r6")
// __xlated("17: r0 = r6")
// __xlated("18: r0 *= 2")
// __xlated("19: exit")
#[link_section = "struct_ops/test_pro_epilogue_goto_start"]
#[no_mangle]
pub unsafe extern "C" fn test_pro_epilogue_goto_start() -> i32 {
    core::arch::asm!(
        "if r1 == 0 goto +5",
        "if r1 == 1 goto +2",
        "r1 = 1",
        "goto -3",
        "r1 = 0",
        "goto -6",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

#[link_section = ".struct_ops.link"]
#[no_mangle]
pub static mut epilogue_goto_start: bpf_testmod_st_ops = bpf_testmod_st_ops {
    test_prologue: test_prologue_goto_start as *mut core::ffi::c_void,
    test_epilogue: test_epilogue_goto_start as *mut core::ffi::c_void,
    test_pro_epilogue: test_pro_epilogue_goto_start as *mut core::ffi::c_void,
};

#[link_section = "syscall"]
#[no_mangle]
// __retval(0)
pub unsafe extern "C" fn syscall_prologue_goto_start(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let mut args: st_ops_args = core::mem::zeroed();

    bpf_kfunc_st_ops_test_prologue(&mut args)
}

#[link_section = "syscall"]
#[no_mangle]
// __retval(20000) /* (EPILOGUE_A [10000]) * 2 */
pub unsafe extern "C" fn syscall_epilogue_goto_start(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let mut args: st_ops_args = core::mem::zeroed();

    bpf_kfunc_st_ops_test_epilogue(&mut args)
}

#[link_section = "syscall"]
#[no_mangle]
// __retval(22000) /* (PROLOGUE_A [1000] + EPILOGUE_A [10000]) * 2 */
pub unsafe extern "C" fn syscall_pro_epilogue_goto_start(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let mut args: st_ops_args = core::mem::zeroed();

    bpf_kfunc_st_ops_test_pro_epilogue(&mut args)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
