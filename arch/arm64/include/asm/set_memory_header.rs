/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by asm/mem_encrypt.h and asm-generic/set_memory.h

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

pub unsafe extern "C" fn can_set_direct_map() -> bool;

// C macro: #define can_set_direct_map can_set_direct_map

pub unsafe extern "C" fn set_memory_valid(addr: ::core::ffi::c_ulong, numpages: ::core::ffi::c_int, enable: ::core::ffi::c_int) -> ::core::ffi::c_int;

pub unsafe extern "C" fn set_direct_map_invalid_noflush(page: *mut page) -> ::core::ffi::c_int;
pub unsafe extern "C" fn set_direct_map_default_noflush(page: *mut page) -> ::core::ffi::c_int;
pub unsafe extern "C" fn set_direct_map_valid_noflush(page: *mut page, nr: ::core::ffi::c_uint, valid: bool) -> ::core::ffi::c_int;
pub unsafe extern "C" fn kernel_page_present(page: *mut page) -> bool;

pub unsafe extern "C" fn set_memory_encrypted(addr: ::core::ffi::c_ulong, numpages: ::core::ffi::c_int) -> ::core::ffi::c_int;
pub unsafe extern "C" fn set_memory_decrypted(addr: ::core::ffi::c_ulong, numpages: ::core::ffi::c_int) -> ::core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
