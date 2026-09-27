/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by asm/mem_encrypt.h and asm-generic/set_memory.h

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

pub unsafe extern "C" fn can_set_direct_map() -> bool;

// C macro: #define can_set_direct_map can_set_direct_map

pub unsafe extern "C" fn set_memory_valid(addr: ::kernel::ffi::c_ulong, numpages: ::kernel::ffi::c_int, enable: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;

pub unsafe extern "C" fn set_direct_map_invalid_noflush(page: *mut page) -> ::kernel::ffi::c_int;
pub unsafe extern "C" fn set_direct_map_default_noflush(page: *mut page) -> ::kernel::ffi::c_int;
pub unsafe extern "C" fn set_direct_map_valid_noflush(page: *mut page, nr: ::kernel::ffi::c_uint, valid: bool) -> ::kernel::ffi::c_int;
pub unsafe extern "C" fn kernel_page_present(page: *mut page) -> bool;

pub unsafe extern "C" fn set_memory_encrypted(addr: ::kernel::ffi::c_ulong, numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
pub unsafe extern "C" fn set_memory_decrypted(addr: ::kernel::ffi::c_ulong, numpages: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
