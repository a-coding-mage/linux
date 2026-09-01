// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C trace header intent:
// #undef TRACE_SYSTEM
// #define TRACE_SYSTEM bpf_testmod
//
// Header guard:
// #if !defined(_BPF_TESTMOD_EVENTS_H) || defined(TRACE_HEADER_MULTI_READ)
// #define _BPF_TESTMOD_EVENTS_H
//
// Dependencies supplied by other files in the original repository:
// #include <linux/tracepoint.h>
// #include "bpf_testmod.h"

use core::ffi::{c_char, c_int, c_void};

pub type pid_t = c_int;
pub type loff_t = i64;
pub type size_t = usize;
pub type u64 = u64;

pub const TRACE_SYSTEM: &str = "bpf_testmod";

unsafe extern "C" {
    pub type task_struct;
    pub type bpf_testmod_test_read_ctx;
    pub type bpf_testmod_test_write_ctx;
    pub type bpf_testmod_test_writable_ctx;
    pub type sk_buff;

    // TRACE_EVENT(bpf_testmod_test_read,
    //     TP_PROTO(struct task_struct *task, struct bpf_testmod_test_read_ctx *ctx),
    //     TP_ARGS(task, ctx),
    //     TP_STRUCT__entry(
    //         __field(pid_t, pid)
    //         __array(char, comm, TASK_COMM_LEN)
    //         __field(loff_t, off)
    //         __field(size_t, len)
    //     ),
    //     TP_fast_assign(
    //         __entry->pid = task->pid;
    //         memcpy(__entry->comm, task->comm, TASK_COMM_LEN);
    //         __entry->off = ctx->off;
    //         __entry->len = ctx->len;
    //     ),
    //     TP_printk("pid=%d comm=%s off=%llu len=%zu",
    //           __entry->pid, __entry->comm, __entry->off, __entry->len)
    // );
    pub fn trace_bpf_testmod_test_read(
        task: *mut task_struct,
        ctx: *mut bpf_testmod_test_read_ctx,
    );

    // A bare tracepoint with no event associated with it
    //
    // DECLARE_TRACE(bpf_testmod_test_write_bare,
    //     TP_PROTO(struct task_struct *task, struct bpf_testmod_test_write_ctx *ctx),
    //     TP_ARGS(task, ctx)
    // );
    pub fn trace_bpf_testmod_test_write_bare(
        task: *mut task_struct,
        ctx: *mut bpf_testmod_test_write_ctx,
    );

    // Used in bpf_testmod_test_read() to test __nullable suffix
    //
    // DECLARE_TRACE(bpf_testmod_test_nullable_bare,
    //     TP_PROTO(struct bpf_testmod_test_read_ctx *ctx__nullable),
    //     TP_ARGS(ctx__nullable)
    // );
    pub fn trace_bpf_testmod_test_nullable_bare(ctx__nullable: *mut bpf_testmod_test_read_ctx);

    // struct sk_buff;
    //
    // DECLARE_TRACE(bpf_testmod_test_raw_tp_null,
    //     TP_PROTO(struct sk_buff *skb),
    //     TP_ARGS(skb)
    // );
    pub fn trace_bpf_testmod_test_raw_tp_null(skb: *mut sk_buff);

    // #undef BPF_TESTMOD_DECLARE_TRACE
    // #ifdef DECLARE_TRACE_WRITABLE
    // #define BPF_TESTMOD_DECLARE_TRACE(call, proto, args, size) \
    //     DECLARE_TRACE_WRITABLE(call, PARAMS(proto), PARAMS(args), size)
    // #else
    // #define BPF_TESTMOD_DECLARE_TRACE(call, proto, args, size) \
    //     DECLARE_TRACE(call, PARAMS(proto), PARAMS(args))
    // #endif
    //
    // BPF_TESTMOD_DECLARE_TRACE(bpf_testmod_test_writable_bare,
    //     TP_PROTO(struct bpf_testmod_test_writable_ctx *ctx),
    //     TP_ARGS(ctx),
    //     sizeof(struct bpf_testmod_test_writable_ctx)
    // );
    pub fn trace_bpf_testmod_test_writable_bare(ctx: *mut bpf_testmod_test_writable_ctx);

    // DECLARE_TRACE(bpf_testmod_fentry_test1,
    //     TP_PROTO(int a),
    //     TP_ARGS(a)
    // );
    pub fn trace_bpf_testmod_fentry_test1(a: c_int);

    // DECLARE_TRACE(bpf_testmod_fentry_test2,
    //     TP_PROTO(int a, u64 b),
    //     TP_ARGS(a, b)
    // );
    pub fn trace_bpf_testmod_fentry_test2(a: c_int, b: u64);
}

#[repr(C)]
pub struct trace_event_raw_bpf_testmod_test_read {
    pub pid: pid_t,
    pub comm: *mut c_char,
    pub off: loff_t,
    pub len: size_t,
}

// Original trace generation footer:
// #endif /* _BPF_TESTMOD_EVENTS_H */
//
// #undef TRACE_INCLUDE_PATH
// #define TRACE_INCLUDE_PATH .
// #define TRACE_INCLUDE_FILE bpf_testmod-events
// #include <trace/define_trace.h>


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
