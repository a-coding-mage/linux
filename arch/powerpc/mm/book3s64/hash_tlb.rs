// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains the routines for flushing entries from the
 * TLB and MMU hash table.
 *
 * Derived from arch/ppc64/mm/init.c and arch/i386/mm/init.c.
 */

// External kernel declarations supplied by other translation units.
extern "C" {
    static mut ppc64_tlb_batch: PerCpu<ppc64_tlb_batch>;
    static mut init_mm: mm_struct;
    static mmu_kernel_ssize: i32;

    fn get_cpu_var(batch: *mut PerCpu<ppc64_tlb_batch>) -> *mut ppc64_tlb_batch;
    fn put_cpu_var(batch: *mut PerCpu<ppc64_tlb_batch>);
    fn get_slice_psize(mm: *mut mm_struct, addr: usize) -> u32;
    fn pte_pagesize_index(mm: *mut mm_struct, addr: usize, pte: usize) -> u32;
    fn is_kernel_addr(addr: usize) -> bool;
    fn user_segment_size(addr: usize) -> i32;
    fn get_user_vsid(context: *mut mm_context, addr: usize, ssize: i32) -> usize;
    fn get_kernel_vsid(addr: usize, ssize: i32) -> usize;
    fn hpt_vpn(addr: usize, vsid: usize, ssize: i32) -> usize;
    fn __real_pte(pte: pte_t, ptep: *mut pte_t, offset: i32) -> real_pte_t;
    fn __pte(pte: usize) -> pte_t;
    fn is_lazy_mmu_mode_active() -> bool;
    fn flush_hash_page(vpn: usize, pte: real_pte_t, psize: u32, ssize: i32, local: bool);
    fn mm_is_thread_local(mm: *mut mm_struct) -> bool;
    fn __flush_tlb_pending(batch: *mut ppc64_tlb_batch);
    fn flush_hash_range(n: i32, local: bool);
    fn find_init_mm_pte(addr: usize, hugepage_shift: *mut i32) -> *mut pte_t;
    fn pte_val(pte: pte_t) -> usize;
    fn local_irq_save(flags: *mut usize);
    fn lazy_mmu_mode_enable();
    fn lazy_mmu_mode_disable();
    fn local_irq_restore(flags: usize);
    fn pte_offset_map(pmd: *mut pmd_t, addr: usize) -> *mut pte_t;
    fn pte_unmap(pte: *mut pte_t);
}

#[repr(C)]
pub struct PerCpu<T>(pub core::marker::PhantomData<T>);

#[repr(C)]
pub struct ppc64_tlb_batch {
    pub index: i32,
    pub mm: *mut mm_struct,
    pub psize: u32,
    pub ssize: i32,
    pub pte: [real_pte_t; PPC64_TLB_BATCH_NR as usize],
    pub vpn: [usize; PPC64_TLB_BATCH_NR as usize],
}

#[repr(C)] pub struct mm_struct { pub context: mm_context }
#[repr(C)] pub struct mm_context;
#[repr(C)] pub struct pte_t;
#[repr(C)] pub struct pmd_t;
#[repr(C)] pub struct mmu_gather;
#[repr(C)] pub struct real_pte_t;

extern "C" {
    static mmu_psize_defs: [mmu_psize_def; 32];
}
#[repr(C)] pub struct mmu_psize_def { pub shift: u32 }

const PPC64_TLB_BATCH_NR: i32 = 192;
const MMU_PAGE_16G: u32 = 0;
const PAGE_SIZE: usize = 4096;
const PAGE_MASK: usize = !(PAGE_SIZE - 1);
const PMD_SIZE: usize = 1 << 21;
const PTRS_PER_PTE: i32 = 512;
const PTRS_PER_PMD: i32 = 512;
const PTRS_PER_PUD: i32 = 512;
const H_PAGE_HASHPTE: usize = 1 <<  HPAGE_HASHPTE_SHIFT;
const HPAGE_HASHPTE_SHIFT: usize = 0;

