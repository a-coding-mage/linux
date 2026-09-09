/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/page.h
 *
 *  Copyright (C) 1995-2003 Russell King
 */

// Dependency intent: <vdso/page.h>, <asm/page-nommu.h>, <asm/glue.h>,
// <asm/pgtable-*-types.h>, <asm/memory.h>, and asm-generic headers are
// supplied by the surrounding kernel translation.

#[cfg(not(feature = "config_mmu"))]
// The CONFIG_MMU=false branch supplies its declarations from asm/page-nommu.h.

#[cfg(feature = "config_mmu")]
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[cfg(feature = "config_mmu")]
#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[cfg(feature = "config_mmu")]
#[repr(C)]
pub struct cpu_user_fns {
    pub cpu_clear_user_highpage:
        Option<unsafe extern "C" fn(page: *mut page, vaddr: usize)>,
    pub cpu_copy_user_highpage: Option<unsafe extern "C" fn(
        to: *mut page,
        from: *mut page,
        vaddr: usize,
        vma: *mut vm_area_struct,
    )>,
}

#[cfg(feature = "config_mmu")]
extern "C" {
    pub fn fa_copy_user_highpage(
        to: *mut page, from: *mut page, vaddr: usize, vma: *mut vm_area_struct,
    );
    pub fn fa_clear_user_highpage(page: *mut page, vaddr: usize);
    pub fn feroceon_copy_user_highpage(
        to: *mut page, from: *mut page, vaddr: usize, vma: *mut vm_area_struct,
    );
    pub fn feroceon_clear_user_highpage(page: *mut page, vaddr: usize);
    pub fn v4_mc_copy_user_highpage(
        to: *mut page, from: *mut page, vaddr: usize, vma: *mut vm_area_struct,
    );
    pub fn v4_mc_clear_user_highpage(page: *mut page, vaddr: usize);
    pub fn v4wb_copy_user_highpage(
        to: *mut page, from: *mut page, vaddr: usize, vma: *mut vm_area_struct,
    );
    pub fn v4wb_clear_user_highpage(page: *mut page, vaddr: usize);
    pub fn v4wt_copy_user_highpage(
        to: *mut page, from: *mut page, vaddr: usize, vma: *mut vm_area_struct,
    );
    pub fn v4wt_clear_user_highpage(page: *mut page, vaddr: usize);
    pub fn xsc3_mc_copy_user_highpage(
        to: *mut page, from: *mut page, vaddr: usize, vma: *mut vm_area_struct,
    );
    pub fn xsc3_mc_clear_user_highpage(page: *mut page, vaddr: usize);
    pub fn xscale_mc_copy_user_highpage(
        to: *mut page, from: *mut page, vaddr: usize, vma: *mut vm_area_struct,
    );
    pub fn xscale_mc_clear_user_highpage(page: *mut page, vaddr: usize);

    #[cfg(feature = "multi_user")]
    pub static mut cpu_user: cpu_user_fns;

    #[cfg(not(feature = "multi_user"))]
    pub fn __cpu_clear_user_highpage(page: *mut page, vaddr: usize);
    #[cfg(not(feature = "multi_user"))]
    pub fn __cpu_copy_user_highpage(
        to: *mut page, from: *mut page, vaddr: usize, vma: *mut vm_area_struct,
    );

    pub fn memset(ptr: *mut core::ffi::c_void, value: i32, size: usize)
        -> *mut core::ffi::c_void;
    pub fn copy_page(to: *mut core::ffi::c_void, from: *const core::ffi::c_void);
}

#[cfg(feature = "config_mmu")]
#[inline]
pub unsafe fn clear_user_highpage(page: *mut page, vaddr: usize) {
    __cpu_clear_user_highpage(page, vaddr)
}

#[cfg(feature = "config_mmu")]
#[inline]
pub unsafe fn copy_user_highpage(
    to: *mut page, from: *mut page, vaddr: usize, vma: *mut vm_area_struct,
) {
    __cpu_copy_user_highpage(to, from, vaddr, vma)
}

#[cfg(feature = "config_mmu")]
#[inline]
pub unsafe fn clear_page(page: *mut core::ffi::c_void) {
    memset(page, 0, PAGE_SIZE);
}

#[cfg(feature = "config_mmu")]
pub const __HAVE_ARCH_COPY_USER_HIGHPAGE: bool = true;

#[cfg(feature = "config_kuser_helpers")]
pub const __HAVE_ARCH_GATE_AREA: i32 = 1;

// CONFIG_ARM_LPAE selects pgtable-3level-types.h; otherwise pgtable-2level-types.h.
#[cfg(all(feature = "config_mmu", not(feature = "config_arm_lpae"), feature = "config_vmap_stack"))]
pub const ARCH_PAGE_TABLE_SYNC_MASK: usize = PGTBL_PMD_MODIFIED;

pub type pgtable_t = *mut page;

#[cfg(feature = "config_have_arch_pfn_valid")]
extern "C" {
    pub fn pfn_valid(pfn: usize) -> i32;
}

pub const VMA_DATA_DEFAULT_FLAGS: usize = VMA_DATA_FLAGS_TSK_EXEC;

// Provided by asm/memory.h and asm-generic headers.
extern "C" {
    pub static PAGE_SIZE: usize;
}

// The following symbols are supplied by the included kernel headers.
extern "C" {
    pub static PGTBL_PMD_MODIFIED: usize;
    pub static VMA_DATA_FLAGS_TSK_EXEC: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
