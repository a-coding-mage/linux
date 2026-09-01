// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

// C dependencies intentionally not expanded here:
// #define BPF_NO_KFUNC_PROTOTYPES
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"
// #include "bpf_experimental.h"
// #include <bpf_arena_common.h>
// #include "../test_kmods/bpf_testmod_kfunc.h"

type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;

unsafe extern "C" {
    static NUMA_NO_NODE: core::ffi::c_int;
    static PAGE_SIZE: u64;

    fn bpf_arena_alloc_pages(
        arena: *mut core::ffi::c_void,
        addr: *mut core::ffi::c_void,
        page_cnt: u64,
        node: core::ffi::c_int,
        flags: u64,
    ) -> *mut core::ffi::c_void;
    fn bpf_arena_free_pages(
        arena: *mut core::ffi::c_void,
        ptr: *mut core::ffi::c_void,
        page_cnt: u64,
    );
    fn bpf_addr_space_cast(addr: u64, dst_as: u32, src_as: u32) -> u64;

    fn bpf_kfunc_arena_arg_test(arg: *mut u64) -> u64;
    fn bpf_kfunc_arena_cap_test(arg: *mut u64) -> u64;
    fn bpf_kfunc_arena_cap_nullable_test(arg: *mut u64) -> u64;
    fn bpf_kfunc_arena_args5_test(
        arg1: *mut u64,
        arg2: *mut u64,
        arg3: *mut u64,
        arg4: *mut u64,
        arg5: *mut u64,
    ) -> u64;
    fn bpf_kfunc_arena_mixed_test(arg1: *mut u64, arg2: *mut u64) -> u64;
    fn bpf_kfunc_arena_stack_arg_test(
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: *mut u64,
    ) -> u64;
}

// Original C map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARENA);
//     __uint(map_flags, BPF_F_MMAPABLE);
//     /* page 0 hosts the arena global, page 1 is for allocations */
//     __uint(max_entries, 2);
// } arena SEC(".maps");
#[repr(C)]
pub struct arena_map_def {
    pub type_: u32,
    pub map_flags: u32,
    /* page 0 hosts the arena global, page 1 is for allocations */
    pub max_entries: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut arena: arena_map_def = arena_map_def {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: 2,
};

unsafe extern "C" {
    static BPF_MAP_TYPE_ARENA: u32;
    static BPF_F_MMAPABLE: u32;
}

/*
 * Occupies page 0 so no allocation lands at arena offset 0, which the
 * nullable tests below must be able to tell apart from NULL.
 */
#[no_mangle]
pub static mut arena_pad: u64 = 0;

/* volatile to force the scalar reloads below */
#[no_mangle]
pub static mut stash: u64 = 0;

// SEC("syscall")
// __arch_x86_64
// __arch_arm64
// __success __retval(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_arg_forms(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;

    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    {
        let mut val: *mut u64;
        let mut ret: u64;

        val = bpf_arena_alloc_pages(
            &raw mut arena as *mut core::ffi::c_void,
            core::ptr::null_mut(),
            1,
            NUMA_NO_NODE,
            0,
        ) as *mut u64;
        if val.is_null() {
            return 1;
        }

        /* PTR_TO_ARENA argument */
        *val = 41;
        ret = bpf_kfunc_arena_arg_test(val as *mut u64);
        if ret != 41 || *val != 42 {
            return 2;
        }

        /* the low 32 bits as a scalar */
        core::ptr::write_volatile(&raw mut stash, val as u64 as u32 as u64);
        ret = bpf_kfunc_arena_arg_test(core::ptr::read_volatile(&raw const stash) as *mut u64);
        if ret != 42 || *val != 43 {
            return 3;
        }

        /* the full user address as a scalar */
        core::ptr::write_volatile(&raw mut stash, val as u64);
        bpf_addr_space_cast(core::ptr::read_volatile(&raw const stash), 1, 0);
        ret = bpf_kfunc_arena_arg_test(core::ptr::read_volatile(&raw const stash) as *mut u64);
        if ret != 43 || *val != 44 {
            return 4;
        }

        bpf_arena_free_pages(
            &raw mut arena as *mut core::ffi::c_void,
            val as *mut core::ffi::c_void,
            1,
        );
    }
    // #endif
    0
}

/*
 * Pin the rebase semantics using the capture kfuncs, which return the raw
 * argument value: __arena rebases unconditionally, so zero low 32 bits
 * arrive as the arena kernel base, while __arena__nullable turns them into
 * NULL.
 */
// SEC("syscall")
// __arch_x86_64
// __arch_arm64
// __success __retval(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_arg_rebase(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;

    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    {
        let mut val: *mut u64;
        let mut base: u64;
        let mut off: u64;

        val = bpf_arena_alloc_pages(
            &raw mut arena as *mut core::ffi::c_void,
            core::ptr::null_mut(),
            1,
            NUMA_NO_NODE,
            0,
        ) as *mut u64;
        if val.is_null() {
            return 1;
        }

        base = bpf_kfunc_arena_cap_test(core::ptr::null_mut());
        if base == 0 {
            return 2;
        }

        /* only the low 32 bits contribute */
        core::ptr::write_volatile(&raw mut stash, 0xbadc0ffe00000000);
        if bpf_kfunc_arena_cap_test(core::ptr::read_volatile(&raw const stash) as *mut u64) != base {
            return 3;
        }

        off = val as u64 as u32 as u64;
        if bpf_kfunc_arena_cap_test(val as *mut u64) != base + off {
            return 4;
        }

        if bpf_kfunc_arena_cap_nullable_test(core::ptr::null_mut()) != 0 {
            return 5;
        }

        core::ptr::write_volatile(&raw mut stash, 0xbadc0ffe00000000);
        if bpf_kfunc_arena_cap_nullable_test(core::ptr::read_volatile(&raw const stash) as *mut u64) != 0 {
            return 6;
        }

        if bpf_kfunc_arena_cap_nullable_test(val as *mut u64) != base + off {
            return 7;
        }

        bpf_arena_free_pages(
            &raw mut arena as *mut core::ffi::c_void,
            val as *mut core::ffi::c_void,
            1,
        );
    }
    // #endif
    0
}

