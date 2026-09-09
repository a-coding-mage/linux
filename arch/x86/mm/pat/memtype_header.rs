/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_char;

// Supplied by the surrounding translation unit.
extern "C" {
    pub static mut pat_debug_enable: i32;
}

#[macro_export]
macro_rules! dprintk {
    ($fmt:expr $(, $arg:expr)*) => {{
        unsafe {
            if $crate::pat_debug_enable != 0 {
                $crate::pr_info!(concat!("x86/PAT: ", $fmt) $(, $arg)*);
            }
        }
    }};
}

#[repr(C)]
pub struct memtype {
    pub start: u64,
    pub end: u64,
    pub subtree_max_end: u64,
    pub type_: enum_page_cache_mode,
    pub rb: rb_node,
}

pub type enum_page_cache_mode = i32;

pub const _PAGE_CACHE_MODE_UC: enum_page_cache_mode = 0;
pub const _PAGE_CACHE_MODE_UC_MINUS: enum_page_cache_mode = 1;
pub const _PAGE_CACHE_MODE_WB: enum_page_cache_mode = 2;
pub const _PAGE_CACHE_MODE_WC: enum_page_cache_mode = 3;
pub const _PAGE_CACHE_MODE_WT: enum_page_cache_mode = 4;
pub const _PAGE_CACHE_MODE_WP: enum_page_cache_mode = 5;

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

pub unsafe fn cattr_name(pcm: enum_page_cache_mode) -> *const c_char {
    match pcm {
        _PAGE_CACHE_MODE_UC => b"uncached\0".as_ptr() as *const c_char,
        _PAGE_CACHE_MODE_UC_MINUS => b"uncached-minus\0".as_ptr() as *const c_char,
        _PAGE_CACHE_MODE_WB => b"write-back\0".as_ptr() as *const c_char,
        _PAGE_CACHE_MODE_WC => b"write-combining\0".as_ptr() as *const c_char,
        _PAGE_CACHE_MODE_WT => b"write-through\0".as_ptr() as *const c_char,
        _PAGE_CACHE_MODE_WP => b"write-protected\0".as_ptr() as *const c_char,
        _ => b"broken\0".as_ptr() as *const c_char,
    }
}

// CONFIG_X86_PAT is a build-time condition preserved as a Rust cfg feature.
#[cfg(feature = "CONFIG_X86_PAT")]
extern "C" {
    pub fn memtype_check_insert(
        entry_new: *mut memtype,
        new_type: *mut enum_page_cache_mode,
    ) -> i32;
    pub fn memtype_erase(start: u64, end: u64) -> *mut memtype;
    pub fn memtype_lookup(addr: u64) -> *mut memtype;
    pub fn memtype_copy_nth_element(entry_out: *mut memtype, pos: i64) -> i32;
}

#[cfg(not(feature = "CONFIG_X86_PAT"))]
pub unsafe fn memtype_check_insert(
    _entry_new: *mut memtype,
    _new_type: *mut enum_page_cache_mode,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_X86_PAT"))]
pub unsafe fn memtype_erase(_start: u64, _end: u64) -> *mut memtype {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_X86_PAT"))]
pub unsafe fn memtype_lookup(_addr: u64) -> *mut memtype {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_X86_PAT"))]
pub unsafe fn memtype_copy_nth_element(_out: *mut memtype, _pos: i64) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
