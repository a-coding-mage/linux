/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/rmap.h. Included kernel types and operations are
 * supplied by the surrounding translation unit. */

#[repr(C)]
pub struct anon_vma {
    pub root: *mut anon_vma,
    pub rwsem: rw_semaphore,
    pub refcount: atomic_t,
    pub num_children: c_ulong,
    pub num_active_vmas: c_ulong,
    pub parent: *mut anon_vma,
    pub rb_root: rb_root_cached,
}

#[repr(C)]
pub struct anon_vma_chain {
    pub vma: *mut vm_area_struct,
    pub anon_vma: *mut anon_vma,
    pub same_vma: list_head,
    pub rb: rb_node,
    pub rb_subtree_last: c_ulong,
    #[cfg(CONFIG_DEBUG_VM_RB)]
    pub cached_vma_start: c_ulong,
    #[cfg(CONFIG_DEBUG_VM_RB)]
    pub cached_vma_last: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ttu_flags {
    TTU_USE_SHARED_ZEROPAGE = 0x2,
    TTU_SPLIT_HUGE_PMD = 0x4,
    TTU_IGNORE_MLOCK = 0x8,
    TTU_SYNC = 0x10,
    TTU_HWPOISON = 0x20,
    TTU_BATCH_FLUSH = 0x40,
    TTU_RMAP_LOCKED = 0x80,
}

pub type rmap_t = c_int;
pub const RMAP_NONE: rmap_t = 0;
pub const RMAP_EXCLUSIVE: rmap_t = BIT(0) as rmap_t;
pub const PVMW_SYNC: c_uint = 1 << 0;
pub const PVMW_MIGRATION: c_uint = 1 << 1;
pub const PVMW_PGTABLE_CROSSED: c_uint = 1 << 16;

#[repr(C)]
pub struct page_vma_mapped_walk {
    pub pfn: c_ulong,
    pub nr_pages: c_ulong,
    pub pgoff: pgoff_t,
    pub vma: *mut vm_area_struct,
    pub address: c_ulong,
    pub pmd: *mut pmd_t,
    pub pte: *mut pte_t,
    pub ptl: *mut spinlock_t,
    pub flags: c_uint,
    pub pgoff_is_anon: bool,
}

#[repr(C)]
pub struct rmap_walk_control {
    pub arg: *mut c_void,
    pub try_lock: bool,
    pub contended: bool,
    pub rmap_one: Option<unsafe extern "C" fn(*mut folio, *mut vm_area_struct, c_ulong, *mut c_void) -> bool>,
    pub done: Option<unsafe extern "C" fn(*mut folio) -> c_int>,
    pub anon_lock: Option<unsafe extern "C" fn(*const folio, *mut rmap_walk_control) -> *mut anon_vma>,
    pub invalid_vma: Option<unsafe extern "C" fn(*mut vm_area_struct, *mut c_void) -> bool>,
}

extern "C" {
    pub fn anon_vma_init();
    pub fn folio_move_anon_rmap(*mut folio, *mut vm_area_struct);
    pub fn folio_add_anon_rmap_ptes(*mut folio, *mut page, c_int, *mut vm_area_struct, c_ulong, rmap_t);
    pub fn folio_add_anon_rmap_pmd(*mut folio, *mut page, *mut vm_area_struct, c_ulong, rmap_t);
    pub fn folio_add_new_anon_rmap(*mut folio, *mut vm_area_struct, c_ulong, rmap_t);
    pub fn folio_add_file_rmap_ptes(*mut folio, *mut page, c_int, *mut vm_area_struct);
    pub fn folio_add_file_rmap_pmd(*mut folio, *mut page, *mut vm_area_struct);
    pub fn folio_add_file_rmap_pud(*mut folio, *mut page, *mut vm_area_struct);
    pub fn folio_remove_rmap_ptes(*mut folio, *mut page, c_int, *mut vm_area_struct);
    pub fn folio_remove_rmap_pmd(*mut folio, *mut page, *mut vm_area_struct);
    pub fn folio_remove_rmap_pud(*mut folio, *mut page, *mut vm_area_struct);
    pub fn hugetlb_add_anon_rmap(*mut folio, *mut vm_area_struct, c_ulong, rmap_t);
    pub fn hugetlb_add_new_anon_rmap(*mut folio, *mut vm_area_struct, c_ulong);
    pub fn folio_referenced(*mut folio, c_int, *mut mem_cgroup, *mut vma_flags_t) -> c_int;
    pub fn try_to_migrate(*mut folio, ttu_flags);
    pub fn try_to_unmap(*mut folio, ttu_flags);
    pub fn make_device_exclusive(*mut mm_struct, c_ulong, *mut c_void, *mut *mut folio) -> *mut page;
    pub fn page_vma_mapped_walk(*mut page_vma_mapped_walk) -> bool;
    pub fn page_address_in_vma(*const folio, *const page, *const vm_area_struct) -> c_ulong;
    pub fn folio_mkclean(*mut folio) -> c_int;
    pub fn mapping_wrprotect_range(*mut address_space, pgoff_t, c_ulong, c_ulong) -> c_int;
    pub fn pfn_mkclean_range(c_ulong, c_ulong, pgoff_t, *mut vm_area_struct) -> c_int;
    pub fn remove_migration_ptes(*mut folio, *mut folio, ttu_flags);
    pub fn rmap_walk(*mut folio, *mut rmap_walk_control);
    pub fn rmap_walk_locked(*mut folio, *mut rmap_walk_control);
    pub fn folio_lock_anon_vma_read(*const folio, *mut rmap_walk_control) -> *mut anon_vma;
}

#[cfg(CONFIG_MMU)]
extern "C" {
    pub fn hugetlb_try_dup_anon_rmap(*mut folio, *mut vm_area_struct) -> c_int;
    pub fn hugetlb_try_share_anon_rmap(*mut folio) -> c_int;
    pub fn hugetlb_add_file_rmap(*mut folio);
    pub fn hugetlb_remove_rmap(*mut folio);
}

/* Local macro equivalents for the header's one-element convenience wrappers. */
#[macro_export] macro_rules! folio_inc_large_mapcount { ($f:expr,$v:expr) => { unsafe { folio_add_large_mapcount($f,1,$v) } }; }
#[macro_export] macro_rules! folio_dec_large_mapcount { ($f:expr,$v:expr) => { unsafe { folio_sub_large_mapcount($f,1,$v) } }; }
#[macro_export] macro_rules! folio_inc_return_large_mapcount { ($f:expr,$v:expr) => { unsafe { folio_add_return_large_mapcount($f,1,$v) } }; }
#[macro_export] macro_rules! folio_dec_return_large_mapcount { ($f:expr,$v:expr) => { unsafe { folio_sub_return_large_mapcount($f,1,$v) } }; }

/* folio mapcount operations are supplied by the MM implementation. */
extern "C" {
    pub fn folio_set_large_mapcount(*mut folio, c_int, *mut vm_area_struct);
    pub fn folio_add_large_mapcount(*mut folio, c_int, *mut vm_area_struct);
    pub fn folio_add_return_large_mapcount(*mut folio, c_int, *mut vm_area_struct) -> c_int;
    pub fn folio_sub_large_mapcount(*mut folio, c_int, *mut vm_area_struct);
    pub fn folio_sub_return_large_mapcount(*mut folio, c_int, *mut vm_area_struct) -> c_int;
}

#[macro_export]
macro_rules! DEFINE_FOLIO_VMA_WALK {
    ($name:ident, $folio:expr, $vma:expr, $address:expr, $flags:expr) => {
        let mut $name = $crate::page_vma_mapped_walk {
            pfn: unsafe { folio_pfn($folio) }, nr_pages: unsafe { folio_nr_pages($folio) },
            pgoff: unsafe { folio_pgoff($folio) }, vma: $vma, address: $address,
            pmd: core::ptr::null_mut(), pte: core::ptr::null_mut(), ptl: core::ptr::null_mut(),
            flags: $flags, pgoff_is_anon: unsafe { folio_test_anon($folio) },
        };
    };
}

#[cfg(not(CONFIG_MMU))]
#[inline] pub unsafe fn anon_vma_init_nommu() {}

#[inline(always)]
pub unsafe fn folio_add_anon_rmap_pte(f: *mut folio, p: *mut page, v: *mut vm_area_struct, a: c_ulong, flags: rmap_t) {
    folio_add_anon_rmap_ptes(f, p, 1, v, a, flags)
}
#[inline(always)] pub unsafe fn folio_add_file_rmap_pte(f: *mut folio, p: *mut page, v: *mut vm_area_struct) { folio_add_file_rmap_ptes(f,p,1,v) }
#[inline(always)] pub unsafe fn folio_remove_rmap_pte(f: *mut folio, p: *mut page, v: *mut vm_area_struct) { folio_remove_rmap_ptes(f,p,1,v) }

#[inline]
pub unsafe fn page_vma_mapped_walk_done(p: *mut page_vma_mapped_walk) {
    if !(*p).pte.is_null() && !is_vm_hugetlb_page((*p).vma) { pte_unmap((*p).pte); }
    if !(*p).ptl.is_null() { spin_unlock((*p).ptl); }
}
#[inline]
pub unsafe fn page_vma_mapped_walk_restart(p: *mut page_vma_mapped_walk) {
    WARN_ON_ONCE((*p).pmd.is_null() && (*p).pte.is_null());
    if !(*p).ptl.is_null() { spin_unlock((*p).ptl); } else { WARN_ON_ONCE(true); }
    (*p).ptl = core::ptr::null_mut(); (*p).pmd = core::ptr::null_mut(); (*p).pte = core::ptr::null_mut();
}

#[cfg(not(CONFIG_MMU))]
#[inline] pub unsafe fn folio_referenced_nommu(_: *mut folio, _: c_int, _: *mut mem_cgroup, f: *mut vma_flags_t) -> c_int { vma_flags_clear_all(f); 0 }

/* The following helpers preserve the original macro interfaces. */
#[macro_export] macro_rules! folio_add_anon_rmap_pte { ($f:expr,$p:expr,$v:expr,$a:expr,$x:expr) => { unsafe { $crate::folio_add_anon_rmap_ptes($f,$p,1,$v,$a,$x) } }; }
#[macro_export] macro_rules! folio_add_file_rmap_pte { ($f:expr,$p:expr,$v:expr) => { unsafe { $crate::folio_add_file_rmap_ptes($f,$p,1,$v) } }; }
#[macro_export] macro_rules! folio_remove_rmap_pte { ($f:expr,$p:expr,$v:expr) => { unsafe { $crate::folio_remove_rmap_ptes($f,$p,1,$v) } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
