/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Stack depot - a stack trace storage that avoids duplication.
 *
 * Stack depot is intended to be used by subsystems that need to store and
 * later retrieve many potentially duplicated stack traces without wasting
 * memory.
 *
 * For example, KASAN needs to save allocation and free stack traces for each
 * object. Storing two stack traces per object requires a lot of memory (e.g.
 * SLUB_DEBUG needs 256 bytes per object for that). Since allocation and free
 * stack traces often repeat, using stack depot allows to save about 100x space.
 *
 * Author: Alexander Potapenko <glider@google.com>
 * Copyright (C) 2016 Google, Inc.
 *
 * Based on the code by Dmitry Chernenkov.
 */

/* Dependency supplied by the surrounding kernel translation. */

pub type depot_stack_handle_t = u32;

pub const STACK_DEPOT_EXTRA_BITS: usize = 5;
pub const DEPOT_HANDLE_BITS: usize = core::mem::size_of::<depot_stack_handle_t>() * 8;
pub const DEPOT_POOL_ORDER: usize = 2;
/* DEPOT_POOL_SIZE depends on the externally supplied PAGE_SHIFT. */
pub const DEPOT_STACK_ALIGN: usize = 4;
/* DEPOT_OFFSET_BITS and DEPOT_POOL_INDEX_BITS depend on PAGE_SHIFT. */

/* CONFIG_STACKDEPOT-controlled declarations are preserved below. */
#[cfg(CONFIG_STACKDEPOT)]
#[repr(C)]
pub union handle_parts {
    pub handle: depot_stack_handle_t,
    pub bits: u32,
}

#[cfg(CONFIG_STACKDEPOT)]
#[repr(C)]
pub struct stack_record {
    pub hash_list: list_head,
    pub hash: u32,
    pub size: u32,
    pub handle: handle_parts,
    pub count: refcount_t,
    pub entries_or_free: stack_record_entries,
}

#[cfg(CONFIG_STACKDEPOT)]
#[repr(C)]
pub union stack_record_entries {
    pub entries: [c_ulong; CONFIG_STACKDEPOT_MAX_FRAMES],
    pub freelist: stack_record_freelist,
}

#[cfg(CONFIG_STACKDEPOT)]
#[repr(C)]
pub struct stack_record_freelist {
    pub free_list: list_head,
    pub rcu_state: c_ulong,
}

pub type depot_flags_t = u32;
pub const STACK_DEPOT_FLAG_CAN_ALLOC: depot_flags_t = 0x0001;
pub const STACK_DEPOT_FLAG_GET: depot_flags_t = 0x0002;
pub const STACK_DEPOT_FLAGS_NUM: u32 = 2;
pub const STACK_DEPOT_FLAGS_MASK: depot_flags_t = ((1u32 << STACK_DEPOT_FLAGS_NUM) - 1);

#[cfg(CONFIG_STACKDEPOT)]
extern "C" {
    pub fn stack_depot_init() -> c_int;
    pub fn stack_depot_request_early_init();
    pub fn stack_depot_early_init() -> c_int;
}

#[cfg(not(CONFIG_STACKDEPOT))]
#[inline]
pub fn stack_depot_init() -> c_int { 0 }

#[cfg(not(CONFIG_STACKDEPOT))]
#[inline]
pub fn stack_depot_request_early_init() {}

#[cfg(not(CONFIG_STACKDEPOT))]
#[inline]
pub fn stack_depot_early_init() -> c_int { 0 }

extern "C" {
    pub fn stack_depot_save_flags(
        entries: *mut c_ulong,
        nr_entries: c_uint,
        alloc_flags: gfp_t,
        depot_flags: depot_flags_t,
    ) -> depot_stack_handle_t;
    pub fn stack_depot_save(
        entries: *mut c_ulong,
        nr_entries: c_uint,
        alloc_flags: gfp_t,
    ) -> depot_stack_handle_t;
    pub fn __stack_depot_get_stack_record(handle: depot_stack_handle_t) -> *mut stack_record;
    pub fn stack_depot_fetch(
        handle: depot_stack_handle_t,
        entries: *mut *mut c_ulong,
    ) -> c_uint;
    pub fn stack_depot_print(stack: depot_stack_handle_t);
    pub fn stack_depot_snprint(
        handle: depot_stack_handle_t,
        buf: *mut c_char,
        size: size_t,
        spaces: c_int,
    ) -> c_int;
    pub fn stack_depot_put(handle: depot_stack_handle_t);
    pub fn stack_depot_set_extra_bits(
        handle: depot_stack_handle_t,
        extra_bits: c_uint,
    ) -> depot_stack_handle_t;
    pub fn stack_depot_get_extra_bits(handle: depot_stack_handle_t) -> c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
