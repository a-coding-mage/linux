/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Rust translation of linux/mmzone.h.
 *
 * Original C header dependencies:
 * - <linux/atomic.h> supplies atomic_long_t.
 * - struct pglist_data is forward-declared before the typedef definition.
 */

extern "C" {
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

pub(crate) use for_each_online_pgdat;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum zone_type {
    __MAX_NR_ZONES = 0,
}

pub const MAX_NR_ZONES: usize = zone_type::__MAX_NR_ZONES as usize;
pub const MAX_PAGE_ORDER: usize = 10;
pub const MAX_ORDER_NR_PAGES: usize = 1usize << MAX_PAGE_ORDER;

pub const pageblock_order: usize = MAX_PAGE_ORDER;
pub const pageblock_nr_pages: usize = 1usize << pageblock_order;

#[inline]
pub const fn pageblock_align(pfn: usize) -> usize {
    (pfn + pageblock_nr_pages - 1) & !(pageblock_nr_pages - 1)
}

#[inline]
pub const fn pageblock_start_pfn(pfn: usize) -> usize {
    pfn & !(pageblock_nr_pages - 1)
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

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
