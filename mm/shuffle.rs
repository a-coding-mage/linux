// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2018 Intel Corporation. All rights reserved.

// Dependencies supplied by the surrounding kernel translation unit.

static mut PAGE_ALLOC_SHUFFLE_KEY: StaticKeyFalse = StaticKeyFalse;

static mut shuffle_param: bool = false;

unsafe extern "C" fn shuffle_param_set(
    val: *const core::ffi::c_char,
    kp: *const kernel_param,
) -> core::ffi::c_int {
    if param_set_bool(val, kp) != 0 {
        return -EINVAL;
    }
    if *( (*kp).arg as *const bool) {
        static_branch_enable(&raw mut PAGE_ALLOC_SHUFFLE_KEY);
    }
    0
}

static shuffle_param_ops: kernel_param_ops = kernel_param_ops {
    set: Some(shuffle_param_set),
    get: Some(param_get_bool),
};

/*
 * For two pages to be swapped in the shuffle, they must be free (on a
 * 'free_area' lru), have the same order, and have the same migratetype.
 */
unsafe extern "C" fn shuffle_valid_page(
    zone: *mut zone,
    pfn: c_ulong,
    order: core::ffi::c_int,
) -> *mut page {
    let page = pfn_to_online_page(pfn);

    /*
     * Given we're dealing with randomly selected pfns in a zone we
     * need to ask questions like...
     */

    /* ... is the page managed by the buddy? */
    if page.is_null() {
        return core::ptr::null_mut();
    }

    /* ... is the page assigned to the same zone? */
    if page_zone(page) != zone {
        return core::ptr::null_mut();
    }

    /* ...is the page free and currently on a free_area list? */
    if !PageBuddy(page) {
        return core::ptr::null_mut();
    }

    /*
     * ...is the page on the same list as the page we will
     * shuffle it with?
     */
    if buddy_order(page) != order {
        return core::ptr::null_mut();
    }

    page
}

/*
 * Fisher-Yates shuffle the freelist which prescribes iterating through an
 * array, pfns in this case, and randomly swapping each entry with another in
 * the span, end_pfn - start_pfn.
 *
 * To keep the implementation simple it does not attempt to correct for sources
 * of bias in the distribution, like modulo bias or pseudo-random number
 * generator bias. I.e. the expectation is that this shuffling raises the bar
 * for attacks that exploit the predictability of page allocations, but need not
 * be a perfect shuffle.
 */
const SHUFFLE_RETRY: core::ffi::c_int = 10;

unsafe extern "C" fn __shuffle_zone(z: *mut zone) {
    let mut i: c_ulong;
    let mut flags: c_ulong;
    let mut start_pfn = (*z).zone_start_pfn;
    let end_pfn = zone_end_pfn(z);
    let order: core::ffi::c_int = SHUFFLE_ORDER;
    let order_pages: c_ulong = 1u32.wrapping_shl(order as u32) as c_ulong;

    spin_lock_irqsave(&mut (*z).lock, &mut flags);
    start_pfn = ALIGN(start_pfn, order_pages);
    i = start_pfn;
    while i < end_pfn {
        let mut j: c_ulong;
        let mut migratetype: core::ffi::c_int;
        let mut retry: core::ffi::c_int;
        let page_i: *mut page;
        let mut page_j: *mut page;

        /*
         * We expect page_i, in the sub-range of a zone being added
         * (@start_pfn to @end_pfn), to more likely be valid compared to
         * page_j randomly selected in the span @zone_start_pfn to
         * @spanned_pages.
         */
        page_i = shuffle_valid_page(z, i, order);
        if page_i.is_null() {
            i = i.wrapping_add(order_pages);
            continue;
        }

        retry = 0;
        loop {
            if retry >= SHUFFLE_RETRY {
                break;
            }
            /*
             * Pick a random order aligned page in the zone span as
             * a swap target. If the selected pfn is a hole, retry
             * up to SHUFFLE_RETRY attempts find a random valid pfn
             * in the zone.
             */
            j = (*z).zone_start_pfn.wrapping_add(ALIGN_DOWN(
                get_random_long() % (*z).spanned_pages,
                order_pages,
            ));
            page_j = shuffle_valid_page(z, j, order);
            if !page_j.is_null() && page_j != page_i {
                break;
            }
            retry += 1;
        }
        if retry >= SHUFFLE_RETRY {
            pr_debug("%s: failed to swap %#lx\n", "__shuffle_zone", i);
            i = i.wrapping_add(order_pages);
            continue;
        }

        /*
         * Each migratetype corresponds to its own list, make sure the
         * types match otherwise we're moving pages to lists where they
         * do not belong.
         */
        migratetype = get_pageblock_migratetype(page_i);
        if get_pageblock_migratetype(page_j) != migratetype {
            pr_debug("%s: migratetype mismatch %#lx\n", "__shuffle_zone", i);
            i = i.wrapping_add(order_pages);
            continue;
        }

        list_swap(&mut (*page_i).lru, &mut (*page_j).lru);

        pr_debug("%s: swap: %#lx -> %#lx\n", "__shuffle_zone", i, j);

        /* take it easy on the zone lock */
        if i % (100u64.wrapping_mul(order_pages)) == 0 {
            spin_unlock_irqrestore(&mut (*z).lock, flags);
            cond_resched();
            spin_lock_irqsave(&mut (*z).lock, &mut flags);
        }
        i = i.wrapping_add(order_pages);
    }
    spin_unlock_irqrestore(&mut (*z).lock, flags);
}

/*
 * __shuffle_free_memory - reduce the predictability of the page allocator
 * @pgdat: node page data
 */
unsafe extern "C" fn __shuffle_free_memory(pgdat: *mut pg_data_t) {
    let mut z = (*pgdat).node_zones;
    for _ in 0..MAX_NR_ZONES {
        shuffle_zone(z);
        z = z.add(1);
    }
}

unsafe extern "C" fn shuffle_pick_tail() -> bool {
    static mut rand: u64 = 0;
    static mut rand_bits: u8 = 0;
    let ret: bool;

    /*
     * The lack of locking is deliberate. If 2 threads race to
     * update the rand state it just adds to the entropy.
     */
    if rand_bits == 0 {
        rand_bits = 64;
        rand = get_random_u64();
    }

    ret = rand & 1 != 0;

    rand_bits -= 1;
    rand >>= 1;

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
