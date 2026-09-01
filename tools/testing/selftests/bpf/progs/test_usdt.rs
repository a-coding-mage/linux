// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// Translated from C includes:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/usdt.bpf.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type u64 = ::core::ffi::c_ulonglong;
pub type __u64 = ::core::ffi::c_ulonglong;
pub type uintptr_t = usize;

#[repr(C)]
pub struct pt_regs {
    pub ip: ::core::ffi::c_ulong,
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_get_prandom_u32() -> ::core::ffi::c_uint;
    fn bpf_usdt_cookie(ctx: *mut pt_regs) -> u64;
    fn bpf_usdt_arg_cnt(ctx: *mut pt_regs) -> ::core::ffi::c_int;
    fn bpf_usdt_arg(
        ctx: *mut pt_regs,
        arg_num: ::core::ffi::c_uint,
        res: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
    fn bpf_usdt_arg_size(ctx: *mut pt_regs, arg_num: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

pub static mut my_pid: ::core::ffi::c_int = 0;

pub static mut usdt0_called: ::core::ffi::c_int = 0;
pub static mut usdt0_cookie: u64 = 0;
pub static mut usdt0_arg_cnt: ::core::ffi::c_int = 0;
pub static mut usdt0_arg_ret: ::core::ffi::c_int = 0;
pub static mut usdt0_arg_size: ::core::ffi::c_int = 0;

// SEC("usdt")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt0(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    let mut tmp: ::core::ffi::c_long = 0;

    if my_pid != (bpf_get_current_pid_tgid() >> 32) as ::core::ffi::c_int {
        return 0;
    }

    core::intrinsics::atomic_xadd_seqcst(&raw mut usdt0_called, 1);

    usdt0_cookie = bpf_usdt_cookie(ctx);
    usdt0_arg_cnt = bpf_usdt_arg_cnt(ctx);
    /* should return -ENOENT for any arg_num */
    usdt0_arg_ret = bpf_usdt_arg(ctx, bpf_get_prandom_u32(), &mut tmp);
    usdt0_arg_size = bpf_usdt_arg_size(ctx, bpf_get_prandom_u32());
    return 0;
}

pub static mut usdt3_called: ::core::ffi::c_int = 0;
pub static mut usdt3_cookie: u64 = 0;
pub static mut usdt3_arg_cnt: ::core::ffi::c_int = 0;
pub static mut usdt3_arg_rets: [::core::ffi::c_int; 3] = [0; 3];
pub static mut usdt3_args: [u64; 3] = [0; 3];
pub static mut usdt3_arg_sizes: [::core::ffi::c_int; 3] = [0; 3];

// SEC("usdt//proc/self/exe:test:usdt3")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt3(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    let mut tmp: ::core::ffi::c_long = 0;

    if my_pid != (bpf_get_current_pid_tgid() >> 32) as ::core::ffi::c_int {
        return 0;
    }

    core::intrinsics::atomic_xadd_seqcst(&raw mut usdt3_called, 1);

    usdt3_cookie = bpf_usdt_cookie(ctx);
    usdt3_arg_cnt = bpf_usdt_arg_cnt(ctx);

    usdt3_arg_rets[0] = bpf_usdt_arg(ctx, 0, &mut tmp);
    usdt3_args[0] = tmp as ::core::ffi::c_int as u64;
    usdt3_arg_sizes[0] = bpf_usdt_arg_size(ctx, 0);

    usdt3_arg_rets[1] = bpf_usdt_arg(ctx, 1, &mut tmp);
    usdt3_args[1] = tmp as ::core::ffi::c_long as u64;
    usdt3_arg_sizes[1] = bpf_usdt_arg_size(ctx, 1);

    usdt3_arg_rets[2] = bpf_usdt_arg(ctx, 2, &mut tmp);
    usdt3_args[2] = tmp as uintptr_t as u64;
    usdt3_arg_sizes[2] = bpf_usdt_arg_size(ctx, 2);

    return 0;
}

pub static mut usdt12_called: ::core::ffi::c_int = 0;
pub static mut usdt12_cookie: u64 = 0;
pub static mut usdt12_arg_cnt: ::core::ffi::c_int = 0;
pub static mut usdt12_args: [u64; 12] = [0; 12];
pub static mut usdt12_arg_sizes: [::core::ffi::c_int; 12] = [0; 12];

// SEC("usdt//proc/self/exe:test:usdt12")
// Original C used BPF_USDT(usdt12, int a1, int a2, long a3, long a4, unsigned a5,
//                         long a6, __u64 a7, uintptr_t a8, int a9, short a10,
//                         short a11, signed char a12), which supplies ctx.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt12(
    ctx: *mut pt_regs,
    a1: ::core::ffi::c_int,
    a2: ::core::ffi::c_int,
    a3: ::core::ffi::c_long,
    a4: ::core::ffi::c_long,
    a5: ::core::ffi::c_uint,
    a6: ::core::ffi::c_long,
    a7: __u64,
    a8: uintptr_t,
    a9: ::core::ffi::c_int,
    a10: ::core::ffi::c_short,
    a11: ::core::ffi::c_short,
    a12: ::core::ffi::c_schar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;

