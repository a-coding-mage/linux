/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int};

/* Dependencies in the original header:
 * "strbuf.h", <subcmd/pager.h>, "../ui/ui.h",
 * <linux/compiler.h>, and <linux/string.h>.
 */

pub const CMD_EXEC_PATH: &[u8] = b"--exec-path\0";
pub const CMD_DEBUGFS_DIR: &[u8] = b"--debugfs-dir=\0";

pub const EXEC_PATH_ENVIRONMENT: &[u8] = b"PERF_EXEC_PATH\0";
pub const PERF_DEBUGFS_ENVIRONMENT: &[u8] = b"PERF_DEBUGFS_DIR\0";
pub const PERF_TRACEFS_ENVIRONMENT: &[u8] = b"PERF_TRACEFS_DIR\0";
pub const PERF_PAGER_ENVIRONMENT: &[u8] = b"PERF_PAGER\0";

unsafe extern "C" {
    pub fn split_cmdline(cmdline: *mut c_char, argv: *mut *const *const c_char) -> c_int;
}

pub const fn alloc_nr(x: usize) -> usize {
    ((x).wrapping_add(16)).wrapping_mul(3).wrapping_div(2)
}

#[inline]
pub unsafe fn is_absolute_path(path: *const c_char) -> c_int {
    unsafe { (*path == b'/' as c_char) as c_int }
}

unsafe extern "C" {
    /* Original declaration is annotated with __printf(3, 4). */
    pub unsafe fn mkpath(path_buf: *mut c_char, sz: usize, fmt: *const c_char, ...) -> *mut c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
