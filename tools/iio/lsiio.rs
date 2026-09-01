// SPDX-License-Identifier: GPL-2.0-only
/*
 * Industrial I/O utilities - lsiio.c
 *
 * Copyright (c) 2010 Manuel Stahl <manuel.stahl@iis.fraunhofer.de>
 */

use std::os::raw::{c_char, c_int, c_void};

// Dependencies from C headers and "iio_utils.h".
extern "C" {
    static mut iio_dir: *const c_char;
    static mut optind: c_int;
    static mut stderr: *mut FILE;

    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn free(ptr: *mut c_void);
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;

    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn rewinddir(dirp: *mut DIR);

    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn read_sysfs_string(
        filename: *const c_char,
        basedir: *const c_char,
        str_: *mut c_char,
    ) -> c_int;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

const EOF: c_int = -1;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const IIO_MAX_NAME_LENGTH: usize = 64;

/* enum verbosity */
const VERBLEVEL_DEFAULT: c_int = 0; /* 0 gives lspci behaviour */
const VERBLEVEL_SENSORS: c_int = 1; /* 1 lists sensors */

static mut verblevel: c_int = VERBLEVEL_DEFAULT;

static TYPE_DEVICE_STR: &[u8; 11] = b"iio:device\0";
static TYPE_TRIGGER_STR: &[u8; 8] = b"trigger\0";

#[no_mangle]
pub static mut type_device: *const c_char = TYPE_DEVICE_STR.as_ptr() as *const c_char;
#[no_mangle]
pub static mut type_trigger: *const c_char = TYPE_TRIGGER_STR.as_ptr() as *const c_char;

unsafe fn errno() -> c_int {
    *__errno_location()
}

#[inline]
unsafe fn check_prefix(str_: *const c_char, prefix: *const c_char) -> c_int {
    (strlen(str_) > strlen(prefix) && strncmp(str_, prefix, strlen(prefix)) == 0) as c_int
}

#[inline]
unsafe fn check_postfix(str_: *const c_char, postfix: *const c_char) -> c_int {
    (strlen(str_) > strlen(postfix)
        && strcmp(str_.add(strlen(str_) - strlen(postfix)), postfix) == 0) as c_int
}

unsafe fn dump_channels(dev_dir_name: *const c_char) -> c_int {
    let mut dp: *mut DIR;
    let mut ent: *mut dirent;

    dp = opendir(dev_dir_name);
    if dp.is_null() {
        return -errno();
    }

    loop {
        ent = readdir(dp);
        if ent.is_null() {
            break;
        }
        if check_prefix((*ent).d_name.as_ptr(), b"in_\0".as_ptr() as *const c_char) != 0
            && (check_postfix((*ent).d_name.as_ptr(), b"_raw\0".as_ptr() as *const c_char) != 0
                || check_postfix((*ent).d_name.as_ptr(), b"_input\0".as_ptr() as *const c_char)
                    != 0)
        {
            printf(
                b"   %-10s\n\0".as_ptr() as *const c_char,
                (*ent).d_name.as_ptr(),
            );
        }
    }

    if closedir(dp) == -1 {
        -errno()
    } else {
        0
    }
}

unsafe fn dump_one_device(dev_dir_name: *const c_char) -> c_int {
    let mut name: [c_char; IIO_MAX_NAME_LENGTH] = [0; IIO_MAX_NAME_LENGTH];
    let mut dev_idx: c_int = 0;
    let mut ret: c_int;

    ret = sscanf(
        dev_dir_name.add(strlen(iio_dir) + strlen(type_device)),
        b"%i\0".as_ptr() as *const c_char,
        &mut dev_idx as *mut c_int,
    );
    if ret != 1 {
        return -EINVAL;
    }

    ret = read_sysfs_string(
        b"name\0".as_ptr() as *const c_char,
        dev_dir_name,
        name.as_mut_ptr(),
    );
    if ret < 0 {
        return ret;
    }

    printf(
        b"Device %03d: %s\n\0".as_ptr() as *const c_char,
        dev_idx,
        name.as_ptr(),
    );

    if verblevel >= VERBLEVEL_SENSORS {
        return dump_channels(dev_dir_name);
    }

    0
}

unsafe fn dump_one_trigger(dev_dir_name: *const c_char) -> c_int {
    let mut name: [c_char; IIO_MAX_NAME_LENGTH] = [0; IIO_MAX_NAME_LENGTH];
    let mut dev_idx: c_int = 0;
    let mut ret: c_int;

    ret = sscanf(
        dev_dir_name.add(strlen(iio_dir) + strlen(type_trigger)),
        b"%i\0".as_ptr() as *const c_char,
        &mut dev_idx as *mut c_int,
    );
    if ret != 1 {
        return -EINVAL;
    }

    ret = read_sysfs_string(
        b"name\0".as_ptr() as *const c_char,
        dev_dir_name,
        name.as_mut_ptr(),
    );
    if ret < 0 {
        return ret;
    }

    printf(
        b"Trigger %03d: %s\n\0".as_ptr() as *const c_char,
        dev_idx,
        name.as_ptr(),
    );

    0
}

unsafe fn dump_devices() -> c_int {
    let mut ent: *mut dirent;
    let mut ret: c_int;
    let mut dp: *mut DIR;

    dp = opendir(iio_dir);
    if dp.is_null() {
        fprintf(
            stderr,
            b"No industrial I/O devices available\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }

    loop {
        ent = readdir(dp);
        if ent.is_null() {
            break;
        }
        if check_prefix(
            (*ent).d_name.as_ptr(),
            type_device,
        ) != 0
        {
            let mut dev_dir_name: *mut c_char = std::ptr::null_mut();

            if asprintf(
                &mut dev_dir_name as *mut *mut c_char,
                b"%s%s\0".as_ptr() as *const c_char,
                iio_dir,
                (*ent).d_name.as_ptr(),
            ) < 0
            {
                ret = -ENOMEM;
                goto_error_close_dir(dp, ret);
                return ret;
            }

            ret = dump_one_device(dev_dir_name);
            if ret != 0 {
                free(dev_dir_name as *mut c_void);
                goto_error_close_dir(dp, ret);
                return ret;
            }

            free(dev_dir_name as *mut c_void);
            if verblevel >= VERBLEVEL_SENSORS {
                printf(b"\n\0".as_ptr() as *const c_char);
            }
        }
    }
    rewinddir(dp);
    loop {
        ent = readdir(dp);
        if ent.is_null() {
            break;
        }
        if check_prefix(
            (*ent).d_name.as_ptr(),
            type_trigger,
        ) != 0
        {
            let mut dev_dir_name: *mut c_char = std::ptr::null_mut();

            if asprintf(
                &mut dev_dir_name as *mut *mut c_char,
                b"%s%s\0".as_ptr() as *const c_char,
                iio_dir,
                (*ent).d_name.as_ptr(),
            ) < 0
            {
                ret = -ENOMEM;
                goto_error_close_dir(dp, ret);
                return ret;
            }

            ret = dump_one_trigger(dev_dir_name);
            if ret != 0 {
                free(dev_dir_name as *mut c_void);
                goto_error_close_dir(dp, ret);
                return ret;
            }

            free(dev_dir_name as *mut c_void);
        }
    }

    if closedir(dp) == -1 {
        -errno()
    } else {
        0
    }
}

unsafe fn goto_error_close_dir(dp: *mut DIR, ret: c_int) {
    if closedir(dp) == -1 {
        perror(b"dump_devices(): Failed to close directory\0".as_ptr() as *const c_char);
    }

    let _ = ret;
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut c: c_int;
    let mut err: c_int = 0;

    loop {
        c = getopt(argc, argv, b"v\0".as_ptr() as *const c_char);
        if c == EOF {
            break;
        }
        match c {
            v if v == b'v' as c_int => {
                verblevel += 1;
            }

            q if q == b'?' as c_int => {
                err += 1;
            }

            _ => {
                err += 1;
            }
        }
    }
    if err != 0 || argc > optind {
        fprintf(
            stderr,
            b"Usage: lsiio [options]...\nList industrial I/O devices\n  -v  Increase verbosity (may be given multiple times)\n\0"
                .as_ptr() as *const c_char,
        );
        exit(1);
    }

    dump_devices()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
