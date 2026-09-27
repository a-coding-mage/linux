/* SPDX-License-Identifier: GPL-2.0 */
/*
 * TLB flushing for 64-bit hash-MMU CPUs
 *
 * C header guard and include directives omitted; referenced types and symbols
 * are supplied by the surrounding kernel translation.
 */

pub const PPC64_TLB_BATCH_NR: usize = 192;

#[repr(C)]
pub struct ppc64_tlb_batch {
    pub index: ::kernel::ffi::c_ulong,
    pub mm: *mut mm_struct,
    pub pte: [real_pte_t; PPC64_TLB_BATCH_NR],
    pub vpn: [::kernel::ffi::c_ulong; PPC64_TLB_BATCH_NR],
    pub psize: ::kernel::ffi::c_uint,
    pub ssize: ::kernel::ffi::c_int,
}

unsafe extern "C" {
    pub static mut ppc64_tlb_batch: ppc64_tlb_batch;

    pub fn __flush_tlb_pending(batch: *mut ppc64_tlb_batch);
    pub fn radix_enabled() -> bool;
    pub fn preempt_disable();
    pub fn preempt_enable();
    pub fn this_cpu_ptr<T>(ptr: *mut T) -> *mut T;

    pub fn hash__tlbiel_all(action: ::kernel::ffi::c_uint);
    pub fn flush_hash_page(
        vpn: ::kernel::ffi::c_ulong,
        pte: real_pte_t,
        psize: ::kernel::ffi::c_int,
        ssize: ::kernel::ffi::c_int,
        flags: ::kernel::ffi::c_ulong,
    );
    pub fn flush_hash_range(number: ::kernel::ffi::c_ulong, local: ::kernel::ffi::c_int);
    pub fn flush_hash_hugepage(
        vsid: ::kernel::ffi::c_ulong,
        addr: ::kernel::ffi::c_ulong,
        pmdp: *mut pmd_t,
        psize: ::kernel::ffi::c_uint,
        ssize: ::kernel::ffi::c_int,
        flags: ::kernel::ffi::c_ulong,
    );
    pub fn hash__tlb_flush(tlb: *mut mmu_gather);

    #[cfg(CONFIG_PPC_64S_HASH_MMU)]
    pub fn __flush_hash_table_range(
        start: ::kernel::ffi::c_ulong,
        end: ::kernel::ffi::c_ulong,
    );
}

#[inline]
pub unsafe fn arch_enter_lazy_mmu_mode() {
    if radix_enabled() {
        return;
    }
    /*
     * apply_to_page_range can call us this preempt enabled when operating on
     * kernel page tables.
     */
    preempt_disable();
}

#[inline]
pub unsafe fn arch_flush_lazy_mmu_mode() {
    if radix_enabled() {
        return;
    }
    let batch: *mut ppc64_tlb_batch = this_cpu_ptr(&raw mut ppc64_tlb_batch);

    if (*batch).index != 0 {
        __flush_tlb_pending(batch);
    }
}

#[inline]
pub unsafe fn arch_leave_lazy_mmu_mode() {
    if radix_enabled() {
        return;
    }

    arch_flush_lazy_mmu_mode();
    preempt_enable();
}

#[cfg(not(CONFIG_PPC_64S_HASH_MMU))]
#[inline]
pub unsafe fn __flush_hash_table_range(
    _start: ::kernel::ffi::c_ulong,
    _end: ::kernel::ffi::c_ulong,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
