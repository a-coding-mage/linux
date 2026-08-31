// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook
// C dependencies: vmlinux.h, asm/unistd.h, bpf/bpf_helpers.h,
// bpf/bpf_tracing.h, bpf_misc.h, bpf/usdt.bpf.h

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::c_void;
use core::sync::atomic::{AtomicI64, Ordering};

type __u32 = u32;
type __u64 = u64;

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

const CPU_MASK: i32 = 255;
const MAX_CPUS: usize = (CPU_MASK + 1) as usize; /* should match MAX_BUCKETS in benchs/bench_trigger.c */

/* matches struct counter in bench.h */
#[repr(C, align(128))]
pub struct counter {
    pub value: i64,
}

#[no_mangle]
pub static mut hits: [counter; MAX_CPUS] = [counter { value: 0 }; MAX_CPUS];

unsafe extern "C" {
    fn bpf_get_smp_processor_id() -> i32;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_get_stack(ctx: *mut c_void, buf: *mut c_void, size: __u32, flags: __u64) -> i64;
    fn bpf_get_numa_node_id() -> i64;
}

#[inline(always)]
unsafe fn inc_counter() {
    let cpu: i32 = unsafe { bpf_get_smp_processor_id() };
    let idx = (cpu & CPU_MASK) as usize;
    let value = unsafe { &raw mut hits[idx].value };

    unsafe {
        (*(value as *mut AtomicI64)).fetch_add(1, Ordering::SeqCst);
    }
}

#[no_mangle]
pub static stacktrace: i32 = 0;

type stack_trace_t = [__u64; 128];

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
//      __uint(max_entries, 1);
//     __type(key, __u32);
//     __type(value, stack_trace_t);
// } stack_heap SEC(".maps");
#[repr(C)]
pub struct stack_heap_map {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut stack_heap: stack_heap_map = stack_heap_map { _private: [] };

#[inline(always)]
unsafe fn do_stacktrace(ctx: *mut c_void) {
    if unsafe { core::ptr::read_volatile(&raw const stacktrace) } == 0 {
        return;
    }

    let key: __u32 = 0;
    let ptr = unsafe {
        bpf_map_lookup_elem(
            &raw mut stack_heap as *mut c_void,
            &key as *const __u32 as *const c_void,
        ) as *mut __u64
    };

    if !ptr.is_null() {
        unsafe {
            bpf_get_stack(
                ctx,
                ptr as *mut c_void,
                core::mem::size_of::<stack_trace_t>() as __u32,
                0,
            );
        }
    }
}

#[inline(always)]
unsafe fn handle(ctx: *mut c_void) {
    unsafe {
        inc_counter();
        do_stacktrace(ctx);
    }
}

#[no_mangle]
#[link_section = "?uprobe"]
pub unsafe extern "C" fn bench_trigger_uprobe(_ctx: *mut c_void) -> i32 {
    unsafe {
        inc_counter();
    }
    0
}

#[no_mangle]
#[link_section = "?uprobe.multi"]
pub unsafe extern "C" fn bench_trigger_uprobe_multi(_ctx: *mut c_void) -> i32 {
    unsafe {
        inc_counter();
    }
    0
}

#[no_mangle]
pub static batch_iters: i32 = 0;

#[no_mangle]
#[link_section = "?raw_tp"]
pub unsafe extern "C" fn trigger_kernel_count(_ctx: *mut c_void) -> i32 {
    let mut i: i32 = 0;

    while i < unsafe { core::ptr::read_volatile(&raw const batch_iters) } {
        unsafe {
            inc_counter();
            bpf_get_numa_node_id();
        }
        i += 1;
    }

    0
}

#[no_mangle]
#[link_section = "?raw_tp"]
pub unsafe extern "C" fn trigger_driver(_ctx: *mut c_void) -> i32 {
    let mut i: i32 = 0;

    while i < unsafe { core::ptr::read_volatile(&raw const batch_iters) } {
        unsafe {
            bpf_get_numa_node_id(); /* attach point for benchmarking */
        }
        i += 1;
    }

    0
}

unsafe extern "C" {
    // Original declaration used __ksym __weak.
    fn bpf_modify_return_test_tp(nonce: i32) -> i32;
}

#[no_mangle]
#[link_section = "?raw_tp"]
pub unsafe extern "C" fn trigger_driver_kfunc(_ctx: *mut c_void) -> i32 {
    let mut i: i32 = 0;

    while i < unsafe { core::ptr::read_volatile(&raw const batch_iters) } {
        unsafe {
            bpf_modify_return_test_tp(0); /* attach point for benchmarking */
        }
        i += 1;
    }

    0
}

#[no_mangle]
#[link_section = "?kprobe/bpf_get_numa_node_id"]
pub unsafe extern "C" fn bench_trigger_kprobe(ctx: *mut c_void) -> i32 {
    unsafe {
        handle(ctx);
    }
    0
}

#[no_mangle]
#[link_section = "?kretprobe/bpf_get_numa_node_id"]
pub unsafe extern "C" fn bench_trigger_kretprobe(ctx: *mut c_void) -> i32 {
    unsafe {
        handle(ctx);
    }
    0
}

#[no_mangle]
#[link_section = "?kprobe.multi/bpf_get_numa_node_id"]
pub unsafe extern "C" fn bench_trigger_kprobe_multi(ctx: *mut c_void) -> i32 {
    unsafe {
        handle(ctx);
    }
    0
}

#[no_mangle]
#[link_section = "?kprobe.multi/bpf_get_numa_node_id"]
pub unsafe extern "C" fn bench_kprobe_multi_empty(_ctx: *mut c_void) -> i32 {
    0
}

#[no_mangle]
#[link_section = "?kretprobe.multi/bpf_get_numa_node_id"]
pub unsafe extern "C" fn bench_trigger_kretprobe_multi(ctx: *mut c_void) -> i32 {
    unsafe {
        handle(ctx);
    }
    0
}

#[no_mangle]
#[link_section = "?kretprobe.multi/bpf_get_numa_node_id"]
pub unsafe extern "C" fn bench_kretprobe_multi_empty(_ctx: *mut c_void) -> i32 {
    0
}

#[no_mangle]
#[link_section = "?fentry/bpf_get_numa_node_id"]
pub unsafe extern "C" fn bench_trigger_fentry(ctx: *mut c_void) -> i32 {
    unsafe {
        handle(ctx);
    }
    0
}

#[no_mangle]
#[link_section = "?fexit/bpf_get_numa_node_id"]
pub unsafe extern "C" fn bench_trigger_fexit(ctx: *mut c_void) -> i32 {
    unsafe {
        handle(ctx);
    }
    0
}

#[no_mangle]
#[link_section = "?fmod_ret/bpf_modify_return_test_tp"]
pub unsafe extern "C" fn bench_trigger_fmodret(ctx: *mut c_void) -> i32 {
    unsafe {
        handle(ctx);
    }
    -22
}

#[no_mangle]
#[link_section = "?tp/bpf_test_run/bpf_trigger_tp"]
pub unsafe extern "C" fn bench_trigger_tp(ctx: *mut c_void) -> i32 {
    unsafe {
        handle(ctx);
    }
    0
}

#[no_mangle]
#[link_section = "?raw_tp/bpf_trigger_tp"]
pub unsafe extern "C" fn bench_trigger_rawtp(ctx: *mut c_void) -> i32 {
    unsafe {
        handle(ctx);
    }
    0
}

#[no_mangle]
#[link_section = "?usdt"]
pub unsafe extern "C" fn bench_trigger_usdt(_ctx: *mut c_void) -> i32 {
    unsafe {
        inc_counter();
    }
    0
}
