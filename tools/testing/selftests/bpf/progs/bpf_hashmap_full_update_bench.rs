// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Bytedance */

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, "bpf_misc.h".

type u32 = u32;
type u64 = u64;
type __u32 = u32;

const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_ANY: u64 = 0;
const MAX_ENTRIES: u32 = 1000;

#[used]
#[link_section = "license"]
static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct HashMapBench {
    // Original C declaration uses BPF map definition macros:
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __type(key, u32);
    // __type(value, u64);
    // __uint(max_entries, MAX_ENTRIES);
    type_: u32,
    max_entries: u32,
}

#[used]
#[link_section = ".maps"]
static mut hash_map_bench: HashMapBench = HashMapBench {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: MAX_ENTRIES,
};

#[repr(align(256))]
pub struct AlignedPercpuTime(pub [u64; 256]);

static mut percpu_time: AlignedPercpuTime = AlignedPercpuTime([0; 256]);
static mut nr_loops: u64 = 0;

extern "C" {
    fn bpf_map_update_elem(
        map: *mut HashMapBench,
        key: *const u32,
        value: *const u64,
        flags: u64,
    ) -> i64;
    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_loop(
        nr_loops: u64,
        callback_fn: extern "C" fn(__u32, *mut u32) -> i32,
        callback_ctx: *mut core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

extern "C" fn loop_update_callback(index: __u32, key: *mut u32) -> i32 {
    let init_val: u64 = 1;

    unsafe {
        bpf_map_update_elem(
            core::ptr::addr_of_mut!(hash_map_bench),
            key,
            core::ptr::addr_of!(init_val),
            BPF_ANY,
        );
    }
    0
}

// Original section: SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[link_section = "fentry/sys_getpgid"]
pub extern "C" fn benchmark(ctx: *mut core::ffi::c_void) -> i32 {
    unsafe {
        let cpu: u32 = bpf_get_smp_processor_id();
        let mut key: u32 = cpu.wrapping_add(MAX_ENTRIES);
        let start_time: u64 = bpf_ktime_get_ns();

        bpf_loop(
            nr_loops,
            loop_update_callback,
            core::ptr::addr_of_mut!(key).cast::<core::ffi::c_void>(),
            0,
        );
        percpu_time.0[(cpu & 255) as usize] = bpf_ktime_get_ns().wrapping_sub(start_time);
        0
    }
}
