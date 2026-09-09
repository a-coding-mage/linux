/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding compiler, init, I/O, and setup
// interfaces are intentionally left external to this header translation.

use core::ffi::c_void;

extern "C" {
    fn extend_brk(len: u32, align: usize) -> *mut c_void;
    fn early_memremap(offset: usize, size: usize) -> *mut c_void;
    fn early_memunmap(addr: *mut c_void, size: usize);
    fn memremap(offset: usize, size: usize, flags: u64) -> *mut c_void;
    fn memunmap(addr: *mut c_void);
    static MEMREMAP_WB: u64;
}

/// Equivalent of the C `static __always_inline __init` helper.
#[inline(always)]
pub unsafe fn dmi_alloc(len: u32) -> *mut c_void {
    extend_brk(len, core::mem::size_of::<i32>())
}

/* Use early IO mappings for DMI because it's initialized early */
#[inline(always)]
pub unsafe fn dmi_early_remap(offset: usize, size: usize) -> *mut c_void {
    early_memremap(offset, size)
}

#[inline(always)]
pub unsafe fn dmi_early_unmap(addr: *mut c_void, size: usize) {
    early_memunmap(addr, size)
}

#[inline(always)]
pub unsafe fn dmi_remap(offset: usize, size: usize) -> *mut c_void {
    memremap(offset, size, MEMREMAP_WB)
}

#[inline(always)]
pub unsafe fn dmi_unmap(addr: *mut c_void) {
    memunmap(addr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
