/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * mm-internal API for the page (buddy) allocator. Public API lives in
 * include/linux/gfp.h.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const ALLOC_DEFAULT: u32 = 0;
/* The ALLOC_WMARK bits are used as an index to zone->watermark */
pub const ALLOC_WMARK_MIN: u32 = WMARK_MIN;
pub const ALLOC_WMARK_LOW: u32 = WMARK_LOW;
pub const ALLOC_WMARK_HIGH: u32 = WMARK_HIGH;
pub const ALLOC_NO_WATERMARKS: u32 = 0x04; /* don't check watermarks at all */

/* Mask to get the watermark bits */
pub const ALLOC_WMARK_MASK: u32 = ALLOC_NO_WATERMARKS - 1;

/* Only MMU archs have async oom victim reclaim - aka oom_reaper. */
#[cfg(feature = "CONFIG_MMU")]
pub const ALLOC_OOM: u32 = 0x08;
#[cfg(not(feature = "CONFIG_MMU"))]
pub const ALLOC_OOM: u32 = ALLOC_NO_WATERMARKS;

pub const ALLOC_NON_BLOCK: u32 = 0x10; /* Caller cannot block. */
pub const ALLOC_MIN_RESERVE: u32 = 0x20; /* __GFP_HIGH set. */
pub const ALLOC_CPUSET: u32 = 0x40; /* check for correct cpuset */
pub const ALLOC_CMA: u32 = 0x80; /* allow allocations from CMA areas */
#[cfg(feature = "CONFIG_ZONE_DMA32")]
pub const ALLOC_NOFRAGMENT: u32 = 0x100; /* avoid mixing pageblock types */
#[cfg(not(feature = "CONFIG_ZONE_DMA32"))]
pub const ALLOC_NOFRAGMENT: u32 = 0x0;
pub const ALLOC_HIGHATOMIC: u32 = 0x200; /* Allows access to MIGRATE_HIGHATOMIC */
pub const ALLOC_NOLOCK: u32 = 0x400; /* Only use spin_trylock in allocation path */
pub const ALLOC_KSWAPD: u32 = 0x800; /* allow waking of kswapd */
/* Avoid alloc_tag recursion for internal allocations. */
pub const ALLOC_NO_CODETAG: u32 = 0x1000;

/* Flags that allow allocations below the min watermark. */
pub const ALLOC_RESERVES: u32 = ALLOC_NON_BLOCK | ALLOC_MIN_RESERVE | ALLOC_HIGHATOMIC | ALLOC_OOM;

#[repr(C)]
pub struct alloc_context {
    pub zonelist: *mut zonelist,
    pub nodemask: *const nodemask_t,
    pub preferred_zoneref: *mut zoneref,
    pub migratetype: i32,
    pub highest_zoneidx: zone_type,
    pub spread_dirty_pages: bool,
    pub alloc_flags: u32,
}

#[inline]
pub unsafe fn buddy_order(page: *mut page) -> u32 {
    /* PageBuddy() must be checked by the caller */
    page_private(page)
}

#[inline]
pub unsafe fn buddy_order_unsafe(page: *mut page) -> u32 {
    page_private(page) // READ_ONCE(page_private(page))
}

#[inline]
pub unsafe fn page_is_buddy(page: *mut page, buddy: *mut page, order: u32) -> bool {
    if !page_is_guard(buddy) && !PageBuddy(buddy) { return false; }
    if buddy_order(buddy) != order { return false; }
    if page_zone_id(page) != page_zone_id(buddy) { return false; }
    VM_BUG_ON_PAGE(page_count(buddy) != 0, buddy);
    true
}

#[inline]
pub fn __find_buddy_pfn(page_pfn: u64, order: u32) -> u64 {
    page_pfn ^ (1u64 << order)
}

#[inline]
pub unsafe fn find_buddy_page_pfn(page: *mut page, pfn: u64, order: u32, buddy_pfn: *mut u64) -> *mut page {
    let __buddy_pfn = __find_buddy_pfn(pfn, order);
    let buddy = page.offset((__buddy_pfn.wrapping_sub(pfn)) as isize);
    if !buddy_pfn.is_null() { *buddy_pfn = __buddy_pfn; }
    if page_is_buddy(page, buddy, order) { buddy } else { core::ptr::null_mut() }
}

