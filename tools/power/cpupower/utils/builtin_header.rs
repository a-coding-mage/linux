/* SPDX-License-Identifier: GPL-2.0 */

unsafe extern "C" {
    pub fn cmd_set(argc: ::std::os::raw::c_int, argv: *const *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn cmd_info(argc: ::std::os::raw::c_int, argv: *const *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn cmd_freq_set(argc: ::std::os::raw::c_int, argv: *const *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn cmd_freq_info(argc: ::std::os::raw::c_int, argv: *const *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn cmd_idle_set(argc: ::std::os::raw::c_int, argv: *const *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn cmd_idle_info(argc: ::std::os::raw::c_int, argv: *const *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn cmd_cap_info(argc: ::std::os::raw::c_int, argv: *const *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn cmd_cap_set(argc: ::std::os::raw::c_int, argv: *const *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn cmd_monitor(argc: ::std::os::raw::c_int, argv: *const *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
