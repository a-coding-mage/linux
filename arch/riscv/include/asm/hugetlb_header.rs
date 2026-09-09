/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding architecture and generic headers.

pub unsafe inline fn arch_clear_hugetlb_flags(folio: *mut folio) {
    clear_bit(PG_dcache_clean, &mut (*folio).flags.f);
}

// #define arch_clear_hugetlb_flags arch_clear_hugetlb_flags

// Preserved build-time condition: CONFIG_ARCH_ENABLE_HUGEPAGE_MIGRATION.
#[cfg(CONFIG_ARCH_ENABLE_HUGEPAGE_MIGRATION)]
extern "C" {
    pub fn arch_hugetlb_migration_supported(h: *mut hstate) -> bool;
}

// #define arch_hugetlb_migration_supported arch_hugetlb_migration_supported

// Preserved build-time condition: CONFIG_RISCV_ISA_SVNAPOT.
#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub const __HAVE_ARCH_HUGE_PTE_CLEAR: bool = true;

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
extern "C" {
    pub fn huge_pte_clear(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        sz: ::core::ffi::c_ulong,
    );

    pub fn set_huge_pte_at(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
        sz: ::core::ffi::c_ulong,
    );

    pub fn huge_ptep_get_and_clear(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        sz: ::core::ffi::c_ulong,
    ) -> pte_t;

    pub fn huge_ptep_clear_flush(
        vma: *mut vm_area_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
    ) -> pte_t;

    pub fn huge_ptep_set_wrprotect(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
    );

    pub fn huge_ptep_set_access_flags(
        vma: *mut vm_area_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
        dirty: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn huge_ptep_get(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
    ) -> pte_t;

    pub fn arch_make_huge_pte(
        entry: pte_t,
        shift: ::core::ffi::c_uint,
        flags: vm_flags_t,
    ) -> pte_t;
}

#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub const __HAVE_ARCH_HUGE_SET_HUGE_PTE_AT: bool = true;
#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub const __HAVE_ARCH_HUGE_PTEP_GET_AND_CLEAR: bool = true;
#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub const __HAVE_ARCH_HUGE_PTEP_CLEAR_FLUSH: bool = true;
#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub const __HAVE_ARCH_HUGE_PTEP_SET_WRPROTECT: bool = true;
#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub const __HAVE_ARCH_HUGE_PTEP_SET_ACCESS_FLAGS: bool = true;
#[cfg(CONFIG_RISCV_ISA_SVNAPOT)]
pub const __HAVE_ARCH_HUGE_PTEP_GET: bool = true;

// #define arch_make_huge_pte arch_make_huge_pte

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
