// SPDX-License-Identifier: GPL-2.0
/*
 * KMSAN initialization routines.
 *
 * Copyright (C) 2017-2021 Google LLC
 * Author: Alexander Potapenko <glider@google.com>
 *
 */

// Dependencies supplied by the surrounding kernel translation unit:
// kmsan.h, asm/sections.h, linux/mm.h, linux/memblock.h, and ../page_alloc.h.

const NUM_FUTURE_RANGES: usize = 128;

#[repr(C)]
struct start_end_pair {
    start: u64,
    end: u64,
}

static mut start_end_pairs: [start_end_pair; NUM_FUTURE_RANGES] =
    [start_end_pair { start: 0, end: 0 }; NUM_FUTURE_RANGES];
static mut future_index: i32 = 0;

/*
 * Record a range of memory for which the metadata pages will be created once
 * the page allocator becomes available.
 */
unsafe fn kmsan_record_future_shadow_range(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) {
    let mut nstart = start as u64;
    let mut nend = end as u64;
    let mut cstart: u64;
    let mut cend: u64;
    let mut merged = false;

    KMSAN_WARN_ON(future_index == NUM_FUTURE_RANGES as i32);
    KMSAN_WARN_ON((nstart >= nend) || (!IS_ENABLED(CONFIG_S390) && nstart == 0) || nend == 0);
    nstart = ALIGN_DOWN(nstart, PAGE_SIZE);
    nend = ALIGN(nend, PAGE_SIZE);

    /*
     * Scan the existing ranges to see if any of them overlaps with
     * [start, end). In that case, merge the two ranges instead of
     * creating a new one.
     * The number of ranges is less than 20, so there is no need to organize
     * them into a more intelligent data structure.
     */
    for i in 0..future_index {
        cstart = start_end_pairs[i as usize].start;
        cend = start_end_pairs[i as usize].end;
        if (cstart < nstart && cend < nstart) || (cstart > nend && cend > nend) {
            /* ranges are disjoint - do not merge */
            continue;
        }
        start_end_pairs[i as usize].start = min(nstart, cstart);
        start_end_pairs[i as usize].end = max(nend, cend);
        merged = true;
        break;
    }
    if merged {
        return;
    }
    start_end_pairs[future_index as usize].start = nstart;
    start_end_pairs[future_index as usize].end = nend;
    future_index += 1;
}

/*
 * Initialize the shadow for existing mappings during kernel initialization.
 * These include kernel text/data sections, NODE_DATA and future ranges
 * registered while creating other data (e.g. percpu).
 *
 * Allocations via memblock can be only done before slab is initialized.
 */
unsafe fn kmsan_init_shadow() {
    let nd_size: usize = core::mem::size_of::<pg_data_t>();
    let mut p_start: phys_addr_t;
    let mut p_end: phys_addr_t;
    let mut loop_: u64;
    let mut nid: i32;

    for_each_reserved_mem_range!(loop_, p_start, p_end) {
        kmsan_record_future_shadow_range(phys_to_virt(p_start), phys_to_virt(p_end));
    }
    /* Allocate shadow for .data */
    kmsan_record_future_shadow_range(_sdata, _edata);

    for_each_online_node!(nid) {
        kmsan_record_future_shadow_range(
            NODE_DATA(nid),
            (NODE_DATA(nid) as *mut i8).add(nd_size) as *mut core::ffi::c_void,
        );
    }

    for i in 0..future_index {
        kmsan_init_alloc_meta_for_range(
            start_end_pairs[i as usize].start as *mut core::ffi::c_void,
            start_end_pairs[i as usize].end as *mut core::ffi::c_void,
        );
    }
}

#[repr(C)]
struct metadata_page_pair {
    shadow: *mut page,
    origin: *mut page,
}
static mut held_back: [metadata_page_pair; NR_PAGE_ORDERS] =
    [metadata_page_pair { shadow: core::ptr::null_mut(), origin: core::ptr::null_mut() }; NR_PAGE_ORDERS];

/*
 * Eager metadata allocation. When the memblock allocator is freeing pages to
 * pagealloc, we use 2/3 of them as metadata for the remaining 1/3.
 * We store the pointers to the returned blocks of pages in held_back[] grouped
 * by their order: when kmsan_memblock_free_pages() is called for the first
 * time with a certain order, it is reserved as a shadow block, for the second
 * time - as an origin block. On the third time the incoming block receives its
 * shadow and origin ranges from the previously saved shadow and origin blocks,
 * after which held_back[order] can be used again.
 *
 * At the very end there may be leftover blocks in held_back[]. They are
 * collected later by kmsan_memblock_discard().
 */
