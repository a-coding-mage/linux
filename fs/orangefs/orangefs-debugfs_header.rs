/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

unsafe extern "C" {
    pub fn orangefs_debugfs_init(arg: i32);
    pub fn orangefs_debugfs_cleanup();
    pub fn orangefs_prepare_debugfs_help_string(arg: i32) -> i32;
    pub fn orangefs_debugfs_new_client_mask(arg: *mut c_void) -> i32;
    pub fn orangefs_debugfs_new_client_string(arg: *mut c_void) -> i32;
    pub fn orangefs_debugfs_new_debug(arg: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
