/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of the PowerPC page header. */

/* C header dependencies: linux/types.h, linux/kernel.h, linux/bug.h,
 * asm/types.h, asm/asm-const.h, vdso/page.h, asm/page_{32,64}.h,
 * asm/pgtable-types.h, asm-generic/memory_model.h. */

#[cfg(not(feature = "hugetlb_page"))]
pub const HPAGE_SHIFT: usize = PAGE_SHIFT;
#[cfg(all(feature = "hugetlb_page", feature = "ppc_book3s_64"))]
extern "C" {
    pub static mut hpage_shift: ::core::ffi::c_uint;
}
#[cfg(all(feature = "hugetlb_page", feature = "ppc_8xx"))]
pub const HPAGE_SHIFT: usize = 19; /* 512k pages */
#[cfg(all(feature = "hugetlb_page", feature = "ppc_e500"))]
pub const HPAGE_SHIFT: usize = 22; /* 4M pages */

#[cfg(not(feature = "hugetlb_page"))]
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
#[cfg(not(feature = "hugetlb_page"))]
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);
#[cfg(not(feature = "hugetlb_page"))]
pub const HUGETLB_PAGE_ORDER: usize = HPAGE_SHIFT - PAGE_SHIFT;
#[cfg(not(feature = "hugetlb_page"))]
pub const HUGE_MAX_HSTATE: usize = MMU_PAGE_COUNT - 1;

pub const KERNELBASE: usize = CONFIG_KERNEL_START;
pub const PAGE_OFFSET: usize = CONFIG_PAGE_OFFSET;
pub const LOAD_OFFSET: usize = CONFIG_KERNEL_START - CONFIG_PHYSICAL_START;

#[cfg(feature = "nonstatic_kernel")]
extern "C" {
    pub static mut memstart_addr: phys_addr_t;
    pub static mut kernstart_addr: phys_addr_t;
    #[cfg(all(feature = "relocatable", feature = "ppc32"))]
    pub static mut virt_phys_offset: i64;
}

#[cfg(feature = "nonstatic_kernel")]
pub const PHYSICAL_START: phys_addr_t = unsafe { kernstart_addr };
#[cfg(not(feature = "nonstatic_kernel"))]
pub const PHYSICAL_START: usize = CONFIG_PHYSICAL_START;

#[cfg(all(feature = "ppc32", feature = "booke"))]
pub const VIRT_PHYS_OFFSET: usize = if cfg!(feature = "relocatable") {
    unsafe { virt_phys_offset as usize }
} else {
    KERNELBASE - PHYSICAL_START
};

#[cfg(feature = "ppc64")]
pub const MEMORY_START: usize = 0;
#[cfg(all(not(feature = "ppc64"), feature = "nonstatic_kernel"))]
pub const MEMORY_START: phys_addr_t = unsafe { memstart_addr };
#[cfg(all(not(feature = "ppc64"), not(feature = "nonstatic_kernel")))]
pub const MEMORY_START: usize = PHYSICAL_START + PAGE_OFFSET - KERNELBASE;

#[cfg(feature = "flatmem")]
pub const ARCH_PFN_OFFSET: usize = (MEMORY_START >> PAGE_SHIFT) as usize;

#[cfg(all(feature = "ppc32", feature = "booke"))]
#[inline]
pub unsafe fn __va<T>(x: T) -> *mut core::ffi::c_void {
    ((x as phys_addr_t).wrapping_add(VIRT_PHYS_OFFSET as phys_addr_t)) as usize as *mut core::ffi::c_void
}
#[cfg(all(feature = "ppc32", feature = "booke"))]
#[inline]
pub unsafe fn __pa<T>(x: *const T) -> phys_addr_t {
    (x as usize as phys_addr_t).wrapping_sub(VIRT_PHYS_OFFSET as phys_addr_t)
}

#[cfg(all(not(feature = "ppc32"), feature = "ppc64"))]
#[inline]
pub unsafe fn __va<T>(x: T) -> *mut core::ffi::c_void {
    ((x as phys_addr_t) | PAGE_OFFSET as phys_addr_t) as usize as *mut core::ffi::c_void
}
#[cfg(all(not(feature = "ppc32"), feature = "ppc64"))]
#[inline]
pub unsafe fn __pa<T>(x: *const T) -> usize {
    (x as usize) & 0x0fffffffffffffffusize
}

#[cfg(all(not(feature = "ppc32"), not(feature = "ppc64")))]
#[inline]
pub unsafe fn __va<T>(x: T) -> *mut core::ffi::c_void {
    ((x as phys_addr_t) + PAGE_OFFSET as phys_addr_t - MEMORY_START as phys_addr_t) as usize as *mut core::ffi::c_void
}
#[cfg(all(not(feature = "ppc32"), not(feature = "ppc64")))]
#[inline]
pub unsafe fn __pa<T>(x: *const T) -> usize {
    (x as usize) - PAGE_OFFSET + MEMORY_START
}

#[inline]
pub unsafe fn virt_to_pfn(kaddr: *const core::ffi::c_void) -> usize {
    (__pa(kaddr) >> PAGE_SHIFT) as usize
}

#[inline]
pub unsafe fn pfn_to_kaddr(pfn: usize) -> *const core::ffi::c_void {
    __va(pfn << PAGE_SHIFT) as *const core::ffi::c_void
}

#[inline]
pub unsafe fn virt_to_page(kaddr: *const core::ffi::c_void) -> *mut page {
    pfn_to_page(virt_to_pfn(kaddr))
}

#[inline]
pub unsafe fn virt_addr_valid(vaddr: *const core::ffi::c_void) -> bool {
    let addr = vaddr as usize;
    addr >= PAGE_OFFSET && addr < high_memory as usize && pfn_valid(virt_to_pfn(addr as *const core::ffi::c_void))
}

pub const VMA_DATA_DEFAULT_FLAGS32: usize = VMA_DATA_FLAGS_TSK_EXEC;
pub const VMA_DATA_DEFAULT_FLAGS64: usize = VMA_DATA_FLAGS_NON_EXEC;

#[cfg(feature = "ppc_book3e_64")]
#[inline]
pub fn is_kernel_addr(x: usize) -> bool { x >= 0x8000000000000000usize }
#[cfg(all(not(feature = "ppc_book3e_64"), feature = "ppc_book3s_64"))]
#[inline]
pub fn is_kernel_addr(x: usize) -> bool { x >= PAGE_OFFSET }
#[cfg(all(not(feature = "ppc_book3e_64"), not(feature = "ppc_book3s_64")))]
#[inline]
pub fn is_kernel_addr(x: usize) -> bool { x >= TASK_SIZE }

#[repr(C)]
pub struct page { _private: [u8; 0] }
extern "C" {
    pub fn clear_user_page(page: *mut core::ffi::c_void, vaddr: usize, pg: *mut page);
    pub fn copy_user_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void, vaddr: usize, p: *mut page);
    pub fn devmem_is_allowed(pfn: usize) -> i32;
    pub static mut kernstart_virt_addr: usize;
}

#[cfg(feature = "ppc_smlpar")]
extern "C" { pub fn arch_free_page(page: *mut page, order: i32); }

#[inline]
pub unsafe fn kaslr_offset() -> usize { kernstart_virt_addr - KERNELBASE }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
