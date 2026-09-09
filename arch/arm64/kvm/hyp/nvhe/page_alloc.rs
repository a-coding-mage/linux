// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Google LLC
 * Author: Quentin Perret <qperret@google.com>
 */

static mut __hyp_vmemmap: u64 = 0;

/*
 * Index the hyp_vmemmap to find a potential buddy page, but make no assumption
 * about its current state.
 *
 * Example buddy-tree for a 4-pages physically contiguous pool:
 *
 *                 o : Page 3
 *                /
 *               o-o : Page 2
 *              /
 *             /   o : Page 1
 *            /   /
 *           o---o-o : Page 0
 *    Order  2   1 0
 *
 * Example of requests on this pool:
 *   __find_buddy_nocheck(pool, page 0, order 0) => page 1
 *   __find_buddy_nocheck(pool, page 0, order 1) => page 2
 *   __find_buddy_nocheck(pool, page 1, order 0) => page 0
 *   __find_buddy_nocheck(pool, page 2, order 0) => page 3
 */
unsafe fn __find_buddy_nocheck(
    pool: *mut hyp_pool,
    p: *mut hyp_page,
    order: u8,
) -> *mut hyp_page {
    let mut addr: phys_addr_t = hyp_page_to_phys(p);
    addr ^= PAGE_SIZE << order;

    /*
     * Don't return a page outside the pool range -- it belongs to
     * something else and may not be mapped in hyp_vmemmap.
     */
    if addr < (*pool).range_start || addr >= (*pool).range_end {
        return core::ptr::null_mut();
    }

    hyp_phys_to_page(addr)
}

/* Find a buddy page currently available for allocation */
unsafe fn __find_buddy_avail(
    pool: *mut hyp_pool,
    p: *mut hyp_page,
    order: u8,
) -> *mut hyp_page {
    let buddy = __find_buddy_nocheck(pool, p, order);

    if buddy.is_null() || (*buddy).order != order || (*buddy).refcount != 0 {
        return core::ptr::null_mut();
    }

    buddy
}

/*
 * Pages that are available for allocation are tracked in free-lists, so we use
 * the pages themselves to store the list nodes to avoid wasting space. As the
 * allocator always returns zeroed pages (which are zeroed on the hyp_put_page()
 * path to optimize allocation speed), we also need to clean-up the list node in
 * each page when we take it out of the list.
 */
unsafe fn page_remove_from_list(p: *mut hyp_page) {
    let node: *mut list_head = hyp_page_to_virt(p) as *mut list_head;

    __list_del_entry(node);
    memset(node as *mut core::ffi::c_void, 0, core::mem::size_of::<list_head>());
}

unsafe fn page_add_to_list(p: *mut hyp_page, head: *mut list_head) {
    let node: *mut list_head = hyp_page_to_virt(p) as *mut list_head;

    INIT_LIST_HEAD(node);
    list_add_tail(node, head);
}

unsafe fn node_to_page(node: *mut list_head) -> *mut hyp_page {
    hyp_virt_to_page(node as *mut core::ffi::c_void)
}

unsafe fn __hyp_attach_page(pool: *mut hyp_pool, mut p: *mut hyp_page) {
    let phys = hyp_page_to_phys(p);
    let mut buddy: *mut hyp_page;
    let mut coalesce = true;
    let mut order = (*p).order;

    /*
     * 'external' pages are never coalesced and their ->order field
     * untrusted as they bypass hyp_pool_init(). Enforce order-0.
     */
    if phys < (*pool).range_start || phys >= (*pool).range_end {
        order = 0;
        coalesce = false;
    }

    memset(hyp_page_to_virt(p), 0, PAGE_SIZE << order);

    if !coalesce {
        (*p).order = order;
        page_add_to_list(p, &mut (*pool).free_area[order as usize]);
        return;
    }

    /*
     * Only the first struct hyp_page of a high-order page (otherwise known
     * as the 'head') should have p->order set. The non-head pages should
     * have p->order = HYP_NO_ORDER. Here @p may no longer be the head
     * after coalescing, so make sure to mark it HYP_NO_ORDER proactively.
     */
    (*p).order = HYP_NO_ORDER;
    while (order + 1 <= (*pool).max_order) {
        buddy = __find_buddy_avail(pool, p, order);
        if buddy.is_null() {
            break;
        }

        /* Take the buddy out of its list, and coalesce with @p */
        page_remove_from_list(buddy);
        (*buddy).order = HYP_NO_ORDER;
        p = if (p as usize <= buddy as usize) { p } else { buddy };
        order += 1;
    }

    /* Mark the new head, and insert it */
    (*p).order = order;
    page_add_to_list(p, &mut (*pool).free_area[order as usize]);
}

