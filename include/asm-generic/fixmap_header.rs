/*
 * fixmap.h: compile-time virtual memory allocation
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998 Ingo Molnar
 *
 * Support of BIGMEM added by Gerhard Wichert, Siemens AG, July 1999
 * x86_32 and x86_64 integration by Gustavo F. Padovan, February 2009
 * Break out common bits to asm-generic by Mark Salter, November 2013
 */

// Dependencies supplied by other translated headers:
// FIXADDR_TOP, PAGE_SHIFT, PAGE_MASK, PAGE_SIZE, FIXADDR_START,
// __end_of_fixed_addresses, BUILD_BUG_ON, BUG_ON, __set_fixmap, and the
// page-protection constants/macros used below.

#[inline(always)]
pub const fn __fix_to_virt(x: usize) -> usize {
    FIXADDR_TOP - (x << PAGE_SHIFT)
}

#[inline(always)]
pub const fn __virt_to_fix(x: usize) -> usize {
    (FIXADDR_TOP - (x & PAGE_MASK)) >> PAGE_SHIFT
}

/*
 * 'index to address' translation. If anyone tries to use the idx
 * directly without translation, we catch the bug with a NULL-deference
 * kernel oops. Illegal ranges of incoming indices are caught too.
 */
#[inline(always)]
pub unsafe fn fix_to_virt(idx: u32) -> usize {
    BUILD_BUG_ON!(idx >= __end_of_fixed_addresses);
    __fix_to_virt(idx as usize)
}

#[inline]
pub unsafe fn virt_to_fix(vaddr: usize) -> usize {
    BUG_ON!(vaddr >= FIXADDR_TOP || vaddr < FIXADDR_START);
    __virt_to_fix(vaddr)
}

/*
 * Provide some reasonable defaults for page flags.
 * Not all architectures use all of these different types and some
 * architectures use different names.
 */

#[macro_export]
macro_rules! set_fixmap {
    ($idx:expr, $phys:expr) => {
        __set_fixmap!($idx, $phys, FIXMAP_PAGE_NORMAL)
    };
}

#[macro_export]
macro_rules! clear_fixmap {
    ($idx:expr) => {
        __set_fixmap!($idx, 0, FIXMAP_PAGE_CLEAR)
    };
}

/* Return a pointer with offset calculated */
#[macro_export]
macro_rules! __set_fixmap_offset {
    ($idx:expr, $phys:expr, $flags:expr) => {{
        __set_fixmap!($idx, $phys, $flags);
        fix_to_virt($idx) + (($phys) & (PAGE_SIZE - 1))
    }};
}

#[macro_export]
macro_rules! set_fixmap_offset {
    ($idx:expr, $phys:expr) => {
        __set_fixmap_offset!($idx, $phys, FIXMAP_PAGE_NORMAL)
    };
}

/*
 * Some hardware wants to get fixmapped without caching.
 */
#[macro_export]
macro_rules! set_fixmap_nocache {
    ($idx:expr, $phys:expr) => {
        __set_fixmap!($idx, $phys, FIXMAP_PAGE_NOCACHE)
    };
}

#[macro_export]
macro_rules! set_fixmap_offset_nocache {
    ($idx:expr, $phys:expr) => {
        __set_fixmap_offset!($idx, $phys, FIXMAP_PAGE_NOCACHE)
    };
}

/*
 * Some fixmaps are for IO
 */
#[macro_export]
macro_rules! set_fixmap_io {
    ($idx:expr, $phys:expr) => {
        __set_fixmap!($idx, $phys, FIXMAP_PAGE_IO)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
