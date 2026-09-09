/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Functions used by the KMSAN runtime.
 *
 * Copyright (C) 2017-2022 Google LLC
 * Author: Alexander Potapenko <glider@google.com>
 */

// C header dependencies are supplied by the surrounding kernel translation.

pub const KMSAN_ALLOCA_MAGIC_ORIGIN: u32 = 0xabcd0100;
pub const KMSAN_CHAIN_MAGIC_ORIGIN: u32 = 0xabcd0200;

pub const KMSAN_POISON_NOCHECK: u32 = 0x0;
pub const KMSAN_POISON_CHECK: u32 = 0x1;
pub const KMSAN_POISON_FREE: u32 = 0x2;

pub const KMSAN_ORIGIN_SIZE: u32 = 4;
pub const KMSAN_MAX_ORIGIN_DEPTH: u32 = 7;

pub const KMSAN_STACK_DEPTH: u32 = 64;

pub const KMSAN_META_SHADOW: bool = false;
pub const KMSAN_META_ORIGIN: bool = true;

/* A pair of metadata pointers to be returned by the instrumentation functions. */
#[repr(C)]
pub struct shadow_origin_ptr {
    pub shadow: *mut core::ffi::c_void,
    pub origin: *mut core::ffi::c_void,
}

extern "C" {
    pub fn kmsan_get_shadow_origin_ptr(
        addr: *mut core::ffi::c_void,
        size: u64,
        store: bool,
    ) -> shadow_origin_ptr;
    pub fn kmsan_init_alloc_meta_for_range(
        start: *mut core::ffi::c_void,
        end: *mut core::ffi::c_void,
    );

    pub fn kmsan_print_origin(origin: depot_stack_handle_t);

    pub fn kmsan_report(
        origin: depot_stack_handle_t,
        address: *mut core::ffi::c_void,
        size: i32,
        off_first: i32,
        off_last: i32,
        user_addr: *const core::ffi::c_void,
        reason: kmsan_bug_reason,
    );

    pub fn kmsan_save_stack_with_flags(
        flags: gfp_t,
        extra_bits: u32,
    ) -> depot_stack_handle_t;

    pub fn kmsan_internal_memmove_metadata(
        dst: *mut core::ffi::c_void,
        src: *mut core::ffi::c_void,
        n: usize,
    );
    pub fn kmsan_internal_poison_memory(
        address: *mut core::ffi::c_void,
        size: usize,
        flags: gfp_t,
        poison_flags: u32,
    );
    pub fn kmsan_internal_unpoison_memory(
        address: *mut core::ffi::c_void,
        size: usize,
        checked: bool,
    );
    pub fn kmsan_internal_set_shadow_origin(
        address: *mut core::ffi::c_void,
        size: usize,
        b: i32,
        origin: u32,
        checked: bool,
    );
    pub fn kmsan_internal_chain_origin(id: depot_stack_handle_t) -> depot_stack_handle_t;

    pub fn kmsan_internal_task_create(task: *mut task_struct);

    pub fn kmsan_metadata_is_contiguous(addr: *mut core::ffi::c_void, size: usize) -> bool;
    pub fn kmsan_internal_check_memory(
        addr: *mut core::ffi::c_void,
        size: usize,
        user_addr: *const core::ffi::c_void,
        reason: i32,
    );

    pub fn kmsan_vmalloc_to_page_or_null(vaddr: *mut core::ffi::c_void) -> *mut page;
    pub fn kmsan_setup_meta(
        page: *mut page,
        shadow: *mut page,
        origin: *mut page,
        order: i32,
    );
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kmsan_bug_reason {
    REASON_ANY,
    REASON_COPY_TO_USER,
    REASON_SUBMIT_URB,
}

#[inline(always)]
pub unsafe fn kmsan_get_context() -> *mut kmsan_ctx {
    if in_task() {
        &mut (*current).kmsan_ctx
    } else {
        raw_cpu_ptr(&kmsan_percpu_ctx)
    }
}

#[inline(always)]
pub unsafe fn kmsan_in_runtime() -> bool {
    if (hardirq_count() >> HARDIRQ_SHIFT) > 1 {
        return true;
    }
    if in_nmi() {
        return true;
    }
    (*kmsan_get_context()).kmsan_in_runtime
}

#[inline(always)]
pub unsafe fn kmsan_enter_runtime() {
    let ctx: *mut kmsan_ctx = kmsan_get_context();
    KMSAN_WARN_ON({
        (*ctx).kmsan_in_runtime += 1;
        (*ctx).kmsan_in_runtime - 1
    });
}

#[inline(always)]
pub unsafe fn kmsan_leave_runtime() {
    let ctx: *mut kmsan_ctx = kmsan_get_context();
    (*ctx).kmsan_in_runtime -= 1;
    KMSAN_WARN_ON((*ctx).kmsan_in_runtime);
}

#[inline(always)]
pub const fn kmsan_extra_bits(depth: u32, uaf: bool) -> u32 {
    (depth << 1) | (uaf as u32)
}

#[inline(always)]
pub const fn kmsan_uaf_from_eb(extra_bits: u32) -> bool {
    (extra_bits & 1) != 0
}

#[inline(always)]
pub const fn kmsan_depth_from_eb(extra_bits: u32) -> u32 {
    extra_bits >> 1
}

#[inline]
pub unsafe fn kmsan_internal_is_module_addr(vaddr: *mut core::ffi::c_void) -> bool {
    (vaddr as u64 >= MODULES_VADDR) && (vaddr as u64 < MODULES_END)
}

#[inline]
pub unsafe fn kmsan_internal_is_vmalloc_addr(addr: *mut core::ffi::c_void) -> bool {
    (addr as u64 >= VMALLOC_START) && (addr as u64 < VMALLOC_END)
}

// External kernel declarations referenced by this header.
extern "C" {
    pub static mut kmsan_percpu_ctx: kmsan_ctx;
    pub static mut current: *mut task_struct;
    pub fn in_task() -> bool;
    pub fn raw_cpu_ptr(ptr: *mut kmsan_ctx) -> *mut kmsan_ctx;
    pub fn hardirq_count() -> u32;
    pub fn in_nmi() -> bool;
    pub fn KMSAN_WARN_ON(value: u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
