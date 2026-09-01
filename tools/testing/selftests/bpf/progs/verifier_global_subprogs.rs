// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/*
 * C dependencies removed from executable Rust:
 * <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
 * "bpf_misc.h", "xdp_metadata.h", "bpf_kfuncs.h", "err.h".
 *
 * Verifier/test annotations such as SEC(), __success, __failure, __log_level,
 * __msg, __weak, __noinline, __arg_ctx, __arg_nonnull, and __auxiliary are
 * preserved as comments at their original declaration sites.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

type bool_ = bool;
type u64 = u64;
type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct bpf_raw_tracepoint_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_regs_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_pt_regs {
    _private: [u8; 0],
}

type bpf_user_pt_regs_t = pt_regs;

#[repr(C)]
pub struct bpf_perf_event_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter__task {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dummy_ops {
    pub test_1: *mut c_void,
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xdp_md {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_stack(ctx: *mut c_void, buf: *mut c_void, size: usize, flags: u64) -> i32;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_tail_call(ctx: *mut c_void, map: *mut c_void, index: u32);
    fn bpf_dynptr_data(dptr: *mut bpf_dynptr, offset: u32, len: u32) -> *mut c_void;
    fn bpf_dynptr_slice(
        dptr: *mut bpf_dynptr,
        offset: u32,
        buffer: *mut c_void,
        len: u32,
    ) -> *mut c_void;
    fn bpf_dynptr_from_xdp(ctx: *mut xdp_md, flags: u64, dptr: *mut bpf_dynptr) -> i32;
    fn set_if_not_errno_or_zero(ret: i32, val: i32);
}

pub static mut arr: [i32; 1] = [0; 1];
pub static mut unkn_idx: i32 = 0;
pub static call_dead_subprog: bool_ = false;

// __noinline
#[inline(never)]
pub unsafe extern "C" fn global_bad() -> i64 {
    unsafe { arr[unkn_idx as usize] as i64 } /* BOOM */
}

// __noinline
#[inline(never)]
pub unsafe extern "C" fn global_good() -> i64 {
    unsafe { arr[0] as i64 }
}

// __noinline
#[inline(never)]
pub unsafe extern "C" fn global_calls_bad() -> i64 {
    unsafe { global_good() + global_bad() } /* does BOOM indirectly */
}

// __noinline
#[inline(never)]
pub unsafe extern "C" fn global_calls_good_only() -> i64 {
    unsafe { global_good() }
}

// __noinline
#[inline(never)]
pub unsafe extern "C" fn global_dead() -> i64 {
    unsafe { (arr[0] * 2) as i64 }
}

// SEC("?raw_tp")
// __success __log_level(6)
// main prog is validated completely first
// __msg("('global_calls_good_only') is global and assumed valid.")
// eventually global_good() is transitively validated as well
// __msg("Validating global_good() func")
// __msg("('global_good') is safe for any args that match its prototype")
// __msg("subprog 0 (chained_global_func_calls_success) main insns_self 7 insns_total 7 stack")
// __msg("subprog {{[0-9]+}} (global_calls_good_only) global insns_self 2 insns_total 2 stack")
// If defined(__BPF_CPU_VERSION__) && __BPF_CPU_VERSION__ >= 4:
// __msg("subprog {{[0-9]+}} (global_good) global insns_self 3 insns_total 3 stack")
// __msg("processed 12 insns")
// Else:
// __msg("subprog {{[0-9]+}} (global_good) global insns_self 5 insns_total 5 stack")
// __msg("processed 14 insns")
pub unsafe extern "C" fn chained_global_func_calls_success() -> i32 {
    let mut sum: i32 = 0;

    if call_dead_subprog {
        sum += unsafe { global_dead() as i32 };
    }
    unsafe { (global_calls_good_only() as i32).wrapping_add(sum) }
}

// SEC("?raw_tp")
// __failure __log_level(2)
// main prog validated successfully first
// __msg("('global_calls_bad') is global and assumed valid.")
// eventually we validate global_bad() and fail
// __msg("Validating global_bad() func")
// __msg("math between map_value pointer and register") /* BOOM */
pub unsafe extern "C" fn chained_global_func_calls_bad() -> i32 {
    unsafe { global_calls_bad() as i32 }
}

/* do out of bounds access forcing verifier to fail verification if this
 * global func is called
 */
// __noinline
#[inline(never)]
pub unsafe extern "C" fn global_unsupp(mem: *const i32) -> i32 {
    if mem.is_null() {
        return 0;
    }
    unsafe { *mem.add(100) } /* BOOM */
}

pub static skip_unsupp_global: bool_ = true;

// SEC("?raw_tp")
// __success
pub unsafe extern "C" fn guarded_unsupp_global_called() -> i32 {
    if !skip_unsupp_global {
        return unsafe { global_unsupp(core::ptr::null()) };
    }
    0
}

// SEC("?raw_tp")
// __failure __log_level(2)
// __msg("Func#1 ('global_unsupp') is global and assumed valid.")
// __msg("Validating global_unsupp() func#1...")
// __msg("value is outside of the allowed memory range")
pub unsafe extern "C" fn unguarded_unsupp_global_called() -> i32 {
    let x: i32 = 0;

    unsafe { global_unsupp(&x) }
}

pub static mut stack: [i64; 128] = [0; 128];

// __weak
pub unsafe extern "C" fn subprog_nullable_ptr_bad(p: *mut i32) -> i32 {
    unsafe { (*p) * 2 } /* bad, missing null check */
}

// SEC("?raw_tp")
// __failure __log_level(2)
// __msg("invalid mem access 'mem_or_null'")
pub unsafe extern "C" fn arg_tag_nullable_ptr_fail(_ctx: *mut c_void) -> i32 {
    let mut x: i32 = 42;

    unsafe { subprog_nullable_ptr_bad(&mut x) }
}

#[repr(C)]
pub struct user_struct_t {
    pub x: i32,
}

// __noinline __weak
#[inline(never)]
pub unsafe extern "C" fn subprog_user_anon_mem(t: *mut user_struct_t) -> i32 {
    if !t.is_null() {
        unsafe { (*t).x }
    } else {
        0
    }
}

// SEC("?tracepoint")
// __failure __log_level(2)
// __msg("Caller passes invalid args into func#1 ('subprog_user_anon_mem')")
pub unsafe extern "C" fn anon_user_mem_invalid(ctx: *mut c_void) -> i32 {
    /* can't pass PTR_TO_CTX as user memory */
    unsafe { subprog_user_anon_mem(ctx as *mut user_struct_t) }
}

// SEC("?tracepoint")
// __success __log_level(2)
// __msg("Func#1 ('subprog_user_anon_mem') is safe for any args that match its prototype")
pub unsafe extern "C" fn anon_user_mem_valid(_ctx: *mut c_void) -> i32 {
    let mut t = user_struct_t { x: 42 };

    unsafe { subprog_user_anon_mem(&mut t) }
}

// __noinline __weak
#[inline(never)]
pub unsafe extern "C" fn subprog_user_anon_mem_huge(p: *mut [i32; 0x3fffffff]) -> i32 {
    if !p.is_null() {
        unsafe { (*p)[1] }
    } else {
        0
    }
}

// SEC("?tracepoint")
// __failure __log_level(2)
// __msg("R1 memory size 4294967292 is too large")
pub unsafe extern "C" fn anon_user_mem_huge_size_invalid(_ctx: *mut c_void) -> i32 {
    let mut p: *mut [i32; 0x3fffffff];
    let tiny: i32 = 42;

    p = (&tiny as *const i32 as *mut c_void) as *mut [i32; 0x3fffffff];
    unsafe { subprog_user_anon_mem_huge(p) + tiny }
}

// __noinline __weak; p1 and p2 are __arg_nonnull
#[inline(never)]
pub unsafe extern "C" fn subprog_nonnull_ptr_good(p1: *mut i32, p2: *mut i32) -> i32 {
    unsafe { (*p1) * (*p2) } /* good, no need for NULL checks */
}

pub static mut x: i32 = 47;

// SEC("?raw_tp")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_nonnull_ptr_good(_ctx: *mut c_void) -> i32 {
    let mut y: i32 = 74;

