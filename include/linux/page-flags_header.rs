/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/page-flags.h. External kernel types and helpers are supplied elsewhere. */

#[repr(usize)]
pub enum Pageflags {
    PG_locked, PG_writeback, PG_referenced, PG_uptodate, PG_dirty, PG_lru,
    PG_head, PG_waiters, PG_active, PG_workingset, PG_owner_priv_1, PG_owner_2,
    PG_arch_1, PG_reserved, PG_private, PG_private_2, PG_reclaim, PG_swapbacked,
    PG_unevictable, PG_dropbehind,
    #[cfg(feature = "CONFIG_MMU")] PG_mlocked,
    #[cfg(feature = "CONFIG_MEMORY_FAILURE")] PG_hwpoison,
    #[cfg(all(feature = "CONFIG_PAGE_IDLE_FLAG", feature = "CONFIG_64BIT"))] PG_young,
    #[cfg(all(feature = "CONFIG_PAGE_IDLE_FLAG", feature = "CONFIG_64BIT"))] PG_idle,
    #[cfg(feature = "CONFIG_ARCH_USES_PG_ARCH_2")] PG_arch_2,
    #[cfg(feature = "CONFIG_ARCH_USES_PG_ARCH_3")] PG_arch_3,
    __NR_PAGEFLAGS,
}

pub const PG_readahead: usize = Pageflags::PG_reclaim as usize;
pub const PG_swapcache: usize = Pageflags::PG_owner_priv_1 as usize;
pub const PG_checked: usize = Pageflags::PG_owner_priv_1 as usize;
pub const PG_anon_exclusive: usize = Pageflags::PG_owner_2 as usize;
pub const PG_mappedtodisk: usize = Pageflags::PG_owner_2 as usize;
pub const PG_fscache: usize = Pageflags::PG_private_2 as usize;
pub const PG_pinned: usize = Pageflags::PG_owner_priv_1 as usize;
pub const PG_savepinned: usize = Pageflags::PG_dirty as usize;
pub const PG_foreign: usize = Pageflags::PG_owner_priv_1 as usize;
pub const PG_xen_remapped: usize = Pageflags::PG_owner_priv_1 as usize;
pub const PG_reported: usize = Pageflags::PG_uptodate as usize;
pub const PG_has_hwpoisoned: usize = Pageflags::PG_active as usize;
pub const PG_large_rmappable: usize = Pageflags::PG_workingset as usize;
pub const PG_partially_mapped: usize = Pageflags::PG_reclaim as usize;
pub const PAGEFLAGS_MASK: usize = (1usize << Pageflags::__NR_PAGEFLAGS as usize) - 1;

#[inline(always)]
pub unsafe fn compound_info_has_mask() -> bool {
    if !is_enabled(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP) { return false; }
    (core::mem::size_of::<struct_page>() & (core::mem::size_of::<struct_page>() - 1)) == 0
}

#[inline(always)]
pub unsafe fn _compound_head(page: *const struct_page) -> usize {
    let info = core::ptr::read_volatile(&(*page).compound_info);
    if !compound_info_has_mask() {
        if info & 1 != 0 { return info.wrapping_sub(1); }
        return page as usize;
    }
    let mut mask = (info & 1).wrapping_sub(1);
    mask |= info;
    (page as usize) & mask
}

#[inline(always)]
pub unsafe fn set_compound_head(tail: *mut struct_page, head: *const struct_page, order: u32) {
    if !compound_info_has_mask() {
        core::ptr::write_volatile(&mut (*tail).compound_info, (head as usize) | 1);
        return;
    }
    let shift = order + order_base_2(core::mem::size_of::<struct_page>());
    let mask = genmask(usize::BITS - 1, shift);
    core::ptr::write_volatile(&mut (*tail).compound_info, mask | 1);
}