unsafe fn kmsan_memblock_free_pages(page: *mut page, order: u32) -> bool {
    let shadow: *mut page;
    let origin: *mut page;

    if held_back[order as usize].shadow.is_null() {
        held_back[order as usize].shadow = page;
        return false;
    }
    if held_back[order as usize].origin.is_null() {
        held_back[order as usize].origin = page;
        return false;
    }
    shadow = held_back[order as usize].shadow;
    origin = held_back[order as usize].origin;
    kmsan_setup_meta(page, shadow, origin, order);

    held_back[order as usize].shadow = core::ptr::null_mut();
    held_back[order as usize].origin = core::ptr::null_mut();
    true
}

const MAX_BLOCKS: usize = 8;
#[repr(C)]
struct smallstack {
    items: [*mut page; MAX_BLOCKS],
    index: i32,
    order: i32,
}

static mut collect: smallstack = smallstack {
    items: [core::ptr::null_mut(); MAX_BLOCKS],
    index: 0,
    order: MAX_PAGE_ORDER,
};

unsafe fn smallstack_push(stack: *mut smallstack, pages: *mut page) {
    KMSAN_WARN_ON((*stack).index == MAX_BLOCKS as i32);
    (*stack).items[(*stack).index as usize] = pages;
    (*stack).index += 1;
}

unsafe fn smallstack_pop(stack: *mut smallstack) -> *mut page {
    KMSAN_WARN_ON((*stack).index == 0);
    (*stack).index -= 1;
    let ret = (*stack).items[(*stack).index as usize];
    (*stack).items[(*stack).index as usize] = core::ptr::null_mut();
    ret
}

unsafe fn do_collection() {
    let page: *mut page;
    let shadow: *mut page;
    let origin: *mut page;

    while collect.index >= 3 {
        page = smallstack_pop(&mut collect);
        shadow = smallstack_pop(&mut collect);
        origin = smallstack_pop(&mut collect);
        kmsan_setup_meta(page, shadow, origin, collect.order as u32);
        __free_pages_core(page, collect.order as u32, MEMINIT_EARLY);
    }
}

unsafe fn collect_split() {
    let mut tmp = smallstack {
        items: [core::ptr::null_mut(); MAX_BLOCKS],
        order: collect.order - 1,
        index: 0,
    };
    let page: *mut page;

    if collect.order == 0 {
        return;
    }
    while collect.index != 0 {
        page = smallstack_pop(&mut collect);
        smallstack_push(&mut tmp, page.add(0));
        smallstack_push(&mut tmp, page.add(1usize << tmp.order as usize));
    }
    core::ptr::copy_nonoverlapping(&tmp, &mut collect, 1);
}

/*
 * Memblock is about to go away. Split the page blocks left over in held_back[]
 * and return 1/3 of that memory to the system.
 */
unsafe fn kmsan_memblock_discard() {
    /*
     * For each order=N:
     *  - push held_back[N].shadow and .origin to @collect;
     *  - while there are >= 3 elements in @collect, do garbage collection:
     *    - pop 3 ranges from @collect;
     *    - use two of them as shadow and origin for the third one;
     *    - repeat;
     *  - split each remaining element from @collect into 2 ranges of
     *    order=N-1,
     *  - repeat.
     */
    collect.order = MAX_PAGE_ORDER;
    for i in (0..=MAX_PAGE_ORDER).rev() {
        if !held_back[i as usize].shadow.is_null() {
            smallstack_push(&mut collect, held_back[i as usize].shadow);
        }
        if !held_back[i as usize].origin.is_null() {
            smallstack_push(&mut collect, held_back[i as usize].origin);
        }
        held_back[i as usize].shadow = core::ptr::null_mut();
        held_back[i as usize].origin = core::ptr::null_mut();
        do_collection();
        collect_split();
    }
}

unsafe fn kmsan_init_runtime() {
    /* Assuming current is init_task */
    kmsan_internal_task_create(current);
    kmsan_memblock_discard();
    pr_info!("Starting KernelMemorySanitizer\n");
    pr_info!("ATTENTION: KMSAN is a debugging tool! Do not use it on production machines!\n");
    kmsan_enabled = true;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
