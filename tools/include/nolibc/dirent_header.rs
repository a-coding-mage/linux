/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Directory access for NOLIBC
 * Copyright (C) 2025 Thomas Weißschuh <linux@weissschuh.net>
 */

/* C header dependencies: nolibc.h, compiler.h, stdint.h, types.h, fcntl.h,
 * and <linux/limits.h>.
 */

use core::ffi::{c_char, c_int};
use core::mem::{size_of, size_of_val, MaybeUninit};
use core::ptr::null_mut;

#[repr(C)]
pub struct dirent {
    pub d_ino: ino_t,
    pub d_name: [c_char; (NAME_MAX + 1) as usize],
}

/* See comment of FILE in stdio.h */
#[repr(C)]
pub struct DIR {
    pub dummy: [c_char; 1],
}

pub unsafe fn fdopendir(fd: c_int) -> *mut DIR {
    if fd < 0 {
        SET_ERRNO(EBADF);
        return null_mut();
    }
    (!(fd as intptr_t)) as *mut DIR
}

pub unsafe fn opendir(name: *const c_char) -> *mut DIR {
    let fd: c_int;

    fd = open(name, O_RDONLY);
    if fd == -1 {
        return null_mut();
    }
    fdopendir(fd)
}

pub unsafe fn closedir(dirp: *mut DIR) -> c_int {
    let i: intptr_t = dirp as intptr_t;

    if i >= 0 {
        SET_ERRNO(EBADF);
        return -1;
    }
    close(!i)
}

#[repr(C)]
struct __nolibc_readdir_buf {
    ldir: linux_dirent64,
    name: [c_char; (NAME_MAX + 1) as usize],
}

pub unsafe fn readdir_r(
    dirp: *mut DIR,
    entry: *mut dirent,
    result: *mut *mut dirent,
) -> c_int {
    /* C used a char buffer aligned as struct linux_dirent64. */
    let mut buf = MaybeUninit::<__nolibc_readdir_buf>::uninit();
    let ldir = buf.as_mut_ptr() as *mut linux_dirent64;
    let i: intptr_t = dirp as intptr_t;
    let fd: c_int;
    let mut ret: c_int;

    if i >= 0 {
        return EBADF;
    }

    fd = !i;

    ret = _sys_getdents64(fd, ldir, size_of::<__nolibc_readdir_buf>());
    if ret < 0 {
        return -ret;
    }
    if ret == 0 {
        *result = null_mut();
        return 0;
    }

    /*
     * getdents64() returns as many entries as fit the buffer.
     * readdir() can only return one entry at a time.
     * Make sure the non-returned ones are not skipped.
     */
    ret = _sys_lseek(fd, (*ldir).d_off, SEEK_SET);
    if ret < 0 {
        return -ret;
    }

    (*entry).d_ino = (*ldir).d_ino;
    /* the destination should always be big enough */
    strlcpy(
        (*entry).d_name.as_mut_ptr(),
        (*ldir).d_name.as_ptr(),
        size_of_val(&(*entry).d_name),
    );
    *result = entry;
    0
}
