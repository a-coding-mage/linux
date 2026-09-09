/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation: asm/hvcall.h

pub const RIC_FLUSH_TLB: u32 = 0;
pub const RIC_FLUSH_PWC: u32 = 1;
pub const RIC_FLUSH_ALL: u32 = 2;

#[repr(C)]
pub struct vm_area_struct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct mm_struct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct mmu_gather {
    _unused: [u8; 0],
}

// These constants and the definition are supplied by the surrounding kernel translation.
extern "C" {
    static mmu_psize_defs: [MmuPsizeDef; 0];
}

#[repr(C)]
pub struct MmuPsizeDef {
    pub ap: ::core::ffi::c_int,
}

#[inline]
pub unsafe fn psize_to_rpti_pgsize(psize: ::core::ffi::c_ulong) -> u64 {
    if psize == MMU_PAGE_4K {
        return H_RPTI_PAGE_4K;
    }
    if psize == MMU_PAGE_64K {
        return H_RPTI_PAGE_64K;
    }
    if psize == MMU_PAGE_2M {
        return H_RPTI_PAGE_2M;
    }
    if psize == MMU_PAGE_1G {
        return H_RPTI_PAGE_1G;
    }
    H_RPTI_PAGE_ALL
}

#[inline]
pub unsafe fn mmu_get_ap(psize: ::core::ffi::c_int) -> ::core::ffi::c_int {
    (*mmu_psize_defs.as_ptr().add(psize as usize)).ap
}

// Under CONFIG_PPC_RADIX_MMU these are external definitions. Otherwise the inline
// fallbacks below issue WARN_ON(1). The build configuration selects the applicable form.
#[cfg(CONFIG_PPC_RADIX_MMU)]
extern "C" {
    pub fn radix__tlbiel_all(action: u32);
    pub fn radix__flush_tlb_lpid_page(lpid: u32, addr: ::core::ffi::c_ulong, page_size: ::core::ffi::c_ulong);
    pub fn radix__flush_pwc_lpid(lpid: u32);
    pub fn radix__flush_all_lpid(lpid: u32);
    pub fn radix__flush_all_lpid_guest(lpid: u32);
}

#[cfg(not(CONFIG_PPC_RADIX_MMU))]
#[inline]
pub unsafe fn radix__tlbiel_all(_action: u32) { WARN_ON(1); }

#[cfg(not(CONFIG_PPC_RADIX_MMU))]
#[inline]
pub unsafe fn radix__flush_tlb_lpid_page(_lpid: u32, _addr: ::core::ffi::c_ulong, _page_size: ::core::ffi::c_ulong) { WARN_ON(1); }

#[cfg(not(CONFIG_PPC_RADIX_MMU))]
#[inline]
pub unsafe fn radix__flush_pwc_lpid(_lpid: u32) { WARN_ON(1); }

#[cfg(not(CONFIG_PPC_RADIX_MMU))]
#[inline]
pub unsafe fn radix__flush_all_lpid(_lpid: u32) { WARN_ON(1); }

#[cfg(not(CONFIG_PPC_RADIX_MMU))]
#[inline]
pub unsafe fn radix__flush_all_lpid_guest(_lpid: u32) { WARN_ON(1); }

extern "C" {
    pub fn radix__flush_hugetlb_tlb_range(vma: *mut vm_area_struct, start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn radix__flush_tlb_range_psize(mm: *mut mm_struct, start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong, psize: ::core::ffi::c_int);
    pub fn radix__flush_tlb_pwc_range_psize(mm: *mut mm_struct, start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong, psize: ::core::ffi::c_int);
    pub fn radix__flush_pmd_tlb_range(vma: *mut vm_area_struct, start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn radix__flush_pud_tlb_range(vma: *mut vm_area_struct, start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn radix__flush_tlb_range(vma: *mut vm_area_struct, start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn radix__flush_tlb_kernel_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn radix__local_flush_tlb_mm(mm: *mut mm_struct);
    pub fn radix__local_flush_all_mm(mm: *mut mm_struct);
    pub fn radix__local_flush_tlb_page(vma: *mut vm_area_struct, vmaddr: ::core::ffi::c_ulong);
    pub fn radix__local_flush_tlb_page_psize(mm: *mut mm_struct, vmaddr: ::core::ffi::c_ulong, psize: ::core::ffi::c_int);
    pub fn radix__tlb_flush(tlb: *mut mmu_gather);
}

#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn radix__flush_tlb_mm(mm: *mut mm_struct);
    pub fn radix__flush_all_mm(mm: *mut mm_struct);
    pub fn radix__flush_tlb_page(vma: *mut vm_area_struct, vmaddr: ::core::ffi::c_ulong);
    pub fn radix__flush_tlb_page_psize(mm: *mut mm_struct, vmaddr: ::core::ffi::c_ulong, psize: ::core::ffi::c_int);
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn radix__flush_tlb_mm(mm: *mut mm_struct) { radix__local_flush_tlb_mm(mm); }
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn radix__flush_all_mm(mm: *mut mm_struct) { radix__local_flush_all_mm(mm); }
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn radix__flush_tlb_page(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong) { radix__local_flush_tlb_page(vma, addr); }
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn radix__flush_tlb_page_psize(mm: *mut mm_struct, addr: ::core::ffi::c_ulong, p: ::core::ffi::c_int) { radix__local_flush_tlb_page_psize(mm, addr, p); }

extern "C" {
    pub fn radix__flush_tlb_collapsed_pmd(mm: *mut mm_struct, addr: ::core::ffi::c_ulong);
    pub fn radix__flush_tlb_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
