// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/* BPF_NO_KFUNC_PROTOTYPES */
/* Includes translated as external dependencies:
 * vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h, bpf_misc.h,
 * bpf_experimental.h, bpf_arena_common.h
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type u64 = u64;
type __u8 = u8;

const ARENA_SIZE: u64 = 1u64 << 32;
const PAGE_CNT: i32 = 100;

/* External constants/macros supplied by BPF headers. */
unsafe extern "C" {
    static PAGE_SIZE: usize;
    static __PAGE_SIZE: usize;
    static NUMA_NO_NODE: i32;
    static can_loop: bool;
}

/* External helpers supplied by BPF headers. The C __arena address space is
 * represented with raw pointers in this file-local Rust translation.
 */
unsafe extern "C" {
    fn arena_base(arena: *mut ArenaMap) -> *mut u8;
    fn bpf_arena_alloc_pages(
        arena: *mut ArenaMap,
        addr: *mut core::ffi::c_void,
        page_cnt: usize,
        node: i32,
        flags: u64,
    ) -> *mut u8;
    fn bpf_arena_reserve_pages(
        arena: *mut ArenaMap,
        addr: *mut u8,
        page_cnt: usize,
    ) -> i32;
    fn bpf_arena_free_pages(
        arena: *mut ArenaMap,
        addr: *mut core::ffi::c_void,
        page_cnt: usize,
    );
    fn barrier();
}

#[repr(C)]
pub struct ArenaMap {
    /* __uint(type, BPF_MAP_TYPE_ARENA);
     * __uint(map_flags, BPF_F_MMAPABLE);
     * __uint(max_entries, ARENA_SIZE / PAGE_SIZE);
     */
    _unused: u32,
}

#[unsafe(link_section = ".maps")]
static mut arena: ArenaMap = ArenaMap { _unused: 0 };

#[unsafe(link_section = "syscall")]
/* __success __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn big_alloc1(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    /* #if defined(__BPF_FEATURE_ADDR_SPACE_CAST) */
    let mut page1: *mut u8;
    let mut page2: *mut u8;
    let mut no_page: *mut u8;
    let mut page3: *mut u8;
    let base: u64;

    base = unsafe { arena_base(&raw mut arena) as u64 };

    page1 = unsafe { bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) };
    if page1.is_null() {
        return 1;
    }

    if page1 as u64 != base {
        return 15;
    }

    unsafe { core::ptr::write_volatile(page1, 1) };
    page2 = unsafe {
        bpf_arena_alloc_pages(
            &raw mut arena,
            (ARENA_SIZE - 2 * PAGE_SIZE as u64) as *mut core::ffi::c_void,
            1,
            NUMA_NO_NODE,
            0,
        )
    };
    if page2.is_null() {
        return 2;
    }
    unsafe { core::ptr::write_volatile(page2, 2) };

    /* Test for the guard region at the end of the arena. */
    no_page = unsafe {
        bpf_arena_alloc_pages(
            &raw mut arena,
            (ARENA_SIZE - PAGE_SIZE as u64) as *mut core::ffi::c_void,
            1,
            NUMA_NO_NODE,
            0,
        )
    };
    if !no_page.is_null() {
        return 16;
    }

    no_page = unsafe {
        bpf_arena_alloc_pages(
            &raw mut arena,
            ARENA_SIZE as *mut core::ffi::c_void,
            1,
            NUMA_NO_NODE,
            0,
        )
    };
    if !no_page.is_null() {
        return 3;
    }
    if unsafe { core::ptr::read_volatile(page1) } != 1 {
        return 4;
    }
    if unsafe { core::ptr::read_volatile(page2) } != 2 {
        return 5;
    }
    unsafe { bpf_arena_free_pages(&raw mut arena, page1 as *mut core::ffi::c_void, 1) };
    if unsafe { core::ptr::read_volatile(page2) } != 2 {
        return 6;
    }
    if unsafe { core::ptr::read_volatile(page1) } != 0 {
        /* use-after-free should return 0 */
        return 7;
    }
    page3 = unsafe { bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) };
    if page3.is_null() {
        return 8;
    }
    unsafe { core::ptr::write_volatile(page3, 3) };
    if page1 != page3 {
        return 9;
    }
    if unsafe { core::ptr::read_volatile(page2) } != 2 {
        return 10;
    }
    if unsafe { core::ptr::read_volatile(page1.add(PAGE_SIZE)) } != 0 {
        return 11;
    }
    if unsafe { core::ptr::read_volatile(page1.sub(PAGE_SIZE)) } != 0 {
        return 12;
    }
    if unsafe { core::ptr::read_volatile(page2.add(PAGE_SIZE)) } != 0 {
        return 13;
    }
    if unsafe { core::ptr::read_volatile(page2.sub(PAGE_SIZE)) } != 0 {
        return 14;
    }
    /* #endif */
    0
}

