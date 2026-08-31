// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook

// C includes translated as dependency intent:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

pub const PERF_MAX_STACK_DEPTH: usize = 127;

pub type stack_trace_t = [__u64; PERF_MAX_STACK_DEPTH];

extern "C" {
    fn bpf_get_stackid(ctx: *mut core::ffi::c_void, map: *mut core::ffi::c_void, flags: __u64) -> __s64;
}

pub type __u64 = u64;
pub type __s64 = i64;
pub type __u32 = u32;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

// Map definition translated from:
// struct {
//     __uint(type, BPF_MAP_TYPE_STACK_TRACE);
//     __uint(max_entries, 16384);
//     __type(key, __u32);
//     __type(value, stack_trace_t);
// } stackmap SEC(".maps");
#[repr(C)]
pub struct stackmap_map {
    _private: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut stackmap: stackmap_map = stackmap_map { _private: [] };

extern "C" {
    #[link_name = "CONFIG_UNWINDER_ORC"]
    pub static CONFIG_UNWINDER_ORC: bool;
}

/*
 * This function is here to have CONFIG_UNWINDER_ORC
 * used and added to object BTF.
 */
#[no_mangle]
pub unsafe extern "C" fn unused() -> i32 {
    if CONFIG_UNWINDER_ORC {
        0
    } else {
        1
    }
}

#[no_mangle]
pub static mut stack_key: __u32 = 0;

#[link_section = "kprobe"]
#[no_mangle]
pub unsafe extern "C" fn kprobe_test(ctx: *mut pt_regs) -> i32 {
    stack_key = bpf_get_stackid(
        ctx as *mut core::ffi::c_void,
        core::ptr::addr_of_mut!(stackmap) as *mut core::ffi::c_void,
        0,
    ) as __u32;
    0
}

#[link_section = "kprobe.multi"]
#[no_mangle]
pub unsafe extern "C" fn kprobe_multi_test(ctx: *mut pt_regs) -> i32 {
    stack_key = bpf_get_stackid(
        ctx as *mut core::ffi::c_void,
        core::ptr::addr_of_mut!(stackmap) as *mut core::ffi::c_void,
        0,
    ) as __u32;
    0
}

#[link_section = "raw_tp/bpf_testmod_test_read"]
#[no_mangle]
pub unsafe extern "C" fn rawtp_test(ctx: *mut core::ffi::c_void) -> i32 {
    /* Skip ebpf program entry in the stack. */
    stack_key = bpf_get_stackid(
        ctx,
        core::ptr::addr_of_mut!(stackmap) as *mut core::ffi::c_void,
        0,
    ) as __u32;
    0
}

#[link_section = "fentry/bpf_testmod_stacktrace_test"]
#[no_mangle]
pub unsafe extern "C" fn fentry_test(ctx: *mut pt_regs) -> i32 {
    /*
     * Skip 2 bpf_program/trampoline stack entries:
     * - bpf_prog_bd1f7a949f55fb03_fentry_test
     * - bpf_trampoline_182536277701
     */
    stack_key = bpf_get_stackid(
        ctx as *mut core::ffi::c_void,
        core::ptr::addr_of_mut!(stackmap) as *mut core::ffi::c_void,
        2,
    ) as __u32;
    0
}

#[link_section = "fexit/bpf_testmod_stacktrace_test"]
#[no_mangle]
pub unsafe extern "C" fn fexit_test(ctx: *mut pt_regs) -> i32 {
    /* Skip 2 bpf_program/trampoline stack entries, check fentry_test. */
    stack_key = bpf_get_stackid(
        ctx as *mut core::ffi::c_void,
        core::ptr::addr_of_mut!(stackmap) as *mut core::ffi::c_void,
        2,
    ) as __u32;
    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
