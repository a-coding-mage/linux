// SPDX-License-Identifier: GPL-2.0
/*
 * PPC Huge TLB Page Support for Book3E MMU
 *
 * Copyright (C) 2009 David Gibson, IBM Corporation.
 * Copyright (C) 2011 Becky Bruce, Freescale Semiconductor
 *
 */

// Linux and architecture headers provide the types, constants, and functions
// referenced below.

#[cfg(target_pointer_width = "64")]
#[inline]
unsafe fn tlb1_next() -> i32 {
    let paca = get_paca();
    let tcd = (*paca).tcd_ptr;
    let this = (*tcd).esel_next;
    let mut next = this + 1;
    if next >= (*tcd).esel_max {
        next = (*tcd).esel_first;
    }
    (*tcd).esel_next = next;
    this
}

#[cfg(target_pointer_width = "64")]
#[inline]
unsafe fn book3e_tlb_lock() {
    let paca = get_paca();
    let mut tmp: usize;
    let token: i32 = smp_processor_id() + 1;

    /* Besides being unnecessary in the absence of SMT, this check prevents
     * trying to do lbarx/stbcx. on e5500 which does not implement either. */
    if !cpu_has_feature(CPU_FTR_SMT) {
        return;
    }

    core::arch::asm!(
        ".machine push; .machine e6500;",
        "1: lbarx {tmp}, 0, {lock};",
        "cmpwi {tmp}, 0; bne 2f;",
        "stbcx. {token}, 0, {lock}; bne 1b; b 3f;",
        "2: lbzx {tmp}, 0, {lock}; cmpwi {tmp}, 0; bne 2b; b 1b;",
        "3: .machine pop;",
        tmp = lateout(reg) tmp,
        lock = in(reg) &(*(*paca).tcd_ptr).lock,
        token = in(reg) token,
        options(nostack)
    );
}

#[cfg(target_pointer_width = "64")]
#[inline]
unsafe fn book3e_tlb_unlock() {
    let paca = get_paca();
    if !cpu_has_feature(CPU_FTR_SMT) {
        return;
    }
    isync();
    (*(*paca).tcd_ptr).lock = 0;
}

#[cfg(not(target_pointer_width = "64"))]
#[inline]
unsafe fn tlb1_next() -> i32 {
    let ncams = mfspr(SPRN_TLB1CFG) & TLBnCFG_N_ENTRY;
    let index = this_cpu_read(next_tlbcam_idx);
    if unlikely(index == ncams - 1) {
        __this_cpu_write(next_tlbcam_idx, tlbcam_index);
    } else {
        __this_cpu_inc(next_tlbcam_idx);
    }
    index
}

#[cfg(not(target_pointer_width = "64"))]
#[inline]
unsafe fn book3e_tlb_lock() {}

#[cfg(not(target_pointer_width = "64"))]
#[inline]
unsafe fn book3e_tlb_unlock() {}

#[inline]
unsafe fn book3e_tlb_exists(ea: usize, pid: usize) -> i32 {
    let mut found: i32 = 0;
    mtspr(SPRN_MAS6, pid << 16);
    core::arch::asm!(
        "tlbsx 0, {ea}",
        "mfspr {found}, 0x271",
        "srwi {found}, {found}, 31",
        ea = in(reg) ea,
        found = inout(reg) found,
        options(nostack)
    );
    found
}

unsafe fn book3e_hugetlb_preload(vma: *mut vm_area_struct, ea: usize, pte: pte_t) {
    let mut mas1: usize;
    let mut mas2: usize;
    let mut mas7_3: u64;
    let psize = vma_mmu_pagesize(vma);
    let shift = __ilog2(psize);
    let tsize = shift - 10;
    let mut flags: usize = 0;

    if unlikely(is_kernel_addr(ea)) { return; }
    let mm = (*vma).vm_mm;
    local_irq_save(&mut flags);
    book3e_tlb_lock();
    if unlikely(book3e_tlb_exists(ea, (*mm).context.id) != 0) {
        book3e_tlb_unlock();
        local_irq_restore(flags);
        return;
    }
    let index = tlb1_next();
    mtspr(SPRN_MAS0, MAS0_ESEL(index) | MAS0_TLBSEL(1));
    mas1 = MAS1_VALID | MAS1_TID((*mm).context.id) | MAS1_TSIZE(tsize);
    mas2 = ea & !((1usize << shift) - 1);
    mas2 |= (pte_val(pte) >> PTE_WIMGE_SHIFT) & MAS2_WIMGE_MASK;
    mas7_3 = (pte_pfn(pte) as u64) << PAGE_SHIFT;
    mas7_3 |= ((pte_val(pte) >> PTE_BAP_SHIFT) & MAS3_BAP_MASK) as u64;
    if !pte_dirty(pte) { mas7_3 &= !(MAS3_SW | MAS3_UW) as u64; }
    mtspr(SPRN_MAS1, mas1);
    mtspr(SPRN_MAS2, mas2);
    if mmu_has_feature(MMU_FTR_BIG_PHYS) { mtspr(SPRN_MAS7, upper_32_bits(mas7_3)); }
    mtspr(SPRN_MAS3, lower_32_bits(mas7_3));
    core::arch::asm!("tlbwe", options(nostack));
    book3e_tlb_unlock();
    local_irq_restore(flags);
}

pub unsafe fn __update_mmu_cache(vma: *mut vm_area_struct, address: usize, ptep: *mut pte_t) {
    if is_vm_hugetlb_page(vma) { book3e_hugetlb_preload(vma, address, *ptep); }
}

pub unsafe fn flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: usize) {
    let hstate = hstate_file((*vma).vm_file);
    let tsize = huge_page_shift(hstate) - 10;
    __flush_tlb_page((*vma).vm_mm, vmaddr, tsize, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
