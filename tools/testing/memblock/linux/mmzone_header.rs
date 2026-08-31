/* SPDX-License-Identifier: GPL-2.0 */

/* From <linux/atomic.h>: atomic_long_t is an external dependency. */
/* From <linux/memory_hotplug.h>: memory hotplug declarations are external dependencies. */

unsafe extern "C" {
    pub fn first_online_pgdat() -> *mut pglist_data;
    pub fn next_online_pgdat(pgdat: *mut pglist_data) -> *mut pglist_data;
}

macro_rules! for_each_online_pgdat {
    ($pgdat:ident, $body:block) => {{
        $pgdat = unsafe { first_online_pgdat() };
        while !$pgdat.is_null() {
            $body
            $pgdat = unsafe { next_online_pgdat($pgdat) };
        }
    }};
}

#[repr(C)]
pub enum zone_type {
    __MAX_NR_ZONES,
}

pub const MAX_NR_ZONES: usize = zone_type::__MAX_NR_ZONES as usize;
pub const MAX_PAGE_ORDER: usize = 10;
pub const MAX_ORDER_NR_PAGES: usize = 1usize << MAX_PAGE_ORDER;

pub const pageblock_order: usize = MAX_PAGE_ORDER;
pub const pageblock_nr_pages: usize = 1usize << pageblock_order;

macro_rules! pageblock_align {
    ($pfn:expr) => {
        ALIGN!(($pfn), pageblock_nr_pages)
    };
}

macro_rules! pageblock_start_pfn {
    ($pfn:expr) => {
        ALIGN_DOWN!(($pfn), pageblock_nr_pages)
    };
}

#[repr(C)]
pub struct zone {
    pub managed_pages: atomic_long_t,
}

#[repr(C)]
pub struct pglist_data {
    pub node_zones: [zone; MAX_NR_ZONES],
}

pub type pg_data_t = pglist_data;

#[repr(C)]
pub enum migratetype {
    MIGRATE_CMA,
}
