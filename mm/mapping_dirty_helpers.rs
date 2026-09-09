// SPDX-License-Identifier: GPL-2.0

// Kernel headers and symbols referenced by this translation are supplied by
// other compilation units.

use core::ffi::c_int;

pub type CBool = bool;
pub type PgoffT = usize;
pub type VmFlagsT = usize;

#[repr(C)]
pub struct pte_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pmd_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pud_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct address_space {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct vm_area_struct {
    pub vm_start: usize,
    _private: [u8; 0],
}
#[repr(C)]
pub struct mmu_notifier_range {
    pub start: usize,
    pub end: usize,
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_walk {
    pub vma: *mut vm_area_struct,
    pub mm: *mut mm_struct,
    pub private: *mut core::ffi::c_void,
    pub action: c_int,
}

#[repr(C)]
pub struct mm_walk_ops {
    pub pte_entry: Option<unsafe extern "C" fn(*mut pte_t, usize, usize, *mut mm_walk) -> c_int>,
    pub pmd_entry: Option<unsafe extern "C" fn(*mut pmd_t, usize, usize, *mut mm_walk) -> c_int>,
    pub pud_entry: Option<unsafe extern "C" fn(*mut pud_t, usize, usize, *mut mm_walk) -> c_int>,
    pub test_walk: Option<unsafe extern "C" fn(usize, usize, *mut mm_walk) -> c_int>,
    pub pre_vma: Option<unsafe extern "C" fn(usize, usize, *mut mm_walk) -> c_int>,
    pub post_vma: Option<unsafe extern "C" fn(*mut mm_walk)>,
}

pub const ACTION_CONTINUE: c_int = 0;
pub const MMU_NOTIFY_PROTECTION_PAGE: c_int = 0;
pub const VM_SHARED: VmFlagsT = 0;
pub const VM_MAYWRITE: VmFlagsT = 0;
pub const VM_HUGETLB: VmFlagsT = 0;
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SHIFT: usize = 12;

unsafe extern "C" {
    fn ptep_get(pte: *mut pte_t) -> pte_t;
    fn pte_write(pte: pte_t) -> bool;
    fn ptep_modify_prot_start(vma: *mut vm_area_struct, addr: usize, pte: *mut pte_t) -> pte_t;
    fn pte_wrprotect(pte: pte_t) -> pte_t;
    fn ptep_modify_prot_commit(vma: *mut vm_area_struct, addr: usize, pte: *mut pte_t, old: pte_t, new: pte_t);
    fn pte_dirty(pte: pte_t) -> bool;
    fn pte_mkclean(pte: pte_t) -> pte_t;
    fn pmdp_get_lockless(pmd: *mut pmd_t) -> pmd_t;
    fn pmd_trans_huge(pmd: pmd_t) -> bool;
    fn pmd_write(pmd: pmd_t) -> bool;
    fn pmd_dirty(pmd: pmd_t) -> bool;
    fn pudp_get(pud: *mut pud_t) -> pud_t;
    fn pud_trans_huge(pud: pud_t) -> bool;
    fn pud_write(pud: pud_t) -> bool;
    fn pud_dirty(pud: pud_t) -> bool;
    fn warn_on(condition: bool) -> bool;
    fn mmu_notifier_range_init(range: *mut mmu_notifier_range, event: c_int, flags: usize, mm: *mut mm_struct, start: usize, end: usize);
    fn mmu_notifier_invalidate_range_start(range: *mut mmu_notifier_range);
    fn mmu_notifier_invalidate_range_end(range: *mut mmu_notifier_range);
    fn flush_cache_range(vma: *mut vm_area_struct, start: usize, end: usize);
    fn inc_tlb_flush_pending(mm: *mut mm_struct);
    fn dec_tlb_flush_pending(mm: *mut mm_struct);
    fn mm_tlb_flush_nested(mm: *mut mm_struct) -> bool;
    fn flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    fn read_once(value: *mut VmFlagsT) -> VmFlagsT;
    fn vma_start_pgoff(vma: *mut vm_area_struct) -> PgoffT;
    fn i_mmap_lock_read(mapping: *mut address_space);
    fn i_mmap_unlock_read(mapping: *mut address_space);
    fn walk_page_mapping(mapping: *mut address_space, first_index: PgoffT, nr: PgoffT, ops: *const mm_walk_ops, private: *mut core::ffi::c_void) -> c_int;
    fn __set_bit(bit: PgoffT, bitmap: *mut usize);
}

#[repr(C)]
pub struct wp_walk {
    pub range: mmu_notifier_range,
    pub tlbflush_start: usize,
    pub tlbflush_end: usize,
    pub total: usize,
}

unsafe extern "C" fn wp_pte(pte: *mut pte_t, addr: usize, _end: usize, walk: *mut mm_walk) -> c_int {
    let wpwalk = (*walk).private as *mut wp_walk;
    let mut ptent = ptep_get(pte);

    if pte_write(ptent) {
        let old_pte = ptep_modify_prot_start((*walk).vma, addr, pte);
        ptent = pte_wrprotect(old_pte);
        ptep_modify_prot_commit((*walk).vma, addr, pte, old_pte, ptent);
        (*wpwalk).total += 1;
        (*wpwalk).tlbflush_start = (*wpwalk).tlbflush_start.min(addr);
        (*wpwalk).tlbflush_end = (*wpwalk).tlbflush_end.max(addr + PAGE_SIZE);
    }

    0
}

#[repr(C)]
pub struct clean_walk {
    pub base: wp_walk,
    pub bitmap_pgoff: PgoffT,
    pub bitmap: *mut usize,
    pub start: PgoffT,
    pub end: PgoffT,
}

unsafe extern "C" fn clean_record_pte(pte: *mut pte_t, addr: usize, _end: usize, walk: *mut mm_walk) -> c_int {
    let wpwalk = (*walk).private as *mut wp_walk;
    let cwalk = wpwalk as *mut clean_walk;
    let mut ptent = ptep_get(pte);

    if pte_dirty(ptent) {
        let pgoff = ((addr - (*(*walk).vma).vm_start) >> PAGE_SHIFT)
            + vma_start_pgoff((*walk).vma) - (*cwalk).bitmap_pgoff;
        let old_pte = ptep_modify_prot_start((*walk).vma, addr, pte);
        ptent = pte_mkclean(old_pte);
        ptep_modify_prot_commit((*walk).vma, addr, pte, old_pte, ptent);

        (*wpwalk).total += 1;
        (*wpwalk).tlbflush_start = (*wpwalk).tlbflush_start.min(addr);
        (*wpwalk).tlbflush_end = (*wpwalk).tlbflush_end.max(addr + PAGE_SIZE);
        __set_bit(pgoff, (*cwalk).bitmap);
        (*cwalk).start = (*cwalk).start.min(pgoff);
        (*cwalk).end = (*cwalk).end.max(pgoff + 1);
    }

    0
}

unsafe extern "C" fn wp_clean_pmd_entry(pmd: *mut pmd_t, _addr: usize, _end: usize, walk: *mut mm_walk) -> c_int {
    let pmdval = pmdp_get_lockless(pmd);
    if pmd_trans_huge(pmdval) {
        warn_on(pmd_write(pmdval) || pmd_dirty(pmdval));
        (*walk).action = ACTION_CONTINUE;
    }
    0
}

unsafe extern "C" fn wp_clean_pud_entry(pud: *mut pud_t, _addr: usize, _end: usize, walk: *mut mm_walk) -> c_int {
    // CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD
    let pudval = pudp_get(pud);
    if pud_trans_huge(pudval) {
        warn_on(pud_write(pudval) || pud_dirty(pudval));
        (*walk).action = ACTION_CONTINUE;
    }
    0
}

unsafe extern "C" fn wp_clean_pre_vma(start: usize, end: usize, walk: *mut mm_walk) -> c_int {
    let wpwalk = (*walk).private as *mut wp_walk;
    (*wpwalk).tlbflush_start = end;
    (*wpwalk).tlbflush_end = start;
    mmu_notifier_range_init(&mut (*wpwalk).range, MMU_NOTIFY_PROTECTION_PAGE, 0, (*walk).mm, start, end);
    mmu_notifier_invalidate_range_start(&mut (*wpwalk).range);
    flush_cache_range((*walk).vma, start, end);
    inc_tlb_flush_pending((*walk).mm);
    0
}

unsafe extern "C" fn wp_clean_post_vma(walk: *mut mm_walk) {
    let wpwalk = (*walk).private as *mut wp_walk;
    if mm_tlb_flush_nested((*walk).mm) {
        flush_tlb_range((*walk).vma, (*wpwalk).range.start, (*wpwalk).range.end);
    } else if (*wpwalk).tlbflush_end > (*wpwalk).tlbflush_start {
        flush_tlb_range((*walk).vma, (*wpwalk).tlbflush_start, (*wpwalk).tlbflush_end);
    }
    mmu_notifier_invalidate_range_end(&mut (*wpwalk).range);
    dec_tlb_flush_pending((*walk).mm);
}

unsafe extern "C" fn wp_clean_test_walk(_start: usize, _end: usize, walk: *mut mm_walk) -> c_int {
    let vm_flags = read_once(&mut (*(*walk).vma).vm_start as *mut usize);
    if (vm_flags & (VM_SHARED | VM_MAYWRITE | VM_HUGETLB)) != (VM_SHARED | VM_MAYWRITE) { 1 } else { 0 }
}

pub static clean_walk_ops: mm_walk_ops = mm_walk_ops {
    pte_entry: Some(clean_record_pte), pmd_entry: Some(wp_clean_pmd_entry),
    pud_entry: Some(wp_clean_pud_entry), test_walk: Some(wp_clean_test_walk),
    pre_vma: Some(wp_clean_pre_vma), post_vma: Some(wp_clean_post_vma),
};

pub static wp_walk_ops: mm_walk_ops = mm_walk_ops {
    pte_entry: Some(wp_pte), pmd_entry: Some(wp_clean_pmd_entry),
    pud_entry: Some(wp_clean_pud_entry), test_walk: Some(wp_clean_test_walk),
    pre_vma: Some(wp_clean_pre_vma), post_vma: Some(wp_clean_post_vma),
};

pub unsafe extern "C" fn wp_shared_mapping_range(mapping: *mut address_space, first_index: PgoffT, nr: PgoffT) -> usize {
    let mut wpwalk: wp_walk = core::mem::zeroed();
    i_mmap_lock_read(mapping);
    warn_on(walk_page_mapping(mapping, first_index, nr, &wp_walk_ops, &mut wpwalk as *mut _ as *mut core::ffi::c_void) != 0);
    i_mmap_unlock_read(mapping);
    wpwalk.total
}

pub unsafe extern "C" fn clean_record_shared_mapping_range(mapping: *mut address_space, first_index: PgoffT, nr: PgoffT, bitmap_pgoff: PgoffT, bitmap: *mut usize, start: *mut PgoffT, end: *mut PgoffT) -> usize {
    let none_set = *start >= *end;
    let mut cwalk: clean_walk = core::mem::zeroed();
    cwalk.bitmap_pgoff = bitmap_pgoff;
    cwalk.bitmap = bitmap;
    cwalk.start = if none_set { nr } else { *start };
    cwalk.end = if none_set { 0 } else { *end };
    i_mmap_lock_read(mapping);
    warn_on(walk_page_mapping(mapping, first_index, nr, &clean_walk_ops, &mut cwalk.base as *mut _ as *mut core::ffi::c_void) != 0);
    i_mmap_unlock_read(mapping);
    *start = cwalk.start;
    *end = cwalk.end;
    cwalk.base.total
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
