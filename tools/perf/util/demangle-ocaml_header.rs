/* SPDX-License-Identifier: GPL-2.0 */

unsafe extern "C" {
    pub fn ocaml_demangle_sym(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