extern "C" {
    pub fn __pageblock_pfn_to_page(start_pfn: u64, end_pfn: u64, zone: *mut zone) -> *mut page;
    pub fn __free_pages_core(page: *mut page, order: u32, context: meminit_context);
    pub fn post_alloc_hook(page: *mut page, order: u32, gfp_flags: gfp_t, alloc_flags: u32);
    pub fn free_pages_prepare(page: *mut page, order: u32) -> bool;
    pub static mut user_min_free_kbytes: i32;
    pub fn __alloc_frozen_pages_noprof(gfp: gfp_t, order: u32, nid: i32, nodemask: *mut nodemask_t, alloc_flags: u32) -> *mut page;
    pub fn free_frozen_pages(page: *mut page, order: u32);
    pub fn free_unref_folios(fbatch: *mut folio_batch);
    pub fn alloc_frozen_pages_noprof(gfp: gfp_t, order: u32) -> *mut page;
    pub fn alloc_frozen_pages_nolock_noprof(gfp_flags: gfp_t, nid: i32, order: u32) -> *mut page;
    pub fn free_frozen_pages_nolock(page: *mut page, order: u32);
    pub fn __alloc_pages_noprof(gfp: gfp_t, order: u32, preferred_nid: i32, nodemask: *mut nodemask_t, alloc_flags: u32) -> *mut page;
    pub fn zone_pcp_reset(zone: *mut zone);
    pub fn zone_pcp_disable(zone: *mut zone);
    pub fn zone_pcp_enable(zone: *mut zone);
    pub fn zone_pcp_init(zone: *mut zone);
}

#[inline]
pub unsafe fn pageblock_pfn_to_page(start_pfn: u64, end_pfn: u64, zone: *mut zone) -> *mut page {
    if (*zone).contiguous { pfn_to_page(start_pfn) } else { __pageblock_pfn_to_page(start_pfn, end_pfn, zone) }
}

#[inline]
pub unsafe fn free_area_empty(area: *mut free_area, migratetype: i32) -> bool {
    list_empty(&mut (*area).free_list[migratetype as usize])
}

pub const GFP_MOVABLE_SHIFT: u32 = 3;
#[inline]
pub unsafe fn gfp_migratetype(gfp_flags: gfp_t) -> i32 {
    VM_WARN_ON((gfp_flags & (__GFP_RECLAIMABLE | __GFP_MOVABLE)) == (__GFP_RECLAIMABLE | __GFP_MOVABLE));
    BUILD_BUG_ON((1u64 << GFP_MOVABLE_SHIFT) != ___GFP_MOVABLE);
    BUILD_BUG_ON((___GFP_MOVABLE >> GFP_MOVABLE_SHIFT) != MIGRATE_MOVABLE);
    BUILD_BUG_ON((___GFP_RECLAIMABLE >> GFP_MOVABLE_SHIFT) != MIGRATE_RECLAIMABLE);
    BUILD_BUG_ON((((___GFP_MOVABLE | ___GFP_RECLAIMABLE) >> GFP_MOVABLE_SHIFT) != MIGRATE_HIGHATOMIC));
    if unlikely(page_group_by_mobility_disabled) { return MIGRATE_UNMOVABLE; }
    ((gfp_flags & (__GFP_RECLAIMABLE | __GFP_MOVABLE)) >> GFP_MOVABLE_SHIFT) as i32
}

#[repr(C)]
pub enum fallback_result { FALLBACK_FOUND, FALLBACK_EMPTY, FALLBACK_NOCLAIM }

extern "C" {
    pub fn find_suitable_fallback(area: *mut free_area, order: u32, migratetype: i32, claimable: bool, mt_out: *mut i32) -> fallback_result;
    pub fn decay_pcp_high(zone: *mut zone, pcp: *mut per_cpu_pages) -> bool;
    pub fn drain_zone_pages(zone: *mut zone, pcp: *mut per_cpu_pages);
    pub fn drain_all_pages(zone: *mut zone);
    pub fn page_alloc_init_cpuhp();
    pub fn page_alloc_sysctl_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