/* Try to access a reserved page. Behavior should be identical with accessing unallocated pages. */
#[unsafe(link_section = "syscall")]
/* __success __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn access_reserved(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    /* #if defined(__BPF_FEATURE_ADDR_SPACE_CAST) */
    let mut page: *mut u8;
    let base: *mut u8;
    let len: usize = 4;
    let mut ret: i32;
    let mut i: i32;

    /* Get a separate region of the arena. */
    base = unsafe { arena_base(&raw mut arena).add(16384 * PAGE_SIZE) };
    page = base;

    ret = unsafe { bpf_arena_reserve_pages(&raw mut arena, base, len) };
    if ret != 0 {
        return 1;
    }

    /* Try to dirty reserved memory. */
    i = 0;
    while (i as usize) < len && unsafe { can_loop } {
        unsafe { core::ptr::write_volatile(page, 0x5a) };
        i += 1;
    }

    i = 0;
    while (i as usize) < len && unsafe { can_loop } {
        page = unsafe { base.add(i as usize * PAGE_SIZE) };

        /*
         * Error out in case either the write went through,
         * or the address has random garbage.
         */
        if unsafe { core::ptr::read_volatile(page) } == 0x5a {
            return 2 + 2 * i;
        }

        if unsafe { core::ptr::read_volatile(page) } != 0 {
            return 2 + 2 * i + 1;
        }
        i += 1;
    }
    /* #endif */
    0
}

/* Try to allocate a region overlapping with a reservation. */
#[unsafe(link_section = "syscall")]
/* __success __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn request_partially_reserved(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    /* #if defined(__BPF_FEATURE_ADDR_SPACE_CAST) */
    let mut page: *mut u8;
    let base: *mut u8;
    let mut ret: i32;

    /* Add an arbitrary page offset. */
    base = unsafe { arena_base(&raw mut arena).add(4096 * __PAGE_SIZE) };
    page = base;

    ret = unsafe { bpf_arena_reserve_pages(&raw mut arena, base.add(3 * __PAGE_SIZE), 4) };
    if ret != 0 {
        return 1;
    }

    page = unsafe { bpf_arena_alloc_pages(&raw mut arena, base as *mut core::ffi::c_void, 5, NUMA_NO_NODE, 0) };
    if page as u64 != 0u64 {
        return 2;
    }
    /* #endif */
    0
}

#[unsafe(link_section = "syscall")]
/* __success __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_reserved(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    /* #if defined(__BPF_FEATURE_ADDR_SPACE_CAST) */
    let addr: *mut u8;
    let mut page: *mut u8;
    let mut ret: i32;

    /* Add an arbitrary page offset. */
    addr = unsafe { arena_base(&raw mut arena).add(32768 * __PAGE_SIZE) };

    page = unsafe { bpf_arena_alloc_pages(&raw mut arena, addr as *mut core::ffi::c_void, 2, NUMA_NO_NODE, 0) };
    if page.is_null() {
        return 1;
    }

    ret = unsafe { bpf_arena_reserve_pages(&raw mut arena, addr.add(2 * __PAGE_SIZE), 2) };
    if ret != 0 {
        return 2;
    }

    /*
     * Reserved and allocated pages should be interchangeable for
     * bpf_arena_free_pages(). Free a reserved and an allocated
     * page with a single call.
     */
    unsafe { bpf_arena_free_pages(&raw mut arena, addr.add(__PAGE_SIZE) as *mut core::ffi::c_void, 2) };

    /* The free call above should have succeeded, so this allocation should too. */
    page = unsafe { bpf_arena_alloc_pages(&raw mut arena, addr.add(__PAGE_SIZE) as *mut core::ffi::c_void, 2, NUMA_NO_NODE, 0) };
    if page.is_null() {
        return 3;
    }
    /* #endif */
    0
}

/* #if defined(__BPF_FEATURE_ADDR_SPACE_CAST) */
static mut page: [*mut __u8; PAGE_CNT as usize] = [core::ptr::null_mut(); PAGE_CNT as usize]; /* occupies the first page */
static mut base: *mut __u8 = core::ptr::null_mut();

