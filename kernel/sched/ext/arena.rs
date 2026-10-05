/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
 *
 * scx_arena_pool: kernel-side sub-allocator over BPF-arena pages.
 *
 * Each chunk added to @sch->arena_pool comes from one
 * bpf_arena_alloc_pages_sleepable() call and is registered at the
 * kernel-side mapping address.
 *
 * Allocations grow the pool on demand. Underlying arena pages are released
 * when the arena map itself is torn down.
 *
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2026 Tejun Heo <tj@kernel.org>
 */

/*
 * Source continuation at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
 * Native headers own layouts, configured page geometry and primitive ABI.
 * The native leaves are an explicit, unqualified runtime C boundary.
 */
#![no_std]

compile_error!("SOURCE ONLY HOLD: sched_ext arena is not admitted");

use core::ptr;
use kernel::bindings::sched_ext_arena_native::*;
use kernel::ffi::{c_int, c_ulong, c_void};

/* Algorithm-local values from enum scx_arena_consts in the pinned arena.c. */
const SCX_ARENA_MIN_ORDER: c_int = 3; /* 8-byte minimum sub-allocation */
const SCX_ARENA_GROW_PAGES: u32 = 4; /* per growth */
const PAGE_SIZE: usize = LUPOS_SCX_ARENA_PAGE_SIZE as usize;
const PAGE_SHIFT: u32 = LUPOS_SCX_ARENA_PAGE_SHIFT as u32;
const ENOMEM: c_int = LUPOS_SCX_ARENA_ENOMEM as c_int;
const EINVAL: c_int = LUPOS_SCX_ARENA_EINVAL as c_int;

/// Initialize the pool for a scheduler with an arena map.
///
/// # Safety
/// `sch` is a live native scheduler. Its arena map is null or is a stable,
/// referenced native arena map. Its pool slot is initially null, and the caller
/// serializes initialization with all pool users, as in the root/sub enable
/// path under scx_enable_mutex.
/// The caller is in sleepable allocation context and owns teardown on error.
#[no_mangle]
pub unsafe extern "C" fn scx_arena_pool_init(sch: *mut scx_sched) -> c_int {
    // SAFETY: The caller pins the scheduler/map and exclusively initializes
    // this slot. The returned pool is either null or one new native owner.
    unsafe {
        if !lupos_scx_arena_has_map(sch) {
            return 0;
        }

        let pool = lupos_scx_arena_pool_create(SCX_ARENA_MIN_ORDER);
        lupos_scx_arena_set_pool(sch, pool);
        if pool.is_null() {
            return -ENOMEM;
        }
        0
    }
}

/// Clear one retiring chunk's allocated ranges without freeing arena pages.
///
/// # Safety
/// Only the native gen_pool_for_each_chunk callback adapter may enter here.
/// `pool` and `chunk` are live, the chunk belongs to a pool initialized here,
/// and all external allocation/free operations and kernel users of those
/// allocations have drained. Native chunk geometry and its trailing bitmap
/// cover the registered range; the pool's allocation order remains three.
/// The native iterator holds rcu_read_lock; the callback must not sleep or
/// retain either pointer. It accesses allocator metadata only, and does not
/// acquire ownership of the arena-shared payload.
#[export_name = "lupos_scx_arena_clear_chunk"]
pub unsafe extern "C" fn scx_arena_clear_chunk(
    pool: *mut gen_pool,
    chunk: *mut gen_pool_chunk,
    _data: *mut c_void,
) {
    // SAFETY: Teardown excludes external bitmap writers and destruction; the
    // native RCU iterator borrows this live chunk. Native searches use
    // chunk->bits, the inline flexible array, never a pointer loaded from it.
    // The valid order and bounded native searches keep all shifts and e - b
    // within the registered chunk geometry. No arena payload is dereferenced.
    unsafe {
        let order = lupos_scx_arena_pool_order(pool) as u32;
        let start = lupos_scx_arena_chunk_start(chunk);
        let chunk_sz = lupos_scx_arena_chunk_end(chunk).wrapping_sub(start).wrapping_add(1);
        let end_bit = chunk_sz >> order;

        /* Preserve for_each_set_bitrange's [b, e) and next-search ordering. */
        let mut b: c_ulong = 0;
        loop {
            b = lupos_scx_arena_chunk_next_set(chunk, end_bit, b);
            let e = lupos_scx_arena_chunk_next_zero(chunk, end_bit, b.wrapping_add(1));
            if b >= end_bit {
                break;
            }
            lupos_scx_arena_pool_free(
                pool,
                start.wrapping_add(b << order),
                ((e - b) << order) as usize,
            );
            b = e.wrapping_add(1);
        }
    }
}

/*
 * Tear down the pool. Outstanding gen_pool allocations are freed via
 * scx_arena_clear_chunk() so gen_pool_destroy() doesn't BUG. The underlying
 * arena pages are released when the arena map itself is torn down.
 */
