// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains common tag-based KASAN code.
 *
 * Copyright (c) 2018 Google, Inc.
 * Copyright (c) 2020 Google, Inc.
 */

// Kernel headers and local headers from the C translation unit are supplied by
// the surrounding Rust translation unit/dependencies.

pub const KASAN_STACK_RING_SIZE_DEFAULT: usize = 32 << 10;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kasan_arg_stacktrace {
    KASAN_ARG_STACKTRACE_DEFAULT,
    KASAN_ARG_STACKTRACE_OFF,
    KASAN_ARG_STACKTRACE_ON,
}

static mut kasan_arg_stacktrace: kasan_arg_stacktrace =
    kasan_arg_stacktrace::KASAN_ARG_STACKTRACE_DEFAULT;

/* Whether to collect alloc/free stack traces. */
pub static mut kasan_flag_stacktrace: static_key_true = static_key_true {};

/* Non-zero, as initial pointer values are 0. */
pub const STACK_RING_BUSY_PTR: *mut core::ffi::c_void = 1usize as *mut core::ffi::c_void;

pub static mut stack_ring: kasan_stack_ring = kasan_stack_ring {
    lock: RW_LOCK_UNLOCKED,
    ..unsafe { core::mem::zeroed() }
};

/* kasan.stacktrace=off/on */
fn early_kasan_flag_stacktrace(arg: *mut core::ffi::c_char) -> i32 {
    unsafe {
        if arg.is_null() {
            return -EINVAL;
        }

        if strcmp(arg, b"off\0".as_ptr() as *const core::ffi::c_char) == 0 {
            kasan_arg_stacktrace = kasan_arg_stacktrace::KASAN_ARG_STACKTRACE_OFF;
        } else if strcmp(arg, b"on\0".as_ptr() as *const core::ffi::c_char) == 0 {
            kasan_arg_stacktrace = kasan_arg_stacktrace::KASAN_ARG_STACKTRACE_ON;
        } else {
            return -EINVAL;
        }

        0
    }
}
// early_param("kasan.stacktrace", early_kasan_flag_stacktrace);

/* kasan.stack_ring_size=<number of entries> */
fn early_kasan_flag_stack_ring_size(arg: *mut core::ffi::c_char) -> i32 {
    unsafe {
        if arg.is_null() {
            return -EINVAL;
        }

        kstrtoul(arg, 0, &mut (*core::ptr::addr_of_mut!(stack_ring)).size)
    }
}
// early_param("kasan.stack_ring_size", early_kasan_flag_stack_ring_size);

pub unsafe fn kasan_init_tags() {
    match kasan_arg_stacktrace {
        kasan_arg_stacktrace::KASAN_ARG_STACKTRACE_DEFAULT => {
            /* Default is specified by kasan_flag_stacktrace definition. */
        }
        kasan_arg_stacktrace::KASAN_ARG_STACKTRACE_OFF => {
            static_branch_disable(&mut kasan_flag_stacktrace);
        }
        kasan_arg_stacktrace::KASAN_ARG_STACKTRACE_ON => {
            static_branch_enable(&mut kasan_flag_stacktrace);
        }
    }

    if kasan_stack_collection_enabled() {
        if stack_ring.size == 0 {
            stack_ring.size = KASAN_STACK_RING_SIZE_DEFAULT;
        }
        stack_ring.entries = memblock_alloc(
            core::mem::size_of_val(&(*stack_ring.entries)) * stack_ring.size,
            SMP_CACHE_BYTES,
        ) as *mut kasan_stack_ring_entry;
        if WARN_ON(stack_ring.entries.is_null()) {
            static_branch_disable(&mut kasan_flag_stacktrace);
        }
    }
}

unsafe fn save_stack_info(
    cache: *mut kmem_cache,
    object: *mut core::ffi::c_void,
    gfp_flags: gfp_t,
    is_free: bool,
) {
    let mut flags: unsigned_long = 0;
    let stack: depot_stack_handle_t;
    let old_stack: depot_stack_handle_t;
    let mut pos: u64;
    let entry: *mut kasan_stack_ring_entry;
    let mut old_ptr: *mut core::ffi::c_void;

    stack = kasan_save_stack(gfp_flags, STACK_DEPOT_FLAG_CAN_ALLOC | STACK_DEPOT_FLAG_GET);

    /*
     * Prevent save_stack_info() from modifying stack ring
     * when kasan_complete_mode_report_info() is walking it.
     */
    read_lock_irqsave(&mut stack_ring.lock, &mut flags);

    loop {
        pos = atomic64_fetch_add(1, &mut stack_ring.pos);
        entry = stack_ring.entries.add((pos % stack_ring.size) as usize);

        /* Detect stack ring entry slots that are being written to. */
        old_ptr = READ_ONCE((*entry).ptr);
        if old_ptr == STACK_RING_BUSY_PTR {
            continue;
        }
        if !try_cmpxchg(&mut (*entry).ptr, &mut old_ptr, STACK_RING_BUSY_PTR) {
            continue;
        }
        break;
    }

    old_stack = (*entry).track.stack;

    (*entry).size = (*cache).object_size;
    kasan_set_track(&mut (*entry).track, stack);
    (*entry).is_free = is_free;

    (*entry).ptr = object;

    read_unlock_irqrestore(&mut stack_ring.lock, flags);

    if old_stack != 0 {
        stack_depot_put(old_stack);
    }
}

pub unsafe fn kasan_save_alloc_info(cache: *mut kmem_cache, object: *mut core::ffi::c_void, flags: gfp_t) {
    save_stack_info(cache, object, flags, false);
}

pub unsafe fn kasan_save_free_info(cache: *mut kmem_cache, object: *mut core::ffi::c_void) {
    save_stack_info(cache, object, 0, true);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
