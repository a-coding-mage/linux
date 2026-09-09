/* SPDX-License-Identifier: GPL-2.0 */
/*
 * x86 KFENCE support.
 *
 * Copyright (C) 2020, Google LLC.
 */

/* The original header is excluded when building a module. */

#[cfg(not(feature = "module"))]
pub unsafe fn arch_kfence_init_pool() -> bool {
    let mut addr: usize = (&__kfence_pool as *const _ as usize);

    while is_kfence_address(addr as *const core::ffi::c_void) {
        let mut level: u32 = 0;

        if lookup_address(addr, &mut level).is_null() {
            return false;
        }

        if level != PG_LEVEL_4K {
            set_memory_4k(addr, 1);
        }

        addr = addr.wrapping_add(PAGE_SIZE);
    }

    true
}

/* Protect the given page and flush TLB. */
#[cfg(not(feature = "module"))]
pub unsafe fn kfence_protect_page(addr: usize, protect: bool) -> bool {
    let mut level: u32 = 0;
    let pte: *mut pte_t = lookup_address(addr, &mut level);
    let val: pteval_t;
    let new: pteval_t;

    if warn_on(pte.is_null() || level != PG_LEVEL_4K) {
        return false;
    }

    val = pte_val(*pte);

    /*
     * protect requires making the page not-present.  If the PTE is
     * already in the right state, there's nothing to do.
     */
    if protect != ((val & _PAGE_PRESENT) != 0) {
        return true;
    }

    /*
     * Otherwise, flip the Present bit, taking care to avoid writing an
     * L1TF-vulnerable PTE (not present, without the high address bits
     * set).
     */
    new = val ^ _PAGE_PRESENT;
    set_pte(pte, __pte(flip_protnone_guard(val, new, PTE_PFN_MASK)));

    /*
     * If the page was protected (non-present) and we're making it
     * present, there is no need to flush the TLB at all.
     */
    if !protect {
        return true;
    }

    /*
     * We need to avoid IPIs, as we may get KFENCE allocations or faults
     * with interrupts disabled. Therefore, the below is best-effort, and
     * does not flush TLBs on all CPUs. We can tolerate some inaccuracy;
     * lazy fault handling takes care of faults after the page is PRESENT.
     */

    /*
     * Flush this CPU's TLB, assuming whoever did the allocation/free is
     * likely to continue running on this CPU.
     */
    preempt_disable();
    flush_tlb_one_kernel(addr);
    preempt_enable();
    true
}

/* Declarations supplied by the surrounding kernel translation. */
extern "C" {
    static __kfence_pool: u8;
    fn is_kfence_address(addr: *const core::ffi::c_void) -> bool;
    fn lookup_address(addr: usize, level: *mut u32) -> *mut pte_t;
    fn set_memory_4k(addr: usize, numpages: usize);
    fn pte_val(pte: pte_t) -> pteval_t;
    fn set_pte(pte: *mut pte_t, value: pte_t);
    fn __pte(value: pteval_t) -> pte_t;
    fn flip_protnone_guard(val: pteval_t, new: pteval_t, mask: pteval_t) -> pteval_t;
    fn warn_on(condition: bool) -> bool;
    fn preempt_disable();
    fn flush_tlb_one_kernel(addr: usize);
    fn preempt_enable();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