/// Retire the pool, leaving its backing pages owned by the arena map.
///
/// # Safety
/// `sch` remains live; any non-null map stays referenced until this returns.
/// Caller exclusively owns teardown: all alloc/free operations and kernel
/// users of pool allocations have drained, including deferred scheduler
/// callbacks and per-CPU scratch users. Independently shared arena payload
/// remains subject to the native BPF access/fault rules, not Rust exclusivity.
/// The pool is null or was initialized here and has not already been freed.
/// Run in the native scheduler's sleepable reclamation-work context.
#[no_mangle]
pub unsafe extern "C" fn scx_arena_pool_destroy(sch: *mut scx_sched) {
    // SAFETY: The caller has drained all other pool users/readers. The native
    // iterator's RCU read-side section ends before native metadata destruction,
    // which does not wait for a grace period. Its callback is C-typed.
    unsafe {
        let pool = lupos_scx_arena_pool(sch);
        if pool.is_null() {
            return;
        }
        lupos_scx_arena_for_each_chunk(pool);
        lupos_scx_arena_pool_destroy(pool);
        lupos_scx_arena_set_pool(sch, ptr::null_mut());
    }
}

/* Grow the pool by @page_cnt pages. This operation requires a sleepable context. */
/// # Safety
/// Caller satisfies scx_arena_alloc's scheduler/map lifetime and sleepability
/// contract; page_cnt is the u32 count computed by that allocation loop.
unsafe fn scx_arena_grow(sch: *mut scx_sched, page_cnt: u32) -> c_int {
    // SAFETY: Map, pool and rebasing base are pinned across the sleeping calls.
    // The freshly allocated BPF address is registered only after conversion to
    // kernel VA. Failure passes that exact BPF address/count, not the rebased VA,
    // to native non-sleepable rollback. It may defer cleanup or retain pages
    // until map destruction if native free-span allocation fails.
    unsafe {
        let pool = lupos_scx_arena_pool(sch);
        if !lupos_scx_arena_has_map(sch) || pool.is_null() {
            return -EINVAL;
        }

        let p = lupos_scx_arena_pages_alloc(sch, page_cnt);
        if p.is_null() {
            return -ENOMEM;
        }

        let ret = lupos_scx_arena_pool_add(
            pool,
            lupos_scx_arena_to_kaddr(sch, p) as c_ulong,
            (page_cnt as usize).wrapping_mul(PAGE_SIZE),
        );
        if ret != 0 {
            lupos_scx_arena_pages_free(sch, p, page_cnt);
            return ret;
        }
        0
    }
}

/*
 * Allocate @size bytes from the arena pool. Returns kernel VA on success, NULL
 * on failure. May grow the pool via scx_arena_grow() which sleeps. Caller must
 * be in a GFP_KERNEL context.
 */
/// Allocate size bytes, growing the native pool when necessary.
///
/// # Safety
/// `sch` is live. Its pool is null or was initialized here; for a present pool,
/// its arena map reference, pool and rebasing base remain stable through this
/// call and use of the result. Caller is in GFP_KERNEL context,
/// excludes pool initialization/destruction, and follows gen_pool's configured
/// context requirements. Returned memory has native 8-byte allocation alignment,
/// is not newly zeroed by this suballocator, and remains arena-shared memory:
/// this raw pointer does not grant Rust reference access. A map reference does
/// not prove payload contents or page residency. Kernel payload users must keep
/// the native arena access/fault protections and bounds checks, including the
/// explicit checks internal.h requires for accesses beyond GUARD_SZ / 2, and
/// stop using this allocation before free or scheduler/map teardown. Native
/// scratch recovery depends on finding the applicable BPF program on the stack;
/// rebasing does not make a payload access recoverable in arbitrary contexts.
/// This function neither dereferences payload nor validates later accesses.
#[no_mangle]
pub unsafe extern "C" fn scx_arena_alloc(sch: *mut scx_sched, size: usize) -> *mut c_void {
    // SAFETY: The lifetime/context contract holds across native sleeping page
    // allocation and gen_pool addition. No Rust reference into shared native
    // storage is formed; gen_pool retains its own allocation synchronization.
    unsafe {
        lupos_scx_arena_might_sleep();

        let pool = lupos_scx_arena_pool(sch);
        if pool.is_null() {
            return ptr::null_mut();
        }

        loop {
            let kern_va = lupos_scx_arena_pool_alloc(pool, size);
            if kern_va != 0 {
                return kern_va as *mut c_void;
            }
            let page_cnt = core::cmp::max(
                SCX_ARENA_GROW_PAGES,
                (size.wrapping_add(PAGE_SIZE).wrapping_sub(1) >> PAGE_SHIFT) as u32,
            );
            if scx_arena_grow(sch, page_cnt) != 0 {
                return ptr::null_mut();
            }
        }
    }
}

/// Return one native allocation to the pool without releasing backing pages.
///
/// # Safety
/// `sch` remains live and teardown is excluded; its pool is null or remains
/// live. If pool and kern_va are non-null, kern_va is the kernel VA returned by
/// this pool for this exact size, has not been freed, and all kernel users of
/// that suballocation have stopped accessing it. Returning allocator metadata
/// neither revokes BPF/user mappings nor grants ownership of their payload.
/// Caller meets gen_pool's configured context requirements. A BPF arena address
/// must not be passed in place of the kernel VA.
#[no_mangle]
pub unsafe extern "C" fn scx_arena_free(sch: *mut scx_sched, kern_va: *mut c_void, size: usize) {
    // SAFETY: The caller retains the pool and relinquishes precisely one live
    // allocation. gen_pool rounds size and updates its own bitmap atomically.
    unsafe {
        let pool = lupos_scx_arena_pool(sch);
        if !pool.is_null() && !kern_va.is_null() {
            lupos_scx_arena_pool_free(pool, kern_va as c_ulong, size);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
