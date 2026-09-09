/* SPDX-License-Identifier: GPL-2.0 */

// Translated from asm/page_32.h.
// Dependency: asm/page_32_types.h

// The original declarations are only present when __ASSEMBLER__ is not defined.

#[inline]
pub unsafe fn __phys_addr_nodebug(x: usize) -> usize {
    x.wrapping_sub(PAGE_OFFSET)
}

// CONFIG_DEBUG_VIRTUAL selects the external implementation.  Otherwise the
// non-debug implementation above is used.
#[cfg(CONFIG_DEBUG_VIRTUAL)]
extern "C" {
    pub fn __phys_addr(x: usize) -> usize;
}

#[cfg(not(CONFIG_DEBUG_VIRTUAL))]
#[inline]
pub unsafe fn __phys_addr(x: usize) -> usize {
    __phys_addr_nodebug(x)
}

#[inline]
pub unsafe fn __phys_addr_symbol(x: usize) -> usize {
    __phys_addr(x)
}

// Equivalent of RELOC_HIDE((x), 0); relocation hiding is supplied by the
// surrounding platform/runtime.
#[inline]
pub unsafe fn __phys_reloc_hide(x: usize) -> usize {
    x
}

extern "C" {
    pub fn memset(dest: *mut core::ffi::c_void, value: i32, count: usize)
        -> *mut core::ffi::c_void;
    pub fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        count: usize,
    ) -> *mut core::ffi::c_void;
}

/// clear_page() - clear a page using a kernel virtual address.
/// @page: address of kernel page
///
/// Does absolutely no exception handling.
#[inline]
pub unsafe fn clear_page(page: *mut core::ffi::c_void) {
    memset(page, 0, PAGE_SIZE);
}

#[inline]
pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void) {
    memcpy(to, from as *const core::ffi::c_void, PAGE_SIZE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
