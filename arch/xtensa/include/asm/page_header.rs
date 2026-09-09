/*
 * include/asm-xtensa/page.h
 *
 * Rust translation of the Xtensa page declarations and address helpers.
 */

// C includes are represented by the external symbols referenced below.

#[cfg(feature = "CONFIG_MMU")]
pub const PAGE_OFFSET: usize = XCHAL_KSEG_CACHED_VADDR;
#[cfg(feature = "CONFIG_MMU")]
pub const PHYS_OFFSET: usize = XCHAL_KSEG_PADDR;
#[cfg(feature = "CONFIG_MMU")]
pub const MAX_LOW_PFN: usize = PHYS_PFN(XCHAL_KSEG_PADDR) + PHYS_PFN(XCHAL_KSEG_SIZE);

#[cfg(not(feature = "CONFIG_MMU"))]
pub const PAGE_OFFSET: usize = CONFIG_DEFAULT_MEM_START;
#[cfg(not(feature = "CONFIG_MMU"))]
pub const PHYS_OFFSET: usize = CONFIG_DEFAULT_MEM_START;
#[cfg(not(feature = "CONFIG_MMU"))]
pub const MAX_LOW_PFN: usize = PHYS_PFN(0xffff_ffffusize);

// If DCACHE_WAY_SIZE > PAGE_SIZE in the target configuration, use the
// following cache-alias definitions; otherwise DCACHE_ALIAS_ORDER is zero.
pub const DCACHE_ALIAS_ORDER: usize = DCACHE_WAY_SHIFT - PAGE_SHIFT;
pub const DCACHE_ALIAS_MASK: usize = PAGE_MASK & (DCACHE_WAY_SIZE - 1);
#[inline]
pub const fn dcache_alias(a: usize) -> usize {
    (a & DCACHE_ALIAS_MASK) >> PAGE_SHIFT
}
#[inline]
pub const fn dcache_alias_eq(a: usize, b: usize) -> bool {
    ((a ^ b) & DCACHE_ALIAS_MASK) == 0
}
pub const DCACHE_N_COLORS: usize = 1usize << DCACHE_ALIAS_ORDER;

// If ICACHE_WAY_SIZE > PAGE_SIZE in the target configuration, use the
// following cache-alias definitions; otherwise ICACHE_ALIAS_ORDER is zero.
pub const ICACHE_ALIAS_ORDER: usize = ICACHE_WAY_SHIFT - PAGE_SHIFT;
pub const ICACHE_ALIAS_MASK: usize = PAGE_MASK & (ICACHE_WAY_SIZE - 1);
#[inline]
pub const fn icache_alias(a: usize) -> usize {
    (a & ICACHE_ALIAS_MASK) >> PAGE_SHIFT
}
#[inline]
pub const fn icache_alias_eq(a: usize, b: usize) -> bool {
    ((a ^ b) & ICACHE_ALIAS_MASK) == 0
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t {
    pub pte: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgd_t {
    pub pgd: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgprot_t {
    pub pgprot: usize,
}

pub type pgtable_t = *mut page;

#[inline]
pub const fn pte_val(x: pte_t) -> usize { x.pte }
#[inline]
pub const fn pgd_val(x: pgd_t) -> usize { x.pgd }
#[inline]
pub const fn pgprot_val(x: pgprot_t) -> usize { x.pgprot }
#[inline]
pub const fn __pte(x: usize) -> pte_t { pte_t { pte: x } }
#[inline]
pub const fn __pgd(x: usize) -> pgd_t { pgd_t { pgd: x } }
#[inline]
pub const fn __pgprot(x: usize) -> pgprot_t { pgprot_t { pgprot: x } }

pub enum page {}
pub enum vm_area_struct {}

unsafe extern "C" {
    pub fn clear_page(page: *mut core::ffi::c_void);
    pub fn copy_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void);
    #[cfg(all(feature = "CONFIG_MMU", feature = "DCACHE_WAY_SIZE_GT_PAGE_SIZE"))]
    pub fn clear_page_alias(vaddr: *mut core::ffi::c_void, paddr: usize);
    #[cfg(all(feature = "CONFIG_MMU", feature = "DCACHE_WAY_SIZE_GT_PAGE_SIZE"))]
    pub fn copy_page_alias(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void,
                           to_paddr: usize, from_paddr: usize);
}

#[cfg(all(feature = "CONFIG_MMU", feature = "DCACHE_WAY_SIZE_GT_PAGE_SIZE"))]
pub const __HAVE_ARCH_COPY_USER_HIGHPAGE: bool = true;
#[cfg(all(feature = "CONFIG_MMU", feature = "DCACHE_WAY_SIZE_GT_PAGE_SIZE"))]
unsafe extern "C" {
    pub fn clear_user_highpage(page: *mut page, vaddr: usize);
    pub fn copy_user_highpage(to: *mut page, from: *mut page, vaddr: usize,
                              vma: *mut vm_area_struct);
}

// In configurations without cache aliasing, copy_user_page(to, from, vaddr, pg)
// expands directly to copy_page(to, from).

pub const ARCH_PFN_OFFSET: usize = PHYS_OFFSET >> PAGE_SHIFT;

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn ___pa(mut va: usize) -> usize {
    let mut off = va.wrapping_sub(PAGE_OFFSET);
    if off >= XCHAL_KSEG_SIZE { off = off.wrapping_sub(XCHAL_KSEG_SIZE); }
    // CONFIG_XIP_KERNEL selects the alternate Xtensa I/O mapping below.
    #[cfg(not(feature = "CONFIG_XIP_KERNEL"))]
    { off.wrapping_add(PHYS_OFFSET) }
    #[cfg(feature = "CONFIG_XIP_KERNEL")]
    {
        if off < XCHAL_KSEG_SIZE { return off.wrapping_add(PHYS_OFFSET); }
        off = off.wrapping_sub(XCHAL_KSEG_SIZE);
        if off >= XCHAL_KIO_SIZE { off = off.wrapping_sub(XCHAL_KIO_SIZE); }
        off.wrapping_add(XCHAL_KIO_PADDR)
    }
}

#[inline]
pub unsafe fn __pa(x: *const core::ffi::c_void) -> usize {
    #[cfg(feature = "CONFIG_MMU")]
    { ___pa(x as usize) }
    #[cfg(not(feature = "CONFIG_MMU"))]
    { (x as usize).wrapping_sub(PAGE_OFFSET).wrapping_add(PHYS_OFFSET) }
}

#[inline]
pub unsafe fn __va(x: usize) -> *mut core::ffi::c_void {
    x.wrapping_sub(PHYS_OFFSET).wrapping_add(PAGE_OFFSET) as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn virt_to_page(kaddr: *const core::ffi::c_void) -> *mut page {
    pfn_to_page(__pa(kaddr) >> PAGE_SHIFT)
}
#[inline]
pub unsafe fn page_to_virt(p: *mut page) -> *mut core::ffi::c_void {
    __va(page_to_pfn(p) << PAGE_SHIFT)
}
#[inline]
pub unsafe fn virt_addr_valid(kaddr: *const core::ffi::c_void) -> bool {
    pfn_valid(__pa(kaddr) >> PAGE_SHIFT)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
