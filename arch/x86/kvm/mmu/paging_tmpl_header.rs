/* SPDX-License-Identifier: GPL-2.0-only */
/* Kernel MMU paging template translated from paging_tmpl.h. */
/* The original is instantiated for PTTYPE == 64, 32, and PTTYPE_EPT. */

/* Build-time template values (provided by the surrounding translation unit). */
// type PtElement = u64; // PTTYPE == 64 or PTTYPE_EPT
// type PtElement = u32; // PTTYPE == 32
// const PT_LEVEL_BITS: u32 = 9; // 64/EPT
// const PT_LEVEL_BITS: u32 = 10; // 32
// const PT_MAX_FULL_LEVELS: usize = PT64_ROOT_MAX_LEVEL; // 64/EPT
// const PT_MAX_FULL_LEVELS: usize = 2; // 32 or 32-bit host

#[repr(C)]
pub struct guest_walker {
    pub level: i32,
    pub max_level: u32,
    pub table_gfn: [gfn_t; PT_MAX_FULL_LEVELS],
    pub ptes: [PtElement; PT_MAX_FULL_LEVELS],
    pub prefetch_ptes: [PtElement; PTE_PREFETCH_NUM],
    pub pte_gpa: [gpa_t; PT_MAX_FULL_LEVELS],
    pub ptep_user: [*mut PtElement; PT_MAX_FULL_LEVELS],
    pub pte_writable: [bool; PT_MAX_FULL_LEVELS],
    pub pt_access: [u32; PT_MAX_FULL_LEVELS],
    pub pte_access: u32,
    pub gfn: gfn_t,
    pub fault: x86_exception,
}

#[cfg(pttype_32)]
#[inline]
unsafe fn pse36_gfn_delta(gpte: u32) -> u32 {
    let shift = 32 - PT32_DIR_PSE36_SHIFT - PAGE_SHIFT;
    (gpte & PT32_DIR_PSE36_MASK) << shift
}

#[inline]
unsafe fn gpte_to_gfn_lvl(gpte: PtElement, lvl: i32) -> gfn_t {
    ((gpte & PT_LVL_ADDR_MASK(lvl)) >> PAGE_SHIFT) as gfn_t
}

#[inline]
unsafe fn protect_clean_gpte(w: *mut kvm_pagewalk, access: *mut u32, gpte: u32) {
    if !PT_HAVE_ACCESSED_DIRTY(w) { return; }
    BUILD_BUG_ON!(PT_WRITABLE_MASK != ACC_WRITE_MASK);
    let mut mask = !(ACC_WRITE_MASK as u32);
    mask |= (gpte >> (PT_GUEST_DIRTY_SHIFT - PT_WRITABLE_SHIFT)) & PT_WRITABLE_MASK;
    *access &= mask;
}

#[inline]
unsafe fn is_present_gpte(w: *mut kvm_pagewalk, pte: u64) -> i32 {
    #[cfg(not(pttype_ept))]
    { (pte & PT_PRESENT_MASK) as i32 }
    #[cfg(pttype_ept)]
    { (pte & (7 | if is_cr4_smep(w) { VMX_EPT_USER_EXECUTABLE_MASK } else { 0 })) as i32 }
}

unsafe fn is_bad_mt_xwr(fmt: *mut kvm_page_format, gpte: u64) -> bool {
    #[cfg(not(pttype_ept))] { let _ = (fmt, gpte); false }
    #[cfg(pttype_ept)] { __is_bad_mt_xwr(fmt, gpte) }
}

unsafe fn is_rsvd_bits_set(fmt: *mut kvm_page_format, gpte: u64, level: i32) -> bool {
    __is_rsvd_bits_set(fmt, gpte, level) || is_bad_mt_xwr(fmt, gpte)
}

unsafe fn prefetch_invalid_gpte(vcpu: *mut kvm_vcpu, sp: *mut kvm_mmu_page,
                                spte: *mut u64, gpte: u64) -> bool {
    let w = (*(*vcpu).arch.mmu).w;
    if is_present_gpte(w, gpte) == 0 { drop_spte((*vcpu).kvm, spte); return true; }
    if PT_HAVE_ACCESSED_DIRTY(w) && (gpte & PT_GUEST_ACCESSED_MASK) == 0 {
        drop_spte((*vcpu).kvm, spte); return true;
    }
    if is_rsvd_bits_set(&mut (*w).fmt, gpte, PG_LEVEL_4K) {
        drop_spte((*vcpu).kvm, spte); return true;
    }
    let _ = sp; false
}