/*
 * Check that arena's range_tree algorithm allocates pages sequentially
 * on the first pass and then fills in all gaps on the second pass.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alloc_pages(
    page_cnt: i32,
    pages_atonce: i32,
    first_pass: bool,
    max_idx: i32,
    step: i32,
) -> i32 {
    let mut pg: *mut __u8;
    let mut i: i32;
    let mut pg_idx: i32;

    i = 0;
    while i < page_cnt {
        pg = unsafe {
            bpf_arena_alloc_pages(
                &raw mut arena,
                core::ptr::null_mut(),
                pages_atonce as usize,
                NUMA_NO_NODE,
                0,
            )
        };
        if pg.is_null() {
            return step;
        }
        pg_idx = unsafe { pg.offset_from(base) as usize / PAGE_SIZE } as i32;
        if first_pass {
            /* Pages must be allocated sequentially */
            if pg_idx != i {
                return step + 100;
            }
        } else {
            /* Allocator must fill into gaps */
            if pg_idx >= max_idx || (pg_idx & 1) != 0 {
                return step + 200;
            }
        }
        unsafe { core::ptr::write_volatile(pg, pg_idx as __u8) };
        unsafe { page[pg_idx as usize] = pg };
        /* cond_break */
        if unsafe { !can_loop } {
            break;
        }
        i += 1;
    }
    0
}

#[unsafe(link_section = "syscall")]
/* __success __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn big_alloc2(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    let mut pg: *mut __u8;
    let mut i: i32;
    let mut err: i32;

    unsafe {
        base = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    }
    if unsafe { base.is_null() } {
        return 1;
    }
    unsafe { bpf_arena_free_pages(&raw mut arena, base as *mut core::ffi::c_void, 1) };

    err = unsafe { alloc_pages(PAGE_CNT, 1, true, PAGE_CNT, 2) };
    if err != 0 {
        return err;
    }

    /* Clear all even pages */
    i = 0;
    while i < PAGE_CNT {
        pg = unsafe { page[i as usize] };
        if unsafe { core::ptr::read_volatile(pg) } as i32 != i {
            return 3;
        }
        unsafe { bpf_arena_free_pages(&raw mut arena, pg as *mut core::ffi::c_void, 1) };
        unsafe { page[i as usize] = core::ptr::null_mut() };
        /* cond_break */
        if unsafe { !can_loop } {
            break;
        }
        i += 2;
    }

    /* Allocate into freed gaps */
    err = unsafe { alloc_pages(PAGE_CNT / 2, 1, false, PAGE_CNT, 4) };
    if err != 0 {
        return err;
    }

    /* Free pairs of pages */
    i = 0;
    while i < PAGE_CNT {
        pg = unsafe { page[i as usize] };
        if unsafe { core::ptr::read_volatile(pg) } as i32 != i {
            return 5;
        }
        unsafe { bpf_arena_free_pages(&raw mut arena, pg as *mut core::ffi::c_void, 2) };
        unsafe { page[i as usize] = core::ptr::null_mut() };
        unsafe { barrier() };
        unsafe { page[(i + 1) as usize] = core::ptr::null_mut() };
        /* cond_break */
        if unsafe { !can_loop } {
            break;
        }
        i += 4;
    }

    /* Allocate 2 pages at a time into freed gaps */
    err = unsafe { alloc_pages(PAGE_CNT / 4, 2, false, PAGE_CNT, 6) };
    if err != 0 {
        return err;
    }

    /* Check pages without freeing */
    i = 0;
    while i < PAGE_CNT {
        pg = unsafe { page[i as usize] };
        if unsafe { core::ptr::read_volatile(pg) } as i32 != i {
            return 7;
        }
        /* cond_break */
        if unsafe { !can_loop } {
            break;
        }
        i += 2;
    }

    pg = unsafe { bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) };

    if pg.is_null() {
        return 8;
    }
    /*
     * The first PAGE_CNT pages are occupied. The new page
     * must be above.
     */
    if unsafe { pg.offset_from(base) as usize / PAGE_SIZE } < PAGE_CNT as usize {
        return 9;
    }
    0
}

#[unsafe(link_section = "socket")]
/* __success __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn big_alloc3(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    /* #if defined(__BPF_FEATURE_ADDR_SPACE_CAST) */
    let pages: *mut u8;
    let mut i: u64;

    /*
     * Allocate 2051 pages in one go to check how kmalloc_nolock() handles large requests.
     * Since kmalloc_nolock() can allocate up to 1024 struct page * at a time, this call should
     * result in three batches: two batches of 1024 pages each, followed by a final batch of 3
     * pages.
     */
    pages = unsafe { bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 2051, NUMA_NO_NODE, 0) };
    if pages.is_null() {
        return 0;
    }

    i = 0;
    while i < 2051 {
        unsafe { *pages.add(i as usize * PAGE_SIZE) = 123 };
        i += 1;
    }
    i = 0;
    while i < 2051 {
        if unsafe { *pages.add(i as usize * PAGE_SIZE) } != 123 {
            return i as i32;
        }
        i += 1;
    }

    unsafe { bpf_arena_free_pages(&raw mut arena, pages as *mut core::ffi::c_void, 2051) };
    /* #endif */
    0
}
/* #endif */

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
