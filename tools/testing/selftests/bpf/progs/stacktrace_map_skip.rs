// SPDX-License-Identifier: GPL-2.0
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type __u32 = u32;
type __u64 = u64;

const TEST_STACK_DEPTH: usize = 2;
const TEST_MAX_ENTRIES: usize = 16384;

type stack_trace_t = [__u64; TEST_STACK_DEPTH];

#[repr(C)]
pub struct trace_event_raw_sched_switch {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_get_stackid(ctx: *mut trace_event_raw_sched_switch, map: *mut bpf_map, flags: __u64) -> i64;
    fn bpf_map_update_elem(
        map: *mut bpf_map,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
    fn bpf_map_lookup_elem(
        map: *mut bpf_map,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_get_stack(
        ctx: *mut trace_event_raw_sched_switch,
        buf: *mut core::ffi::c_void,
        size: __u32,
        flags: __u64,
    ) -> i64;
}

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_STACK_TRACE);
//     __uint(max_entries, TEST_MAX_ENTRIES);
//     __type(key, __u32);
//     __type(value, stack_trace_t);
// } stackmap SEC(".maps");
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut stackmap: bpf_map = bpf_map { _private: [] };

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, TEST_MAX_ENTRIES);
//     __type(key, __u32);
//     __type(value, __u32);
// } stackid_hmap SEC(".maps");
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut stackid_hmap: bpf_map = bpf_map { _private: [] };

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, TEST_MAX_ENTRIES);
//     __type(key, __u32);
//     __type(value, stack_trace_t);
// } stack_amap SEC(".maps");
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut stack_amap: bpf_map = bpf_map { _private: [] };

#[unsafe(no_mangle)]
pub static mut pid: core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut control: core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut failed: core::ffi::c_int = 0;

#[unsafe(link_section = "tracepoint/sched/sched_switch")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oncpu(ctx: *mut trace_event_raw_sched_switch) -> core::ffi::c_int {
    let max_len: __u32 = (TEST_STACK_DEPTH * core::mem::size_of::<__u64>()) as __u32;
    let mut key: __u32 = 0;
    let val: __u32 = 0;
    let stack_p: *mut __u64;

    if pid as __u64 != (bpf_get_current_pid_tgid() >> 32) {
        return 0;
    }

    if control != 0 {
        return 0;
    }

    /* it should allow skipping whole buffer size entries */
    key = bpf_get_stackid(ctx, core::ptr::addr_of_mut!(stackmap), TEST_STACK_DEPTH as __u64) as __u32;
    if (key as core::ffi::c_int) >= 0 {
        /* The size of stackmap and stack_amap should be the same */
        bpf_map_update_elem(
            core::ptr::addr_of_mut!(stackid_hmap),
            core::ptr::addr_of!(key).cast::<core::ffi::c_void>(),
            core::ptr::addr_of!(val).cast::<core::ffi::c_void>(),
            0,
        );
        stack_p = bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(stack_amap),
            core::ptr::addr_of!(key).cast::<core::ffi::c_void>(),
        )
        .cast::<__u64>();
        if !stack_p.is_null() {
            bpf_get_stack(
                ctx,
                stack_p.cast::<core::ffi::c_void>(),
                max_len,
                TEST_STACK_DEPTH as __u64,
            );
            /* it wrongly skipped all the entries and filled zero */
            if *stack_p.add(0) == 0 {
                failed = 1;
            }
        }
    } else {
        /* old kernel doesn't support skipping that many entries */
        failed = 2;
    }

    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