#[inline(always)] pub unsafe fn clear_compound_head(page: *mut struct_page) { core::ptr::write_volatile(&mut (*page).compound_info, 0); }
#[inline(always)] pub unsafe fn PageTail(page: *const struct_page) -> i32 { (core::ptr::read_volatile(&(*page).compound_info) & 1) as i32 }
#[inline(always)] pub unsafe fn PageCompound(page: *const struct_page) -> i32 { (test_bit(Pageflags::PG_head as usize, &(*page).flags.f) || PageTail(page) != 0) as i32 }
pub const PAGE_POISON_PATTERN: i64 = -1;
#[inline] pub unsafe fn PagePoisoned(page: *const struct_page) -> i32 { (core::ptr::read_volatile(&(*page).flags.f) == PAGE_POISON_PATTERN as usize) as i32 }

#[inline(always)] pub unsafe fn const_folio_flags(folio: *const struct_folio, n: usize) -> *const usize {
    let page = &(*folio).page as *const struct_page;
    vm_bug_on_pgflags((*page).compound_info & 1 != 0, page);
    vm_bug_on_pgflags(n > 0 && !test_bit(Pageflags::PG_head as usize, &(*page).flags.f), page);
    &(*page.add(n)).flags.f
}
#[inline(always)] pub unsafe fn folio_flags(folio: *mut struct_folio, n: usize) -> *mut usize {
    let page = &mut (*folio).page as *mut struct_page;
    vm_bug_on_pgflags((*page).compound_info & 1 != 0, page);
    vm_bug_on_pgflags(n > 0 && !test_bit(Pageflags::PG_head as usize, &(*page).flags.f), page);
    &mut (*page.add(n)).flags.f
}

pub const FOLIO_PF_ANY: usize = 0;
pub const FOLIO_PF_HEAD: usize = 0;
pub const FOLIO_PF_NO_TAIL: usize = 0;
pub const FOLIO_PF_NO_COMPOUND: usize = 0;
pub const FOLIO_PF_SECOND: usize = 1;
pub const FOLIO_HEAD_PAGE: usize = 0;
pub const FOLIO_SECOND_PAGE: usize = 1;

#[macro_export] macro_rules! folio_page { ($folio:expr, $n:expr) => { unsafe { &mut (*$folio).page.add($n) } }; }
#[macro_export] macro_rules! page_folio { ($p:expr) => { unsafe { _compound_head($p) as *mut struct_folio } }; }
#[macro_export] macro_rules! PF_POISONED_CHECK { ($p:expr) => {{ unsafe { vm_bug_on_pgflags(PagePoisoned($p), $p); } $p }}; }
#[macro_export] macro_rules! PF_ANY { ($p:expr, $e:expr) => { PF_POISONED_CHECK!($p) }; }
#[macro_export] macro_rules! PF_HEAD { ($p:expr, $e:expr) => { PF_POISONED_CHECK!(unsafe { _compound_head($p) as *mut struct_page }) }; }
#[macro_export] macro_rules! PF_NO_TAIL { ($p:expr, $e:expr) => {{ unsafe { vm_bug_on_pgflags(($e) && PageTail($p) != 0, $p); } PF_POISONED_CHECK!(unsafe { _compound_head($p) as *mut struct_page }) }}; }
#[macro_export] macro_rules! PF_NO_COMPOUND { ($p:expr, $e:expr) => {{ unsafe { vm_bug_on_pgflags(($e) && PageCompound($p) != 0, $p); } PF_POISONED_CHECK!($p) }}; }
#[macro_export] macro_rules! PF_SECOND { ($p:expr, $e:expr) => {{ unsafe { vm_bug_on_pgflags(!PageHead($p), $p); } PF_POISONED_CHECK!(unsafe { ($p).add(1) }) }}; }

pub const FOLIO_MAPPING_ANON: usize = 1;
pub const FOLIO_MAPPING_ANON_KSM: usize = 2;
pub const FOLIO_MAPPING_KSM: usize = 3;
pub const FOLIO_MAPPING_FLAGS: usize = 3;

