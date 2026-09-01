/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::c_char;

pub const PATH_TO_CPU: &[u8; 25] = b"/sys/devices/system/cpu/\0";

/* C header condition: define MAX_LINE_LEN only when not already defined. */
pub const MAX_LINE_LEN: usize = 4096;

pub const SYSFS_PATH_MAX: usize = 255;

unsafe extern "C" {
    pub fn is_valid_path(path: *const c_char) -> i32;
    pub fn cpupower_read_sysfs(path: *const c_char, buf: *mut c_char, buflen: usize) -> u32;
    pub fn cpupower_write_sysfs(path: *const c_char, buf: *mut c_char, buflen: usize) -> u32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
