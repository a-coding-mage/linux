/* SPDX-License-Identifier: GPL-2.0 */

// Forward declaration from the C header:
// struct perf_env;
#[repr(C)]
pub struct perf_env {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn perf_env__lookup_objdump(env: *mut perf_env, path: *mut *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn perf_env__single_address_space(env: *mut perf_env) -> bool;
}
