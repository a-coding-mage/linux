/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: <asm/shmparam.h>

#[allow(non_camel_case_types)]
pub enum page {}

extern "C" {
    pub fn flush_dcache_page(page: *mut page);
    pub fn clear_page(addr: *mut core::ffi::c_void);
    pub fn copy_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void);
}

#[inline]
unsafe fn pages_do_alias(addr1: usize, addr2: usize) -> usize {
    (addr1 ^ addr2) & (SHMLBA - 1)
}

// #define clear_user_page clear_user_page
#[inline]
unsafe fn clear_user_page(
    addr: *mut core::ffi::c_void,
    vaddr: usize,
    page: *mut page,
) {
    clear_page(addr);
    if pages_do_alias(addr as usize, vaddr & PAGE_MASK) != 0 {
        flush_dcache_page(page);
    }
}

#[inline]
unsafe fn copy_user_page(
    to: *mut core::ffi::c_void,
    from: *mut core::ffi::c_void,
    vaddr: usize,
    page: *mut page,
) {
    copy_page(to, from);
    if pages_do_alias(to as usize, vaddr & PAGE_MASK) != 0 {
        flush_dcache_page(page);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
