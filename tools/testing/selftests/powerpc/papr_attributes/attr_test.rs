// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PAPR Energy attributes sniff test
 * This checks if the papr folders and contents are populated relating to
 * the energy and frequency attributes
 *
 * Copyright 2022, Pratik Rajesh Sampat, IBM Corp.
 */

// C includes translated as external dependencies:
// errno.h, stdio.h, string.h, dirent.h, sys/types.h, sys/stat.h, unistd.h,
// stdlib.h, and "utils.h".

use core::ffi::{c_char, c_int, c_void};

const POWER_PERFORMANCE_MODE: c_int = 1;
const IDLE_POWER_SAVER_STATUS: c_int = 2;
const MIN_FREQ: c_int = 3;
const STAT_FREQ: c_int = 4;
const MAX_FREQ: c_int = 6;
const PROC_FOLDING_STATUS: c_int = 8;

const INVALID: c_int = 0;
const STR_VAL: c_int = 1;
const NUM_VAL: c_int = 2;

const ENOENT: c_int = 2;
const EOF: c_int = -1;

type ModeT = u32;

#[repr(C)]
struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct dirent {
    d_ino: u64,
    d_off: i64,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

#[repr(C)]
struct stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: ModeT,
    st_uid: u32,
    st_gid: u32,
    __pad0: c_int,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    st_atime: i64,
    st_atime_nsec: i64,
    st_mtime: i64,
    st_mtime_nsec: i64,
    st_ctime: i64,
    st_ctime_nsec: i64,
    __unused: [i64; 3],
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgetc(stream: *mut FILE) -> c_int;

    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;

    // From "utils.h"; C macros are preserved as external test helpers.
    fn SKIP_IF(condition: bool);
    fn FAIL_IF(condition: bool);
}

fn S_ISDIR(mode: ModeT) -> bool {
    (mode & 0o170000) == 0o040000
}

unsafe extern "C" fn value_type(id: c_int) -> c_int {
    let val_type: c_int;

    match id {
        POWER_PERFORMANCE_MODE | IDLE_POWER_SAVER_STATUS => {
            val_type = STR_VAL;
        }
        MIN_FREQ | STAT_FREQ | MAX_FREQ | PROC_FOLDING_STATUS => {
            val_type = NUM_VAL;
        }
        _ => {
            val_type = INVALID;
        }
    }

    val_type
}

unsafe extern "C" fn verify_energy_info() -> c_int {
    let path: *const c_char = c"/sys/firmware/papr/energy_scale_info".as_ptr();
    let mut entry: *mut dirent;
    let mut s: stat = core::mem::zeroed();
    let dirp: *mut DIR;

    *__errno_location() = 0;
    if stat(path, &mut s) != 0 {
        SKIP_IF(*__errno_location() == ENOENT);
        FAIL_IF(*__errno_location() != 0);
    }

    FAIL_IF(!S_ISDIR(s.st_mode));

    dirp = opendir(path);

    loop {
        entry = readdir(dirp);
        if entry.is_null() {
            break;
        }

        let mut file_name: [c_char; 64] = [0; 64];
        let id: c_int;
        let attr_type: c_int;
        let mut f: *mut FILE;

        if strcmp((*entry).d_name.as_ptr(), c".".as_ptr()) == 0
            || strcmp((*entry).d_name.as_ptr(), c"..".as_ptr()) == 0
        {
            continue;
        }

        id = atoi((*entry).d_name.as_ptr());
        attr_type = value_type(id);
        FAIL_IF(attr_type == INVALID);

        /* Check if the files exist and have data in them */
        sprintf(
            file_name.as_mut_ptr(),
            c"%s/%d/desc".as_ptr(),
            path,
            id,
        );
        f = fopen(file_name.as_ptr(), c"r".as_ptr());
        FAIL_IF(f.is_null());
        FAIL_IF(fgetc(f) == EOF);

        sprintf(
            file_name.as_mut_ptr(),
            c"%s/%d/value".as_ptr(),
            path,
            id,
        );
        f = fopen(file_name.as_ptr(), c"r".as_ptr());
        FAIL_IF(f.is_null());
        FAIL_IF(fgetc(f) == EOF);

        if attr_type == STR_VAL {
            sprintf(
                file_name.as_mut_ptr(),
                c"%s/%d/value_desc".as_ptr(),
                path,
                id,
            );
            f = fopen(file_name.as_ptr(), c"r".as_ptr());
            FAIL_IF(f.is_null());
            FAIL_IF(fgetc(f) == EOF);
        }
    }

    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            verify_energy_info,
            c"papr_attributes".as_ptr(),
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
