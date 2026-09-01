// SPDX-License-Identifier: GPL-2.0
/*
 * Linux Security Module infrastructure tests
 *
 * Copyright © 2023 Casey Schaufler <casey@schaufler-ca.com>
 */

// C dependencies: linux/lsm.h, fcntl.h, string.h, stdio.h, stdlib.h,
// unistd.h, sys/types.h, and common.h.

use core::ffi::{c_char, c_int, c_void};

const PROCATTR: &[u8] = b"/proc/self/attr/\0";

#[no_mangle]
pub unsafe extern "C" fn read_proc_attr(
    attr: *const c_char,
    value: *mut c_char,
    size: libc::size_t,
) -> c_int {
    let mut fd: c_int;
    let mut len: c_int;
    let mut path: *mut c_char;

    len = (libc::strlen(PROCATTR.as_ptr() as *const c_char)
        + libc::strlen(attr)
        + 1) as c_int;
    path = libc::calloc(len as libc::size_t, 1) as *mut c_char;
    if path.is_null() {
        return -1;
    }
    libc::sprintf(
        path,
        b"%s%s\0".as_ptr() as *const c_char,
        PROCATTR.as_ptr() as *const c_char,
        attr,
    );

    fd = libc::open(path, libc::O_RDONLY);
    libc::free(path as *mut c_void);

    if fd < 0 {
        return -1;
    }
    len = libc::read(fd, value as *mut c_void, size) as c_int;

    libc::close(fd);

    /* Ensure value is terminated */
    if len <= 0 || len as libc::size_t == size {
        return -1;
    }
    *value.add(len as usize) = b'\0' as c_char;

    path = libc::strchr(value, b'\n' as c_int);
    if !path.is_null() {
        *path = b'\0' as c_char;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn read_sysfs_lsms(lsms: *mut c_char, size: libc::size_t) -> c_int {
    let mut fp: *mut libc::FILE;
    let mut red: libc::size_t;

    fp = libc::fopen(
        b"/sys/kernel/security/lsm\0".as_ptr() as *const c_char,
        b"r\0".as_ptr() as *const c_char,
    );
    if fp.is_null() {
        return -1;
    }
    red = libc::fread(lsms as *mut c_void, 1, size, fp);
    libc::fclose(fp);

    if red <= 0 || red == size {
        return -1;
    }
    *lsms.add(red) = b'\0' as c_char;
    0
}

#[no_mangle]
pub unsafe extern "C" fn attr_lsm_count() -> c_int {
    let names: *mut c_char = libc::calloc(libc::sysconf(libc::_SC_PAGESIZE) as libc::size_t, 1)
        as *mut c_char;
    let mut count: c_int = 0;

    if names.is_null() {
        return 0;
    }

    if read_sysfs_lsms(
        names,
        libc::sysconf(libc::_SC_PAGESIZE) as libc::size_t,
    ) != 0
    {
        libc::free(names as *mut c_void);
        return count;
    }

    if !libc::strstr(names, b"selinux\0".as_ptr() as *const c_char).is_null() {
        count += 1;
    }
    if !libc::strstr(names, b"smack\0".as_ptr() as *const c_char).is_null() {
        count += 1;
    }
    if !libc::strstr(names, b"apparmor\0".as_ptr() as *const c_char).is_null() {
        count += 1;
    }

    libc::free(names as *mut c_void);
    count
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
