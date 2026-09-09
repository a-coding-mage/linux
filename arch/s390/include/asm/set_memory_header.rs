/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation: linux/mutex.h.

extern "C" {
    pub static mut cpa_mutex: mutex;
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

pub const _SET_MEMORY_RO_BIT: u32 = 0;
pub const _SET_MEMORY_RW_BIT: u32 = 1;
pub const _SET_MEMORY_NX_BIT: u32 = 2;
pub const _SET_MEMORY_X_BIT: u32 = 3;
pub const _SET_MEMORY_4K_BIT: u32 = 4;
pub const _SET_MEMORY_INV_BIT: u32 = 5;
pub const _SET_MEMORY_DEF_BIT: u32 = 6;

pub const SET_MEMORY_RO: usize = 1usize << _SET_MEMORY_RO_BIT;
pub const SET_MEMORY_RW: usize = 1usize << _SET_MEMORY_RW_BIT;
pub const SET_MEMORY_NX: usize = 1usize << _SET_MEMORY_NX_BIT;
pub const SET_MEMORY_X: usize = 1usize << _SET_MEMORY_X_BIT;
pub const SET_MEMORY_4K: usize = 1usize << _SET_MEMORY_4K_BIT;
pub const SET_MEMORY_INV: usize = 1usize << _SET_MEMORY_INV_BIT;
pub const SET_MEMORY_DEF: usize = 1usize << _SET_MEMORY_DEF_BIT;

extern "C" {
    pub fn __set_memory(addr: usize, numpages: usize, flags: usize) -> i32;
}

// Preserve the C self-referential macro alias.
pub const set_memory_rox: () = ();

/*
 * Generate two variants of each set_memory() function:
 *
 * set_memory_yy(unsigned long addr, int numpages);
 * __set_memory_yy(void *start, void *end);
 *
 * The second variant exists for both convenience to avoid the usual
 * (unsigned long) casts, but unlike the first variant it can also be used
 * for areas larger than 8TB, which may happen at memory initialization.
 */

// PAGE_SHIFT is supplied by the surrounding kernel translation.
extern "C" {
    pub static PAGE_SHIFT: u32;
}

#[inline]
pub unsafe fn set_memory_ro(addr: usize, numpages: i32) -> i32 {
    __set_memory(addr, numpages as usize, SET_MEMORY_RO)
}

#[inline]
pub unsafe fn __set_memory_ro(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> i32 {
    let numpages = (end as usize).wrapping_sub(start as usize) >> PAGE_SHIFT;
    __set_memory(start as usize, numpages, SET_MEMORY_RO)
}

#[inline]
pub unsafe fn set_memory_rw(addr: usize, numpages: i32) -> i32 {
    __set_memory(addr, numpages as usize, SET_MEMORY_RW)
}

#[inline]
pub unsafe fn __set_memory_rw(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> i32 {
    let numpages = (end as usize).wrapping_sub(start as usize) >> PAGE_SHIFT;
    __set_memory(start as usize, numpages, SET_MEMORY_RW)
}

#[inline]
pub unsafe fn set_memory_nx(addr: usize, numpages: i32) -> i32 {
    __set_memory(addr, numpages as usize, SET_MEMORY_NX)
}

#[inline]
pub unsafe fn __set_memory_nx(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> i32 {
    let numpages = (end as usize).wrapping_sub(start as usize) >> PAGE_SHIFT;
    __set_memory(start as usize, numpages, SET_MEMORY_NX)
}

#[inline]
pub unsafe fn set_memory_x(addr: usize, numpages: i32) -> i32 {
    __set_memory(addr, numpages as usize, SET_MEMORY_X)
}

#[inline]
pub unsafe fn __set_memory_x(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> i32 {
    let numpages = (end as usize).wrapping_sub(start as usize) >> PAGE_SHIFT;
    __set_memory(start as usize, numpages, SET_MEMORY_X)
}

#[inline]
pub unsafe fn set_memory_rox(addr: usize, numpages: i32) -> i32 {
    __set_memory(addr, numpages as usize, SET_MEMORY_RO | SET_MEMORY_X)
}

#[inline]
pub unsafe fn __set_memory_rox(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> i32 {
    let numpages = (end as usize).wrapping_sub(start as usize) >> PAGE_SHIFT;
    __set_memory(start as usize, numpages, SET_MEMORY_RO | SET_MEMORY_X)
}

#[inline]
pub unsafe fn set_memory_rwnx(addr: usize, numpages: i32) -> i32 {
    __set_memory(addr, numpages as usize, SET_MEMORY_RW | SET_MEMORY_NX)
}

#[inline]
pub unsafe fn __set_memory_rwnx(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> i32 {
    let numpages = (end as usize).wrapping_sub(start as usize) >> PAGE_SHIFT;
    __set_memory(start as usize, numpages, SET_MEMORY_RW | SET_MEMORY_NX)
}

#[inline]
pub unsafe fn set_memory_4k(addr: usize, numpages: i32) -> i32 {
    __set_memory(addr, numpages as usize, SET_MEMORY_4K)
}

#[inline]
pub unsafe fn __set_memory_4k(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> i32 {
    let numpages = (end as usize).wrapping_sub(start as usize) >> PAGE_SHIFT;
    __set_memory(start as usize, numpages, SET_MEMORY_4K)
}

// Dependency supplied by the surrounding kernel translation: struct page and bool.
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

extern "C" {
    pub fn set_direct_map_invalid_noflush(page: *mut page) -> i32;
    pub fn set_direct_map_default_noflush(page: *mut page) -> i32;
    pub fn set_direct_map_valid_noflush(page: *mut page, nr: u32, valid: bool) -> i32;
    pub fn kernel_page_present(page: *mut page) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
