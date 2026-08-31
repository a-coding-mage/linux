/* SPDX-License-Identifier: GPL-2.0 */

// Header guard omitted in Rust: __PERF_LEVENSHTEIN_H.

unsafe extern "C" {
    pub fn levenshtein(
        string1: *const ::std::os::raw::c_char,
        string2: *const ::std::os::raw::c_char,
        swap_penalty: ::std::os::raw::c_int,
        substition_penalty: ::std::os::raw::c_int,
        insertion_penalty: ::std::os::raw::c_int,
        deletion_penalty: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
