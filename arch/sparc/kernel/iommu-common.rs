// SPDX-License-Identifier: GPL-2.0
/*
 * IOMMU mmap management and range allocation functions.
 * Based almost entirely upon the powerpc iommu allocator.
 */

// Dependencies supplied by the surrounding kernel translation.

static mut iommu_large_alloc: libc::c_ulong = 15;

// DEFINE_PER_CPU(unsigned int, iommu_hash_common);
static mut iommu_hash_common: u32 = 0;

#[inline]
unsafe fn need_flush(iommu: *mut iommu_map_table) -> bool {
    ((*iommu).flags & IOMMU_NEED_FLUSH) != 0
}

#[inline]
unsafe fn set_flush(iommu: *mut iommu_map_table) {
    (*iommu).flags |= IOMMU_NEED_FLUSH;
}

#[inline]
unsafe fn clear_flush(iommu: *mut iommu_map_table) {
    (*iommu).flags &= !IOMMU_NEED_FLUSH;
}

unsafe fn setup_iommu_pool_hash() {
    static mut do_once: bool = false;

    if do_once {
        return;
    }
    do_once = true;
    // for_each_possible_cpu(i)
    let mut i: u32 = 0;
    while i < NR_CPUS {
        // per_cpu(iommu_hash_common, i) = hash_32(i, IOMMU_POOL_HASHBITS);
        iommu_hash_common = hash_32(i, IOMMU_POOL_HASHBITS);
        i += 1;
    }
}

/*
 * Initialize iommu_pool entries for the iommu_map_table. `num_entries'
 * is the number of table entries. If `large_pool' is set to true,
 * the top 1/4 of the table will be set aside for pool allocations
 * of more than iommu_large_alloc pages.
 */
pub unsafe fn iommu_tbl_pool_init(
    iommu: *mut iommu_map_table,
    num_entries: libc::c_ulong,
    table_shift: u32,
    lazy_flush: Option<unsafe extern "C" fn(*mut iommu_map_table)>,
    large_pool: bool,
    npools: u32,
    skip_span_boundary_check: bool,
) {
    let mut start: u32 = 0;
    let p: *mut iommu_pool = &mut (*iommu).large_pool;

    setup_iommu_pool_hash();
    if npools == 0 {
        (*iommu).nr_pools = IOMMU_NR_POOLS;
    } else {
        (*iommu).nr_pools = npools;
    }
    BUG_ON(npools > IOMMU_NR_POOLS);

    (*iommu).table_shift = table_shift;
    (*iommu).lazy_flush = lazy_flush;
    start = 0;
    if skip_span_boundary_check {
        (*iommu).flags |= IOMMU_NO_SPAN_BOUND;
    }
    if large_pool {
        (*iommu).flags |= IOMMU_HAS_LARGE_POOL;
    }

    if !large_pool {
        (*iommu).poolsize = num_entries / (*iommu).nr_pools as libc::c_ulong;
    } else {
        (*iommu).poolsize = (num_entries * 3 / 4) / (*iommu).nr_pools as libc::c_ulong;
    }
    let mut i = 0;
    while i < (*iommu).nr_pools {
        spin_lock_init(&mut (*iommu).pools[i as usize].lock);
        (*iommu).pools[i as usize].start = start as libc::c_ulong;
        (*iommu).pools[i as usize].hint = start as libc::c_ulong;
        start += (*iommu).poolsize as u32;
        (*iommu).pools[i as usize].end = start as libc::c_ulong - 1;
        i += 1;
    }
    if !large_pool {
        return;
    }
    spin_lock_init(&mut (*p).lock);
    (*p).start = start as libc::c_ulong;
    (*p).hint = (*p).start;
    (*p).end = num_entries;
}

