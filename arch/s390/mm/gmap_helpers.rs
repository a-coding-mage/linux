// SPDX-License-Identifier: GPL-2.0
/*
 *  Helper functions for KVM guest address space mapping code
 *
 *    Copyright IBM Corp. 2007, 2025
 */

// Kernel dependencies supplied by other translation units.

#[allow(non_camel_case_types)]
type c_int = i32;
#[allow(non_camel_case_types)]
type c_ulong = usize;

#[repr(C)] pub struct mm_struct { pub context: mm_context }
#[repr(C)] pub struct mm_context { pub allow_cow_sharing: bool }
#[repr(C)] pub struct vm_area_struct { pub vm_flags: usize, pub vm_start: usize, pub vm_end: usize }
#[repr(C)] pub struct mm_walk { pub private: *mut core::ffi::c_void, pub vma: *mut vm_area_struct }
#[repr(C)] pub struct pte_t;
#[repr(C)] pub struct pmd_t;
#[repr(C)] pub struct pud_t;
#[repr(C)] pub struct p4d_t;
#[repr(C)] pub struct pgd_t;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct softleaf_t;
#[repr(C)] pub struct mm_walk_ops {
    pub pte_entry: Option<unsafe extern "C" fn(*mut pte_t, usize, usize, *mut mm_walk) -> c_int>,
    pub walk_lock: c_int,
}

extern "C" {
    static mut current: *mut task_struct;
    fn pgd_offset(mm: *mut mm_struct, addr: usize) -> *mut pgd_t;
    fn pgdp_get(p: *mut pgd_t) -> pgd_t;
    fn pgd_none(p: pgd_t) -> bool; fn pgd_present(p: pgd_t) -> bool;
    fn p4d_offset_lockless(p: *mut pgd_t, v: pgd_t, addr: usize) -> *mut p4d_t;
    fn p4dp_get(p: *mut p4d_t) -> p4d_t;
    fn p4d_none(p: p4d_t) -> bool; fn p4d_present(p: p4d_t) -> bool;
    fn pud_offset_lockless(p: *mut p4d_t, v: p4d_t, addr: usize) -> *mut pud_t;
    fn pudp_get(p: *mut pud_t) -> pud_t;
    fn pud_none(p: pud_t) -> bool; fn pud_leaf(p: pud_t) -> bool; fn pud_present(p: pud_t) -> bool;
    fn pmd_offset_lockless(p: *mut pud_t, v: pud_t, addr: usize) -> *mut pmd_t;
    fn pmdp_get_lockless(p: *mut pmd_t) -> pmd_t;
    fn pmd_none(p: pmd_t) -> bool; fn pmd_leaf(p: pmd_t) -> bool; fn pmd_present(p: pmd_t) -> bool;
    fn pte_offset_map_rw_nolock(mm: *mut mm_struct, pmd: *mut pmd_t, addr: usize, val: *mut pmd_t, lock: *mut *mut spinlock_t) -> *mut pte_t;
    fn spin_trylock(lock: *mut spinlock_t) -> bool;
    fn pmd_same(a: pmd_t, b: pmd_t) -> bool;
    fn pte_unmap_unlock(pte: *mut pte_t, lock: *mut spinlock_t); fn pte_unmap(pte: *mut pte_t);
    fn vma_lookup(mm: *mut mm_struct, addr: usize) -> *mut vm_area_struct;
    fn is_vm_hugetlb_page(vma: *mut vm_area_struct) -> bool;
    fn mmap_assert_locked(mm: *mut mm_struct); fn mmap_assert_write_locked(mm: *mut mm_struct);
    fn softleaf_from_pte(pte: pte_t) -> softleaf_t; fn pte_swap(pte: pte_t) -> bool;
    fn softleaf_is_swap(sl: softleaf_t) -> bool; fn dec_mm_counter(mm: *mut mm_struct, counter: c_int);
    fn swap_put_entries_direct(sl: softleaf_t, count: usize); fn pte_clear(mm: *mut mm_struct, addr: usize, pte: *mut pte_t);
    fn find_vma_intersection(mm: *mut mm_struct, start: usize, end: usize) -> *mut vm_area_struct;
    fn zap_vma_range(vma: *mut vm_area_struct, start: usize, len: usize);
    fn is_zero_pfn(pfn: usize) -> bool; fn pte_pfn(pte: pte_t) -> usize; fn vma_is_cow_mapping(vma: *mut vm_area_struct) -> bool;
    fn walk_page_range_vma(vma: *mut vm_area_struct, start: usize, end: usize, ops: *const mm_walk_ops, private: *mut usize) -> c_int;
    fn handle_mm_fault(vma: *mut vm_area_struct, addr: usize, flags: c_int, regs: *mut core::ffi::c_void) -> c_int;
    fn ksm_disable(mm: *mut mm_struct) -> c_int;
    fn vma_iterator_first(mm: *mut mm_struct, addr: usize) -> *mut vm_area_struct;
    fn vma_iterator_next(vma: *mut vm_area_struct) -> *mut vm_area_struct;
}
#[repr(C)] pub struct task_struct { pub mm: *mut mm_struct }

const EAGAIN: c_int = 11; const EFAULT: c_int = 14; const ENOMEM: c_int = 12;
const MM_SWAPENTS: c_int = 0; const VM_PFNMAP: usize = 1 << 0;
const PGWALK_WRLOCK: c_int = 1; const FAULT_FLAG_UNSHARE: c_int = 1 << 0;
const FAULT_FLAG_REMOTE: c_int = 1 << 1; const VM_FAULT_OOM: c_int = 1 << 2;
const _PAGE_UNUSED: i64 = 1;

