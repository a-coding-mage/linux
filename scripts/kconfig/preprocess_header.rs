/* SPDX-License-Identifier: GPL-2.0-only */

// C header guard: PREPROCESS_H

#[repr(C)]
pub enum variable_flavor {
    VAR_SIMPLE,
    VAR_RECURSIVE,
    VAR_APPEND,
}

#[repr(C)]
pub struct gstr {
    _private: [u8; 0],
}

extern "C" {
    pub fn env_write_dep(gs: *mut gstr);
    pub fn variable_add(
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        flavor: variable_flavor,
    );
    pub fn variable_all_del();
    pub fn expand_dollar(str_: *mut *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn expand_one_token(str_: *mut *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
