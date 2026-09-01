/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int};

// C dependencies: <stdbool.h>, <unistd.h>
// On most systems <limits.h> would have given us this, but not on some systems
// (e.g. GNU/Hurd).
pub const PATH_MAX: usize = 4096;

/*
 * The xxxx__mountpoint() entry points find the first match mount point for each
 * filesystems listed below, where xxxx is the filesystem type.
 *
 * The interface is as follows:
 *
 * - If a mount point is found on first call, it is cached and used for all
 *   subsequent calls.
 *
 * - If a mount point is not found, NULL is returned on first call and all
 *   subsequent calls.
 */
unsafe extern "C" {
    pub fn sysfs__mountpoint() -> *const c_char;
    pub fn sysfs__mount() -> *const c_char;
    pub fn sysfs__configured() -> bool;

    pub fn procfs__mountpoint() -> *const c_char;
    pub fn procfs__mount() -> *const c_char;
    pub fn procfs__configured() -> bool;

    pub fn debugfs__mountpoint() -> *const c_char;
    pub fn debugfs__mount() -> *const c_char;
    pub fn debugfs__configured() -> bool;

    pub fn tracefs__mountpoint() -> *const c_char;
    pub fn tracefs__mount() -> *const c_char;
    pub fn tracefs__configured() -> bool;

    pub fn hugetlbfs__mountpoint() -> *const c_char;
    pub fn hugetlbfs__mount() -> *const c_char;
    pub fn hugetlbfs__configured() -> bool;

    pub fn bpf_fs__mountpoint() -> *const c_char;
    pub fn bpf_fs__mount() -> *const c_char;
    pub fn bpf_fs__configured() -> bool;

    pub fn cgroupfs_find_mountpoint(
        buf: *mut c_char,
        maxlen: usize,
        subsys: *const c_char,
    ) -> c_int;

    pub fn filename__read_int(filename: *const c_char, value: *mut c_int) -> c_int;
    pub fn filename__read_ull(filename: *const c_char, value: *mut u64) -> c_int;
    pub fn filename__read_xll(filename: *const c_char, value: *mut u64) -> c_int;
    pub fn filename__read_str(
        filename: *const c_char,
        buf: *mut *mut c_char,
        sizep: *mut usize,
    ) -> c_int;

    pub fn filename__write_int(filename: *const c_char, value: c_int) -> c_int;

    pub fn procfs__read_str(
        entry: *const c_char,
        buf: *mut *mut c_char,
        sizep: *mut usize,
    ) -> c_int;

    pub fn sysctl__read_int(sysctl: *const c_char, value: *mut c_int) -> c_int;
    pub fn sysfs__read_int(entry: *const c_char, value: *mut c_int) -> c_int;
    pub fn sysfs__read_ull(entry: *const c_char, value: *mut u64) -> c_int;
    pub fn sysfs__read_xll(entry: *const c_char, value: *mut u64) -> c_int;
    pub fn sysfs__read_str(
        entry: *const c_char,
        buf: *mut *mut c_char,
        sizep: *mut usize,
    ) -> c_int;
    pub fn sysfs__read_bool(entry: *const c_char, value: *mut bool) -> c_int;

    pub fn sysfs__write_int(entry: *const c_char, value: c_int) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
