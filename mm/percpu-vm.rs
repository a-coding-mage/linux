// SPDX-License-Identifier: GPL-2.0-only
/*
 * mm/percpu-vm.c - vmalloc area based chunk allocation
 *
 * Copyright (C) 2010        SUSE Linux Products GmbH
 * Copyright (C) 2010        Tejun Heo <tj@kernel.org>
 *
 * Chunks are mapped into vmalloc areas and populated page by page.
 * This is the default chunk allocator.
 */

unsafe fn pcpu_chunk_page(chunk: *mut pcpu_chunk, cpu: c_uint, page_idx: c_int) -> *mut page {
    // must not be used on pre-mapped chunk
    WARN_ON((*chunk).immutable);
    vmalloc_to_page(pcpu_chunk_addr(chunk, cpu, page_idx) as *mut c_void)
}

unsafe fn pcpu_get_pages(gfp: gfp_t) -> *mut *mut page {
    static mut PAGES: *mut *mut page = core::ptr::null_mut();
    let pages_size: usize = pcpu_nr_units * pcpu_unit_pages * core::mem::size_of::<*mut page>();

    lockdep_assert_held(&pcpu_alloc_mutex);
    if PAGES.is_null() && gfp != 0 {
        PAGES = pcpu_mem_zalloc(pages_size, gfp) as *mut *mut page;
    }
    PAGES
}

unsafe fn pcpu_get_pages_cached() -> *mut *mut page {
    pcpu_get_pages(0)
}

unsafe fn pcpu_free_pages(
    _chunk: *mut pcpu_chunk,
    pages: *mut *mut page,
    page_start: c_int,
    page_end: c_int,
) {
    let mut cpu: c_uint;
    let mut i: c_int;
    for_each_possible_cpu!(cpu) {
        i = page_start;
        while i < page_end {
            let p = *pages.add(pcpu_page_idx(cpu, i) as usize);
            if !p.is_null() {
                __free_page(p);
            }
            i += 1;
        }
    }
}

unsafe fn pcpu_alloc_pages(
    _chunk: *mut pcpu_chunk,
    pages: *mut *mut page,
    page_start: c_int,
    page_end: c_int,
    mut gfp: gfp_t,
) -> c_int {
    let mut cpu: c_uint;
    let mut tcpu: c_uint;
    let mut i: c_int;
    gfp |= __GFP_HIGHMEM;

    for_each_possible_cpu!(cpu) {
        i = page_start;
        while i < page_end {
            let pagep = pages.add(pcpu_page_idx(cpu, i) as usize);
            *pagep = alloc_pages_node(cpu_to_node(cpu), gfp, 0);
            if (*pagep).is_null() {
                while i > page_start {
                    i -= 1;
                    __free_page(*pages.add(pcpu_page_idx(cpu, i) as usize));
                }
                for_each_possible_cpu!(tcpu) {
                    if tcpu == cpu { break; }
                    let mut j = page_start;
                    while j < page_end {
                        __free_page(*pages.add(pcpu_page_idx(tcpu, j) as usize));
                        j += 1;
                    }
                }
                return -ENOMEM;
            }
            i += 1;
        }
    }
    0
}

unsafe fn pcpu_pre_unmap_flush(chunk: *mut pcpu_chunk, page_start: c_int, page_end: c_int) {
    flush_cache_vunmap(
        pcpu_chunk_addr(chunk, pcpu_low_unit_cpu, page_start),
        pcpu_chunk_addr(chunk, pcpu_high_unit_cpu, page_end),
    );
}

unsafe fn __pcpu_unmap_pages(addr: c_ulong, nr_pages: c_int) {
    vunmap_range_noflush(addr, addr + ((nr_pages as c_ulong) << PAGE_SHIFT));
}

unsafe fn pcpu_unmap_pages(
    chunk: *mut pcpu_chunk,
    pages: *mut *mut page,
    page_start: c_int,
    page_end: c_int,
) {
    let mut cpu: c_uint;
    for_each_possible_cpu!(cpu) {
        let mut i = page_start;
        while i < page_end {
            let p = pcpu_chunk_page(chunk, cpu, i);
            WARN_ON(p.is_null());
            *pages.add(pcpu_page_idx(cpu, i) as usize) = p;
            i += 1;
        }
        __pcpu_unmap_pages(pcpu_chunk_addr(chunk, cpu, page_start), page_end - page_start);
    }
}

unsafe fn pcpu_post_unmap_tlb_flush(chunk: *mut pcpu_chunk, page_start: c_int, page_end: c_int) {
    flush_tlb_kernel_range(
        pcpu_chunk_addr(chunk, pcpu_low_unit_cpu, page_start),
        pcpu_chunk_addr(chunk, pcpu_high_unit_cpu, page_end),
    );
}

