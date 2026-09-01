/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::c_char;

// C++ callers used extern "C" linkage for this declaration.
unsafe extern "C" {
    pub fn cxx_demangle_sym(str: *const c_char, params: bool, modifiers: bool) -> *mut c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
