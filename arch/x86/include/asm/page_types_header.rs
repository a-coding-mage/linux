/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/const.h, linux/types.h, linux/mem_encrypt.h, and vdso/page.h.

pub const __VIRTUAL_MASK: usize = (1usize << __VIRTUAL_MASK_SHIFT) - 1;

/* Cast P*D_MASK to a signed type so that it is sign-extended if
   virtual addresses are 32-bits but physical addresses are larger
   (ie, 32-bit PAE). */
#[cfg(feature = "CONFIG_DYNAMIC_PHYSICAL_MASK")]
#[inline]
pub fn __PHYSICAL_MASK() -> phys_addr_t {
    unsafe { physical_mask }
}

#[cfg(not(feature = "CONFIG_DYNAMIC_PHYSICAL_MASK"))]
pub const __PHYSICAL_MASK: phys_addr_t =
    ((1u64 << __PHYSICAL_MASK_SHIFT) - 1) as phys_addr_t;

#[inline]
pub fn PHYSICAL_PAGE_MASK() -> phys_addr_t {
    ((PAGE_MASK as isize) as phys_addr_t) & __PHYSICAL_MASK_VALUE()
}

#[inline]
pub fn PHYSICAL_PMD_PAGE_MASK() -> phys_addr_t {
    ((PMD_MASK as isize) as phys_addr_t) & __PHYSICAL_MASK_VALUE()
}

#[inline]
pub fn PHYSICAL_PUD_PAGE_MASK() -> phys_addr_t {
    ((PUD_MASK as isize) as phys_addr_t) & __PHYSICAL_MASK_VALUE()
}

#[cfg(feature = "CONFIG_DYNAMIC_PHYSICAL_MASK")]
#[inline]
fn __PHYSICAL_MASK_VALUE() -> phys_addr_t {
    unsafe { physical_mask }
}

#[cfg(not(feature = "CONFIG_DYNAMIC_PHYSICAL_MASK"))]
#[inline]
fn __PHYSICAL_MASK_VALUE() -> phys_addr_t {
    __PHYSICAL_MASK
}

pub const HPAGE_SHIFT: usize = PMD_SHIFT;
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);
pub const HUGETLB_PAGE_ORDER: usize = HPAGE_SHIFT - PAGE_SHIFT;

pub const HUGE_MAX_HSTATE: usize = 2;

pub const PAGE_OFFSET: usize = __PAGE_OFFSET as usize;

pub const VMA_DATA_DEFAULT_FLAGS: usize = VMA_DATA_FLAGS_TSK_EXEC;

/* Physical address where kernel should be loaded. */
pub const LOAD_PHYSICAL_ADDR: usize =
    __ALIGN_KERNEL_MASK(CONFIG_PHYSICAL_START, CONFIG_PHYSICAL_ALIGN - 1);

pub const __START_KERNEL: usize = __START_KERNEL_map + LOAD_PHYSICAL_ADDR;

// CONFIG_X86_64 selects asm/page_64_types.h and PUD_SHIFT; otherwise
// asm/page_32_types.h is selected and PMD_SHIFT is used.
#[cfg(feature = "CONFIG_X86_64")]
pub const IOREMAP_MAX_ORDER: usize = PUD_SHIFT;

#[cfg(not(feature = "CONFIG_X86_64"))]
pub const IOREMAP_MAX_ORDER: usize = PMD_SHIFT;

#[cfg(feature = "CONFIG_DYNAMIC_PHYSICAL_MASK")]
extern "C" {
    pub static mut physical_mask: phys_addr_t;
}

extern "C" {
    pub fn devmem_is_allowed(pagenr: libc::c_ulong) -> libc::c_int;
    pub static mut max_low_pfn_mapped: libc::c_ulong;
    pub static mut max_pfn_mapped: libc::c_ulong;
}

#[inline]
pub unsafe fn get_max_mapped() -> phys_addr_t {
    (max_pfn_mapped as phys_addr_t) << PAGE_SHIFT
}

extern "C" {
    pub fn pfn_range_is_mapped(start_pfn: libc::c_ulong, end_pfn: libc::c_ulong) -> bool;
    pub fn initmem_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
