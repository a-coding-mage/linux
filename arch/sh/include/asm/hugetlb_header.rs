/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the SH architecture hugetlb header.
// The C include dependencies are supplied by other translated units.

pub const __HAVE_ARCH_HUGE_PTEP_CLEAR_FLUSH: bool = true;

pub unsafe fn huge_ptep_clear_flush(
    _vma: *mut vm_area_struct,
    _addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
) -> pte_t {
    // C: return *ptep;
    ::core::ptr::read(ptep)
}

pub unsafe fn arch_clear_hugetlb_flags(folio: *mut folio) {
    // C: clear_bit(PG_dcache_clean, &folio->flags.f);
    clear_bit(PG_dcache_clean, &mut (*folio).flags.f);
}

// C macro alias: #define arch_clear_hugetlb_flags arch_clear_hugetlb_flags
// Generic hugetlb declarations are supplied by the translated generic header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
