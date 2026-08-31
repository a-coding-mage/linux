/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    pub fn exec_cmd_init(
        exec_name: *const c_char,
        prefix: *const c_char,
        exec_path: *const c_char,
        exec_path_env: *const c_char,
    );

    pub fn set_argv_exec_path(exec_path: *const c_char);
    pub fn extract_argv0_path(path: *const c_char) -> *const c_char;
    pub fn setup_path();
    pub fn execv_cmd(argv: *const *const c_char) -> c_int; /* NULL terminated */
    pub fn execl_cmd(cmd: *const c_char, ...) -> c_int;
    /* get_argv_exec_path and system_path return malloc'd string, caller must free it */
    pub fn get_argv_exec_path() -> *mut c_char;
    pub fn system_path(path: *const c_char) -> *mut c_char;
}
