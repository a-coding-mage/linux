/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2009 Chen Liqin <liqin.chen@sunplusct.com>
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2017 SiFive
 * Copyright (C) 2017 XiaojingZhu <zhuxiaoj@ict.ac.cn>
 */

/* Translated from the RISC-V page header. C preprocessor configuration
 * conditions are retained as Rust cfg conditions where applicable. */

pub const HPAGE_SHIFT: usize = PMD_SHIFT;
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);
pub const HUGETLB_PAGE_ORDER: usize = HPAGE_SHIFT - PAGE_SHIFT;

/* PAGE_OFFSET is the first address of the first page of memory. */
#[cfg(all(CONFIG_MMU, CONFIG_64BIT))]
pub const PAGE_OFFSET_L5: usize = 0xff60000000000000;
#[cfg(all(CONFIG_MMU, CONFIG_64BIT))]
pub const PAGE_OFFSET_L4: usize = 0xffffaf8000000000;
#[cfg(all(CONFIG_MMU, CONFIG_64BIT))]
pub const PAGE_OFFSET_L3: usize = 0xffffffd600000000;
#[cfg(all(CONFIG_MMU, CONFIG_64BIT))]
pub static PAGE_OFFSET: usize = unsafe { kernel_map.page_offset };
#[cfg(all(CONFIG_MMU, not(CONFIG_64BIT)))]
pub const PAGE_OFFSET: usize = 0xc0000000;
#[cfg(not(CONFIG_MMU))]
pub static PAGE_OFFSET: usize = unsafe { phys_ram_base as usize };

#[cfg(CONFIG_RISCV_ISA_ZICBOZ)]
extern "C" {
    pub fn clear_page(page: *mut core::ffi::c_void);
}

pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *const core::ffi::c_void) {
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, PAGE_SIZE);
}

pub unsafe fn copy_user_page(
    vto: *mut core::ffi::c_void,
    vfrom: *const core::ffi::c_void,
    _vaddr: usize,
    _topg: *mut Page,
) {
    copy_page(vto, vfrom);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgd_t { pub pgd: usize }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t { pub pte: usize }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgprot_t { pub pgprot: usize }

pub type pgtable_t = *mut Page;

#[inline] pub const fn pte_val(x: pte_t) -> usize { x.pte }
#[inline] pub const fn pgd_val(x: pgd_t) -> usize { x.pgd }
#[inline] pub const fn pgprot_val(x: pgprot_t) -> usize { x.pgprot }
#[inline] pub const fn __pte(x: usize) -> pte_t { pte_t { pte: x } }
#[inline] pub const fn __pgd(x: usize) -> pgd_t { pgd_t { pgd: x } }
#[inline] pub const fn __pgprot(x: usize) -> pgprot_t { pgprot_t { pgprot: x } }

#[cfg(CONFIG_64BIT)]
pub const PTE_FMT: &str = "%016lx";
#[cfg(not(CONFIG_64BIT))]
pub const PTE_FMT: &str = "%08lx";

#[cfg(all(CONFIG_64BIT, CONFIG_MMU))]
pub const MIN_MEMBLOCK_ADDR: usize = 0;

pub const ARCH_PFN_OFFSET: usize = PFN_DOWN(unsafe { phys_ram_base as usize });

#[repr(C)]
pub struct kernel_mapping {
    pub virt_addr: usize,
    pub virt_offset: usize,
    pub phys_addr: usize,
    pub size: usize,
    pub va_pa_offset: usize,
    pub page_offset: usize,
    pub va_kernel_pa_offset: usize,
}

extern "C" {
    pub static mut kernel_map: kernel_mapping;
    pub static mut phys_ram_base: usize;
    pub static mut vmemmap_start_pfn: usize;
}

pub unsafe fn is_kernel_mapping(x: usize) -> bool {
    x >= kernel_map.virt_addr && x < kernel_map.virt_addr + kernel_map.size
}

pub unsafe fn is_linear_mapping(x: usize) -> bool {
    x >= PAGE_OFFSET && (!cfg!(CONFIG_64BIT) || x < PAGE_OFFSET + KERN_VIRT_SIZE)
}

#[cfg(not(CONFIG_DEBUG_VIRTUAL))]
#[inline] pub unsafe fn linear_mapping_pa_to_va(x: usize) -> *mut core::ffi::c_void {
    (x + kernel_map.va_pa_offset) as *mut core::ffi::c_void
}
#[cfg(CONFIG_DEBUG_VIRTUAL)]
extern "C" { pub fn linear_mapping_pa_to_va(x: usize) -> *mut core::ffi::c_void; }

