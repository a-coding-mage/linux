/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2019 SiFive
 */

/* C header guard: _ASM_RISCV_SET_MEMORY_H */

/* C preprocessor condition: __ASSEMBLER__ */
/*
 * Functions to change memory attributes.
 */
#[cfg(feature = "CONFIG_MMU")]
extern "C" {
    pub fn set_memory_ro(addr: ::core::ffi::c_ulong, numpages: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn set_memory_rw(addr: ::core::ffi::c_ulong, numpages: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn set_memory_x(addr: ::core::ffi::c_ulong, numpages: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn set_memory_nx(addr: ::core::ffi::c_ulong, numpages: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn set_memory_rw_nx(addr: ::core::ffi::c_ulong, numpages: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_MMU")]
#[inline(always)]
pub unsafe fn set_kernel_memory(
    startp: *mut ::core::ffi::c_char,
    endp: *mut ::core::ffi::c_char,
    set_memory: unsafe extern "C" fn(::core::ffi::c_ulong, ::core::ffi::c_int) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let start = startp as ::core::ffi::c_ulong;
    let end = endp as ::core::ffi::c_ulong;
    let num_pages = (PAGE_ALIGN!(end - start)) >> PAGE_SHIFT;

    set_memory(start, num_pages as ::core::ffi::c_int)
}

#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub unsafe fn set_memory_ro(_addr: ::core::ffi::c_ulong, _numpages: ::core::ffi::c_int) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub unsafe fn set_memory_rw(_addr: ::core::ffi::c_ulong, _numpages: ::core::ffi::c_int) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub unsafe fn set_memory_x(_addr: ::core::ffi::c_ulong, _numpages: ::core::ffi::c_int) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub unsafe fn set_memory_nx(_addr: ::core::ffi::c_ulong, _numpages: ::core::ffi::c_int) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub unsafe fn set_memory_rw_nx(_addr: ::core::ffi::c_ulong, _numpages: ::core::ffi::c_int) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub unsafe fn set_kernel_memory(
    _startp: *mut ::core::ffi::c_char,
    _endp: *mut ::core::ffi::c_char,
    _set_memory: unsafe extern "C" fn(::core::ffi::c_ulong, ::core::ffi::c_int) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int { 0 }

extern "C" {
    pub fn set_direct_map_invalid_noflush(page: *mut page) -> ::core::ffi::c_int;
    pub fn set_direct_map_default_noflush(page: *mut page) -> ::core::ffi::c_int;
    pub fn set_direct_map_valid_noflush(page: *mut page, nr: ::core::ffi::c_ulong, valid: bool) -> ::core::ffi::c_int;
    pub fn kernel_page_present(page: *mut page) -> bool;
}

/* `struct page` is supplied by the surrounding kernel bindings. */
pub type page = ::core::ffi::c_void;

#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
#[cfg(feature = "CONFIG_64BIT")]
pub const SECTION_ALIGN: u32 = 1 << 21;
#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
#[cfg(not(feature = "CONFIG_64BIT"))]
pub const SECTION_ALIGN: u32 = 1 << 22;
#[cfg(not(feature = "CONFIG_STRICT_KERNEL_RWX"))]
pub const SECTION_ALIGN: usize = L1_CACHE_BYTES;

pub const PECOFF_SECTION_ALIGNMENT: u32 = 0x1000;
pub const PECOFF_FILE_ALIGNMENT: u32 = 0x200;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
