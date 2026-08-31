// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/* BPF_NO_KFUNC_PROTOTYPES */
/* Dependencies from vmlinux.h, bpf_helpers.h, bpf_experimental.h,
 * bpf_arena_common.h, bpf_testmod.h, and bpf_testmod_kfunc.h are supplied
 * externally by the BPF build environment.
 */

type u64 = u64;
type u32 = u32;

const BPF_MAP_TYPE_ARENA: u32 = 33;
const BPF_F_MMAPABLE: u32 = 1024;
const NUMA_NO_NODE: i32 = -1;

#[repr(C)]
pub struct bpf_testmod_ops3 {
    pub test_arena: *mut core::ffi::c_void,
    pub test_arena_nullable: *mut core::ffi::c_void,
    pub test_arena_stack: *mut core::ffi::c_void,
    pub test_arena_multislot: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct ArenaMap {
    /* __uint(type, BPF_MAP_TYPE_ARENA); */
    pub type_: u32,
    /* __uint(map_flags, BPF_F_MMAPABLE); */
    pub map_flags: u32,
    /* page 0 hosts the arena globals, page 1 is for allocations */
    /* __uint(max_entries, 2); */
    pub max_entries: u32,
}

unsafe extern "C" {
    fn bpf_arena_alloc_pages(
        map: *mut ArenaMap,
        addr: *mut core::ffi::c_void,
        page_cnt: u64,
        node: i32,
        flags: u64,
    ) -> *mut u64;
    fn bpf_arena_free_pages(map: *mut ArenaMap, ptr: *mut core::ffi::c_void, page_cnt: u64);
    fn bpf_testmod_ops3_call_test_arena(arg: *mut u64) -> i32;
    fn bpf_testmod_ops3_call_test_arena_nullable(arg: *mut u64) -> i32;
    fn bpf_testmod_ops3_call_test_arena_stack(arg: *mut u64) -> i32;
    fn bpf_testmod_ops3_call_test_arena_multislot(arg: *mut u64) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut arena: ArenaMap = ArenaMap {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: 2,
};

/* also associates the callbacks with the arena */
#[unsafe(no_mangle)]
pub static mut arena_touch: u64 = 0;
/* raw value of the last __arena ctx argument, captured by test_arena_cb */
#[unsafe(no_mangle)]
pub static mut cb_ptr_val: u64 = 0;

#[unsafe(link_section = "struct_ops/test_arena")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_arena_cb(ctx: *mut core::ffi::c_ulonglong) -> i32 {
    let ptr: *mut u64 = *ctx.add(0) as *mut u64;

    arena_touch = arena_touch.wrapping_add(1);
    cb_ptr_val = *ctx.add(0) as u64;
    *ptr = (*ptr).wrapping_add(1);
    return 0;
}

#[unsafe(link_section = "struct_ops/test_arena_nullable")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_arena_nullable_cb(ctx: *mut core::ffi::c_ulonglong) -> i32 {
    let ptr: *mut u64 = *ctx.add(0) as *mut u64;

    arena_touch = arena_touch.wrapping_add(1);
    if ptr.is_null() {
        return 0xbee;
    }
    *ptr = (*ptr).wrapping_add(1);
    return 0;
}

#[unsafe(link_section = "struct_ops/test_arena_stack")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_arena_stack_cb(ctx: *mut core::ffi::c_ulonglong) -> i32 {
    let ptr: *mut u64 = *ctx.add(8) as *mut u64;

    arena_touch = arena_touch.wrapping_add(1);
    /* pin the slot layout: the leading args fill ctx[0]..ctx[7] */
    if *ctx.add(0) != 1 || *ctx.add(7) != 8 {
        return 0xbad;
    }
    *ptr = (*ptr).wrapping_add(1);
    return 0;
}

#[unsafe(link_section = "struct_ops/test_arena_multislot")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_arena_multislot_cb(ctx: *mut core::ffi::c_ulonglong) -> i32 {
    let ptr: *mut u64 = *ctx.add(2) as *mut u64;

    arena_touch = arena_touch.wrapping_add(1);
    /*
     * The 16-byte struct occupies ctx[0] and ctx[1], so @ptr is argument
     * one but slot two. Getting that wrong hands the callback a scalar.
     */
    if *ctx.add(0) != 11 || *ctx.add(1) != 22 {
        return 0xbad;
    }
    *ptr = (*ptr).wrapping_add(1);
    return 0;
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_arena: bpf_testmod_ops3 = bpf_testmod_ops3 {
    test_arena: test_arena_cb as *mut core::ffi::c_void,
    test_arena_nullable: test_arena_nullable_cb as *mut core::ffi::c_void,
    test_arena_stack: test_arena_stack_cb as *mut core::ffi::c_void,
    test_arena_multislot: test_arena_multislot_cb as *mut core::ffi::c_void,
};

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trigger(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    /* #if defined(__BPF_FEATURE_ADDR_SPACE_CAST) */
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        let mut val: *mut u64;
        let mut ret: i32;

        val = bpf_arena_alloc_pages(
            &raw mut arena,
            core::ptr::null_mut(),
            1,
            NUMA_NO_NODE,
            0,
        );
        if val.is_null() {
            return 1;
        }

        *val = 41;
        ret = bpf_testmod_ops3_call_test_arena(val as *mut u64);
        if ret != 0 {
            return 2;
        }
        if *val != 42 {
            return 3;
        }

        /*
         * The callback must have seen exactly (u32)(kaddr - kern_vm_start),
         * which is the arena offset of val with the upper 32 bits clear.
         */
        if cb_ptr_val != val as u64 as u32 as u64 {
            return 4;
        }

        ret = bpf_testmod_ops3_call_test_arena_nullable(val as *mut u64);
        if ret != 0 {
            return 5;
        }
        if *val != 43 {
            return 6;
        }

        /* NULL survives the nullable kfunc and the trampoline as NULL */
        ret = bpf_testmod_ops3_call_test_arena_nullable(core::ptr::null_mut());
        if ret != 0xbee {
            return 7;
        }

        /* the arena pointer is stack-passed into the trampoline here */
        ret = bpf_testmod_ops3_call_test_arena_stack(val as *mut u64);
        if ret != 0 {
            return 8;
        }
        if *val != 44 {
            return 9;
        }

        /* a multi-slot arg precedes the arena pointer here */
        ret = bpf_testmod_ops3_call_test_arena_multislot(val as *mut u64);
        if ret != 0 {
            return 10;
        }
        if *val != 45 {
            return 11;
        }

        bpf_arena_free_pages(&raw mut arena, val as *mut core::ffi::c_void, 1);
    }
    /* #endif */
    return 0;
}