    unsafe { subprog_nonnull_ptr_good(&raw mut x, &mut y) }
}

// SEC("?raw_tp")
// __failure __log_level(2)
// __msg("R1 is expected to be non-NULL")
pub unsafe extern "C" fn arg_tag_nonnull_ptr_null_bad(_ctx: *mut c_void) -> i32 {
    let mut y: i32 = 74;

    unsafe { subprog_nonnull_ptr_good(core::ptr::null_mut(), &mut y) }
}

/* this global subprog can be now called from many types of entry progs, each
 * with different context type
 */
// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn subprog_ctx_tag(ctx: *mut c_void) -> i32 {
    unsafe {
        bpf_get_stack(
            ctx,
            core::ptr::addr_of_mut!(stack) as *mut c_void,
            core::mem::size_of_val(&raw const stack),
            0,
        )
    }
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn raw_tp_canonical(_ctx: *mut bpf_raw_tracepoint_args) -> i32 {
    0
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn raw_tp_u64_array(_ctx: *mut u64) -> i32 {
    0
}

// SEC("?raw_tp")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_ctx_raw_tp(ctx: *mut c_void) -> i32 {
    unsafe {
        subprog_ctx_tag(ctx)
            + raw_tp_canonical(ctx as *mut bpf_raw_tracepoint_args)
            + raw_tp_u64_array(ctx as *mut u64)
    }
}

// SEC("?raw_tp.w")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_ctx_raw_tp_writable(ctx: *mut c_void) -> i32 {
    unsafe {
        subprog_ctx_tag(ctx)
            + raw_tp_canonical(ctx as *mut bpf_raw_tracepoint_args)
            + raw_tp_u64_array(ctx as *mut u64)
    }
}

