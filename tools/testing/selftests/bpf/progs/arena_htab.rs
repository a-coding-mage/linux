// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
// #define BPF_NO_KFUNC_PROTOTYPES
// Dependencies in the original C source:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// <bpf/bpf_core_read.h>, "bpf_experimental.h", "bpf_arena_htab.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

type __u64 = u64;

#[repr(C)]
pub struct arena_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
}

extern "C" {
    static BPF_MAP_TYPE_ARENA: u32;
    static BPF_F_MMAPABLE: u32;

    fn bpf_alloc(size: usize) -> *mut core::ffi::c_void;
    fn cast_kern<T>(ptr: *mut T);
    fn cast_user<T>(ptr: *mut T);
    fn htab_init(htab: *mut htab);
    fn htab_update_elem(htab: *mut htab, key: __u64, value: __u64);

    static mut can_loop: bool;
}

#[repr(C)]
pub struct htab {
    _unused: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static arena: arena_map_def = arena_map_def {
    type_: unsafe { BPF_MAP_TYPE_ARENA },
    map_flags: unsafe { BPF_F_MMAPABLE },
    max_entries: 100, /* number of pages */
};

#[no_mangle]
pub static mut htab_for_user: *mut htab = core::ptr::null_mut();

#[no_mangle]
pub static mut skip: bool = false;

#[no_mangle]
pub static mut zero: i32 = 0;

#[no_mangle]
pub static mut arr1: [u8; 100000] = [0; 100000];

#[no_mangle]
pub static mut arr2: [u8; 1000] = [0; 1000];

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn arena_htab_llvm(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    // Original C condition:
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST) || defined(BPF_ARENA_FORCE_ASM)
    #[cfg(any(__BPF_FEATURE_ADDR_SPACE_CAST, BPF_ARENA_FORCE_ASM))]
    {
        let htab: *mut htab;
        let arr: *mut u8 = arr1.as_mut_ptr();
        let mut i: __u64;

        htab = bpf_alloc(core::mem::size_of::<htab>()) as *mut htab;
        cast_kern(htab);
        htab_init(htab);

        cast_kern(arr);

        /* first run. No old elems in the table */
        i = zero as __u64;
        while i < 100000 && can_loop {
            htab_update_elem(htab, i, i);
            *arr.add(i as usize) = i as u8;
            i = i.wrapping_add(1);
        }

        /* should replace some elems with new ones */
        i = zero as __u64;
        while i < 1000 && can_loop {
            htab_update_elem(htab, i, i);
            /* Access mem to make the verifier use bounded loop logic */
            arr2[i as usize] = i as u8;
            i = i.wrapping_add(1);
        }
        cast_user(htab);
        htab_for_user = htab;
    }

    #[cfg(not(any(__BPF_FEATURE_ADDR_SPACE_CAST, BPF_ARENA_FORCE_ASM)))]
    {
        skip = true;
    }

    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
