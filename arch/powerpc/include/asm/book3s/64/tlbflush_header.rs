/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the PowerPC Book3S 64 TLB flush header.
// Dependencies supplied by the surrounding kernel translation are intentionally external.

pub const MMU_NO_CONTEXT: usize = usize::MAX;

pub const TLB_INVAL_SCOPE_GLOBAL: i32 = 0;
pub const TLB_INVAL_SCOPE_LPID: i32 = 1;

extern "C" {
    fn early_radix_enabled() -> bool;
    fn radix_enabled() -> bool;
    fn radix__tlbiel_all(scope: i32);
    fn hash__tlbiel_all(scope: i32);
    fn radix__flush_pmd_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    fn radix__flush_pud_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    fn radix__flush_hugetlb_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    fn radix__flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    fn radix__flush_tlb_kernel_range(start: usize, end: usize);
    fn radix__local_flush_tlb_mm(mm: *mut mm_struct);
    fn radix__local_flush_tlb_page(vma: *mut vm_area_struct, vmaddr: usize);
    fn radix__local_flush_tlb_page_psize(mm: *mut mm_struct, vmaddr: usize, psize: i32);
    fn radix__tlb_flush(tlb: *mut mmu_gather);
    fn hash__tlb_flush(tlb: *mut mmu_gather);
    fn radix__flush_tlb_mm(mm: *mut mm_struct);
    fn radix__flush_tlb_page(vma: *mut vm_area_struct, vmaddr: usize);
}

#[repr(C)]
pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)]
pub struct mm_struct { _private: [u8; 0] }
#[repr(C)]
pub struct mmu_gather { _private: [u8; 0] }
#[repr(C)]
pub struct pte_t { pub val: usize }
#[repr(C)]
pub struct pmd_t { pub val: usize }

extern "C" {
    fn pte_val(pte: pte_t) -> usize;
    fn pmd_val(pmd: pmd_t) -> usize;
}

extern "C" {
    static mut tlbie_capable: bool;
    static mut tlbie_enabled: bool;
}

pub unsafe fn tlbiel_all() {
    if early_radix_enabled() {
        radix__tlbiel_all(TLB_INVAL_SCOPE_GLOBAL);
    } else {
        hash__tlbiel_all(TLB_INVAL_SCOPE_GLOBAL);
    }
}

pub unsafe fn tlbiel_all_lpid(radix: bool) {
    if radix {
        radix__tlbiel_all(TLB_INVAL_SCOPE_LPID);
    } else {
        hash__tlbiel_all(TLB_INVAL_SCOPE_LPID);
    }
}

pub unsafe fn flush_pmd_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize) {
    if radix_enabled() { radix__flush_pmd_tlb_range(vma, start, end); }
}

pub unsafe fn flush_pud_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize) {
    if radix_enabled() { radix__flush_pud_tlb_range(vma, start, end); }
}

pub unsafe fn flush_hugetlb_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize) {
    if radix_enabled() { radix__flush_hugetlb_tlb_range(vma, start, end); }
}

pub unsafe fn flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize) {
    if radix_enabled() { radix__flush_tlb_range(vma, start, end); }
}

pub unsafe fn flush_tlb_kernel_range(start: usize, end: usize) {
    if radix_enabled() { radix__flush_tlb_kernel_range(start, end); }
}

pub unsafe fn local_flush_tlb_mm(mm: *mut mm_struct) {
    if radix_enabled() { radix__local_flush_tlb_mm(mm); }
}

pub unsafe fn local_flush_tlb_page(vma: *mut vm_area_struct, vmaddr: usize) {
    if radix_enabled() { radix__local_flush_tlb_page(vma, vmaddr); }
}

pub unsafe fn local_flush_tlb_page_psize(mm: *mut mm_struct, vmaddr: usize, psize: i32) {
    if radix_enabled() { radix__local_flush_tlb_page_psize(mm, vmaddr, psize); }
}

pub unsafe fn tlb_flush(tlb: *mut mmu_gather) {
    if radix_enabled() { radix__tlb_flush(tlb); } else { hash__tlb_flush(tlb); }
}

// CONFIG_SMP selects the distributed implementation; otherwise these aliases are local calls.
#[cfg(CONFIG_SMP)]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    if radix_enabled() { radix__flush_tlb_mm(mm); }
}
#[cfg(not(CONFIG_SMP))]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) { local_flush_tlb_mm(mm); }

#[cfg(CONFIG_SMP)]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, vmaddr: usize) {
    if radix_enabled() { radix__flush_tlb_page(vma, vmaddr); }
}
#[cfg(not(CONFIG_SMP))]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, vmaddr: usize) {
    local_flush_tlb_page(vma, vmaddr);
}

pub unsafe fn flush_tlb_fix_spurious_fault(
    _vma: *mut vm_area_struct, _address: usize, _ptep: *mut pte_t,
) {
    // Book3S 64 does not require spurious fault flushes; the PTE is re-fetched by the MMU.
}

extern "C" {
    fn VM_WARN_ON_ONCE(condition: bool);
}

extern "C" {
    static _PAGE_PRIVILEGED: usize;
    static _PAGE_PTE: usize;
    static _PAGE_PRESENT: usize;
    static _PAGE_RWX: usize;
    static _PAGE_DIRTY: usize;
    static _PAGE_ACCESSED: usize;
}

pub unsafe fn __pte_flags_need_flush(oldval: usize, newval: usize) -> bool {
    let delta = oldval ^ newval;
    if !radix_enabled() { return true; }
    VM_WARN_ON_ONCE(oldval & _PAGE_PRIVILEGED != 0);
    VM_WARN_ON_ONCE(newval & _PAGE_PRIVILEGED != 0);
    VM_WARN_ON_ONCE(oldval & _PAGE_PTE == 0);
    VM_WARN_ON_ONCE(newval & _PAGE_PTE == 0);
    VM_WARN_ON_ONCE(oldval & _PAGE_PRESENT == 0);
    VM_WARN_ON_ONCE(newval & _PAGE_PRESENT == 0);
    if delta & !(_PAGE_RWX | _PAGE_DIRTY | _PAGE_ACCESSED) != 0 { return true; }
    if (delta & !_PAGE_ACCESSED) & oldval != 0 { return true; }
    false
}

pub unsafe fn pte_needs_flush(oldpte: pte_t, newpte: pte_t) -> bool {
    __pte_flags_need_flush(pte_val(oldpte), pte_val(newpte))
}

pub unsafe fn huge_pmd_needs_flush(oldpmd: pmd_t, newpmd: pmd_t) -> bool {
    __pte_flags_need_flush(pmd_val(oldpmd), pmd_val(newpmd))
}

pub unsafe fn cputlb_use_tlbie() -> bool { tlbie_enabled }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