pub unsafe fn iommu_tbl_range_alloc(
    dev: *mut device,
    iommu: *mut iommu_map_table,
    npages: libc::c_ulong,
    handle: *mut libc::c_ulong,
    mask: libc::c_ulong,
    align_order: u32,
) -> libc::c_ulong {
    let pool_hash: u32 = iommu_hash_common;
    let mut n: libc::c_ulong;
    let mut end: libc::c_ulong;
    let mut start: libc::c_ulong;
    let mut limit: libc::c_ulong;
    let mut boundary_size: libc::c_ulong;
    let mut pool: *mut iommu_pool;
    let mut pass = 0;
    let mut pool_nr: u32;
    let npools = (*iommu).nr_pools;
    let flags: libc::c_ulong;
    let large_pool = ((*iommu).flags & IOMMU_HAS_LARGE_POOL) != 0;
    let largealloc = large_pool && npages > iommu_large_alloc;
    let mut shift: libc::c_ulong;
    let mut align_mask: libc::c_ulong = 0;

    if align_order > 0 {
        align_mask = !0 as libc::c_ulong >> (BITS_PER_LONG - align_order);
    }
    if unlikely(npages == 0) {
        WARN_ON_ONCE(1);
        return IOMMU_ERROR_CODE;
    }
    if largealloc {
        pool = &mut (*iommu).large_pool;
        pool_nr = 0;
    } else {
        pool_nr = pool_hash & (npools - 1);
        pool = &mut (*iommu).pools[pool_nr as usize];
    }
    spin_lock_irqsave(&mut (*pool).lock, &flags);

    loop {
        if pass == 0 && !handle.is_null() && *handle != 0 && *handle >= (*pool).start && *handle < (*pool).end {
            start = *handle;
        } else {
            start = (*pool).hint;
        }
        limit = (*pool).end;
        if start >= limit { start = (*pool).start; }
        shift = (*iommu).table_map_base >> (*iommu).table_shift;
        if limit + shift > mask {
            limit = mask - shift + 1;
            if (start & mask) >= limit || pass > 0 {
                spin_unlock(&mut (*pool).lock);
                pool = &mut (*iommu).pools[0];
                spin_lock(&mut (*pool).lock);
                start = (*pool).start;
            } else { start &= mask; }
        }
        if ((*iommu).flags & IOMMU_NO_SPAN_BOUND) != 0 {
            shift = 0;
            boundary_size = (*iommu).poolsize * npools as libc::c_ulong;
        } else {
            boundary_size = dma_get_seg_boundary_nr_pages(dev, (*iommu).table_shift);
        }
        n = iommu_area_alloc((*iommu).map, limit, start, npages, shift, boundary_size, align_mask);
        if n == !0 as libc::c_ulong {
            if likely(pass == 0) {
                (*pool).hint = (*pool).start;
                set_flush(iommu); pass += 1; continue;
            } else if !largealloc && pass <= npools {
                spin_unlock(&mut (*pool).lock);
                pool_nr = (pool_nr + 1) & (npools - 1);
                pool = &mut (*iommu).pools[pool_nr as usize];
                spin_lock(&mut (*pool).lock); (*pool).hint = (*pool).start;
                set_flush(iommu); pass += 1; continue;
            } else { n = IOMMU_ERROR_CODE; break; }
        }
        if let Some(flush) = (*iommu).lazy_flush {
            if n < (*pool).hint || need_flush(iommu) {
                clear_flush(iommu); flush(iommu);
            }
        }
        end = n + npages; (*pool).hint = end;
        if !handle.is_null() { *handle = end; }
        break;
    }
    spin_unlock_irqrestore(&mut (*pool).lock, flags);
    n
}

unsafe fn get_pool(tbl: *mut iommu_map_table, entry: libc::c_ulong) -> *mut iommu_pool {
    let largepool_start = (*tbl).large_pool.start;
    if ((*tbl).flags & IOMMU_HAS_LARGE_POOL) != 0 && entry >= largepool_start {
        &mut (*tbl).large_pool
    } else {
        let pool_nr = entry / (*tbl).poolsize;
        BUG_ON(pool_nr >= (*tbl).nr_pools as libc::c_ulong);
        &mut (*tbl).pools[pool_nr as usize]
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
