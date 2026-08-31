/* SPDX-License-Identifier: GPL-2.0 */

// C header dependency intent: declarations originally used `struct common_params`
// and C `bool`; the concrete struct definition is supplied by another file.
#[repr(C)]
pub struct common_params {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn osnoise_top_parse_args(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> *mut common_params;
    pub fn osnoise_hist_parse_args(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> *mut common_params;
    pub fn timerlat_top_parse_args(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> *mut common_params;
    pub fn timerlat_hist_parse_args(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> *mut common_params;

    pub static mut in_unit_test: bool;
}
