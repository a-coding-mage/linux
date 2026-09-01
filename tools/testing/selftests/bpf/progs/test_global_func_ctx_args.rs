// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */
/* Dependencies in the original C source:
 * "vmlinux.h"
 * <bpf/bpf_helpers.h>
 * <bpf/bpf_tracing.h>
 * <bpf/bpf_core_read.h>
 * "bpf_misc.h"
 */

use core::ffi::{c_int, c_long, c_void};

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

static mut stack: [c_long; 256] = [0; 256];

#[repr(C)]
pub struct bpf_user_pt_regs_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct pt_regs_struct_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_raw_tracepoint_args {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_perf_event_data {
    _unused: [u8; 0],
}

extern "C" {
    fn bpf_get_stack(ctx: *mut c_void, buf: *mut c_void, size: usize, flags: u64) -> c_int;
}

/*
 * KPROBE contexts
 */

/* __weak */
#[no_mangle]
pub unsafe extern "C" fn kprobe_typedef_ctx_subprog(ctx: *mut bpf_user_pt_regs_t) -> c_int {
    bpf_get_stack(
        ctx as *mut c_void,
        stack.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&stack),
        0,
    )
}

/* SEC("?kprobe") */
/* __success */
#[no_mangle]
pub unsafe extern "C" fn kprobe_typedef_ctx(ctx: *mut c_void) -> c_int {
    kprobe_typedef_ctx_subprog(ctx as *mut bpf_user_pt_regs_t)
}

/* s390x defines:
 *
 * typedef user_pt_regs bpf_user_pt_regs_t;
 * typedef struct { ... } user_pt_regs;
 *
 * And so "canonical" underlying struct type is anonymous.
 * So on s390x only valid ways to have PTR_TO_CTX argument in global subprogs
 * are:
 *   - bpf_user_pt_regs_t *ctx (typedef);
 *   - struct bpf_user_pt_regs_t *ctx (backwards compatible struct hack);
 *   - void *ctx __arg_ctx (arg:ctx tag)
 *
 * Other architectures also allow using underlying struct types (e.g.,
 * `struct pt_regs *ctx` for x86-64)
 */
/* Original C condition: #ifndef bpf_target_s390 */
/* Original C macro:
 * #define pt_regs_struct_t typeof(*(__PT_REGS_CAST((struct pt_regs *)NULL)))
 */

/* __weak */
#[no_mangle]
pub unsafe extern "C" fn kprobe_struct_ctx_subprog(ctx: *mut pt_regs_struct_t) -> c_int {
    bpf_get_stack(
        ctx as *mut c_void,
        stack.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&stack),
        0,
    )
}

/* SEC("?kprobe") */
/* __success */
#[no_mangle]
pub unsafe extern "C" fn kprobe_resolved_ctx(ctx: *mut c_void) -> c_int {
    kprobe_struct_ctx_subprog(ctx as *mut pt_regs_struct_t)
}

/* End original C condition: #ifndef bpf_target_s390 */

/* this is current hack to make this work on old kernels */
#[repr(C)]
pub struct bpf_user_pt_regs_t_workaround {
    _unused: [u8; 0],
}

/* __weak */
#[no_mangle]
pub unsafe extern "C" fn kprobe_workaround_ctx_subprog(
    ctx: *mut bpf_user_pt_regs_t_workaround,
) -> c_int {
    bpf_get_stack(
        ctx as *mut c_void,
        stack.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&stack),
        0,
    )
}

/* SEC("?kprobe") */
/* __success */
#[no_mangle]
pub unsafe extern "C" fn kprobe_workaround_ctx(ctx: *mut c_void) -> c_int {
    kprobe_workaround_ctx_subprog(ctx as *mut bpf_user_pt_regs_t_workaround)
}

/*
 * RAW_TRACEPOINT contexts
 */

/* __weak */
#[no_mangle]
pub unsafe extern "C" fn raw_tp_ctx_subprog(ctx: *mut bpf_raw_tracepoint_args) -> c_int {
    bpf_get_stack(
        ctx as *mut c_void,
        stack.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&stack),
        0,
    )
}

/* SEC("?raw_tp") */
/* __success */
#[no_mangle]
pub unsafe extern "C" fn raw_tp_ctx(ctx: *mut c_void) -> c_int {
    raw_tp_ctx_subprog(ctx as *mut bpf_raw_tracepoint_args)
}

/*
 * RAW_TRACEPOINT_WRITABLE contexts
 */

/* __weak */
#[no_mangle]
pub unsafe extern "C" fn raw_tp_writable_ctx_subprog(
    ctx: *mut bpf_raw_tracepoint_args,
) -> c_int {
    bpf_get_stack(
        ctx as *mut c_void,
        stack.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&stack),
        0,
    )
}

/* SEC("?raw_tp") */
/* __success */
#[no_mangle]
pub unsafe extern "C" fn raw_tp_writable_ctx(ctx: *mut c_void) -> c_int {
    raw_tp_writable_ctx_subprog(ctx as *mut bpf_raw_tracepoint_args)
}

/*
 * PERF_EVENT contexts
 */

/* __weak */
#[no_mangle]
pub unsafe extern "C" fn perf_event_ctx_subprog(ctx: *mut bpf_perf_event_data) -> c_int {
    bpf_get_stack(
        ctx as *mut c_void,
        stack.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&stack),
        0,
    )
}

/* SEC("?perf_event") */
/* __success */
#[no_mangle]
pub unsafe extern "C" fn perf_event_ctx(ctx: *mut c_void) -> c_int {
    perf_event_ctx_subprog(ctx as *mut bpf_perf_event_data)
}

/* this global subprog can be now called from many types of entry progs, each
 * with different context type
 */
/* __weak; ctx has original C annotation: __arg_ctx */
#[no_mangle]
pub unsafe extern "C" fn subprog_ctx_tag(ctx: *mut c_void) -> c_int {
    bpf_get_stack(
        ctx,
        stack.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&stack),
        0,
    )
}

#[repr(C)]
pub struct my_struct {
    pub x: c_int,
}

/* __weak; ctx1 and ctx2 have original C annotation: __arg_ctx */
#[no_mangle]
pub unsafe extern "C" fn subprog_multi_ctx_tags(
    ctx1: *mut c_void,
    mem: *mut my_struct,
    ctx2: *mut c_void,
) -> c_int {
    if mem.is_null() {
        return 0;
    }

    bpf_get_stack(
        ctx1,
        stack.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&stack),
        0,
    ) + (*mem).x
        + bpf_get_stack(
            ctx2,
            stack.as_mut_ptr() as *mut c_void,
            core::mem::size_of_val(&stack),
            0,
        )
}

/* SEC("?raw_tp") */
/* __success __log_level(2) */
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_raw_tp(ctx: *mut c_void) -> c_int {
    let mut x = my_struct { x: 123 };

    subprog_ctx_tag(ctx) + subprog_multi_ctx_tags(ctx, &mut x, ctx)
}

/* SEC("?perf_event") */
/* __success __log_level(2) */
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_perf(ctx: *mut c_void) -> c_int {
    let mut x = my_struct { x: 123 };

    subprog_ctx_tag(ctx) + subprog_multi_ctx_tags(ctx, &mut x, ctx)
}

/* SEC("?kprobe") */
/* __success __log_level(2) */
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_kprobe(ctx: *mut c_void) -> c_int {
    let mut x = my_struct { x: 123 };

    subprog_ctx_tag(ctx) + subprog_multi_ctx_tags(ctx, &mut x, ctx)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
