/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm/page.h.

// Preserved from CONFIG_HUGETLB_PAGE. These declarations are enabled when
// the corresponding build-time configuration is enabled.
#[cfg(feature = "CONFIG_HUGETLB_PAGE")]
#[repr(C)]
pub struct pud_huge_patch_entry {
    pub addr: ::core::ffi::c_uint,
    pub insn: ::core::ffi::c_uint,
}

#[cfg(feature = "CONFIG_HUGETLB_PAGE")]
extern "C" {
    pub static mut __pud_huge_patch: pud_huge_patch_entry;
    pub static mut __pud_huge_patch_end: pud_huge_patch_entry;
}

pub const __HAVE_ARCH_HUGE_SET_HUGE_PTE_AT: bool = true;
extern "C" {
    pub fn set_huge_pte_at(
        mm: *mut mm_struct,
        addr: ::core::primitive::c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
        sz: ::core::primitive::c_ulong,
    );
    pub fn __set_huge_pte_at(
        mm: *mut mm_struct,
        addr: ::core::primitive::c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
    );
}

pub const __HAVE_ARCH_HUGE_PTEP_GET_AND_CLEAR: bool = true;
extern "C" {
    pub fn huge_ptep_get_and_clear(
        mm: *mut mm_struct,
        addr: ::core::primitive::c_ulong,
        ptep: *mut pte_t,
        sz: ::core::primitive::c_ulong,
    ) -> pte_t;
}

pub const __HAVE_ARCH_HUGE_PTEP_CLEAR_FLUSH: bool = true;
pub unsafe fn huge_ptep_clear_flush(
    _vma: *mut vm_area_struct,
    _addr: ::core::primitive::c_ulong,
    ptep: *mut pte_t,
) -> pte_t {
    *ptep
}

pub const __HAVE_ARCH_HUGE_PTEP_SET_WRPROTECT: bool = true;
pub unsafe fn huge_ptep_set_wrprotect(
    mm: *mut mm_struct,
    addr: ::core::primitive::c_ulong,
    ptep: *mut pte_t,
) {
    let old_pte: pte_t = *ptep;
    __set_huge_pte_at(mm, addr, ptep, pte_wrprotect(old_pte));
}

pub const __HAVE_ARCH_HUGE_PTEP_SET_ACCESS_FLAGS: bool = true;
pub unsafe fn huge_ptep_set_access_flags(
    vma: *mut vm_area_struct,
    addr: ::core::primitive::c_ulong,
    ptep: *mut pte_t,
    pte: pte_t,
    _dirty: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let changed: ::core::ffi::c_int = if !pte_same(*ptep, pte) { 1 } else { 0 };
    if changed != 0 {
        __set_huge_pte_at((*vma).vm_mm, addr, ptep, pte);
        flush_tlb_page(vma, addr);
    }
    changed
}

// Dependency supplied by asm-generic/hugetlb.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
