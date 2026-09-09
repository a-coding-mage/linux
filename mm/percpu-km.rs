// SPDX-License-Identifier: GPL-2.0-only
/*
 * mm/percpu-km.c - kernel memory based chunk allocation
 *
 * Copyright (C) 2010        SUSE Linux Products GmbH
 * Copyright (C) 2010        Tejun Heo <tj@kernel.org>
 *
 * Chunks are allocated as a contiguous kernel memory using gfp
 * allocation.  This is to be used on nommu architectures.
 *
 * To use percpu-km,
 *
 * - define CONFIG_NEED_PER_CPU_KM from the arch Kconfig.
 *
 * - CONFIG_NEED_PER_CPU_PAGE_FIRST_CHUNK must not be defined.  It's
 *   not compatible with PER_CPU_KM.  EMBED_FIRST_CHUNK should work
 *   fine.
 *
 * - NUMA is not supported.  When setting up the first chunk,
 *   @cpu_distance_fn should be NULL or report all CPUs to be nearer
 *   than or at LOCAL_DISTANCE.
 *
 * - It's best if the chunk size is power of two multiple of
 *   PAGE_SIZE.  Because each chunk is allocated as a contiguous
 *   kernel memory block using alloc_pages(), memory will be wasted if
 *   chunk size is not aligned.  percpu-km code will whine about it.
 */

// CONFIG_SMP && CONFIG_NEED_PER_CPU_PAGE_FIRST_CHUNK produces a C build error:
// contiguous percpu allocation is incompatible with paged first chunk.

unsafe fn pcpu_post_unmap_tlb_flush(
    _chunk: *mut pcpu_chunk,
    _page_start: c_int,
    _page_end: c_int,
) {
    /* nothing */
}

unsafe fn pcpu_populate_chunk(
    _chunk: *mut pcpu_chunk,
    _page_start: c_int,
    _page_end: c_int,
    _gfp: gfp_t,
) -> c_int {
    0
}

unsafe fn pcpu_depopulate_chunk(
    _chunk: *mut pcpu_chunk,
    _page_start: c_int,
    _page_end: c_int,
) {
    /* nada */
}

unsafe fn pcpu_create_chunk(gfp: gfp_t) -> *mut pcpu_chunk {
    let nr_pages: c_int = (*pcpu_group_sizes.as_ptr()) >> PAGE_SHIFT;
    let chunk: *mut pcpu_chunk;
    let pages: *mut page;
    let mut flags: c_ulong;
    let mut i: c_int;

    chunk = pcpu_alloc_chunk(gfp);
    if chunk.is_null() {
        return core::ptr::null_mut();
    }

    pages = alloc_pages(gfp, order_base_2(nr_pages));
    if pages.is_null() {
        pcpu_free_chunk(chunk);
        return core::ptr::null_mut();
    }

    i = 0;
    while i < nr_pages {
        pcpu_set_page_chunk(pages.add(i as usize), chunk);
        i += 1;
    }

    (*chunk).data = pages as *mut c_void;
    (*chunk).base_addr = page_address(pages);

    spin_lock_irqsave(&mut pcpu_lock, &mut flags);
    pcpu_chunk_populated(chunk, 0, (*chunk).nr_pages);
    spin_unlock_irqrestore(&mut pcpu_lock, flags);

    pcpu_stats_chunk_alloc();
    trace_percpu_create_chunk((*chunk).base_addr);

    chunk
}

unsafe fn pcpu_destroy_chunk(chunk: *mut pcpu_chunk) {
    let nr_pages: c_int = (*pcpu_group_sizes.as_ptr()) >> PAGE_SHIFT;

    if chunk.is_null() {
        return;
    }

    pcpu_stats_chunk_dealloc();
    trace_percpu_destroy_chunk((*chunk).base_addr);

    if !(*chunk).data.is_null() {
        let pages: *mut page = (*chunk).data as *mut page;
        let mut i: c_int;

        /* clear chunk info from each page before free them */
        i = 0;
        while i < nr_pages {
            pcpu_set_page_chunk(pages.add(i as usize), core::ptr::null_mut());
            i += 1;
        }
        __free_pages((*chunk).data as *mut page, order_base_2(nr_pages));
    }
    pcpu_free_chunk(chunk);
}

unsafe fn pcpu_addr_to_page(addr: *mut c_void) -> *mut page {
    virt_to_page(addr)
}

unsafe fn pcpu_verify_alloc_info(ai: *const pcpu_alloc_info) -> c_int {
    let nr_pages: usize;
    let alloc_pages: usize;

    /* all units must be in a single group */
    if (*ai).nr_groups != 1 {
        pr_crit("can't handle more than one group\n");
        return -EINVAL;
    }

    nr_pages = (((*ai).groups[0].nr_units * (*ai).unit_size) >> PAGE_SHIFT) as usize;
    alloc_pages = roundup_pow_of_two(nr_pages);

    if alloc_pages > nr_pages {
        pr_warn(
            "wasting %zu pages per chunk\n",
            alloc_pages - nr_pages,
        );
    }

    0
}

unsafe fn pcpu_should_reclaim_chunk(_chunk: *mut pcpu_chunk) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
