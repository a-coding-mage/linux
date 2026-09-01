#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_variables)]

use core::ffi::c_void;

/* C dependencies: assert.h, string.h, stdlib.h, errno.h, limits.h, stdio.h */

pub type dma_addr_t = c_ulong;
pub type c_ulong = u64;
pub type c_uint = u32;
pub type c_int = i32;
pub type c_long = i64;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

#[inline]
pub const fn unlikely<T>(x: T) -> T {
    x
}

macro_rules! BUG_ON {
    ($x:expr) => {
        assert!(!$x)
    };
}

macro_rules! WARN_ON {
    ($condition:expr) => {{
        let __ret_warn_on: c_int = if $condition { 1 } else { 0 };
        unlikely(__ret_warn_on)
    }};
}

macro_rules! WARN_ON_ONCE {
    ($condition:expr) => {{
        let __ret_warn_on: c_int = if $condition { 1 } else { 0 };
        if unlikely(__ret_warn_on) != 0 {
            assert!(false);
        }
        unlikely(__ret_warn_on)
    }};
}

pub const PAGE_SIZE: c_ulong = 4096;
pub const PAGE_SHIFT: c_ulong = 12;
pub const PAGE_MASK: c_ulong = !(PAGE_SIZE - 1);

macro_rules! __ALIGN_KERNEL_MASK {
    ($x:expr, $mask:expr) => {
        (($x).wrapping_add($mask)) & !($mask)
    };
}

macro_rules! __ALIGN_KERNEL {
    ($x:expr, $a:expr) => {
        __ALIGN_KERNEL_MASK!($x, ($a).wrapping_sub(1))
    };
}

macro_rules! ALIGN {
    ($x:expr, $a:expr) => {
        __ALIGN_KERNEL!($x, $a)
    };
}

macro_rules! ALIGN_DOWN {
    ($x:expr, $a:expr) => {
        __ALIGN_KERNEL!($x.wrapping_sub(($a).wrapping_sub(1)), $a)
    };
}

macro_rules! PAGE_ALIGN {
    ($addr:expr) => {
        ALIGN!($addr, PAGE_SIZE)
    };
}

macro_rules! offset_in_page {
    ($p:expr) => {
        (($p as c_ulong) & !PAGE_MASK)
    };
}

macro_rules! virt_to_page {
    ($x:expr) => {
        ($x as *mut c_void)
    };
}

macro_rules! page_address {
    ($x:expr) => {
        ($x as *mut c_void)
    };
}

#[inline]
pub unsafe fn page_to_phys(page: *mut page) -> c_ulong {
    assert!(false);

    0
}

macro_rules! page_to_pfn {
    ($page:expr) => {
        (($page as c_ulong) / PAGE_SIZE)
    };
}

macro_rules! pfn_to_page {
    ($pfn:expr) => {
        (($pfn).wrapping_mul(PAGE_SIZE) as *mut c_void)
    };
}

macro_rules! __min {
    ($t1:ty, $t2:ty, $min1:ident, $min2:ident, $x:expr, $y:expr) => {{
        let $min1: $t1 = $x;
        let $min2: $t2 = $y;
        if $min1 < $min2 {
            $min1
        } else {
            $min2
        }
    }};
}

/* Token-pasting and unique-id preprocessor helpers have no direct Rust item form. */

macro_rules! min {
    ($x:expr, $y:expr) => {{
        let min1_ = $x;
        let min2_ = $y;
        if min1_ < min2_ {
            min1_
        } else {
            min2_
        }
    }};
}

macro_rules! min_t {
    ($type:ty, $x:expr, $y:expr) => {
        __min!($type, $type, min1_, min2_, $x, $y)
    };
}

#[inline]
pub const fn pagefault_disabled() -> c_int {
    0
}

#[inline]
pub unsafe fn kmap(page: *mut page) -> *mut c_void {
    assert!(false);

    core::ptr::null_mut()
}

#[inline]
pub unsafe fn kmap_atomic(page: *mut page) -> *mut c_void {
    assert!(false);

    core::ptr::null_mut()
}

#[inline]
pub unsafe fn kunmap(addr: *mut c_void) {
    assert!(false);
}

#[inline]
pub unsafe fn kunmap_atomic(addr: *mut c_void) {
    assert!(false);
}

#[inline]
pub unsafe fn __get_free_page(flags: c_uint) -> c_ulong {
    malloc(PAGE_SIZE as usize) as c_ulong
}

#[inline]
pub unsafe fn free_page(page: c_ulong) {
    free(page as *mut c_void);
}

#[inline]
pub unsafe fn kmalloc(size: c_uint, flags: c_uint) -> *mut c_void {
    malloc(size as usize)
}

#[inline]
pub unsafe fn kmalloc_array(n: c_uint, size: c_uint, flags: c_uint) -> *mut c_void {
    malloc(n.wrapping_mul(size) as usize)
}

#[inline]
pub unsafe fn kfree(x: *mut c_void) {
    free(x);
}

macro_rules! kmemleak_alloc {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {};
}

macro_rules! kmemleak_free {
    ($a:expr) => {};
}

#[inline]
pub const fn PageSlab<T>(p: *const T) -> c_int {
    0
}

#[inline]
pub const fn flush_dcache_page<T>(p: *const T) {}

pub const MAX_ERRNO: c_ulong = 4095;

#[inline]
pub const fn IS_ERR_VALUE(x: c_ulong) -> bool {
    unlikely(x >= (0usize.wrapping_sub(MAX_ERRNO as usize)) as c_ulong)
}

#[inline]
#[must_use]
pub const unsafe fn ERR_PTR(error: c_long) -> *mut c_void {
    error as usize as *mut c_void
}

#[inline]
#[must_use]
pub const unsafe fn PTR_ERR(ptr: *const c_void) -> c_long {
    ptr as usize as c_long
}

#[inline]
#[must_use]
pub const unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    IS_ERR_VALUE(ptr as c_ulong)
}

#[inline]
#[must_use]
pub const unsafe fn PTR_ERR_OR_ZERO(ptr: *const c_void) -> c_int {
    if IS_ERR(ptr) {
        PTR_ERR(ptr) as c_int
    } else {
        0
    }
}

macro_rules! IS_ENABLED {
    ($x:expr) => {
        0
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