#[inline] pub unsafe fn kernel_mapping_pa_to_va(y: usize) -> *mut core::ffi::c_void {
    (y + kernel_map.va_kernel_pa_offset) as *mut core::ffi::c_void
}
#[inline] pub unsafe fn __pa_to_va_nodebug(x: usize) -> *mut core::ffi::c_void { linear_mapping_pa_to_va(x) }

#[cfg(not(CONFIG_DEBUG_VIRTUAL))]
#[inline] pub unsafe fn linear_mapping_va_to_pa(x: usize) -> usize { x - kernel_map.va_pa_offset }
#[cfg(CONFIG_DEBUG_VIRTUAL)]
extern "C" { pub fn linear_mapping_va_to_pa(x: usize) -> usize; }

#[inline] pub unsafe fn kernel_mapping_va_to_pa(y: usize) -> usize { y - kernel_map.va_kernel_pa_offset }
#[inline] pub unsafe fn __va_to_pa_nodebug(x: usize) -> usize {
    if is_linear_mapping(x) { linear_mapping_va_to_pa(x) } else { kernel_mapping_va_to_pa(x) }
}

#[cfg(CONFIG_DEBUG_VIRTUAL)]
extern "C" { pub fn __virt_to_phys(x: usize) -> usize; pub fn __phys_addr_symbol(x: usize) -> usize; }
#[cfg(not(CONFIG_DEBUG_VIRTUAL))]
#[inline] pub unsafe fn __virt_to_phys(x: usize) -> usize { __va_to_pa_nodebug(x) }
#[cfg(not(CONFIG_DEBUG_VIRTUAL))]
#[inline] pub unsafe fn __phys_addr_symbol(x: usize) -> usize { __va_to_pa_nodebug(x) }

#[inline] pub unsafe fn __pa_symbol(x: usize) -> usize { __phys_addr_symbol(x) }
#[inline] pub unsafe fn __pa(x: usize) -> usize { __virt_to_phys(x) }
#[inline] pub unsafe fn __va(x: usize) -> *mut core::ffi::c_void { __pa_to_va_nodebug(x) }

#[inline] pub const fn phys_to_pfn(phys: usize) -> usize { PFN_DOWN(phys) }
#[inline] pub const fn pfn_to_phys(pfn: usize) -> usize { PFN_PHYS(pfn) }
#[inline] pub unsafe fn virt_to_pfn(vaddr: usize) -> usize { phys_to_pfn(__pa(vaddr)) }
#[inline] pub unsafe fn pfn_to_virt(pfn: usize) -> *mut core::ffi::c_void { __va(pfn_to_phys(pfn)) }
#[inline] pub unsafe fn virt_to_page(vaddr: usize) -> *mut Page { pfn_to_page(virt_to_pfn(vaddr)) }
#[inline] pub unsafe fn page_to_virt(page: *mut Page) -> *mut core::ffi::c_void { pfn_to_virt(page_to_pfn(page)) }
#[inline] pub unsafe fn sym_to_pfn(x: usize) -> usize { __phys_to_pfn(__pa_symbol(x)) }

#[inline] pub unsafe fn kaslr_offset() -> usize { kernel_map.virt_offset }
#[inline] pub unsafe fn pfn_to_kaddr(pfn: usize) -> *mut core::ffi::c_void { __va(pfn << PAGE_SHIFT) }

pub unsafe fn virt_addr_valid(vaddr: usize) -> bool {
    vaddr >= PAGE_OFFSET && pfn_valid(virt_to_pfn(vaddr))
}

pub const VMA_DATA_DEFAULT_FLAGS: usize = VMA_DATA_FLAGS_NON_EXEC;

/* Dependencies supplied by the surrounding kernel translation. */
extern "C" {
    fn PFN_DOWN(x: usize) -> usize;
    fn PFN_PHYS(x: usize) -> usize;
    fn pfn_to_page(x: usize) -> *mut Page;
    fn page_to_pfn(x: *mut Page) -> usize;
    fn pfn_valid(x: usize) -> bool;
    fn __phys_to_pfn(x: usize) -> usize;
}

pub type Page = core::ffi::c_void;
extern "C" {
    static PMD_SHIFT: usize;
    static PAGE_SHIFT: usize;
    static PAGE_SIZE: usize;
    static KERN_VIRT_SIZE: usize;
    static VMA_DATA_FLAGS_NON_EXEC: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
