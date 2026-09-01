// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook

// Dependencies from the original C includes:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

const PERF_MAX_STACK_DEPTH: usize = 127;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_stack_build_id {
    _private: [u8; 0],
}

type stack_trace_t = [bpf_stack_build_id; PERF_MAX_STACK_DEPTH];

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_MAP_TYPE_STACK_TRACE: u32 = 7;
const BPF_F_STACK_BUILD_ID: u32 = 256;
const BPF_F_USER_STACK: u64 = 256;
const BPF_F_USER_BUILD_ID: u64 = 2048;

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub map_flags: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut control_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    map_flags: 0,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut stackid_hmap: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 16384,
    map_flags: 0,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut stackmap: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_STACK_TRACE,
    max_entries: 128,
    map_flags: BPF_F_STACK_BUILD_ID,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut stack_amap: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 128,
    map_flags: 0,
};

extern "C" {
    fn bpf_map_lookup_elem(map: *mut bpf_map_def, key: *const u32) -> *mut core::ffi::c_void;
    fn bpf_get_stackid(ctx: *mut pt_regs, map: *mut bpf_map_def, flags: u64) -> i64;
    fn bpf_map_update_elem(
        map: *mut bpf_map_def,
        key: *const u32,
        value: *const u32,
        flags: u64,
    ) -> i64;
    fn bpf_get_stack(
        ctx: *mut pt_regs,
        buf: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
    ) -> i64;
}

#[link_section = "kprobe/urandom_read_iter"]
#[no_mangle]
pub unsafe extern "C" fn oncpu(args: *mut pt_regs) -> i32 {
    let max_len: u32 =
        (core::mem::size_of::<bpf_stack_build_id>() * PERF_MAX_STACK_DEPTH) as u32;
    let mut key: u32 = 0;
    let val: u32 = 0;
    let mut value_p: *mut u32;
    let mut stack_p: *mut core::ffi::c_void;

    value_p = bpf_map_lookup_elem(&mut control_map, &key as *const u32) as *mut u32;
    if !value_p.is_null() && *value_p != 0 {
        return 0; /* skip if non-zero *value_p */
    }

    /* The size of stackmap and stackid_hmap should be the same */
    key = bpf_get_stackid(args, &mut stackmap, BPF_F_USER_STACK) as u32;
    if (key as i32) >= 0 {
        bpf_map_update_elem(&mut stackid_hmap, &key as *const u32, &val as *const u32, 0);
        stack_p = bpf_map_lookup_elem(&mut stack_amap, &key as *const u32);
        if !stack_p.is_null() {
            bpf_get_stack(
                args,
                stack_p,
                max_len,
                BPF_F_USER_STACK | BPF_F_USER_BUILD_ID,
            );
        }
    }

    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
