// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn ikm_list_monitors(container: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn ikm_run_monitor(
        monitor: *mut ::std::os::raw::c_char,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
