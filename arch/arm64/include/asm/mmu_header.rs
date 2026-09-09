/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependency: <asm/cputype.h>

pub const MMCF_AARCH32: u64 = 0x1; // mm context flag for AArch32 executables
pub const USER_ASID_BIT: u32 = 48;
pub const USER_ASID_FLAG: u64 = 1u64 << USER_ASID_BIT;

// C declarations supplied by other translation units/headers are referenced here.
#[repr(C)]
pub struct mm_context_t {
    pub id: atomic64_t,
    // CONFIG_COMPAT
    #[cfg(feature = "CONFIG_COMPAT")]
    pub sigpage: *mut core::ffi::c_void,
    pub pinned: refcount_t,
    pub vdso: *mut core::ffi::c_void,
    pub flags: c_ulong,
    pub pkey_allocation_map: u8,
}

/*
 * We use atomic64_read() here because the ASID for an 'mm_struct' can
 * be reallocated when scheduling one of its threads following a
 * rollover event (see new_context() and flush_context()). In this case,
 * a concurrent TLBI (e.g. via try_to_unmap_one() and ptep_clear_flush())
 * may use a stale ASID. This is fine in principle as the new ASID is
 * guaranteed to be clean in the TLB, but the TLBI routines have to take
 * care to handle the following race:
 *
 *    CPU 0                    CPU 1                          CPU 2
 *
 *    // ptep_clear_flush(mm)
 *    xchg_relaxed(pte, 0)
 *    DSB ISHST
 *    old = ASID(mm)
 *         |                                                  <rollover>
 *         |                   new = new_context(mm)
 *         \\-----------------> atomic_set(mm->context.id, new)
 *                             cpu_switch_mm(mm)
 *                             // Hardware walk of pte using new ASID
 *    TLBI(old)
 *
 * In this scenario, the barrier on CPU 0 and the dependency on CPU 1
 * ensure that the page-table walker on CPU 1 *must* see the invalid PTE
 * written by CPU 0.
 */
pub unsafe fn ASID(mm: *const mm_struct) -> u64 {
    atomic64_read(&(*mm).context.id) & 0xffff
}

pub unsafe fn arm64_kernel_unmapped_at_el0() -> bool {
    alternative_has_cap_unlikely(ARM64_UNMAP_KERNEL_AT_EL0)
}

extern "C" {
    pub fn arm64_memblock_init();
    pub fn paging_init();
    pub fn bootmem_init();
    pub fn create_mapping_noalloc(phys: phys_addr_t, virt: c_ulong,
                                  size: phys_addr_t, prot: pgprot_t);
    pub fn create_pgd_mapping(mm: *mut mm_struct, phys: phys_addr_t,
                              virt: c_ulong, size: phys_addr_t,
                              prot: pgprot_t, page_mappings_only: bool);
    pub fn fixmap_remap_fdt(dt_phys: phys_addr_t, size: *mut c_int,
                            prot: pgprot_t) -> *mut core::ffi::c_void;
    pub fn mark_linear_text_alias_ro();
    pub fn split_kernel_leaf_mapping(start: c_ulong, end: c_ulong) -> c_int;
    pub fn linear_map_maybe_split_to_ptes();
}

/*
 * This check is triggered during the early boot before the cpufeature
 * is initialised. Checking the status on the local CPU allows the boot
 * CPU to detect the need for non-global mappings and thus avoiding a
 * pagetable re-write after all the CPUs are booted. This check will be
 * anyway run on individual CPUs, allowing us to get the consistent
 * state once the SMP CPUs are up and thus make the switch to non-global
 * mappings if required.
 */
pub unsafe fn kaslr_requires_kpti() -> bool {
    /* Build-time CONFIG_ARM64_E0PD condition from the C header. */
    #[cfg(feature = "CONFIG_ARM64_E0PD")]
    {
        /*
         * E0PD does a similar job to KPTI so can be used instead
         * where available.
         */
        let mmfr2: u64 = read_sysreg_s(SYS_ID_AA64MMFR2_EL1);
        if cpuid_feature_extract_unsigned_field(mmfr2,
                ID_AA64MMFR2_EL1_E0PD_SHIFT) != 0 {
            return false;
        }
    }

    true
}

#[cfg(feature = "CONFIG_UNMAP_KERNEL_AT_EL0")]
extern "C" {
    pub fn kpti_install_ng_mappings();
}

#[cfg(not(feature = "CONFIG_UNMAP_KERNEL_AT_EL0"))]
pub unsafe fn kpti_install_ng_mappings() {}

extern "C" {
    pub static mut page_alloc_available: bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
