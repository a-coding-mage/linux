/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 1999  Niibe Yutaka
 */

/* <linux/const.h> and <vdso/page.h> provide the following dependent names. */

pub const PTE_MASK: usize = PAGE_MASK;

#[cfg(feature = "CONFIG_HUGETLB_PAGE_SIZE_64K")]
pub const HPAGE_SHIFT: usize = 16;
#[cfg(feature = "CONFIG_HUGETLB_PAGE_SIZE_256K")]
pub const HPAGE_SHIFT: usize = 18;
#[cfg(feature = "CONFIG_HUGETLB_PAGE_SIZE_1MB")]
pub const HPAGE_SHIFT: usize = 20;
#[cfg(feature = "CONFIG_HUGETLB_PAGE_SIZE_4MB")]
pub const HPAGE_SHIFT: usize = 22;
#[cfg(feature = "CONFIG_HUGETLB_PAGE_SIZE_64MB")]
pub const HPAGE_SHIFT: usize = 26;

#[cfg(feature = "CONFIG_HUGETLB_PAGE")]
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
#[cfg(feature = "CONFIG_HUGETLB_PAGE")]
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);
#[cfg(feature = "CONFIG_HUGETLB_PAGE")]
pub const HUGETLB_PAGE_ORDER: usize = HPAGE_SHIFT - PAGE_SHIFT;

extern "C" {
    pub static mut shm_align_mask: usize;
    pub static mut max_low_pfn: usize;
    pub static mut min_low_pfn: usize;
    pub static mut memory_start: usize;
    pub static mut memory_end: usize;
    pub static mut memory_limit: usize;
}

#[inline]
pub unsafe fn pages_do_alias(addr1: usize, addr2: usize) -> usize {
    (addr1 ^ addr2) & shm_align_mask
}

#[macro_export]
macro_rules! clear_page {
    ($page:expr) => {
        unsafe { core::ptr::write_bytes(($page as *mut u8), 0, PAGE_SIZE) }
    };
}

extern "C" {
    pub fn copy_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void);
    pub fn __copy_user(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void, size: usize);
}

#[macro_export]
macro_rules! copy_user_page {
    ($to:expr, $from:expr, $vaddr:expr, $pg:expr) => {
        unsafe { __copy_user($to, $from, PAGE_SIZE) }
    };
}

#[repr(C)]
pub struct page;
#[repr(C)]
pub struct vm_area_struct;

extern "C" {
    pub fn copy_user_highpage(
        to: *mut page,
        from: *mut page,
        vaddr: usize,
        vma: *mut vm_area_struct,
    );
    pub fn clear_user_highpage(page: *mut page, vaddr: usize);
}

/* __HAVE_ARCH_COPY_USER_HIGHPAGE */

#[cfg(feature = "CONFIG_X2TLB")]
#[repr(C)]
pub struct pte_t {
    pub pte_low: usize,
    pub pte_high: usize,
}
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[repr(C)]
pub struct pte_t {
    pub pte_low: usize,
}

#[cfg(feature = "CONFIG_X2TLB")]
#[repr(C)]
pub struct pgprot_t {
    pub pgprot: u64,
}
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[repr(C)]
pub struct pgprot_t {
    pub pgprot: usize,
}

#[cfg(feature = "CONFIG_X2TLB")]
#[repr(C)]
pub struct pgd_t {
    pub pgd: u64,
}
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[repr(C)]
pub struct pgd_t {
    pub pgd: usize,
}

#[cfg(feature = "CONFIG_X2TLB")]
#[inline]
pub fn pte_val(x: pte_t) -> u64 {
    x.pte_low as u64 | ((x.pte_high as u64) << 32)
}
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[inline]
pub fn pte_val(x: pte_t) -> usize {
    x.pte_low
}

#[cfg(feature = "CONFIG_X2TLB")]
#[inline]
pub fn __pte(x: u64) -> pte_t {
    pte_t { pte_low: x as usize, pte_high: x >> 32 }
}
#[cfg(not(feature = "CONFIG_X2TLB"))]
#[inline]
pub fn __pte(x: usize) -> pte_t {
    pte_t { pte_low: x }
}

#[inline]
pub fn pgd_val(x: pgd_t) -> usize { x.pgd as usize }
#[inline]
pub fn pgprot_val(x: pgprot_t) -> usize { x.pgprot as usize }

