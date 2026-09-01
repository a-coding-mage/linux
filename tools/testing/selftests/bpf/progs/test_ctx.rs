// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2026 Valve Corporation.
 * Author: Changwoo Min <changwoo@igalia.com>
 */

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// and "bpf_experimental.h".

use core::ffi::c_void;
use core::sync::atomic::{AtomicI32, Ordering};

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

extern "C" {
    #[link_name = "bpf_kfunc_trigger_ctx_check"]
    fn bpf_kfunc_trigger_ctx_check();

    fn bpf_in_task() -> bool;
    fn bpf_in_hardirq() -> bool;
    fn bpf_in_serving_softirq() -> bool;
}

#[no_mangle]
pub static count_hardirq: AtomicI32 = AtomicI32::new(0);
#[no_mangle]
pub static count_softirq: AtomicI32 = AtomicI32::new(0);
#[no_mangle]
pub static count_task: AtomicI32 = AtomicI32::new(0);

/* Triggered via bpf_prog_test_run from user-space */
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn trigger_all_contexts(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    if bpf_in_task() {
        count_task.fetch_add(1, Ordering::SeqCst);
    }

    /* Trigger the firing of a hardirq and softirq for test. */
    bpf_kfunc_trigger_ctx_check();
    0
}

/* Observer for HardIRQ */
#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_hardirq_fn"]
pub unsafe extern "C" fn on_hardirq() -> i32 {
    if bpf_in_hardirq() {
        count_hardirq.fetch_add(1, Ordering::SeqCst);
    }
    0
}

/* Observer for SoftIRQ */
#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_softirq_fn"]
pub unsafe extern "C" fn on_softirq() -> i32 {
    if bpf_in_serving_softirq() {
        count_softirq.fetch_add(1, Ordering::SeqCst);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