// SEC("?tp_btf/sys_enter")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_ctx_raw_tp_btf(ctx: *mut c_void) -> i32 {
    unsafe {
        subprog_ctx_tag(ctx)
            + raw_tp_canonical(ctx as *mut bpf_raw_tracepoint_args)
            + raw_tp_u64_array(ctx as *mut u64)
    }
}

#[repr(C)]
pub struct whatever {
    _private: [u8; 0],
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn tp_whatever(_ctx: *mut whatever) -> i32 {
    0
}

// SEC("?tp")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_ctx_tp(ctx: *mut c_void) -> i32 {
    unsafe { subprog_ctx_tag(ctx) + tp_whatever(ctx as *mut whatever) }
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn kprobe_subprog_pt_regs(_ctx: *mut pt_regs) -> i32 {
    0
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn kprobe_subprog_typedef(_ctx: *mut bpf_user_pt_regs_t) -> i32 {
    0
}

// SEC("?kprobe")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_ctx_kprobe(ctx: *mut c_void) -> i32 {
    unsafe {
        subprog_ctx_tag(ctx)
            + kprobe_subprog_pt_regs(ctx as *mut pt_regs)
            + kprobe_subprog_typedef(ctx as *mut bpf_user_pt_regs_t)
    }
}

/*
 * __weak int perf_subprog_regs(... __arg_ctx)
 * C selects the ctx type with bpf_target_* preprocessor symbols:
 * riscv: struct user_regs_struct *
 * s390: void *
 * loongarch/arm64/powerpc: struct user_pt_regs *
 * otherwise: struct pt_regs *
 */
pub unsafe extern "C" fn perf_subprog_regs(_ctx: *mut pt_regs) -> i32 {
    0
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn perf_subprog_typedef(_ctx: *mut bpf_user_pt_regs_t) -> i32 {
    0
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn perf_subprog_canonical(_ctx: *mut bpf_perf_event_data) -> i32 {
    0
}

// SEC("?perf_event")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_ctx_perf(ctx: *mut c_void) -> i32 {
    unsafe {
        subprog_ctx_tag(ctx)
            + perf_subprog_regs(ctx as *mut pt_regs)
            + perf_subprog_typedef(ctx as *mut bpf_user_pt_regs_t)
            + perf_subprog_canonical(ctx as *mut bpf_perf_event_data)
    }
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn iter_subprog_void(_ctx: *mut c_void) -> i32 {
    0
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn iter_subprog_typed(_ctx: *mut bpf_iter__task) -> i32 {
    0
}

// SEC("?iter/task")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_ctx_iter_task(ctx: *mut bpf_iter__task) -> i32 {
    unsafe { (iter_subprog_void(ctx as *mut c_void) + iter_subprog_typed(ctx)) & 1 }
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn tracing_subprog_void(_ctx: *mut c_void) -> i32 {
    0
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn tracing_subprog_u64(_ctx: *mut u64) -> i32 {
    0
}

pub static mut acc: i32 = 0;

// SEC("?fentry/" SYS_PREFIX "sys_nanosleep")
// __success __log_level(2)
// BPF_PROG(arg_tag_ctx_fentry)
pub unsafe extern "C" fn arg_tag_ctx_fentry(ctx: *mut c_void) -> i32 {
    unsafe {
        acc += tracing_subprog_void(ctx) + tracing_subprog_u64(ctx as *mut u64);
    }
    0
}

// SEC("?fexit/" SYS_PREFIX "sys_nanosleep")
// __success __log_level(2)
// BPF_PROG(arg_tag_ctx_fexit)
pub unsafe extern "C" fn arg_tag_ctx_fexit(ctx: *mut c_void) -> i32 {
    unsafe {
        acc += tracing_subprog_void(ctx) + tracing_subprog_u64(ctx as *mut u64);
    }
    0
}

// SEC("?fmod_ret/" SYS_PREFIX "sys_nanosleep")
// __success __log_level(2)
// BPF_PROG(arg_tag_ctx_fmod_ret)
pub unsafe extern "C" fn arg_tag_ctx_fmod_ret(ctx: *mut c_void) -> i32 {
    unsafe { tracing_subprog_void(ctx) + tracing_subprog_u64(ctx as *mut u64) }
}

// SEC("?lsm/bpf")
// __success __log_level(2)
// BPF_PROG(arg_tag_ctx_lsm)
pub unsafe extern "C" fn arg_tag_ctx_lsm(ctx: *mut c_void) -> i32 {
    let mut ret: i32;

    ret = unsafe { tracing_subprog_void(ctx) + tracing_subprog_u64(ctx as *mut u64) };
    unsafe {
        set_if_not_errno_or_zero(ret, -1);
    }
    ret
}

// SEC("?struct_ops/test_1")
// __success __log_level(2)
// BPF_PROG(arg_tag_ctx_struct_ops)
pub unsafe extern "C" fn arg_tag_ctx_struct_ops(ctx: *mut c_void) -> i32 {
    unsafe { tracing_subprog_void(ctx) + tracing_subprog_u64(ctx as *mut u64) }
}

// SEC(".struct_ops")
pub static mut dummy_1: bpf_dummy_ops = bpf_dummy_ops {
    test_1: arg_tag_ctx_struct_ops as *mut c_void,
};

// SEC("?syscall")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_ctx_syscall(ctx: *mut c_void) -> i32 {
    unsafe {
        tracing_subprog_void(ctx) + tracing_subprog_u64(ctx as *mut u64) + tp_whatever(ctx as *mut whatever)
    }
}

// __weak; ctx is __arg_ctx
pub unsafe extern "C" fn syscall_array_bpf_for(ctx: *mut c_void) -> i32 {
    let arr = ctx as *mut i32;
    let mut i: i32;

    i = 0;
    while i < 100 {
        unsafe {
            *arr.add(i as usize) *= i;
        }
        i += 1;
    }

    0
}

// SEC("?syscall")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_ctx_syscall_bpf_for(ctx: *mut c_void) -> i32 {
    unsafe { syscall_array_bpf_for(ctx) }
}

// SEC("syscall")
// __auxiliary
pub unsafe extern "C" fn syscall_tailcall_target(ctx: *mut c_void) -> i32 {
    unsafe { syscall_array_bpf_for(ctx) }
}

/*
 * SEC(".maps")
 * struct {
 *     __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
 *     __uint(max_entries, 1);
 *     __uint(key_size, sizeof(__u32));
 *     __array(values, int (void *));
 * } syscall_prog_array = {
 *     .values = {
 *         [0] = (void *)&syscall_tailcall_target,
 *     },
 * };
 */
#[repr(C)]
pub struct syscall_prog_array_t {
    pub values: [*mut c_void; 1],
}

pub static mut syscall_prog_array: syscall_prog_array_t = syscall_prog_array_t {
    values: [syscall_tailcall_target as *mut c_void],
};

// SEC("?syscall")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_ctx_syscall_tailcall(ctx: *mut c_void) -> i32 {
    unsafe {
        bpf_tail_call(ctx, (&raw mut syscall_prog_array) as *mut c_void, 0);
    }
    0
}

// SEC("?syscall")
// __failure __log_level(2)
// __msg("dereference of modified ctx ptr R1 off=8 disallowed")
pub unsafe extern "C" fn arg_tag_ctx_syscall_tailcall_fixed_off_bad(ctx: *mut c_void) -> i32 {
    let mut p = ctx as *mut i8;

    p = unsafe { p.add(8) };
    unsafe {
        bpf_tail_call(p as *mut c_void, (&raw mut syscall_prog_array) as *mut c_void, 0);
    }
    0
}

// SEC("?syscall")
// __failure __log_level(2)
// __msg("variable ctx access var_off=(0x0; 0x4) disallowed")
pub unsafe extern "C" fn arg_tag_ctx_syscall_tailcall_var_off_bad(ctx: *mut c_void) -> i32 {
    let mut off: __u64 = unsafe { bpf_get_prandom_u32() as __u64 };
    let mut p = ctx as *mut i8;

    off &= 4;
    p = unsafe { p.add(off as usize) };
    unsafe {
        bpf_tail_call(p as *mut c_void, (&raw mut syscall_prog_array) as *mut c_void, 0);
    }
    0
}

// SEC("?syscall")
// __failure __log_level(2)
// __msg("dereference of modified ctx ptr R1 off=8 disallowed")
pub unsafe extern "C" fn arg_tag_ctx_syscall_fixed_off_bad(ctx: *mut c_void) -> i32 {
    let mut p = ctx as *mut i8;

    p = unsafe { p.add(8) };
    unsafe { subprog_ctx_tag(p as *mut c_void) }
}

// SEC("?syscall")
// __failure __log_level(2)
// __msg("variable ctx access var_off=(0x0; 0x4) disallowed")
pub unsafe extern "C" fn arg_tag_ctx_syscall_var_off_bad(ctx: *mut c_void) -> i32 {
    let mut off: __u64 = unsafe { bpf_get_prandom_u32() as __u64 };
    let mut p = ctx as *mut i8;

    off &= 4;
    p = unsafe { p.add(off as usize) };
    unsafe { subprog_ctx_tag(p as *mut c_void) }
}

// __weak
pub unsafe extern "C" fn subprog_dynptr(dptr: *mut bpf_dynptr) -> i64 {
    let mut d: *mut i64;
    let mut t: i64;
    let mut buf: [i64; 1] = [0; 1];

    d = unsafe { bpf_dynptr_data(dptr, 0, core::mem::size_of::<i64>() as u32) as *mut i64 };
    if d.is_null() {
        return 0;
    }

    t = unsafe { *d + 1 };

    d = unsafe {
        bpf_dynptr_slice(
            dptr,
            0,
            (&mut buf as *mut [i64; 1]) as *mut c_void,
            core::mem::size_of::<i64>() as u32,
        ) as *mut i64
    };
    if d.is_null() {
        return t;
    }

    t = unsafe { *d + 2 };

    t
}

// SEC("?xdp")
// __success __log_level(2)
pub unsafe extern "C" fn arg_tag_dynptr(ctx: *mut xdp_md) -> i32 {
    let mut dptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    unsafe {
        bpf_dynptr_from_xdp(ctx, 0, dptr.as_mut_ptr());
        subprog_dynptr(dptr.as_mut_ptr()) as i32
    }
}

// __weak
pub unsafe extern "C" fn foo() {}

// SEC("?tc")
// __failure __msg("R0 !read_ok")
pub unsafe extern "C" fn return_from_void_global(_skb: *mut __sk_buff) -> i32 {
    unsafe {
        foo();
        core::arch::asm!("r1 = r0;", options(nostack, preserves_flags));
    }

    0
}

// SEC("license")
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