pub unsafe fn hpte_need_flush(
    mm: *mut mm_struct, mut addr: usize, ptep: *mut pte_t,
    pte: usize, huge: i32,
) {
    let mut vpn: usize;
    let batch = get_cpu_var(&raw mut ppc64_tlb_batch);
    let mut vsid: usize;
    let psize: u32;
    let ssize: i32;
    let rpte: real_pte_t;
    let mut i: i32;
    let offset: i32;

    i = (*batch).index;
    if huge != 0 {
        #[cfg(CONFIG_HUGETLB_PAGE)]
        {
            psize = get_slice_psize(mm, addr);
            addr &= !((1usize << mmu_psize_defs[psize as usize].shift) - 1);
            offset = if psize == MMU_PAGE_16G { PTRS_PER_PUD } else { PTRS_PER_PMD };
        }
        #[cfg(not(CONFIG_HUGETLB_PAGE))]
        {
            panic!("BUG");
            psize = pte_pagesize_index(mm, addr, pte);
            offset = 0;
        }
    } else {
        psize = pte_pagesize_index(mm, addr, pte);
        addr &= PAGE_MASK;
        offset = PTRS_PER_PTE;
    }

    if !is_kernel_addr(addr) {
        ssize = user_segment_size(addr);
        vsid = get_user_vsid(&raw mut (*mm).context, addr, ssize);
    } else {
        vsid = get_kernel_vsid(addr, mmu_kernel_ssize);
        ssize = mmu_kernel_ssize;
    }
    debug_assert!(vsid != 0);
    vpn = hpt_vpn(addr, vsid, ssize);
    rpte = __real_pte(__pte(pte), ptep, offset);

    if !is_lazy_mmu_mode_active() {
        flush_hash_page(vpn, rpte, psize, ssize, mm_is_thread_local(mm));
        put_cpu_var(&raw mut ppc64_tlb_batch);
        return;
    }

    if i != 0 && ((*batch).mm != mm || (*batch).psize != psize || (*batch).ssize != ssize) {
        __flush_tlb_pending(batch);
        i = 0;
    }
    if i == 0 {
        (*batch).mm = mm;
        (*batch).psize = psize;
        (*batch).ssize = ssize;
    }
    (*batch).pte[i as usize] = rpte;
    (*batch).vpn[i as usize] = vpn;
    i += 1;
    (*batch).index = i;
    if i >= PPC64_TLB_BATCH_NR { __flush_tlb_pending(batch); }
    put_cpu_var(&raw mut ppc64_tlb_batch);
}

pub unsafe fn __flush_tlb_pending(batch: *mut ppc64_tlb_batch) {
    let i = (*batch).index;
    let local = mm_is_thread_local((*batch).mm);
    if i == 1 {
        flush_hash_page((*batch).vpn[0], (*batch).pte[0], (*batch).psize, (*batch).ssize, local);
    } else {
        flush_hash_range(i, local);
    }
    (*batch).index = 0;
}

pub unsafe fn hash__tlb_flush(_tlb: *mut mmu_gather) {
    let batch = get_cpu_var(&raw mut ppc64_tlb_batch);
    if (*batch).index != 0 { __flush_tlb_pending(batch); }
    put_cpu_var(&raw mut ppc64_tlb_batch);
}

pub unsafe fn __flush_hash_table_range(mut start: usize, mut end: usize) {
    let mut hugepage_shift: i32;
    let mut flags: usize = 0;
    start = start & !(PAGE_SIZE - 1);
    end = (end + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    local_irq_save(&mut flags);
    lazy_mmu_mode_enable();
    while start < end {
        let ptep = find_init_mm_pte(start, &mut hugepage_shift);
        if ptep.is_null() { start += PAGE_SIZE; continue; }
        let pte = pte_val(*ptep);
        if pte & H_PAGE_HASHPTE == 0 { start += PAGE_SIZE; continue; }
        hpte_need_flush(&raw mut init_mm, start, ptep, pte, hugepage_shift);
        start += PAGE_SIZE;
    }
    lazy_mmu_mode_disable();
    local_irq_restore(flags);
}

pub unsafe fn flush_hash_table_pmd_range(mm: *mut mm_struct, pmd: *mut pmd_t, mut addr: usize) {
    addr &= !(PMD_SIZE - 1);
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    lazy_mmu_mode_enable();
    let start_pte = pte_offset_map(pmd, addr);
    if start_pte.is_null() { lazy_mmu_mode_disable(); local_irq_restore(flags); return; }
    let mut pte = start_pte;
    for _ in 0..PTRS_PER_PTE {
        let pteval = pte_val(*pte);
        if pteval & H_PAGE_HASHPTE != 0 { hpte_need_flush(mm, addr, pte, pteval, 0); }
        addr += PAGE_SIZE;
        pte = pte.add(1);
    }
    pte_unmap(start_pte);
    lazy_mmu_mode_disable();
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
