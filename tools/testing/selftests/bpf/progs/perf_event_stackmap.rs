// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies: "vmlinux.h" and <bpf/bpf_helpers.h>.

// #ifndef PERF_MAX_STACK_DEPTH
pub const PERF_MAX_STACK_DEPTH: usize = 127;
// #endif

pub type stack_trace_t = [u64; PERF_MAX_STACK_DEPTH];

// SEC(".maps")
// struct {
//     __uint(type, BPF_MAP_TYPE_STACK_TRACE);
//     __uint(max_entries, 16384);
//     __type(key, __u32);
//     __type(value, stack_trace_t);
// } stackmap;
#[repr(C)]
pub struct stackmap_def {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut stackmap: stackmap_def;
}

// SEC(".maps")
// struct {
//     __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, __u32);
//     __type(value, stack_trace_t);
// } stackdata_map;
#[repr(C)]
pub struct stackdata_map_def {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut stackdata_map: stackdata_map_def;
}

#[unsafe(no_mangle)]
pub static mut stackid_kernel: i64 = 1;
#[unsafe(no_mangle)]
pub static mut stackid_user: i64 = 1;
#[unsafe(no_mangle)]
pub static mut stack_kernel: i64 = 1;
#[unsafe(no_mangle)]
pub static mut stack_user: i64 = 1;

unsafe extern "C" {
    pub fn bpf_get_stackid(ctx: *mut core::ffi::c_void, map: *mut stackmap_def, flags: u64) -> i64;
    pub fn bpf_map_lookup_elem(
        map: *mut stackdata_map_def,
        key: *const u32,
    ) -> *mut core::ffi::c_void;
    pub fn bpf_get_stack(
        ctx: *mut core::ffi::c_void,
        buf: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
    ) -> i64;
}

// External BPF flag from <bpf/bpf_helpers.h> / UAPI headers.
unsafe extern "C" {
    pub static BPF_F_USER_STACK: u64;
}

// SEC("perf_event")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oncpu(ctx: *mut core::ffi::c_void) -> i32 {
    let trace: *mut stack_trace_t;
    let key: u32 = 0;
    let mut val: i64;

    val = unsafe { bpf_get_stackid(ctx, &raw mut stackmap, 0) };
    if val >= 0 {
        unsafe {
            stackid_kernel = 2;
        }
    }
    val = unsafe { bpf_get_stackid(ctx, &raw mut stackmap, BPF_F_USER_STACK) };
    if val >= 0 {
        unsafe {
            stackid_user = 2;
        }
    }

    trace = unsafe {
        bpf_map_lookup_elem(&raw mut stackdata_map, &key as *const u32) as *mut stack_trace_t
    };
    if trace.is_null() {
        return 0;
    }

    val = unsafe {
        bpf_get_stack(
            ctx,
            trace as *mut core::ffi::c_void,
            core::mem::size_of::<stack_trace_t>() as u32,
            0,
        )
    };
    if val > 0 {
        unsafe {
            stack_kernel = 2;
        }
    }

    val = unsafe {
        bpf_get_stack(
            ctx,
            trace as *mut core::ffi::c_void,
            core::mem::size_of::<stack_trace_t>() as u32,
            BPF_F_USER_STACK,
        )
    };
    if val > 0 {
        unsafe {
            stack_user = 2;
        }
    }

    0
}

// SEC("license")
#[unsafe(no_mangle)]
pub static LICENSE: [u8; 4] = *b"GPL\0";
