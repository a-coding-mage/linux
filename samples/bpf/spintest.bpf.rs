/* Copyright (c) 2016, Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// C dependencies: vmlinux.h, linux/version.h, bpf/bpf_helpers.h,
// and bpf/bpf_tracing.h.

#[cfg(not(any()))]
const PERF_MAX_STACK_DEPTH: u32 = 127;
#[allow(dead_code)]
const PERF_MAX_STACK_DEPTH_DEFAULT: u32 = 127;

#[repr(C)]
pub struct MyMap {
    pub map_type: u32,
    pub key: i64,
    pub value: i64,
    pub max_entries: u32,
}

#[link_section = ".maps"]
pub static mut my_map: MyMap = MyMap {
    map_type: BPF_MAP_TYPE_HASH,
    key: 0,
    value: 0,
    max_entries: 1024,
};

#[repr(C)]
pub struct MyMap2 {
    pub map_type: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[link_section = ".maps"]
pub static mut my_map2: MyMap2 = MyMap2 {
    map_type: BPF_MAP_TYPE_PERCPU_HASH,
    key_size: core::mem::size_of::<c_long>() as u32,
    value_size: core::mem::size_of::<c_long>() as u32,
    max_entries: 1024,
};

#[repr(C)]
pub struct Stackmap {
    pub map_type: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[link_section = ".maps"]
pub static mut stackmap: Stackmap = Stackmap {
    map_type: BPF_MAP_TYPE_STACK_TRACE,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: PERF_MAX_STACK_DEPTH_DEFAULT * core::mem::size_of::<u64>() as u32,
    max_entries: 10000,
};

// C macro PROG(foo), expanded for each attached probe.
#[inline(always)]
unsafe fn prog(ctx: *mut pt_regs, map: *mut MyMap, map2: *mut MyMap2, stack: *mut Stackmap) -> i32 {
    let mut v: c_long = PT_REGS_IP(ctx) as c_long;
    let val: *mut c_long;

    val = bpf_map_lookup_elem(map as *const _, &mut v as *mut _ as *const _);
    let _ = val;
    bpf_map_update_elem(map as *const _, &v as *const _ as *const _, &v as *const _ as *const _, BPF_ANY);
    bpf_map_update_elem(map2 as *const _, &v as *const _ as *const _, &v as *const _ as *const _, BPF_ANY);
    bpf_map_delete_elem(map2 as *const _, &v as *const _ as *const _);
    bpf_get_stackid(ctx, stack as *const _, BPF_F_REUSE_STACKID);
    0
}

// add kprobes to all possible *spin* functions
#[link_section = "kprobe.multi/spin_*lock*"]
pub unsafe extern "C" fn spin_lock(ctx: *mut pt_regs) -> i32 { prog(ctx, &raw mut my_map, &raw mut my_map2, &raw mut stackmap) }
#[link_section = "kprobe.multi/*_spin_on_owner"]
pub unsafe extern "C" fn spin_on_owner(ctx: *mut pt_regs) -> i32 { prog(ctx, &raw mut my_map, &raw mut my_map2, &raw mut stackmap) }
#[link_section = "kprobe.multi/_raw_spin_*lock*"]
pub unsafe extern "C" fn raw_spin_lock(ctx: *mut pt_regs) -> i32 { prog(ctx, &raw mut my_map, &raw mut my_map2, &raw mut stackmap) }

// and to inner bpf helpers
#[link_section = "kprobe/htab_map_update_elem"]
pub unsafe extern "C" fn p15(ctx: *mut pt_regs) -> i32 { prog(ctx, &raw mut my_map, &raw mut my_map2, &raw mut stackmap) }
#[link_section = "kprobe/__htab_percpu_map_update_elem"]
pub unsafe extern "C" fn p16(ctx: *mut pt_regs) -> i32 { prog(ctx, &raw mut my_map, &raw mut my_map2, &raw mut stackmap) }
#[link_section = "kprobe/htab_map_alloc"]
pub unsafe extern "C" fn p17(ctx: *mut pt_regs) -> i32 { prog(ctx, &raw mut my_map, &raw mut my_map2, &raw mut stackmap) }

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
#[link_section = "version"]
pub static mut _version: u32 = LINUX_VERSION_CODE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
