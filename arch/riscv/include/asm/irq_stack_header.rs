/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header.  The Linux include dependencies are supplied
// by the surrounding translation unit.

use core::ffi::{c_int, c_ulong, c_void};

// DECLARE_PER_CPU(ulong *, irq_stack_ptr);
extern "C" {
    pub static mut irq_stack_ptr: *mut c_ulong;

    pub fn call_on_irq_stack(
        regs: *mut crate::pt_regs,
        func: unsafe extern "C" fn(*mut crate::pt_regs),
    );
}

// CONFIG_VMAP_STACK
// To ensure that VMAP'd stack overflow detection works correctly, all VMAP'd
// stacks need to have the same alignment.
#[cfg(CONFIG_VMAP_STACK)]
#[inline]
pub unsafe fn arch_alloc_vmap_stack(
    stack_size: usize,
    node: c_int,
) -> *mut c_ulong {
    let p = __vmalloc_node(
        stack_size,
        THREAD_ALIGN,
        THREADINFO_GFP,
        node,
        __builtin_return_address(0),
    );
    kasan_reset_tag(p) as *mut c_ulong
}

#[cfg(CONFIG_VMAP_STACK)]
extern "C" {
    fn __vmalloc_node(
        size: usize,
        align: usize,
        gfp_mask: c_ulong,
        node: c_int,
        caller: *mut c_void,
    ) -> *mut c_void;
    fn kasan_reset_tag(ptr: *mut c_void) -> *mut c_void;
    fn __builtin_return_address(level: c_int) -> *mut c_void;
}

#[cfg(CONFIG_VMAP_STACK)]
extern "C" {
    static THREAD_ALIGN: usize;
    static THREADINFO_GFP: c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
