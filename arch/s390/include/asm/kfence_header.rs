/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the s390 KFENCE architecture header.

use core::ffi::c_void;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn __kernel_map_pages(page: *mut page, numpages: i32, enable: i32);
    pub fn set_memory_4k(addr: usize, numpages: usize);
    pub fn virt_to_page(addr: *mut c_void) -> *mut page;

    // Supplied by the KFENCE implementation.
    pub static mut __kfence_pool: u8;
}

// `CONFIG_KFENCE` is a build-time configuration condition supplied externally.
#[cfg(feature = "CONFIG_KFENCE")]
#[inline(always)]
pub unsafe fn arch_kfence_init_pool() -> bool {
    let pool_pages: usize = (KFENCE_POOL_SIZE >> PAGE_SHIFT) as usize;

    set_memory_4k((&raw mut __kfence_pool) as usize, pool_pages);
    true
}

#[cfg(not(feature = "CONFIG_KFENCE"))]
#[inline(always)]
pub unsafe fn arch_kfence_init_pool() -> bool {
    true
}

#[inline(always)]
pub const fn arch_kfence_test_address(addr: usize) -> usize {
    addr & PAGE_MASK
}

#[inline]
pub unsafe fn kfence_protect_page(addr: usize, protect: bool) -> bool {
    __kernel_map_pages(virt_to_page(addr as *mut c_void), 1, (!protect) as i32);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
