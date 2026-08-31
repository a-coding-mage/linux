/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/libdw.h. */
/* C dependency intent: #include <linux/types.h> */

use std::os::raw::{c_char, c_uint};

pub type u64 = u64;

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inline_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

/* HAVE_LIBDW_SUPPORT:
 *
 * libdw__addr2line - Convert address to source location using libdw
 * @addr: Address to resolve
 * @file: Pointer to return filename (caller must free)
 * @line_nr: Pointer to return line number
 * @dso: The dso struct
 * @unwind_inlines: Whether to unwind inline function calls
 * @node: Inline node list to append to
 * @sym: The symbol associated with the address
 *
 * This function initializes a Dwfl context for the DSO if not already present,
 * finds the source line information for the given address, and optionally
 * resolves inline function call chains.
 *
 * Returns 1 on success (found), 0 on failure (not found).
 */
#[cfg(HAVE_LIBDW_SUPPORT)]
unsafe extern "C" {
    pub fn libdw__addr2line(
        addr: u64,
        file: *mut *mut c_char,
        line_nr: *mut c_uint,
        dso: *mut dso,
        unwind_inlines: bool,
        node: *mut inline_node,
        sym: *mut symbol,
    ) -> i32;

    /*
     * dso__free_libdw - Free libdw resources associated with the DSO
     * @dso: The dso to free resources for
     *
     * This function cleans up the Dwfl context used for addr2line lookups.
     */
    pub fn dso__free_libdw(dso: *mut dso);
}

/* !HAVE_LIBDW_SUPPORT */
#[cfg(not(HAVE_LIBDW_SUPPORT))]
#[inline]
pub unsafe fn libdw__addr2line(
    addr: u64,
    file: *mut *mut c_char,
    line_nr: *mut c_uint,
    dso: *mut dso,
    unwind_inlines: bool,
    node: *mut inline_node,
    sym: *mut symbol,
) -> i32 {
    let _ = addr;
    let _ = file;
    let _ = line_nr;
    let _ = dso;
    let _ = unwind_inlines;
    let _ = node;
    let _ = sym;
    0
}

#[cfg(not(HAVE_LIBDW_SUPPORT))]
#[inline]
pub unsafe fn dso__free_libdw(dso: *mut dso) {
    let _ = dso;
}
