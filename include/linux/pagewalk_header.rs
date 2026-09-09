/* SPDX-License-Identifier: GPL-2.0 */

// Dependency types and helpers are supplied by the surrounding Linux bindings.

/* Locking requirement during a page walk. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum page_walk_lock {
    /* mmap_lock should be locked for read to stabilize the vma tree */
    PGWALK_RDLOCK = 0,
    /* vma will be write-locked during the walk */
    PGWALK_WRLOCK = 1,
    /* vma is expected to be already write-locked during the walk */
    PGWALK_WRLOCK_VERIFY = 2,
    /* vma is expected to be already read-locked during the walk */
    PGWALK_VMA_RDLOCK_VERIFY = 3,
}

#[repr(C)]
pub struct mm_walk_ops {
    pub pgd_entry: Option<unsafe extern "C" fn(*mut pgd_t, libc::c_ulong, libc::c_ulong, *mut mm_walk) -> libc::c_int>,
    pub p4d_entry: Option<unsafe extern "C" fn(*mut p4d_t, libc::c_ulong, libc::c_ulong, *mut mm_walk) -> libc::c_int>,
    pub pud_entry: Option<unsafe extern "C" fn(*mut pud_t, libc::c_ulong, libc::c_ulong, *mut mm_walk) -> libc::c_int>,
    pub pmd_entry: Option<unsafe extern "C" fn(*mut pmd_t, libc::c_ulong, libc::c_ulong, *mut mm_walk) -> libc::c_int>,
    pub pte_entry: Option<unsafe extern "C" fn(*mut pte_t, libc::c_ulong, libc::c_ulong, *mut mm_walk) -> libc::c_int>,
    pub pte_hole: Option<unsafe extern "C" fn(libc::c_ulong, libc::c_ulong, libc::c_int, *mut mm_walk) -> libc::c_int>,
    pub hugetlb_entry: Option<unsafe extern "C" fn(*mut pte_t, libc::c_ulong, libc::c_ulong, libc::c_ulong, *mut mm_walk) -> libc::c_int>,
    pub test_walk: Option<unsafe extern "C" fn(libc::c_ulong, libc::c_ulong, *mut mm_walk) -> libc::c_int>,
    pub pre_vma: Option<unsafe extern "C" fn(libc::c_ulong, libc::c_ulong, *mut mm_walk) -> libc::c_int>,
    pub post_vma: Option<unsafe extern "C" fn(*mut mm_walk)>,
    pub install_pte: Option<unsafe extern "C" fn(libc::c_ulong, libc::c_ulong, *mut pte_t, *mut mm_walk) -> libc::c_int>,
    pub walk_lock: page_walk_lock,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum page_walk_action {
    /* Descend to next level, splitting huge pages if needed and possible */
    ACTION_SUBTREE = 0,
    /* Continue to next entry at this level (ignoring any subtree) */
    ACTION_CONTINUE = 1,
    /* Call again for this entry */
    ACTION_AGAIN = 2,
}

#[repr(C)]
pub struct mm_walk {
    pub ops: *const mm_walk_ops,
    pub mm: *mut mm_struct,
    pub pgd: *mut pgd_t,
    pub vma: *mut vm_area_struct,
    pub action: page_walk_action,
    pub no_vma: bool,
    pub private: *mut libc::c_void,
}

unsafe extern "C" {
    pub fn walk_page_range(mm: *mut mm_struct, start: libc::c_ulong, end: libc::c_ulong,
        ops: *const mm_walk_ops, private: *mut libc::c_void) -> libc::c_int;
    pub fn walk_kernel_page_table_range(start: libc::c_ulong, end: libc::c_ulong,
        ops: *const mm_walk_ops, pgd: *mut pgd_t, private: *mut libc::c_void) -> libc::c_int;
    pub fn walk_kernel_page_table_range_lockless(start: libc::c_ulong, end: libc::c_ulong,
        ops: *const mm_walk_ops, pgd: *mut pgd_t, private: *mut libc::c_void) -> libc::c_int;
    pub fn walk_page_range_vma(vma: *mut vm_area_struct, start: libc::c_ulong, end: libc::c_ulong,
        ops: *const mm_walk_ops, private: *mut libc::c_void) -> libc::c_int;
    pub fn walk_page_vma(vma: *mut vm_area_struct, ops: *const mm_walk_ops,
        private: *mut libc::c_void) -> libc::c_int;
    pub fn walk_page_mapping(mapping: *mut address_space, first_index: pgoff_t, nr: pgoff_t,
        ops: *const mm_walk_ops, private: *mut libc::c_void) -> libc::c_int;
}

pub type folio_walk_flags_t = u32;

/* Walk shared zeropages (small + huge) as well. */
pub const FW_ZEROPAGE: folio_walk_flags_t = 1u32 << 0;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum folio_walk_level {
    FW_LEVEL_PTE,
    FW_LEVEL_PMD,
    FW_LEVEL_PUD,
}

#[repr(C)]
pub union folio_walk_entry_ptr {
    pub ptep: *mut pte_t,
    pub pudp: *mut pud_t,
    pub pmdp: *mut pmd_t,
}

#[repr(C)]
pub union folio_walk_entry {
    pub pte: pte_t,
    pub pud: pud_t,
    pub pmd: pmd_t,
}

#[repr(C)]
pub struct folio_walk {
    /* public */
    pub page: *mut page,
    pub level: folio_walk_level,
    pub entry_ptr: folio_walk_entry_ptr,
    pub entry: folio_walk_entry,
    /* private */
    pub vma: *mut vm_area_struct,
    pub ptl: *mut spinlock_t,
}

unsafe extern "C" {
    pub fn folio_walk_start(fw: *mut folio_walk, vma: *mut vm_area_struct,
        addr: libc::c_ulong, flags: folio_walk_flags_t) -> *mut folio;
}

/* C macro folio_walk_end(__fw, __vma). */
#[inline]
pub unsafe fn folio_walk_end(fw: *mut folio_walk, vma: *mut vm_area_struct) {
    spin_unlock((*fw).ptl);
    if likely((*fw).level == folio_walk_level::FW_LEVEL_PTE) {
        pte_unmap((*fw).entry_ptr.ptep);
    }
    vma_pgtable_walk_end(vma);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