unsafe fn __hyp_extract_page(
    pool: *mut hyp_pool,
    mut p: *mut hyp_page,
    order: u8,
) -> *mut hyp_page {
    let mut buddy: *mut hyp_page;

    page_remove_from_list(p);
    while (*p).order > order {
        /*
         * The buddy of order n - 1 currently has HYP_NO_ORDER as it
         * is covered by a higher-level page (whose head is @p). Use
         * __find_buddy_nocheck() to find it and inject it in the
         * free_list[n - 1], effectively splitting @p in half.
         */
        (*p).order -= 1;
        buddy = __find_buddy_nocheck(pool, p, (*p).order);
        (*buddy).order = (*p).order;
        page_add_to_list(buddy, &mut (*pool).free_area[(*buddy).order as usize]);
    }

    p
}

unsafe fn __hyp_put_page(pool: *mut hyp_pool, p: *mut hyp_page) {
    if hyp_page_ref_dec_and_test(p) {
        __hyp_attach_page(pool, p);
    }
}

/*
 * Changes to the buddy tree and page refcounts must be done with the hyp_pool
 * lock held. If a refcount change requires an update to the buddy tree (e.g.
 * hyp_put_page()), both operations must be done within the same critical
 * section to guarantee transient states (e.g. a page with null refcount but
 * not yet attached to a free list) can't be observed by well-behaved readers.
 */
pub unsafe fn hyp_put_page(pool: *mut hyp_pool, addr: *mut core::ffi::c_void) {
    let p = hyp_virt_to_page(addr);

    hyp_spin_lock(&mut (*pool).lock);
    __hyp_put_page(pool, p);
    hyp_spin_unlock(&mut (*pool).lock);
}

pub unsafe fn hyp_get_page(pool: *mut hyp_pool, addr: *mut core::ffi::c_void) {
    let p = hyp_virt_to_page(addr);

    hyp_spin_lock(&mut (*pool).lock);
    hyp_page_ref_inc(p);
    hyp_spin_unlock(&mut (*pool).lock);
}

pub unsafe fn hyp_split_page(p: *mut hyp_page) {
    let order = (*p).order;
    (*p).order = 0;
    for i in 1..(1usize << order) {
        let tail = p.add(i);

        (*tail).order = 0;
        hyp_set_page_refcounted(tail);
    }
}

pub unsafe fn hyp_alloc_pages(pool: *mut hyp_pool, order: u8) -> *mut core::ffi::c_void {
    let mut p: *mut hyp_page;
    let mut i = order;

    hyp_spin_lock(&mut (*pool).lock);

    /* Look for a high-enough-order page */
    while i <= (*pool).max_order && list_empty(&(*pool).free_area[i as usize]) {
        i += 1;
    }
    if i > (*pool).max_order {
        hyp_spin_unlock(&mut (*pool).lock);
        return core::ptr::null_mut();
    }

    /* Extract it from the tree at the right order */
    p = node_to_page((*pool).free_area[i as usize].next);
    p = __hyp_extract_page(pool, p, order);

    hyp_set_page_refcounted(p);
    hyp_spin_unlock(&mut (*pool).lock);

    hyp_page_to_virt(p)
}

pub unsafe fn hyp_pool_init(
    pool: *mut hyp_pool,
    pfn: u64,
    nr_pages: u32,
    reserved_pages: u32,
) -> i32 {
    let phys = hyp_pfn_to_phys(pfn);
    let p: *mut hyp_page;
    let mut i: u32;

    hyp_spin_lock_init(&mut (*pool).lock);
    (*pool).max_order = min(MAX_PAGE_ORDER, get_order(nr_pages << PAGE_SHIFT));
    i = 0;
    while i <= (*pool).max_order {
        INIT_LIST_HEAD(&mut (*pool).free_area[i as usize]);
        i += 1;
    }
    (*pool).range_start = phys;
    (*pool).range_end = phys + (nr_pages << PAGE_SHIFT);

    /* Init the vmemmap portion */
    p = hyp_phys_to_page(phys);
    i = 0;
    while i < nr_pages {
        hyp_set_page_refcounted(p.add(i as usize));
        (*p.add(i as usize)).order = 0;
        i += 1;
    }

    /* Attach the unused pages to the buddy tree */
    i = reserved_pages;
    while i < nr_pages {
        __hyp_put_page(pool, p.add(i as usize));
        i += 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
