/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/* Values for nocacheflag and cmode */
pub const IOMAP_FULL_CACHING: i32 = 0;
pub const IOMAP_NOCACHE_SER: i32 = 1;
pub const IOMAP_NOCACHE_NONSER: i32 = 2;
pub const IOMAP_WRITETHROUGH: i32 = 3;

/*
 * These functions exported by arch/m68k/mm/kmap.c.
 * Only needed on MMU enabled systems.
 *
 * The declarations and functions below are present when CONFIG_MMU is set
 * in the C source.
 */
#[cfg(feature = "CONFIG_MMU")]
extern "C" {
    pub fn __ioremap(physaddr: usize, size: usize, cacheflag: i32) -> *mut c_void;
    pub fn iounmap(addr: *mut c_void);
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn ioremap(physaddr: usize, size: usize) -> *mut c_void {
    __ioremap(physaddr, size, IOMAP_NOCACHE_SER)
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn ioremap_wt(physaddr: usize, size: usize) -> *mut c_void {
    __ioremap(physaddr, size, IOMAP_WRITETHROUGH)
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn memset_io(addr: *mut c_void, val: u8, count: i32) {
    core::ptr::write_bytes(addr as *mut u8, val, count as usize);
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn memcpy_fromio(dst: *mut c_void, src: *const c_void, count: i32) {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, count as usize);
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn memcpy_toio(dst: *mut c_void, src: *const c_void, count: i32) {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, count as usize);
}

#[inline]
pub unsafe fn ioport_map(port: usize, _nr: u32) -> *mut c_void {
    port as *mut c_void
}

#[inline]
pub unsafe fn ioport_unmap(_p: *mut c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
