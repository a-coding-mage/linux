/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    pub fn test_cpumap(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn test_threadmap(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn test_evlist(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn test_evsel(argc: c_int, argv: *mut *mut c_char) -> c_int;
}