pub unsafe extern "C" fn try_get_locked_pte(mm: *mut mm_struct, vmaddr: usize, ptl: *mut *mut spinlock_t) -> *mut pte_t {
    let pgdp = pgd_offset(mm, vmaddr); let pgd = pgdp_get(pgdp);
    if pgd_none(pgd) || !pgd_present(pgd) { return core::ptr::null_mut(); }
    let p4dp = p4d_offset_lockless(pgdp, pgd, vmaddr); let p4d = p4dp_get(p4dp);
    if p4d_none(p4d) || !p4d_present(p4d) { return core::ptr::null_mut(); }
    let pudp = pud_offset_lockless(p4dp, p4d, vmaddr); let pud = pudp_get(pudp);
    if pud_none(pud) || pud_leaf(pud) || !pud_present(pud) { return core::ptr::null_mut(); }
    let pmdp = pmd_offset_lockless(pudp, pud, vmaddr); let pmd = pmdp_get_lockless(pmdp);
    if pmd_none(pmd) || pmd_leaf(pmd) || !pmd_present(pmd) { return core::ptr::null_mut(); }
    let mut pmdval = pmd; let ptep = pte_offset_map_rw_nolock(mm, pmdp, vmaddr, &mut pmdval, ptl);
    if ptep.is_null() || !spin_trylock(*ptl) { if !ptep.is_null() { pte_unmap(ptep); } return core::ptr::null_mut(); }
    if !pmd_same(pmdval, pmdp_get_lockless(pmdp)) { pte_unmap_unlock(ptep, *ptl); return core::ptr::null_mut(); }
    ptep
}

pub unsafe extern "C" fn gmap_helper_zap_one_page(mm: *mut mm_struct, vmaddr: usize) {
    mmap_assert_locked(mm); let vma = vma_lookup(mm, vmaddr);
    if vma.is_null() || is_vm_hugetlb_page(vma) { return; }
    let mut ptl = core::ptr::null_mut(); let ptep = try_get_locked_pte(mm, vmaddr, &mut ptl);
    if ptep.is_null() { return; }
    let sl = softleaf_from_pte(*ptep); if pte_swap(*ptep) && softleaf_is_swap(sl) { dec_mm_counter(mm, MM_SWAPENTS); swap_put_entries_direct(sl, 1); pte_clear(mm, vmaddr, ptep); }
    pte_unmap_unlock(ptep, ptl);
}

pub unsafe extern "C" fn gmap_helper_discard(mm: *mut mm_struct, mut vmaddr: usize, end: usize) {
    mmap_assert_locked(mm); while vmaddr < end { let vma = find_vma_intersection(mm, vmaddr, end); if vma.is_null() { return; } if !is_vm_hugetlb_page(vma) { zap_vma_range(vma, vmaddr, core::cmp::min(end, (*vma).vm_end) - vmaddr); } vmaddr = (*vma).vm_end; }
}

pub unsafe extern "C" fn gmap_helper_try_set_pte_unused(mm: *mut mm_struct, vmaddr: usize) {
    let mut ptl = core::ptr::null_mut(); let ptep = try_get_locked_pte(mm, vmaddr, &mut ptl); if ptep.is_null() { return; }
    // __atomic64_or(_PAGE_UNUSED, (long *)ptep)
    pte_unmap_unlock(ptep, ptl);
}

unsafe extern "C" fn find_zeropage_pte_entry(pte: *mut pte_t, addr: usize, _end: usize, walk: *mut mm_walk) -> c_int {
    let found_addr = (*walk).private as *mut usize; if is_zero_pfn(pte_pfn(*pte)) { if !vma_is_cow_mapping((*walk).vma) { return -EFAULT; } *found_addr = addr; return 1; } 0
}
static FIND_ZEROPAGE_OPS: mm_walk_ops = mm_walk_ops { pte_entry: Some(find_zeropage_pte_entry), walk_lock: PGWALK_WRLOCK };

unsafe fn __gmap_helper_unshare_zeropages(mm: *mut mm_struct) -> c_int {
    let mut vma = vma_iterator_first(mm, 0);
    while !vma.is_null() {
        /*
         * We could only look at COW mappings, but it's more future
         * proof to catch unexpected zeropages in other mappings and fail.
         */
        if ((*vma).vm_flags & VM_PFNMAP) == 0 && !is_vm_hugetlb_page(vma) {
            let mut addr = (*vma).vm_start;
            'retry: loop {
                let rc = walk_page_range_vma(vma, addr, (*vma).vm_end, &FIND_ZEROPAGE_OPS, &mut addr);
                if rc < 0 { return rc; }
                if rc == 0 { break 'retry; }
                let fault = handle_mm_fault(vma, addr, FAULT_FLAG_UNSHARE | FAULT_FLAG_REMOTE, core::ptr::null_mut());
                if fault & VM_FAULT_OOM != 0 { return -ENOMEM; }
                // handle_mm_fault() may back out; restart from the current address.
            }
        }
        vma = vma_iterator_next(vma);
    }
    0
}

pub unsafe extern "C" fn gmap_helper_disable_cow_sharing() -> c_int {
    let mm = (*current).mm; mmap_assert_write_locked(mm); if !(*mm).context.allow_cow_sharing { return 0; }
    (*mm).context.allow_cow_sharing = false; let mut rc = __gmap_helper_unshare_zeropages(mm); if rc == 0 { rc = ksm_disable(mm); } if rc != 0 { (*mm).context.allow_cow_sharing = true; } rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
