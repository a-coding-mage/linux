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

/* Dependencies supplied by the surrounding kernel translation. */
extern "C" {
    fn gen_pool_create(min_alloc_order: i32, nid: i32) -> *mut gen_pool;
    fn gen_pool_destroy(pool: *mut gen_pool);
    fn gen_pool_add(pool: *mut gen_pool, addr: usize, size: usize, nid: i32) -> i32;
    fn gen_pool_alloc(pool: *mut gen_pool, size: usize) -> usize;
    fn gen_pool_free(pool: *mut gen_pool, addr: usize, size: usize);
    fn gen_pool_for_each_chunk(
        pool: *mut gen_pool,
        func: unsafe fn(*mut gen_pool, *mut gen_pool_chunk, *mut core::ffi::c_void),
        data: *mut core::ffi::c_void,
    );
    fn bpf_arena_alloc_pages_sleepable(
        arena: *mut core::ffi::c_void,
        numa_node: *mut core::ffi::c_void,
        page_cnt: u32,
        nid: i32,
        flags: u64,
    ) -> *mut core::ffi::c_void;
    fn bpf_arena_free_pages_non_sleepable(
        arena: *mut core::ffi::c_void,
        p: *mut core::ffi::c_void,
        page_cnt: u32,
    );
    fn scx_arena_to_kaddr(
        sch: *mut scx_sched,
        p: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn might_sleep();
}

#[repr(C)]
pub struct gen_pool {
    pub min_alloc_order: i32,
}

#[repr(C)]
pub struct gen_pool_chunk {
    pub end_addr: usize,
    pub start_addr: usize,
    pub bits: *mut usize,
}

#[repr(C)]
pub struct scx_sched {
    pub arena_map: *mut core::ffi::c_void,
    pub arena_pool: *mut gen_pool,
}

const SCX_ARENA_MIN_ORDER: i32 = 3; /* 8-byte minimum sub-allocation */
const SCX_ARENA_GROW_PAGES: u32 = 4; /* per growth */
const NUMA_NO_NODE: i32 = -1;
const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: usize = 12;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;

pub unsafe fn scx_arena_pool_init(sch: *mut scx_sched) -> i32 {
    if (*sch).arena_map.is_null() {
        return 0;
    }

    (*sch).arena_pool = gen_pool_create(SCX_ARENA_MIN_ORDER, NUMA_NO_NODE);
    if (*sch).arena_pool.is_null() {
        return -ENOMEM;
    }
    0
}

unsafe fn scx_arena_clear_chunk(pool: *mut gen_pool, chunk: *mut gen_pool_chunk, _data: *mut core::ffi::c_void) {
    let order = (*pool).min_alloc_order as usize;
    let chunk_sz = (*chunk).end_addr - (*chunk).start_addr + 1;
    let end_bit = chunk_sz >> order;

    /* Equivalent to for_each_set_bitrange(b, e, chunk->bits, end_bit). */
    let mut b = 0usize;
    while b < end_bit {
        let mut e = b;
        while e < end_bit && ((*(*chunk).bits.add(e / usize::BITS as usize)
            >> (e % usize::BITS as usize)) & 1) != 0 {
            e += 1;
        }
        if e > b {
            gen_pool_free(pool, (*chunk).start_addr + (b << order), (e - b) << order);
        }
        b = e + 1;
    }
}

/*
 * Tear down the pool. Outstanding gen_pool allocations are freed via
 * scx_arena_clear_chunk() so gen_pool_destroy() doesn't BUG. The underlying
 * arena pages are released when the arena map itself is torn down.
 */
pub unsafe fn scx_arena_pool_destroy(sch: *mut scx_sched) {
    if (*sch).arena_pool.is_null() {
        return;
    }
    gen_pool_for_each_chunk(
        (*sch).arena_pool,
        scx_arena_clear_chunk,
        core::ptr::null_mut(),
    );
    gen_pool_destroy((*sch).arena_pool);
    (*sch).arena_pool = core::ptr::null_mut();
}

/* Grow the pool by @page_cnt pages. This operation requires a sleepable context. */
unsafe fn scx_arena_grow(sch: *mut scx_sched, page_cnt: u32) -> i32 {
    if (*sch).arena_map.is_null() || (*sch).arena_pool.is_null() {
        return -EINVAL;
    }

    let p = bpf_arena_alloc_pages_sleepable(
        (*sch).arena_map, core::ptr::null_mut(), page_cnt, NUMA_NO_NODE, 0,
    );
    if p.is_null() {
        return -ENOMEM;
    }

    let ret = gen_pool_add(
        (*sch).arena_pool,
        scx_arena_to_kaddr(sch, p) as usize,
        page_cnt as usize * PAGE_SIZE,
        NUMA_NO_NODE,
    );
    if ret != 0 {
        bpf_arena_free_pages_non_sleepable((*sch).arena_map, p, page_cnt);
        return ret;
    }
    0
}

/*
 * Allocate @size bytes from the arena pool. Returns kernel VA on success, NULL
 * on failure. May grow the pool via scx_arena_grow() which sleeps. Caller must
 * be in a GFP_KERNEL context.
 */
pub unsafe fn scx_arena_alloc(sch: *mut scx_sched, size: usize) -> *mut core::ffi::c_void {
    might_sleep();

    if (*sch).arena_pool.is_null() {
        return core::ptr::null_mut();
    }

    loop {
        let kern_va = gen_pool_alloc((*sch).arena_pool, size);
        if kern_va != 0 {
            return kern_va as *mut core::ffi::c_void;
        }
        let page_cnt = core::cmp::max(
            SCX_ARENA_GROW_PAGES,
            ((size + PAGE_SIZE - 1) >> PAGE_SHIFT) as u32,
        );
        if scx_arena_grow(sch, page_cnt) != 0 {
            return core::ptr::null_mut();
        }
    }
}

pub unsafe fn scx_arena_free(sch: *mut scx_sched, kern_va: *mut core::ffi::c_void, size: usize) {
    if !(*sch).arena_pool.is_null() && !kern_va.is_null() {
        gen_pool_free((*sch).arena_pool, kern_va as usize, size);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