// SEC("syscall")
// __arch_x86_64
// __arch_arm64
// __success __retval(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_args5(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;

    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    {
        let mut val: *mut u64;

        val = bpf_arena_alloc_pages(
            &raw mut arena as *mut core::ffi::c_void,
            core::ptr::null_mut(),
            1,
            NUMA_NO_NODE,
            0,
        ) as *mut u64;
        if val.is_null() {
            return 1;
        }

        *val.add(0) = 1;
        *val.add(1) = 2;
        *val.add(2) = 4;
        *val.add(3) = 8;
        *val.add(4) = 16;

        if bpf_kfunc_arena_args5_test(
            val.add(0) as *mut u64,
            val.add(1) as *mut u64,
            val.add(2) as *mut u64,
            val.add(3) as *mut u64,
            val.add(4) as *mut u64,
        ) != 31
        {
            return 2;
        }
        if bpf_kfunc_arena_args5_test(
            val.add(0) as *mut u64,
            val.add(1) as *mut u64,
            val.add(2) as *mut u64,
            val.add(3) as *mut u64,
            core::ptr::null_mut(),
        ) != 15
        {
            return 3;
        }

        bpf_arena_free_pages(
            &raw mut arena as *mut core::ffi::c_void,
            val as *mut core::ffi::c_void,
            1,
        );
    }
    // #endif
    0
}

// SEC("syscall")
// __arch_x86_64
// __arch_arm64
// __success __retval(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_arg_mixed(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;

    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    {
        let mut val: *mut u64;

        val = bpf_arena_alloc_pages(
            &raw mut arena as *mut core::ffi::c_void,
            core::ptr::null_mut(),
            1,
            NUMA_NO_NODE,
            0,
        ) as *mut u64;
        if val.is_null() {
            return 1;
        }

        *val.add(0) = 7;
        *val.add(1) = 5;

        if bpf_kfunc_arena_mixed_test(val.add(0) as *mut u64, core::ptr::null_mut()) != 7 {
            return 2;
        }

        if bpf_kfunc_arena_mixed_test(val.add(0) as *mut u64, val.add(1) as *mut u64) != 12 {
            return 3;
        }

        bpf_arena_free_pages(
            &raw mut arena as *mut core::ffi::c_void,
            val as *mut core::ffi::c_void,
            1,
        );
    }
    // #endif
    0
}

/* kernel-side faults on unpopulated pages recover via the scratch page */
// SEC("syscall")
// __arch_x86_64
// __arch_arm64
// __success __retval(0)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_arg_unpopulated(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;

    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    {
        let mut val: *mut u64;

        val = bpf_arena_alloc_pages(
            &raw mut arena as *mut core::ffi::c_void,
            core::ptr::null_mut(),
            1,
            NUMA_NO_NODE,
            0,
        ) as *mut u64;
        if val.is_null() {
            return 1;
        }

        core::ptr::write_volatile(&raw mut stash, val as u64 + PAGE_SIZE);
        bpf_kfunc_arena_arg_test(core::ptr::read_volatile(&raw const stash) as *mut u64);

        bpf_arena_free_pages(
            &raw mut arena as *mut core::ffi::c_void,
            val as *mut core::ffi::c_void,
            1,
        );
    }
    // #endif
    0
}

// SEC("syscall")
// __arch_x86_64
// __arch_arm64
// __failure __msg("arena pointer requires a program with an associated arena")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_arg_no_arena(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;

    bpf_kfunc_arena_arg_test(1usize as *mut u64);
    0
}

// SEC("syscall")
// __arch_x86_64
// __arch_arm64
// __failure __msg("is not a pointer to arena or scalar")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_arg_bad_reg(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;
    let mut buf: u64 = 0;

    /* use the arena so the program passes the arena presence check */
    bpf_arena_alloc_pages(
        &raw mut arena as *mut core::ffi::c_void,
        core::ptr::null_mut(),
        1,
        NUMA_NO_NODE,
        0,
    );
    bpf_kfunc_arena_arg_test(&mut buf as *mut u64);
    0
}

// #if defined(__BPF_FEATURE_ADDR_SPACE_CAST) && \
//     defined(__BPF_FEATURE_STACK_ARGUMENT)
// SEC("syscall")
// __arch_x86_64
// __arch_arm64
// __failure __msg("arena pointer cannot be a stack argument")
#[cfg(all(feature = "__BPF_FEATURE_ADDR_SPACE_CAST", feature = "__BPF_FEATURE_STACK_ARGUMENT"))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_arg_stack(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;

    bpf_arena_alloc_pages(
        &raw mut arena as *mut core::ffi::c_void,
        core::ptr::null_mut(),
        1,
        NUMA_NO_NODE,
        0,
    );
    bpf_kfunc_arena_stack_arg_test(1, 2, 3, 4, 5, 1usize as *mut u64);
    0
}

// #else
// SEC("syscall")
// __arch_x86_64
// __arch_arm64
// __description("arena_arg_stack: not supported, dummy test")
// __success
#[cfg(not(all(feature = "__BPF_FEATURE_ADDR_SPACE_CAST", feature = "__BPF_FEATURE_STACK_ARGUMENT")))]
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_arg_stack(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;

    0
}
// #endif

#[no_mangle]
#[link_section = "license"]
pub static _license: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
