/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Only include this header file from your architecture's <asm/fb.h>.
 *
 * The C header includes linux/io.h, linux/mm_types.h, linux/pgtable.h, and
 * linux/types.h; their declarations are supplied by the surrounding build.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* The following external types and functions are supplied by the kernel. */
extern "C" {
    pub fn pgprot_writecombine(prot: pgprot_t) -> pgprot_t;
    pub fn __raw_readb(addr: *const c_void) -> u8;
    pub fn __raw_readw(addr: *const c_void) -> u16;
    pub fn __raw_readl(addr: *const c_void) -> u32;
    pub fn __raw_readq(addr: *const c_void) -> u64;
    pub fn __raw_writeb(value: u8, addr: *mut c_void);
    pub fn __raw_writew(value: u16, addr: *mut c_void);
    pub fn __raw_writel(value: u32, addr: *mut c_void);
    pub fn __raw_writeq(value: u64, addr: *mut c_void);
    pub fn memcpy_fromio(to: *mut c_void, from: *const c_void, n: usize);
    pub fn memcpy_toio(to: *mut c_void, from: *const c_void, n: usize);
    pub fn memset_io(addr: *mut c_void, c: i32, n: usize);
}

/* Build-time C preprocessor conditions are represented by the unconditional
 * declarations below; platform-specific implementations may override them.
 */

#[inline]
pub unsafe fn pgprot_framebuffer(
    prot: pgprot_t,
    _vm_start: usize,
    _vm_end: usize,
    _offset: usize,
) -> pgprot_t {
    pgprot_writecombine(prot)
}

#[inline]
pub unsafe fn video_is_primary_device(_dev: *mut device) -> bool {
    false
}

/*
 * I/O helpers for the framebuffer. Prefer these functions over their regular
 * counterparts. The helpers read and write raw framebuffer data.
 */

#[inline]
pub unsafe fn fb_readb(addr: *const c_void) -> u8 {
    __raw_readb(addr)
}

#[inline]
pub unsafe fn fb_readw(addr: *const c_void) -> u16 {
    __raw_readw(addr)
}

#[inline]
pub unsafe fn fb_readl(addr: *const c_void) -> u32 {
    __raw_readl(addr)
}

#[inline]
pub unsafe fn fb_readq(addr: *const c_void) -> u64 {
    __raw_readq(addr)
}

#[inline]
pub unsafe fn fb_writeb(b: u8, addr: *mut c_void) {
    __raw_writeb(b, addr);
}

#[inline]
pub unsafe fn fb_writew(b: u16, addr: *mut c_void) {
    __raw_writew(b, addr);
}

#[inline]
pub unsafe fn fb_writel(b: u32, addr: *mut c_void) {
    __raw_writel(b, addr);
}

#[inline]
pub unsafe fn fb_writeq(b: u64, addr: *mut c_void) {
    __raw_writeq(b, addr);
}

#[inline]
pub unsafe fn fb_memcpy_fromio(to: *mut c_void, from: *const c_void, n: usize) {
    memcpy_fromio(to, from, n);
}

#[inline]
pub unsafe fn fb_memcpy_toio(to: *mut c_void, from: *const c_void, n: usize) {
    memcpy_toio(to, from, n);
}

#[inline]
pub unsafe fn fb_memset_io(addr: *mut c_void, c: i32, n: usize) {
    memset_io(addr, c, n);
}

/* In C, fb_memset is an alias for fb_memset_io. */
pub use fb_memset_io as fb_memset;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
