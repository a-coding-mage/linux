// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

/* Sleepable program on a non-faultable tracepoint should fail to load */
// SEC("tp_btf.s/sched_switch")
// __failure __msg("Sleepable program cannot attach to non-faultable tracepoint")
#[unsafe(link_section = "tp_btf.s/sched_switch")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_sched_switch(
    preempt: bool,
    prev: *mut task_struct,
    next: *mut task_struct,
) -> i32 {
    let _ = preempt;
    let _ = prev;
    let _ = next;

    return 0;
}
