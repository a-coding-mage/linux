// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external expectations:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::arch::asm;
use core::ffi::c_void;
use core::ptr::null_mut;

unsafe extern "C" {
    fn bpf_loop(
        nr_loops: u32,
        callback_fn: extern "C" fn(i32, *mut c_void) -> i32,
        callback_ctx: *mut c_void,
        flags: u64,
    ) -> i64;
}

/* linear chain main -> A -> B */
// __naked __noinline __used
#[inline(never)]
#[used]
unsafe extern "C" fn linear_b() -> u64 {
    unsafe {
        asm!(
            "r0 = 42",
            "exit",
            options(noreturn)
        );
    }
}

// __naked __noinline __used
#[inline(never)]
#[used]
unsafe extern "C" fn linear_a() -> u64 {
    unsafe {
        asm!(
            "call linear_b",
            "exit",
            options(noreturn)
        );
    }
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("topo_order[0] = linear_b")
// __msg("topo_order[1] = linear_a")
// __msg("topo_order[2] = topo_linear")
// __naked
#[no_mangle]
#[link_section = "?raw_tp"]
pub unsafe extern "C" fn topo_linear() -> i32 {
    unsafe {
        asm!(
            "call linear_a",
            "exit",
            options(noreturn)
        );
    }
}

/* diamond main -> A, main -> B, A -> C, B -> C */
// __naked __noinline __used
#[inline(never)]
#[used]
unsafe extern "C" fn diamond_c() -> u64 {
    unsafe {
        asm!(
            "r0 = 1",
            "exit",
            options(noreturn)
        );
    }
}

// __naked __noinline __used
#[inline(never)]
#[used]
unsafe extern "C" fn diamond_b() -> u64 {
    unsafe {
        asm!(
            "call diamond_c",
            "exit",
            options(noreturn)
        );
    }
}

// __naked __noinline __used
#[inline(never)]
#[used]
unsafe extern "C" fn diamond_a() -> u64 {
    unsafe {
        asm!(
            "call diamond_c",
            "exit",
            options(noreturn)
        );
    }
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("topo_order[0] = diamond_c")
// __msg("topo_order[3] = topo_diamond")
// __naked
#[no_mangle]
#[link_section = "?raw_tp"]
pub unsafe extern "C" fn topo_diamond() -> i32 {
    unsafe {
        asm!(
            "call diamond_a",
            "call diamond_b",
            "exit",
            options(noreturn)
        );
    }
}

/* main -> global_a (global) -> static_leaf (static, leaf) */
// __naked __noinline __used
#[inline(never)]
#[used]
unsafe extern "C" fn static_leaf() -> u64 {
    unsafe {
        asm!(
            "r0 = 7",
            "exit",
            options(noreturn)
        );
    }
}

// __noinline __used
#[no_mangle]
#[inline(never)]
#[used]
pub unsafe extern "C" fn global_a(_x: i32) -> i32 {
    unsafe { static_leaf() as i32 }
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("topo_order[0] = static_leaf")
// __msg("topo_order[1] = global_a")
// __msg("topo_order[2] = topo_mixed")
// __naked
#[no_mangle]
#[link_section = "?raw_tp"]
pub unsafe extern "C" fn topo_mixed() -> i32 {
    unsafe {
        asm!(
            "r1 = 0",
            "call global_a",
            "exit",
            options(noreturn)
        );
    }
}

/*
 * shared static callee from global and main:
 * main -> shared_leaf (static)
 * main -> global_b (global) -> shared_leaf (static)
 */
// __naked __noinline __used
#[inline(never)]
#[used]
unsafe extern "C" fn shared_leaf() -> u64 {
    unsafe {
        asm!(
            "r0 = 99",
            "exit",
            options(noreturn)
        );
    }
}

// __noinline __used
#[no_mangle]
#[inline(never)]
#[used]
pub unsafe extern "C" fn global_b(_x: i32) -> i32 {
    unsafe { shared_leaf() as i32 }
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("topo_order[0] = shared_leaf")
// __msg("topo_order[1] = global_b")
// __msg("topo_order[2] = topo_shared")
// __naked
#[no_mangle]
#[link_section = "?raw_tp"]
pub unsafe extern "C" fn topo_shared() -> i32 {
    unsafe {
        asm!(
            "call shared_leaf",
            "r1 = 0",
            "call global_b",
            "exit",
            options(noreturn)
        );
    }
}

/* duplicate calls to the same subprog */
// __naked __noinline __used
#[inline(never)]
#[used]
unsafe extern "C" fn dup_leaf() -> u64 {
    unsafe {
        asm!(
            "r0 = 0",
            "exit",
            options(noreturn)
        );
    }
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("topo_order[0] = dup_leaf")
// __msg("topo_order[1] = topo_dup_calls")
// __naked
#[no_mangle]
#[link_section = "?raw_tp"]
pub unsafe extern "C" fn topo_dup_calls() -> i32 {
    unsafe {
        asm!(
            "call dup_leaf",
            "call dup_leaf",
            "exit",
            options(noreturn)
        );
    }
}

/* main calls bpf_loop() with loop_cb as the callback */
extern "C" fn loop_cb(_idx: i32, _ctx: *mut c_void) -> i32 {
    0
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("topo_order[0] = loop_cb")
// __msg("topo_order[1] = topo_loop_cb")
#[no_mangle]
#[link_section = "?raw_tp"]
pub unsafe extern "C" fn topo_loop_cb() -> i32 {
    unsafe {
        bpf_loop(1, loop_cb, null_mut(), 0);
    }
    0
}

/*
 * bpf_loop callback calling another subprog
 * main -> bpf_loop(callback=loop_cb2) -> loop_cb2 -> loop_cb2_leaf
 */
// __naked __noinline __used
#[inline(never)]
#[used]
unsafe extern "C" fn loop_cb2_leaf() -> u64 {
    unsafe {
        asm!(
            "r0 = 0",
            "exit",
            options(noreturn)
        );
    }
}

extern "C" fn loop_cb2(_idx: i32, _ctx: *mut c_void) -> i32 {
    unsafe { loop_cb2_leaf() as i32 }
}

// SEC("?raw_tp")
// __success __log_level(2)
// __msg("topo_order[0] = loop_cb2_leaf")
// __msg("topo_order[1] = loop_cb2")
// __msg("topo_order[2] = topo_loop_cb_chain")
#[no_mangle]
#[link_section = "?raw_tp"]
pub unsafe extern "C" fn topo_loop_cb_chain() -> i32 {
    unsafe {
        bpf_loop(1, loop_cb2, null_mut(), 0);
    }
    0
}

/* no calls (single subprog) */
// SEC("?raw_tp")
// __success __log_level(2)
// __msg("topo_order[0] = topo_no_calls")
// __naked
#[no_mangle]
#[link_section = "?raw_tp"]
pub unsafe extern "C" fn topo_no_calls() -> i32 {
    unsafe {
        asm!(
            "r0 = 0",
            "exit",
            options(noreturn)
        );
    }
}

#[no_mangle]
#[used]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