#[inline]
pub fn __pgd(x: usize) -> pgd_t { pgd_t { pgd: x as _ } }
#[inline]
pub fn __pgprot(x: usize) -> pgprot_t { pgprot_t { pgprot: x as _ } }

pub type pgtable_t = *mut page;

#[inline]
pub fn pte_pgprot(x: pte_t) -> pgprot_t {
    __pgprot(pte_val(x) as usize & PTE_FLAGS_MASK)
}

/*
 * __MEMORY_START and SIZE are the physical addresses and size of RAM.
 */
pub const __MEMORY_START: usize = CONFIG_MEMORY_START;
pub const __MEMORY_SIZE: usize = CONFIG_MEMORY_SIZE;

/*
 * PHYSICAL_OFFSET is the offset in physical memory where the base
 * of the kernel is loaded.
 */
#[cfg(feature = "CONFIG_PHYSICAL_START")]
pub const PHYSICAL_OFFSET: usize = CONFIG_PHYSICAL_START - __MEMORY_START;
#[cfg(not(feature = "CONFIG_PHYSICAL_START"))]
pub const PHYSICAL_OFFSET: usize = 0;

/* PAGE_OFFSET is the virtual address of the start of kernel address space. */
pub const PAGE_OFFSET: usize = CONFIG_PAGE_OFFSET;

#[cfg(feature = "CONFIG_PMB")]
#[inline]
pub const fn ___pa(x: usize) -> usize { x - PAGE_OFFSET + __MEMORY_START }
#[cfg(feature = "CONFIG_PMB")]
#[inline]
pub const fn ___va(x: usize) -> usize { x + PAGE_OFFSET - __MEMORY_START }
#[cfg(not(feature = "CONFIG_PMB"))]
#[inline]
pub const fn ___pa(x: usize) -> usize { x - PAGE_OFFSET }
#[cfg(not(feature = "CONFIG_PMB"))]
#[inline]
pub const fn ___va(x: usize) -> usize { x + PAGE_OFFSET }

#[inline]
pub fn __pa<T>(x: *const T) -> usize { ___pa(x as usize) }
#[inline]
pub fn __va(x: usize) -> *mut core::ffi::c_void { ___va(x) as *mut core::ffi::c_void }

#[cfg(feature = "CONFIG_UNCACHED_MAPPING")]
#[cfg(feature = "CONFIG_29BIT")]
pub fn UNCAC_ADDR(addr: usize) -> usize { P2SEGADDR(addr) }
#[cfg(feature = "CONFIG_UNCACHED_MAPPING")]
#[cfg(feature = "CONFIG_29BIT")]
pub fn CAC_ADDR(addr: usize) -> usize { P1SEGADDR(addr) }
#[cfg(feature = "CONFIG_UNCACHED_MAPPING")]
#[cfg(not(feature = "CONFIG_29BIT"))]
pub fn UNCAC_ADDR(addr: usize) -> usize { addr - PAGE_OFFSET + uncached_start }
#[cfg(feature = "CONFIG_UNCACHED_MAPPING")]
#[cfg(not(feature = "CONFIG_29BIT"))]
pub fn CAC_ADDR(addr: usize) -> usize { addr - uncached_start + PAGE_OFFSET }
#[cfg(not(feature = "CONFIG_UNCACHED_MAPPING"))]
pub const fn UNCAC_ADDR(addr: usize) -> usize { addr }
#[cfg(not(feature = "CONFIG_UNCACHED_MAPPING"))]
pub const fn CAC_ADDR(addr: usize) -> usize { addr }

#[inline]
pub fn pfn_to_kaddr(pfn: usize) -> *mut core::ffi::c_void { __va(pfn << PAGE_SHIFT) }

pub const PFN_START: usize = __MEMORY_START >> PAGE_SHIFT;
pub const ARCH_PFN_OFFSET: usize = PFN_START;

#[inline]
pub fn virt_to_page<T>(kaddr: *const T) -> *mut page { pfn_to_page(__pa(kaddr) >> PAGE_SHIFT) }
#[inline]
pub fn virt_addr_valid<T>(kaddr: *const T) -> bool { pfn_valid(__pa(kaddr) >> PAGE_SHIFT) }

/* <asm-generic/memory_model.h> and <asm-generic/getorder.h> dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
