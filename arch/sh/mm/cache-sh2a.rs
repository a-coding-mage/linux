// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/sh/mm/cache-sh2a.c
 *
 * Copyright (C) 2008 Yoshinori Sato
 */

// Linux kernel dependencies supplied externally.

const MAX_OCACHE_PAGES: usize = 32;
const MAX_ICACHE_PAGES: usize = 32;

#[cfg(feature = "CONFIG_CACHE_WRITEBACK")]
unsafe fn sh2a_flush_oc_line(v: usize, way: i32) {
    let addr = (v & 0x0000_07f0) | ((way as usize) << 11);
    let mut data: u32;

    data = __raw_readl(CACHE_OC_ADDRESS_ARRAY | addr);
    if (data as usize & CACHE_PHYSADDR_MASK) == (v & CACHE_PHYSADDR_MASK) {
        data &= !SH_CACHE_UPDATED;
        __raw_writel(data, CACHE_OC_ADDRESS_ARRAY | addr);
    }
}

unsafe fn sh2a_invalidate_line(cache_addr: usize, v: usize) {
    /* Set associative bit to hit all ways */
    let addr = (v & 0x0000_07f0) | SH_CACHE_ASSOC;
    __raw_writel((addr & CACHE_PHYSADDR_MASK) as u32, cache_addr | addr);
}

/*
 * Write back the dirty D-caches, but not invalidate them.
 */
unsafe fn sh2a__flush_wback_region(start: *mut core::ffi::c_void, size: i32) {
    #[cfg(feature = "CONFIG_CACHE_WRITEBACK")]
    {
        let mut v: usize;
        let mut begin: usize;
        let mut end: usize;
        let mut flags: usize = 0;
        let nr_ways: i32;

        begin = start as usize & !(L1_CACHE_BYTES - 1);
        end = ((start as usize).wrapping_add(size as usize).wrapping_add(L1_CACHE_BYTES - 1))
            & !(L1_CACHE_BYTES - 1);
        nr_ways = current_cpu_data.dcache.ways;

        local_irq_save(&mut flags);
        jump_to_uncached();

        /* If there are too many pages then flush the entire cache */
        if ((end.wrapping_sub(begin)) >> PAGE_SHIFT) >= MAX_OCACHE_PAGES {
            begin = CACHE_OC_ADDRESS_ARRAY;
            end = begin + (nr_ways as usize * current_cpu_data.dcache.way_size);

            v = begin;
            while v < end {
                let data = __raw_readl(v);
                if data & SH_CACHE_UPDATED != 0 {
                    __raw_writel(data & !SH_CACHE_UPDATED, v);
                }
                v += L1_CACHE_BYTES;
            }
        } else {
            let mut way = 0;
            while way < nr_ways {
                v = begin;
                while v < end {
                    sh2a_flush_oc_line(v, way);
                    v += L1_CACHE_BYTES;
                }
                way += 1;
            }
        }

        back_to_cached();
        local_irq_restore(flags);
    }
}

/*
 * Write back the dirty D-caches and invalidate them.
 */
unsafe fn sh2a__flush_purge_region(start: *mut core::ffi::c_void, size: i32) {
    let mut v: usize;
    let begin = start as usize & !(L1_CACHE_BYTES - 1);
    let end = ((start as usize).wrapping_add(size as usize).wrapping_add(L1_CACHE_BYTES - 1))
        & !(L1_CACHE_BYTES - 1);
    let mut flags: usize = 0;

    local_irq_save(&mut flags);
    jump_to_uncached();

    v = begin;
    while v < end {
        #[cfg(feature = "CONFIG_CACHE_WRITEBACK")]
        {
            let mut way = 0;
            let nr_ways = current_cpu_data.dcache.ways;
            while way < nr_ways {
                sh2a_flush_oc_line(v, way);
                way += 1;
            }
        }
        sh2a_invalidate_line(CACHE_OC_ADDRESS_ARRAY, v);
        v += L1_CACHE_BYTES;
    }

    back_to_cached();
    local_irq_restore(flags);
}

/*
 * Invalidate the D-caches, but no write back please
 */
unsafe fn sh2a__flush_invalidate_region(start: *mut core::ffi::c_void, size: i32) {
    let mut v: usize;
    let begin = start as usize & !(L1_CACHE_BYTES - 1);
    let end = ((start as usize).wrapping_add(size as usize).wrapping_add(L1_CACHE_BYTES - 1))
        & !(L1_CACHE_BYTES - 1);
    let mut flags: usize = 0;

    local_irq_save(&mut flags);
    jump_to_uncached();

    /* If there are too many pages then just blow the cache */
    if ((end.wrapping_sub(begin)) >> PAGE_SHIFT) >= MAX_OCACHE_PAGES {
        __raw_writel(__raw_readl(SH_CCR) | CCR_OCACHE_INVALIDATE, SH_CCR);
    } else {
        v = begin;
        while v < end {
            sh2a_invalidate_line(CACHE_OC_ADDRESS_ARRAY, v);
            v += L1_CACHE_BYTES;
        }
    }

    back_to_cached();
    local_irq_restore(flags);
}

/*
 * Write back the range of D-cache, and purge the I-cache.
 */
unsafe fn sh2a_flush_icache_range(args: *mut core::ffi::c_void) {
    let data = &*(args as *const flusher_data);
    let start = data.addr1 & !(L1_CACHE_BYTES - 1);
    let end = data.addr2.wrapping_add(L1_CACHE_BYTES - 1) & !(L1_CACHE_BYTES - 1);
    let mut v: usize;
    let mut flags: usize = 0;

    #[cfg(feature = "CONFIG_CACHE_WRITEBACK")]
    sh2a__flush_wback_region(start as *mut core::ffi::c_void, end.wrapping_sub(start) as i32);

    local_irq_save(&mut flags);
    jump_to_uncached();

    /* I-Cache invalidate */
    /* If there are too many pages then just blow the cache */
    if ((end.wrapping_sub(start)) >> PAGE_SHIFT) >= MAX_ICACHE_PAGES {
        __raw_writel(__raw_readl(SH_CCR) | CCR_ICACHE_INVALIDATE, SH_CCR);
    } else {
        v = start;
        while v < end {
            sh2a_invalidate_line(CACHE_IC_ADDRESS_ARRAY, v);
            v += L1_CACHE_BYTES;
        }
    }

    back_to_cached();
    local_irq_restore(flags);
}

unsafe fn sh2a_cache_init() {
    local_flush_icache_range = Some(sh2a_flush_icache_range);

    __flush_wback_region = Some(sh2a__flush_wback_region);
    __flush_purge_region = Some(sh2a__flush_purge_region);
    __flush_invalidate_region = Some(sh2a__flush_invalidate_region);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
