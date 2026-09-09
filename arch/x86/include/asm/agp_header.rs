/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation unit:
 * linux/pgtable.h and asm/cacheflush.h.
 */

/*
 * Functions to keep the agpgart mappings coherent with the MMU. The
 * GART gives the CPU a physical alias of pages in memory. The alias
 * region is mapped uncacheable. Make sure there are no conflicting
 * mappings with different cacheability attributes for the same
 * page. This avoids data corruption on some CPUs.
 */

macro_rules! map_page_into_agp {
    ($page:expr) => {
        set_pages_uc($page, 1)
    };
}

macro_rules! unmap_page_from_agp {
    ($page:expr) => {
        set_pages_wb($page, 1)
    };
}

/*
 * Could use CLFLUSH here if the cpu supports it. But then it would
 * need to be called for each cacheline of the whole page so it may
 * not be worth it. Would need a page for it.
 */
macro_rules! flush_agp_cache {
    () => {
        wbinvd()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