unsafe fn __pcpu_map_pages(addr: c_ulong, pages: *mut *mut page, nr_pages: c_int, gfp: gfp_t) -> c_int {
    let flags = memalloc_apply_gfp_scope(gfp);
    let ret = vmap_pages_range_noflush(
        addr,
        addr + ((nr_pages as c_ulong) << PAGE_SHIFT),
        PAGE_KERNEL,
        pages,
        PAGE_SHIFT,
        gfp,
    );
    memalloc_restore_scope(flags);
    ret
}

unsafe fn pcpu_map_pages(
    chunk: *mut pcpu_chunk,
    pages: *mut *mut page,
    page_start: c_int,
    page_end: c_int,
    gfp: gfp_t,
) -> c_int {
    let mut cpu: c_uint;
    let mut tcpu: c_uint;
    for_each_possible_cpu!(cpu) {
        let err = __pcpu_map_pages(
            pcpu_chunk_addr(chunk, cpu, page_start),
            pages.add(pcpu_page_idx(cpu, page_start) as usize),
            page_end - page_start,
            gfp,
        );
        if err < 0 {
            for_each_possible_cpu!(tcpu) {
                __pcpu_unmap_pages(pcpu_chunk_addr(chunk, tcpu, page_start), page_end - page_start);
                if tcpu == cpu { break; }
            }
            pcpu_post_unmap_tlb_flush(chunk, page_start, page_end);
            return err;
        }
        let mut i = page_start;
        while i < page_end {
            pcpu_set_page_chunk(*pages.add(pcpu_page_idx(cpu, i) as usize), chunk);
            i += 1;
        }
    }
    0
}

unsafe fn pcpu_post_map_flush(chunk: *mut pcpu_chunk, page_start: c_int, page_end: c_int) {
    flush_cache_vmap(
        pcpu_chunk_addr(chunk, pcpu_low_unit_cpu, page_start),
        pcpu_chunk_addr(chunk, pcpu_high_unit_cpu, page_end),
    );
}

unsafe fn pcpu_populate_chunk(chunk: *mut pcpu_chunk, page_start: c_int, page_end: c_int, gfp: gfp_t) -> c_int {
    let pages = pcpu_get_pages(gfp);
    if pages.is_null() { return -ENOMEM; }
    if pcpu_alloc_pages(chunk, pages, page_start, page_end, gfp) != 0 { return -ENOMEM; }
    if pcpu_map_pages(chunk, pages, page_start, page_end, gfp) != 0 {
        pcpu_free_pages(chunk, pages, page_start, page_end);
        return -ENOMEM;
    }
    pcpu_post_map_flush(chunk, page_start, page_end);
    0
}

unsafe fn pcpu_depopulate_chunk(chunk: *mut pcpu_chunk, page_start: c_int, page_end: c_int) {
    let pages = pcpu_get_pages_cached();
    BUG_ON(pages.is_null());
    pcpu_pre_unmap_flush(chunk, page_start, page_end);
    pcpu_unmap_pages(chunk, pages, page_start, page_end);
    pcpu_free_pages(chunk, pages, page_start, page_end);
}

unsafe fn pcpu_create_chunk(gfp: gfp_t) -> *mut pcpu_chunk {
    let chunk = pcpu_alloc_chunk(gfp);
    if chunk.is_null() { return core::ptr::null_mut(); }
    let vms = pcpu_get_vm_areas(pcpu_group_offsets, pcpu_group_sizes, pcpu_nr_groups, pcpu_atom_size, gfp);
    if vms.is_null() {
        pcpu_free_chunk(chunk);
        return core::ptr::null_mut();
    }
    (*chunk).data = vms;
    (*chunk).base_addr = (*vms).addr - pcpu_group_offsets[0];
    pcpu_stats_chunk_alloc();
    trace_percpu_create_chunk((*chunk).base_addr);
    chunk
}

unsafe fn pcpu_destroy_chunk(chunk: *mut pcpu_chunk) {
    if chunk.is_null() { return; }
    pcpu_stats_chunk_dealloc();
    trace_percpu_destroy_chunk((*chunk).base_addr);
    if !(*chunk).data.is_null() { pcpu_free_vm_areas((*chunk).data, pcpu_nr_groups); }
    pcpu_free_chunk(chunk);
}

unsafe fn pcpu_addr_to_page(addr: *mut c_void) -> *mut page { vmalloc_to_page(addr) }

unsafe fn pcpu_verify_alloc_info(_ai: *const pcpu_alloc_info) -> c_int { 0 }

unsafe fn pcpu_should_reclaim_chunk(chunk: *mut pcpu_chunk) -> bool {
    if chunk == pcpu_first_chunk || chunk == pcpu_reserved_chunk { return false; }
    ((*chunk).isolated && (*chunk).nr_empty_pop_pages != 0) ||
        (pcpu_nr_empty_pop_pages > PCPU_EMPTY_POP_PAGES_HIGH + (*chunk).nr_empty_pop_pages &&
         (*chunk).nr_empty_pop_pages >= (*chunk).nr_pages / 4)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
