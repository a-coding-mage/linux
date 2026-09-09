/* SPDX-License-Identifier: GPL-2.0 */

// Translated from parisc/include/asm/page.h.
// C includes and build-time configuration symbols are supplied by the surrounding kernel.

pub const HAVE_ARCH_HUGETLB_UNMAPPED_AREA: bool = true;

#[cfg(not(feature = "assembler"))]
pub struct page;
#[cfg(not(feature = "assembler"))]
pub struct vm_area_struct;

#[cfg(not(feature = "assembler"))]
unsafe extern "C" {
    pub fn clear_page_asm(page: *mut core::ffi::c_void);
    pub fn copy_page_asm(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void);
    pub fn copy_user_highpage(
        to: *mut page,
        from: *mut page,
        vaddr: usize,
        vma: *mut vm_area_struct,
    );
}

#[cfg(not(feature = "assembler"))]
#[macro_export]
macro_rules! clear_page {
    ($page:expr) => {{ unsafe { $crate::clear_page_asm($page as *mut core::ffi::c_void) } }};
}
#[cfg(not(feature = "assembler"))]
#[macro_export]
macro_rules! copy_page {
    ($to:expr, $from:expr) => {{
        unsafe {
            $crate::copy_page_asm(
                $to as *mut core::ffi::c_void,
                $from as *mut core::ffi::c_void,
            )
        }
    }};
}

// These are used to make use of C type-checking.
// STRICT_MM_TYPECHECKS is defined by the source header.
#[repr(C)]
pub struct pte_t {
    pub pte: usize,
}
#[repr(C)]
pub struct pgd_t {
    pub pgd: u32,
}
#[repr(C)]
pub struct pgprot_t {
    pub pgprot: usize,
}

#[cfg(feature = "pgtables-3")]
#[repr(C)]
pub struct pmd_t {
    pub pmd: u32,
}

#[cfg(feature = "pgtables-3")]
#[inline]
pub const fn __pmd(x: u32) -> pmd_t { pmd_t { pmd: x } }
#[cfg(feature = "pgtables-3")]
#[inline]
pub const fn pmd_val(x: pmd_t) -> u32 { x.pmd + 0 }
#[inline]
pub const fn pte_val(x: pte_t) -> usize { x.pte }
#[inline]
pub const fn pgd_val(x: pgd_t) -> u32 { x.pgd + 0 }
#[inline]
pub const fn pgprot_val(x: pgprot_t) -> usize { x.pgprot }
#[inline]
pub const fn __pte(x: usize) -> pte_t { pte_t { pte: x } }
#[inline]
pub const fn __pgd(x: u32) -> pgd_t { pgd_t { pgd: x } }
#[inline]
pub const fn __pgprot(x: usize) -> pgprot_t { pgprot_t { pgprot: x } }

#[inline]
pub unsafe fn set_pmd(pmdptr: *mut pmd_t, pmdval: pmd_t) { unsafe { *pmdptr = pmdval; } }
#[cfg(feature = "pgtables-3")]
#[inline]
pub unsafe fn set_pud<T>(pudptr: *mut T, pudval: T) { unsafe { *pudptr = pudval; } }

pub type pgtable_t = *mut page;
#[repr(C)]
pub struct physmem_range_t {
    pub start_pfn: usize,
    pub pages: usize, // PAGE_SIZE pages
}
unsafe extern "C" {
    pub static mut pmem_ranges: physmem_range_t;
    pub static mut npmem_ranges: i32;
}

// WARNING: These definitions must match exactly to sizeof(pte_t), etc.
#[cfg(feature = "64bit")]
pub const BITS_PER_PTE_ENTRY: usize = 3;
#[cfg(not(feature = "64bit"))]
pub const BITS_PER_PTE_ENTRY: usize = 2;
pub const BITS_PER_PMD_ENTRY: usize = 2;
pub const BITS_PER_PGD_ENTRY: usize = 2;
pub const PGD_ENTRY_SIZE: usize = 1usize << BITS_PER_PGD_ENTRY;
pub const PMD_ENTRY_SIZE: usize = 1usize << BITS_PER_PMD_ENTRY;
pub const PTE_ENTRY_SIZE: usize = 1usize << BITS_PER_PTE_ENTRY;

pub const LINUX_GATEWAY_SPACE: usize = 0;
#[cfg(feature = "64bit")]
pub const __PAGE_OFFSET_DEFAULT: usize = 0x40000000;
#[cfg(not(feature = "64bit"))]
pub const __PAGE_OFFSET_DEFAULT: usize = 0x10000000;
// BOOTLOADER selects zero; otherwise __PAGE_OFFSET_DEFAULT is used.
#[cfg(feature = "bootloader")]
pub const __PAGE_OFFSET: usize = 0;
#[cfg(not(feature = "bootloader"))]
pub const __PAGE_OFFSET: usize = __PAGE_OFFSET_DEFAULT;
pub const PAGE_OFFSET: usize = __PAGE_OFFSET;
pub const GATEWAY_PAGE_SIZE: usize = 0x4000;
pub const KERNEL_BINARY_TEXT_START: usize = __PAGE_OFFSET + 0x100000;

#[inline]
pub fn __pa<T>(x: *const T) -> usize { x as usize - PAGE_OFFSET }
#[inline]
pub fn __va(x: usize) -> *mut core::ffi::c_void { (x + PAGE_OFFSET) as *mut core::ffi::c_void }

// CONFIG_HUGETLB_PAGE defines the following constants; PMD_SHIFT, PAGE_SHIFT,
// and page-size encodings are supplied by the surrounding architecture headers.
#[cfg(feature = "hugetlb-page")]
pub const HPAGE_SHIFT: usize = PMD_SHIFT;
#[cfg(feature = "hugetlb-page")]
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
#[cfg(feature = "hugetlb-page")]
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);
#[cfg(feature = "hugetlb-page")]
pub const HUGETLB_PAGE_ORDER: usize = HPAGE_SHIFT - PAGE_SHIFT;

#[inline]
pub fn virt_addr_valid<T>(kaddr: *const T) -> bool {
    unsafe { pfn_valid(__pa(kaddr) >> PAGE_SHIFT) }
}
#[inline]
pub fn virt_to_page<T>(kaddr: *const T) -> *mut page {
    unsafe { pfn_to_page(__pa(kaddr) >> PAGE_SHIFT) }
}

extern "C" {
    fn pfn_valid(pfn: usize) -> bool;
    fn pfn_to_page(pfn: usize) -> *mut page;
}

// PAGE0 is ((struct zeropage *)absolute_pointer(__PAGE_OFFSET)).
// DEFINITION OF THE ZERO-PAGE (PAG0), based on work by Jason Eckhardt.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
