/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * stat definition for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/*
 * C include dependency intent:
 * - "../nolibc.h" is included before the guard to expose all global symbols.
 * - "../arch.h", "../types.h", "../sys.h", and "../sys/sysmacros.h" provide
 *   syscall numbers, constants, struct stat/statx, and makedev().
 */

/*
 * int statx(int fd, const char *path, int flags, unsigned int mask, struct statx *buf);
 * int stat(const char *path, struct stat *buf);
 * int fstatat(int fd, const char *path, struct stat *buf, int flag);
 * int fstat(int fildes, struct stat *buf);
 * int lstat(const char *path, struct stat *buf);
 */

unsafe extern "C" {
    fn __nolibc_syscall5(
        nr: isize,
        arg1: isize,
        arg2: isize,
        arg3: isize,
        arg4: isize,
        arg5: isize,
    ) -> isize;
    fn __nolibc_enosys(
        func: *const core::ffi::c_char,
        arg1: isize,
        arg2: isize,
        arg3: isize,
        arg4: isize,
        arg5: isize,
    ) -> i32;
    fn __sysret(ret: isize) -> isize;
    fn makedev(major: u32, minor: u32) -> u64;
}

pub unsafe fn _sys_statx(
    fd: i32,
    path: *const core::ffi::c_char,
    flags: i32,
    mask: u32,
    buf: *mut statx,
) -> i32 {
    /*
     * C conditional:
     * #ifdef __NR_statx
     *     return __nolibc_syscall5(__NR_statx, fd, path, flags, mask, buf);
     * #else
     *     return __nolibc_enosys(__func__, fd, path, flags, mask, buf);
     * #endif
     */
    unsafe {
        __nolibc_syscall5(
            __NR_statx as isize,
            fd as isize,
            path as isize,
            flags as isize,
            mask as isize,
            buf as isize,
        ) as i32
    }
}

pub unsafe fn statx(
    fd: i32,
    path: *const core::ffi::c_char,
    flags: i32,
    mask: u32,
    buf: *mut statx,
) -> i32 {
    unsafe { __sysret(_sys_statx(fd, path, flags, mask, buf) as isize) as i32 }
}

pub unsafe fn fstatat(
    fd: i32,
    path: *const core::ffi::c_char,
    buf: *mut stat,
    flag: i32,
) -> i32 {
    let mut statx: statx = unsafe { core::mem::zeroed() };
    let ret: isize;

    ret = unsafe {
        __sysret(_sys_statx(
            fd,
            path,
            flag | AT_NO_AUTOMOUNT,
            STATX_BASIC_STATS,
            &mut statx,
        ) as isize)
    };
    if ret == -1 {
        return ret as i32;
    }

    unsafe {
        (*buf).st_dev = makedev(statx.stx_dev_major, statx.stx_dev_minor);
        (*buf).st_ino = statx.stx_ino;
        (*buf).st_mode = statx.stx_mode;
        (*buf).st_nlink = statx.stx_nlink;
        (*buf).st_uid = statx.stx_uid;
        (*buf).st_gid = statx.stx_gid;
        (*buf).st_rdev = makedev(statx.stx_rdev_major, statx.stx_rdev_minor);
        (*buf).st_size = statx.stx_size;
        (*buf).st_blksize = statx.stx_blksize;
        (*buf).st_blocks = statx.stx_blocks;
        (*buf).st_atim.tv_sec = statx.stx_atime.tv_sec;
        (*buf).st_atim.tv_nsec = statx.stx_atime.tv_nsec;
        (*buf).st_mtim.tv_sec = statx.stx_mtime.tv_sec;
        (*buf).st_mtim.tv_nsec = statx.stx_mtime.tv_nsec;
        (*buf).st_ctim.tv_sec = statx.stx_ctime.tv_sec;
        (*buf).st_ctim.tv_nsec = statx.stx_ctime.tv_nsec;
    }

    0
}

pub unsafe fn stat(path: *const core::ffi::c_char, buf: *mut stat) -> i32 {
    unsafe { fstatat(AT_FDCWD, path, buf, 0) }
}

pub unsafe fn fstat(fildes: i32, buf: *mut stat) -> i32 {
    unsafe { fstatat(fildes, c"".as_ptr(), buf, AT_EMPTY_PATH) }
}

pub unsafe fn lstat(path: *const core::ffi::c_char, buf: *mut stat) -> i32 {
    unsafe { fstatat(AT_FDCWD, path, buf, AT_SYMLINK_NOFOLLOW) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