    if my_pid != (bpf_get_current_pid_tgid() >> 32) as ::core::ffi::c_int {
        return 0;
    }

    core::intrinsics::atomic_xadd_seqcst(&raw mut usdt12_called, 1);

    usdt12_cookie = bpf_usdt_cookie(ctx);
    usdt12_arg_cnt = bpf_usdt_arg_cnt(ctx);

    usdt12_args[0] = a1 as u64;
    usdt12_args[1] = a2 as u64;
    usdt12_args[2] = a3 as u64;
    usdt12_args[3] = a4 as u64;
    usdt12_args[4] = a5 as u64;
    usdt12_args[5] = a6 as u64;
    usdt12_args[6] = a7;
    usdt12_args[7] = a8 as u64;
    usdt12_args[8] = a9 as u64;
    usdt12_args[9] = a10 as u64;
    usdt12_args[10] = a11 as u64;
    usdt12_args[11] = a12 as u64;

    i = 0;
    while i < 12 {
        usdt12_arg_sizes[i as usize] = bpf_usdt_arg_size(ctx, i as ::core::ffi::c_uint);
        i += 1;
    }

    return 0;
}

pub static mut usdt_sib_called: ::core::ffi::c_int = 0;
pub static mut usdt_sib_cookie: u64 = 0;
pub static mut usdt_sib_arg_cnt: ::core::ffi::c_int = 0;
pub static mut usdt_sib_arg_ret: ::core::ffi::c_int = 0;
pub static mut usdt_sib_arg: ::core::ffi::c_short = 0;
pub static mut usdt_sib_arg_size: ::core::ffi::c_int = 0;

/*
 * usdt_sib is only tested on x86-related architectures, so it requires
 * manual attach since auto-attach will panic tests under other architectures
 */
// SEC("usdt")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt_sib(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    let mut tmp: ::core::ffi::c_long = 0;

    if my_pid != (bpf_get_current_pid_tgid() >> 32) as ::core::ffi::c_int {
        return 0;
    }

    core::intrinsics::atomic_xadd_seqcst(&raw mut usdt_sib_called, 1);

    usdt_sib_cookie = bpf_usdt_cookie(ctx);
    usdt_sib_arg_cnt = bpf_usdt_arg_cnt(ctx);

    usdt_sib_arg_ret = bpf_usdt_arg(ctx, 0, &mut tmp);
    usdt_sib_arg = tmp as ::core::ffi::c_short;
    usdt_sib_arg_size = bpf_usdt_arg_size(ctx, 0);

    return 0;
}

// Original C condition: #ifdef __TARGET_ARCH_x86
#[cfg(target_arch = "x86_64")]
pub static mut executed: ::core::ffi::c_int = 0;
#[cfg(target_arch = "x86_64")]
pub static mut expected_ip: ::core::ffi::c_ulong = 0;

// SEC("usdt")
#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt_executed(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    if expected_ip == (*ctx).ip {
        executed += 1;
    }
    return 0;
}

#[cfg(target_arch = "x86_64")]
pub static mut arg_total: ::core::ffi::c_int = 0;
#[cfg(target_arch = "x86_64")]
pub static mut arg_bad: ::core::ffi::c_int = 0;
#[cfg(target_arch = "x86_64")]
pub static mut arg_last: [::core::ffi::c_long; 3] = [0; 3];
#[cfg(target_arch = "x86_64")]
pub static mut expected_arg: [::core::ffi::c_long; 3] = [0; 3];
#[cfg(target_arch = "x86_64")]
pub static mut expected_pid: ::core::ffi::c_int = 0;

// SEC("usdt")
// Original C used BPF_USDT(usdt_check_arg, long arg1, long arg2, long arg3),
// which supplies ctx.
#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt_check_arg(
    ctx: *mut pt_regs,
    arg1: ::core::ffi::c_long,
    arg2: ::core::ffi::c_long,
    arg3: ::core::ffi::c_long,
) -> ::core::ffi::c_int {
    if expected_pid != (bpf_get_current_pid_tgid() >> 32) as ::core::ffi::c_int {
        return 0;
    }

    core::intrinsics::atomic_xadd_seqcst(&raw mut arg_total, 1);
    arg_last[0] = arg1;
    arg_last[1] = arg2;
    arg_last[2] = arg3;

    if arg1 != expected_arg[0] || arg2 != expected_arg[1] || arg3 != expected_arg[2] {
        core::intrinsics::atomic_xadd_seqcst(&raw mut arg_bad, 1);
    }

    return 0;
}

// char _license[] SEC("license") = "GPL";
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
