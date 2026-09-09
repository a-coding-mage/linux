/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _SPARC64_PAGE_H */
/* Dependencies: linux/const.h, vdso/page.h, asm-generic/memory_model.h,
 * and asm-generic/getorder.h are supplied externally. */

/* Flushing for D-cache alias handling is only needed if the page size is
 * smaller than 16K.  C preprocessor condition: PAGE_SHIFT < 14. */

pub const HPAGE_SHIFT: u32 = 23;
pub const REAL_HPAGE_SHIFT: u32 = 22;
pub const HPAGE_16GB_SHIFT: u32 = 34;
pub const HPAGE_2GB_SHIFT: u32 = 31;
pub const HPAGE_256MB_SHIFT: u32 = 28;
pub const HPAGE_64K_SHIFT: u32 = 16;
pub const REAL_HPAGE_SIZE: usize = 1usize << REAL_HPAGE_SHIFT;

/* These items are present when CONFIG_HUGETLB_PAGE or
 * CONFIG_TRANSPARENT_HUGEPAGE is enabled in the C build. */
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1usize);
pub const HUGETLB_PAGE_ORDER: u32 = HPAGE_SHIFT - PAGE_SHIFT;
pub const REAL_HPAGE_PER_HPAGE: usize = 1usize << (HPAGE_SHIFT - REAL_HPAGE_SHIFT);
pub const HUGE_MAX_HSTATE: u32 = 5;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

/* C declarations retained as external interfaces. */
unsafe extern "C" {
    pub fn hugetlb_setup(regs: *mut pt_regs);
    pub fn _clear_page(page: *mut core::ffi::c_void);
    pub fn clear_user_page(addr: *mut core::ffi::c_void, vaddr: c_ulong, page: *mut page);
    pub fn copy_user_page(
        to: *mut core::ffi::c_void,
        from: *mut core::ffi::c_void,
        vaddr: c_ulong,
        topage: *mut page,
    );
    pub fn copy_user_highpage(
        to: *mut page,
        from: *mut page,
        vaddr: c_ulong,
        vma: *mut vm_area_struct,
    );
    pub fn copy_highpage(to: *mut page, from: *mut page);
    pub static mut sparc64_va_hole_top: c_ulong;
    pub static mut sparc64_va_hole_bottom: c_ulong;
    pub static mut PAGE_OFFSET: c_ulong;
}

pub type c_ulong = usize;

#[inline]
pub unsafe fn clear_page(x: *mut core::ffi::c_void) {
    _clear_page(x);
}

#[inline]
pub unsafe fn copy_page(x: *mut core::ffi::c_void, y: *const core::ffi::c_void) {
    core::ptr::copy_nonoverlapping(y as *const u8, x as *mut u8, PAGE_SIZE);
}

/* STRICT_MM_TYPECHECKS is enabled. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t { pub pte: c_ulong }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopte_t { pub iopte: c_ulong }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmd_t { pub pmd: c_ulong }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pud_t { pub pud: c_ulong }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgd_t { pub pgd: c_ulong }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgprot_t { pub pgprot: c_ulong }

pub type pgtable_t = *mut pte_t;

#[inline] pub const fn pte_val(x: pte_t) -> c_ulong { x.pte }
#[inline] pub const fn iopte_val(x: iopte_t) -> c_ulong { x.iopte }
#[inline] pub const fn pmd_val(x: pmd_t) -> c_ulong { x.pmd }
#[inline] pub const fn pud_val(x: pud_t) -> c_ulong { x.pud }
#[inline] pub const fn pgd_val(x: pgd_t) -> c_ulong { x.pgd }
#[inline] pub const fn pgprot_val(x: pgprot_t) -> c_ulong { x.pgprot }
#[inline] pub const fn __pte(x: c_ulong) -> pte_t { pte_t { pte: x } }
#[inline] pub const fn __iopte(x: c_ulong) -> iopte_t { iopte_t { iopte: x } }
#[inline] pub const fn __pmd(x: c_ulong) -> pmd_t { pmd_t { pmd: x } }
#[inline] pub const fn __pud(x: c_ulong) -> pud_t { pud_t { pud: x } }
#[inline] pub const fn __pgd(x: c_ulong) -> pgd_t { pgd_t { pgd: x } }
#[inline] pub const fn __pgprot(x: c_ulong) -> pgprot_t { pgprot_t { pgprot: x } }

/* C macro: VA_EXCLUDE_START / VA_EXCLUDE_END. */
#[inline] pub unsafe fn va_exclude_start() -> c_ulong { sparc64_va_hole_bottom - (1usize << 32) }
#[inline] pub unsafe fn va_exclude_end() -> c_ulong { sparc64_va_hole_top + (1usize << 32) }

pub const MAX_PHYS_ADDRESS_BITS: u32 = 53;
pub const ILOG2_4MB: u32 = 22;
pub const ILOG2_256MB: u32 = 28;

#[inline] pub unsafe fn __pa<T>(x: *const T) -> c_ulong { x as c_ulong - PAGE_OFFSET }
#[inline] pub unsafe fn __va(x: c_ulong) -> *mut core::ffi::c_void { (x + PAGE_OFFSET) as *mut core::ffi::c_void }
#[inline] pub unsafe fn pfn_to_kaddr(pfn: c_ulong) -> *mut core::ffi::c_void { __va(pfn << PAGE_SHIFT) }

/* virt_to_page, virt_addr_valid, virt_to_phys, and phys_to_virt depend on
 * externally supplied pfn_to_page, pfn_valid, and page-table definitions. */

#[inline] pub unsafe fn virt_to_phys<T>(kaddr: *const T) -> c_ulong { __pa(kaddr) }
#[inline] pub unsafe fn phys_to_virt(x: c_ulong) -> *mut core::ffi::c_void { __va(x) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
