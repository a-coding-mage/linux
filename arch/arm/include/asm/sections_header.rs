/* SPDX-License-Identifier: GPL-2.0 */
// Translation of <asm-generic/sections.h> is supplied by another dependency.

use core::ffi::c_void;

extern "C" {
    pub static mut _exiprom: u8;

    pub static mut __idmap_text_start: u8;
    pub static mut __idmap_text_end: u8;
    pub static mut __entry_text_start: u8;
    pub static mut __entry_text_end: u8;

    pub fn memory_contains(
        start: *const c_void,
        end: *const c_void,
        addr: *const c_void,
        size: usize,
    ) -> bool;
}

#[inline]
pub unsafe fn in_entry_text(addr: usize) -> bool {
    memory_contains(
        core::ptr::addr_of!(__entry_text_start).cast(),
        core::ptr::addr_of!(__entry_text_end).cast(),
        addr as *const c_void,
        1,
    )
}

#[inline]
pub unsafe fn in_idmap_text(addr: usize) -> bool {
    let a = addr as *const c_void;
    memory_contains(
        core::ptr::addr_of!(__idmap_text_start).cast(),
        core::ptr::addr_of!(__idmap_text_end).cast(),
        a,
        1,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