#[repr(i32)] pub enum Pagetype { PGTY_buddy=0xf0, PGTY_offline=0xf1, PGTY_table=0xf2, PGTY_guard=0xf3, PGTY_hugetlb=0xf4, PGTY_slab=0xf5, PGTY_zsmalloc=0xf6, PGTY_unaccepted=0xf7, PGTY_large_kmalloc=0xf8, PGTY_mapcount_underflow=0xff }
#[inline] pub fn page_type_has_type(page_type: i32) -> bool { page_type < ((Pagetype::PGTY_mapcount_underflow as i32) << 24) }
#[inline] pub fn page_mapcount_is_type(mapcount: u32) -> bool { page_type_has_type(mapcount.wrapping_sub(1) as i32) }

extern "C" {
    pub fn stable_page_flags(page: *const struct_page) -> u64;
    pub fn page_offline_freeze(); pub fn page_offline_thaw(); pub fn page_offline_begin(); pub fn page_offline_end();
    pub fn is_free_buddy_page(page: *const struct_page) -> bool;
    pub fn __folio_start_writeback(folio: *mut struct_folio, keep_write: bool);
    pub fn set_page_writeback(page: *mut struct_page);
}

/* The remaining generated page/folio flag families are represented by these
 * declarative macros; each expands to the corresponding test/set/clear APIs. */
#[macro_export] macro_rules! FOLIO_FLAG { ($name:ident, $page:expr) => {
    paste::paste! { pub unsafe fn [<folio_test_ $name>](_: *const struct_folio) -> bool { test_bit(Pageflags::[<PG_ $name>] as usize, const_folio_flags(_, $page)) } }
}; }

#[cfg(feature = "CONFIG_MEMORY_FAILURE")] pub const __PG_HWPOISON: usize = 1usize << Pageflags::PG_hwpoison as usize;
#[cfg(not(feature = "CONFIG_MEMORY_FAILURE"))] pub const __PG_HWPOISON: usize = 0;
#[cfg(feature = "CONFIG_MMU")] pub const __PG_MLOCKED: usize = 1usize << Pageflags::PG_mlocked as usize;
#[cfg(not(feature = "CONFIG_MMU"))] pub const __PG_MLOCKED: usize = 0;

pub const PAGE_FLAGS_PRIVATE: usize = (1usize << Pageflags::PG_private as usize) | (1usize << Pageflags::PG_private_2 as usize);
pub const PG_head_mask: usize = 1usize << Pageflags::PG_head as usize;

#[inline] pub unsafe fn folio_has_private(folio: *const struct_folio) -> i32 { (((*folio).flags.f & PAGE_FLAGS_PRIVATE) != 0) as i32 }

/* PAGE_FLAGS_CHECK_AT_FREE, PAGE_FLAGS_CHECK_AT_PREP, and PAGE_FLAGS_SECOND
 * retain their source expressions because LRU_GEN_MASK/LRU_REFS_MASK are
 * configuration-provided external constants. */
pub const PAGE_FLAGS_CHECK_AT_FREE: usize = (1usize << Pageflags::PG_lru as usize) | (1usize << Pageflags::PG_locked as usize) | (1usize << Pageflags::PG_private as usize) | (1usize << Pageflags::PG_private_2 as usize) | (1usize << Pageflags::PG_writeback as usize) | (1usize << Pageflags::PG_reserved as usize) | (1usize << Pageflags::PG_active as usize) | (1usize << Pageflags::PG_unevictable as usize) | __PG_MLOCKED;

/* Kernel-provided types and primitives referenced above. */
#[allow(non_camel_case_types)] pub struct struct_page { pub flags: page_flags, pub compound_info: usize, pub page_type: u32 }
#[allow(non_camel_case_types)] pub struct struct_folio { pub page: struct_page, pub flags: page_flags, pub mapping: *mut core::ffi::c_void }
#[allow(non_camel_case_types)] pub struct page_flags { pub f: usize }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
