/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2019 SiFive
 */

/* C header guard: _ASM_RISCV_SET_MEMORY_H */

/* C preprocessor condition: __ASSEMBLER__ */
/*
 * Functions to change memory attributes.
 */
#[cfg(CONFIG_MMU)]
extern "C" {
    pub fn set_memory_ro(addr: ::kernel::ffi::c_ulong, numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn set_memory_rw(addr: ::kernel::ffi::c_ulong, numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn set_memory_x(addr: ::kernel::ffi::c_ulong, numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn set_memory_nx(addr: ::kernel::ffi::c_ulong, numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn set_memory_rw_nx(addr: ::kernel::ffi::c_ulong, numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
}

#[cfg(CONFIG_MMU)]
#[inline(always)]
pub unsafe fn set_kernel_memory(
    startp: *mut ::kernel::ffi::c_char,
    endp: *mut ::kernel::ffi::c_char,
    set_memory: unsafe extern "C" fn(::kernel::ffi::c_ulong, ::kernel::ffi::c_int) -> ::kernel::ffi::c_int,
) -> ::kernel::ffi::c_int {
    let start = startp as ::kernel::ffi::c_ulong;
    let end = endp as ::kernel::ffi::c_ulong;
    let num_pages = (PAGE_ALIGN!(end - start)) >> PAGE_SHIFT;

    set_memory(start, num_pages as ::kernel::ffi::c_int)
}

#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_memory_ro(_addr: ::kernel::ffi::c_ulong, _numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_memory_rw(_addr: ::kernel::ffi::c_ulong, _numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_memory_x(_addr: ::kernel::ffi::c_ulong, _numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_memory_nx(_addr: ::kernel::ffi::c_ulong, _numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_memory_rw_nx(_addr: ::kernel::ffi::c_ulong, _numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_kernel_memory(
    _startp: *mut ::kernel::ffi::c_char,
    _endp: *mut ::kernel::ffi::c_char,
    _set_memory: unsafe extern "C" fn(::kernel::ffi::c_ulong, ::kernel::ffi::c_int) -> ::kernel::ffi::c_int,
) -> ::kernel::ffi::c_int { 0 }

extern "C" {
    pub fn set_direct_map_invalid_noflush(page: *mut page) -> ::kernel::ffi::c_int;
    pub fn set_direct_map_default_noflush(page: *mut page) -> ::kernel::ffi::c_int;
    pub fn set_direct_map_valid_noflush(page: *mut page, nr: ::kernel::ffi::c_ulong, valid: bool) -> ::kernel::ffi::c_int;
    pub fn kernel_page_present(page: *mut page) -> bool;
}

/* `struct page` is supplied by the surrounding kernel bindings. */
pub type page = ::kernel::ffi::c_void;

#[cfg(CONFIG_STRICT_KERNEL_RWX)]
#[cfg(CONFIG_64BIT)]
pub const SECTION_ALIGN: u32 = 1 << 21;
#[cfg(CONFIG_STRICT_KERNEL_RWX)]
#[cfg(not(CONFIG_64BIT))]
pub const SECTION_ALIGN: u32 = 1 << 22;
#[cfg(not(CONFIG_STRICT_KERNEL_RWX))]
pub const SECTION_ALIGN: usize = L1_CACHE_BYTES;

pub const PECOFF_SECTION_ALIGNMENT: u32 = 0x1000;
pub const PECOFF_FILE_ALIGNMENT: u32 = 0x200;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
