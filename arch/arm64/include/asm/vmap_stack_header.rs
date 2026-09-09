/* SPDX-License-Identifier: GPL-2.0 */
// Copyright (C) 2017 Arm Ltd.

// The C header includes linux/gfp.h, linux/vmalloc.h, linux/pgtable.h,
// asm/memory.h, and asm/thread_info.h. Their declarations and constants are
// supplied by the surrounding Rust translation.

/*
 * To ensure that VMAP'd stack overflow detection works correctly, all VMAP'd
 * stacks need to have the same alignment.
 */
#[inline]
pub unsafe fn arch_alloc_vmap_stack(
    stack_size: usize,
    node: i32,
) -> *mut core::ffi::c_ulong {
    let p: *mut core::ffi::c_void;

    p = __vmalloc_node(
        stack_size,
        THREAD_ALIGN,
        THREADINFO_GFP,
        node,
        __builtin_return_address(0),
    );
    kasan_reset_tag(p) as *mut core::ffi::c_ulong
}

extern "C" {
    fn __vmalloc_node(
        size: usize,
        align: usize,
        gfp_mask: usize,
        node: i32,
        caller: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;

    fn kasan_reset_tag(ptr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;

    // C builtin retained as an external declaration for source-level parity.
    fn __builtin_return_address(level: u32) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
