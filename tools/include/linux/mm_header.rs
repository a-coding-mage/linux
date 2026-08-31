/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies removed from executable Rust:
// #include <linux/align.h>
// #include <linux/mmzone.h>
// #include <linux/sizes.h>
// `phys_addr_t` is provided by the translated dependency set.

use core::ffi::{c_ulong, c_void};

pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1usize << PAGE_SHIFT;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

pub const PHYS_ADDR_MAX: phys_addr_t = !0 as phys_addr_t;

#[inline]
pub const fn PAGE_ALIGN(addr: usize) -> usize {
    (addr + PAGE_SIZE - 1) & PAGE_MASK
}

#[inline]
pub const fn PAGE_ALIGN_DOWN(addr: usize) -> usize {
    addr & PAGE_MASK
}

#[inline]
pub unsafe fn __va(x: c_ulong) -> *mut c_void {
    x as *mut c_void
}

#[inline]
pub unsafe fn __pa(x: *const c_void) -> c_ulong {
    x as c_ulong
}

#[inline]
pub unsafe fn __pa_symbol(x: *const c_void) -> c_ulong {
    x as c_ulong
}

#[inline]
pub unsafe fn pfn_to_page(pfn: c_ulong) -> *mut c_void {
    (pfn.wrapping_mul(PAGE_SIZE as c_ulong)) as *mut c_void
}

#[inline]
pub unsafe fn phys_to_virt(address: c_ulong) -> *mut c_void {
    __va(address)
}

#[inline]
pub unsafe fn virt_to_phys(address: *volatile c_void) -> phys_addr_t {
    address as phys_addr_t
}

#[inline]
pub fn totalram_pages_inc() {}

#[inline]
pub fn totalram_pages_add(_count: isize) {}

#[inline]
pub fn early_pfn_to_nid(_pfn: c_ulong) -> i32 {
    0
}
