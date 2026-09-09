/* SPDX-License-Identifier: GPL-2.0 */

// The declarations below refer to types and symbols supplied by the kernel
// translation unit that includes this header.

#[cfg(CONFIG_MEMORY_ISOLATION)]
#[inline]
pub unsafe fn is_migrate_isolate_page(page: *mut page) -> bool {
    get_pageblock_migratetype(page) == MIGRATE_ISOLATE
}

#[cfg(CONFIG_MEMORY_ISOLATION)]
#[inline]
pub const unsafe fn is_migrate_isolate(migratetype: i32) -> bool {
    migratetype == MIGRATE_ISOLATE
}

#[cfg(CONFIG_MEMORY_ISOLATION)]
#[inline]
pub unsafe fn get_pageblock_isolate(page: *mut page) -> bool {
    get_pfnblock_bit(page, page_to_pfn(page), PB_migrate_isolate)
}

#[cfg(CONFIG_MEMORY_ISOLATION)]
#[inline]
pub unsafe fn clear_pageblock_isolate(page: *mut page) {
    clear_pfnblock_bit(page, page_to_pfn(page), PB_migrate_isolate)
}

#[cfg(CONFIG_MEMORY_ISOLATION)]
#[inline]
pub unsafe fn set_pageblock_isolate(page: *mut page) {
    set_pfnblock_bit(page, page_to_pfn(page), PB_migrate_isolate)
}

#[cfg(not(CONFIG_MEMORY_ISOLATION))]
#[inline]
pub const unsafe fn is_migrate_isolate_page(_page: *mut page) -> bool {
    false
}

#[cfg(not(CONFIG_MEMORY_ISOLATION))]
#[inline]
pub const unsafe fn is_migrate_isolate(_migratetype: i32) -> bool {
    false
}

#[cfg(not(CONFIG_MEMORY_ISOLATION))]
#[inline]
pub const unsafe fn get_pageblock_isolate(_page: *mut page) -> bool {
    false
}

#[cfg(not(CONFIG_MEMORY_ISOLATION))]
#[inline]
pub unsafe fn clear_pageblock_isolate(_page: *mut page) {}

#[cfg(not(CONFIG_MEMORY_ISOLATION))]
#[inline]
pub unsafe fn set_pageblock_isolate(_page: *mut page) {}

/*
 * Pageblock isolation modes:
 * PB_ISOLATE_MODE_MEM_OFFLINE - isolate to offline (!allocate) memory
 *                               e.g., skip over PageHWPoison() pages and
 *                               PageOffline() pages. Unmovable pages will be
 *                               reported in this mode.
 * PB_ISOLATE_MODE_CMA_ALLOC   - isolate for CMA allocations
 * PB_ISOLATE_MODE_OTHER       - isolate for other purposes
 */
#[repr(C)]
pub enum pb_isolate_mode {
    PB_ISOLATE_MODE_MEM_OFFLINE,
    PB_ISOLATE_MODE_CMA_ALLOC,
    PB_ISOLATE_MODE_OTHER,
}

extern "C" {
    pub fn init_pageblock_migratetype(
        page: *mut page,
        migratetype: migratetype,
        isolate: bool,
    );

    pub fn pageblock_isolate_and_move_free_pages(zone: *mut zone, page: *mut page) -> bool;
    pub fn pageblock_unisolate_and_move_free_pages(zone: *mut zone, page: *mut page) -> bool;

    pub fn start_isolate_page_range(
        start_pfn: c_ulong,
        end_pfn: c_ulong,
        mode: pb_isolate_mode,
    ) -> i32;

    pub fn undo_isolate_page_range(start_pfn: c_ulong, end_pfn: c_ulong);

    pub fn test_pages_isolated(
        start_pfn: c_ulong,
        end_pfn: c_ulong,
        mode: pb_isolate_mode,
    ) -> i32;

    pub fn page_is_unmovable(
        zone: *mut zone,
        page: *mut page,
        mode: pb_isolate_mode,
        step: *mut c_ulong,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
