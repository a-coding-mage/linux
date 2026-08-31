/* SPDX-License-Identifier: GPL-2.0 */

/*
 * demangle function flags
 */
pub const JAVA_DEMANGLE_NORET: i32 = 0x1; /* do not process return type */

unsafe extern "C" {
    pub fn java_demangle_sym(str: *const ::std::os::raw::c_char, flags: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
