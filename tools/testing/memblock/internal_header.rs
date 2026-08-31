/* SPDX-License-Identifier: GPL-2.0-or-later */

use core::ffi::{c_char, c_int, c_void};

/*
 * Enable memblock_dbg() messages
 */
#[cfg(MEMBLOCK_DEBUG)]
pub static mut memblock_debug: c_int = 1;

unsafe extern "C" {
    pub fn printf(fmt: *const c_char, ...) -> c_int;
    pub fn BUG() -> !;
    pub static PAGE_SHIFT: u32;
}

macro_rules! pr_warn_ratelimited {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        unsafe { printf($fmt, $($arg),*) }
    };
}

#[allow(unused_imports)]
pub(crate) use pr_warn_ratelimited;

#[allow(non_snake_case)]
pub unsafe fn K(x: usize) -> usize {
    x << (unsafe { PAGE_SHIFT } - 10)
}

pub static mut mirrored_kernelcore: bool = false;

#[repr(C)]
pub struct page {}

pub unsafe fn page_address(page: *mut page) -> *mut c_void {
    unsafe {
        BUG();
    }
    page as *mut c_void
}

pub unsafe fn virt_to_page(virt: *mut c_void) -> *mut page {
    unsafe {
        BUG();
    }
    virt as *mut page
}

macro_rules! for_each_valid_pfn {
    ($pfn:ident, $start_pfn:expr, $end_pfn:expr, $body:block) => {{
        $pfn = $start_pfn;
        while $pfn < $end_pfn {
            $body
            $pfn += 1;
        }
    }};
}

#[allow(unused_imports)]
pub(crate) use for_each_valid_pfn;

pub unsafe fn kasan_reset_tag(addr: *const c_void) -> *mut c_void {
    addr as *mut c_void
}

pub unsafe fn __is_kernel(_addr: c_ulong) -> bool {
    false
}

pub type c_ulong = core::ffi::c_ulong;

/* Duplicate C definition of for_each_valid_pfn preserved by the single Rust macro above. */

macro_rules! __SetPageReserved {
    ($p:expr) => {{
        let _ = $p;
    }};
}

#[allow(unused_imports)]
pub(crate) use __SetPageReserved;

pub type phys_addr_t = u64;
pub type size_t = usize;

pub unsafe fn kho_scratch_overlap(_phys: phys_addr_t, _size: size_t) -> bool {
    false
}
