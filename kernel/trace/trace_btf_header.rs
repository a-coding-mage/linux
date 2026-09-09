/* SPDX-License-Identifier: GPL-2.0 */
// Dependency corresponding to <linux/btf.h>.

use core::ffi::c_char;

// Opaque C types supplied by the Linux BTF definitions.
#[repr(C)]
pub struct btf_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_param {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_member {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn btf_find_func_proto(
        func_name: *const c_char,
        btf_p: *mut *mut btf,
    ) -> *const btf_type;

    pub fn btf_get_func_param(
        func_proto: *const btf_type,
        nr: *mut i32,
    ) -> *const btf_param;

    pub fn btf_find_struct_member(
        btf: *mut btf,
        type_: *const btf_type,
        member_name: *const c_char,
        anon_offset: *mut u32,
    ) -> *const btf_member;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
