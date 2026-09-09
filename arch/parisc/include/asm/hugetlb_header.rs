/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm/page.h.

pub const __HAVE_ARCH_HUGE_SET_HUGE_PTE_AT: bool = true;

unsafe extern "C" {
    pub fn set_huge_pte_at(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
        sz: ::core::ffi::c_ulong,
    );
}

pub const __HAVE_ARCH_HUGE_PTEP_GET_AND_CLEAR: bool = true;

unsafe extern "C" {
    pub fn huge_ptep_get_and_clear(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        sz: ::core::ffi::c_ulong,
    ) -> pte_t;
}

pub const __HAVE_ARCH_HUGE_PTEP_CLEAR_FLUSH: bool = true;

#[inline]
pub unsafe fn huge_ptep_clear_flush(
    vma: *mut vm_area_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
) -> pte_t {
    let _ = vma;
    let _ = addr;
    ::core::ptr::read(ptep)
}

pub const __HAVE_ARCH_HUGE_PTEP_SET_WRPROTECT: bool = true;

unsafe extern "C" {
    pub fn huge_ptep_set_wrprotect(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
    );
}

pub const __HAVE_ARCH_HUGE_PTEP_SET_ACCESS_FLAGS: bool = true;

unsafe extern "C" {
    pub fn huge_ptep_set_access_flags(
        vma: *mut vm_area_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
        dirty: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// Declarations supplied by asm-generic/hugetlb.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
