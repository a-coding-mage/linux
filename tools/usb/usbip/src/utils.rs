// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

use core::ffi::{c_char, c_int};

// Dependencies originally provided by:
// <errno.h>, <stdio.h>, <string.h>
// "usbip_common.h", "utils.h", "sysfs_utils.h"
unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;

    fn dbg(format: *const c_char, ...);
    fn write_sysfs_attribute(path: *const c_char, value: *const c_char, len: usize) -> c_int;
}

pub unsafe extern "C" fn modify_match_busid(busid: *mut c_char, add: c_int) -> c_int {
    let attr_name = b"match_busid\0";
    let mut command = [0 as c_char; SYSFS_BUS_ID_SIZE + 4];
    let mut match_busid_attr_path = [0 as c_char; SYSFS_PATH_MAX];
    let rc: c_int;
    let cmd_size: c_int;

    unsafe {
        snprintf(
            match_busid_attr_path.as_mut_ptr(),
            match_busid_attr_path.len(),
            b"%s/%s/%s/%s/%s/%s\0".as_ptr() as *const c_char,
            SYSFS_MNT_PATH,
            SYSFS_BUS_NAME,
            SYSFS_BUS_TYPE,
            SYSFS_DRIVERS_NAME,
            USBIP_HOST_DRV_NAME,
            attr_name.as_ptr() as *const c_char,
        );

        if add != 0 {
            cmd_size = snprintf(
                command.as_mut_ptr(),
                SYSFS_BUS_ID_SIZE + 4,
                b"add %s\0".as_ptr() as *const c_char,
                busid,
            );
        } else {
            cmd_size = snprintf(
                command.as_mut_ptr(),
                SYSFS_BUS_ID_SIZE + 4,
                b"del %s\0".as_ptr() as *const c_char,
                busid,
            );
        }

        rc = write_sysfs_attribute(
            match_busid_attr_path.as_ptr(),
            command.as_ptr(),
            cmd_size as usize,
        );
        if rc < 0 {
            dbg(
                b"failed to write match_busid: %s\0".as_ptr() as *const c_char,
                strerror(*__errno_location()),
            );
            return -1;
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