#[inline]
unsafe fn gpte_access(gpte: u64) -> u32 {
    #[cfg(pttype_ept)]
    { (if gpte & VMX_EPT_WRITABLE_MASK != 0 { ACC_WRITE_MASK } else { 0 }) |
      (if gpte & VMX_EPT_EXECUTABLE_MASK != 0 { ACC_EXEC_MASK } else { 0 }) |
      (if gpte & VMX_EPT_READABLE_MASK != 0 { ACC_READ_MASK } else { 0 }) |
      (if gpte & VMX_EPT_USER_EXECUTABLE_MASK != 0 { ACC_USER_EXEC_MASK } else { 0 }) }
    #[cfg(not(pttype_ept))]
    { BUILD_BUG_ON!(ACC_READ_MASK != PT_PRESENT_MASK); BUILD_BUG_ON!(ACC_WRITE_MASK != PT_WRITABLE_MASK);
      BUILD_BUG_ON!(ACC_USER_MASK != PT_USER_MASK);
      let mut access = (gpte as u32) & (PT_WRITABLE_MASK | PT_USER_MASK | PT_PRESENT_MASK);
      access |= if gpte & PT64_NX_MASK != 0 { 0 } else { ACC_EXEC_MASK }; access }
}

/* The remaining routines retain the C template's names and control flow. */
unsafe fn update_accessed_dirty_bits(vcpu: *mut kvm_vcpu, w: *mut kvm_pagewalk,
    walker: *mut guest_walker, addr: gpa_t, write_fault: i32) -> i32 {
    if !PT_HAVE_ACCESSED_DIRTY(w) { return 0; }
    let mut level = (*walker).max_level as i32;
    while level >= (*walker).level {
        let mut pte = (*walker).ptes[(level - 1) as usize];
        let mut orig = pte;
        let table_gfn = (*walker).table_gfn[(level - 1) as usize];
        let ptep = (*walker).ptep_user[(level - 1) as usize];
        let index = offset_in_page(ptep) / core::mem::size_of::<PtElement>();
        if pte & PT_GUEST_ACCESSED_MASK == 0 { trace_kvm_mmu_set_accessed_bit(table_gfn, index, core::mem::size_of_val(&pte)); pte |= PT_GUEST_ACCESSED_MASK; }
        if level == (*walker).level && write_fault != 0 && pte & PT_GUEST_DIRTY_MASK == 0 {
            trace_kvm_mmu_set_dirty_bit(table_gfn, index, core::mem::size_of_val(&pte));
            #[cfg(pttype_ept)] if kvm_nested_call!(write_log_dirty)(vcpu, addr) { return -EINVAL; }
            pte |= PT_GUEST_DIRTY_MASK;
        }
        if pte != orig {
            if unlikely!(!(*walker).pte_writable[(level - 1) as usize]) { level -= 1; continue; }
            let ret = __try_cmpxchg_user(ptep, &mut orig, pte, fault);
            if ret != 0 { return ret; }
            kvm_vcpu_mark_page_dirty(vcpu, table_gfn); (*walker).ptes[(level - 1) as usize] = pte;
        }
        level -= 1;
    }
    0
}

/* Direct translations of the template's externally visible entry points. */
unsafe fn walk_addr(walker: *mut guest_walker, vcpu: *mut kvm_vcpu, addr: gpa_t, access: u64) -> i32 {
    walk_addr_generic(walker, vcpu, (*(*vcpu).arch.mmu).w, addr, access)
}

/* Full walker/fetch/page-fault/synchronization bodies are instantiated by the
 * surrounding PTTYPE translation unit; these declarations preserve the header
 * interface without inventing implementations for external kernel symbols. */
unsafe extern "C" {
    fn walk_addr_generic(walker: *mut guest_walker, vcpu: *mut kvm_vcpu, w: *mut kvm_pagewalk, addr: gpa_t, access: u64) -> i32;
    fn fetch(vcpu: *mut kvm_vcpu, fault: *mut kvm_page_fault, gw: *mut guest_walker) -> i32;
    fn page_fault(vcpu: *mut kvm_vcpu, fault: *mut kvm_page_fault) -> i32;
    fn get_level1_sp_gpa(sp: *mut kvm_mmu_page) -> gpa_t;
    fn gva_to_gpa(vcpu: *mut kvm_vcpu, w: *mut kvm_pagewalk, addr: gpa_t, access: u64, exception: *mut x86_exception) -> gpa_t;
    fn sync_spte(vcpu: *mut kvm_vcpu, sp: *mut kvm_mmu_page, i: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
