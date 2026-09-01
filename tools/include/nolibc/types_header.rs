/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Special types used by various syscalls for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* C header dependencies:
 * - "nolibc.h" was included first to make sure to include all global symbols.
 * - "std.h", <linux/mman.h>, <linux/stat.h>, <linux/time_types.h>,
 *   <linux/wait.h>, and <linux/time.h> provide external constants and types.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: i64,
}

/* #define _STRUCT_TIMESPEC */

/* Never use with system calls */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: i64,
}

/* In C, timeval is temporarily defined as __nolibc_kernel_timeval while
 * including <linux/time.h>, then undefined.
 */

/* Only the generic macros and types may be defined here. The arch-specific
 * ones such as the O_RDONLY and related macros used by fcntl() and open()
 * must not be defined here.
 */

/* stat flags (WARNING, octal here). We need to check for an existing
 * definition because linux/stat.h may omit to define those if it finds
 * that any glibc header was already included.
 *
 * C condition: #if !defined(S_IFMT)
 */
pub const S_IFDIR: mode_t = 0o040000;
pub const S_IFCHR: mode_t = 0o020000;
pub const S_IFBLK: mode_t = 0o060000;
pub const S_IFREG: mode_t = 0o100000;
pub const S_IFIFO: mode_t = 0o010000;
pub const S_IFLNK: mode_t = 0o120000;
pub const S_IFSOCK: mode_t = 0o140000;
pub const S_IFMT: mode_t = 0o170000;

#[inline]
pub const fn S_ISDIR(mode: mode_t) -> bool {
    (mode & S_IFMT) == S_IFDIR
}

#[inline]
pub const fn S_ISCHR(mode: mode_t) -> bool {
    (mode & S_IFMT) == S_IFCHR
}

#[inline]
pub const fn S_ISBLK(mode: mode_t) -> bool {
    (mode & S_IFMT) == S_IFBLK
}

#[inline]
pub const fn S_ISREG(mode: mode_t) -> bool {
    (mode & S_IFMT) == S_IFREG
}

#[inline]
pub const fn S_ISFIFO(mode: mode_t) -> bool {
    (mode & S_IFMT) == S_IFIFO
}

#[inline]
pub const fn S_ISLNK(mode: mode_t) -> bool {
    (mode & S_IFMT) == S_IFLNK
}

#[inline]
pub const fn S_ISSOCK(mode: mode_t) -> bool {
    (mode & S_IFMT) == S_IFSOCK
}

pub const S_IRWXU: mode_t = 0o0700;
pub const S_IRUSR: mode_t = 0o0400;
pub const S_IWUSR: mode_t = 0o0200;
pub const S_IXUSR: mode_t = 0o0100;

pub const S_IRWXG: mode_t = 0o0070;
pub const S_IRGRP: mode_t = 0o0040;
pub const S_IWGRP: mode_t = 0o0020;
pub const S_IXGRP: mode_t = 0o0010;

pub const S_IRWXO: mode_t = 0o0007;
pub const S_IROTH: mode_t = 0o0004;
pub const S_IWOTH: mode_t = 0o0002;
pub const S_IXOTH: mode_t = 0o0001;

/* dirent types */
pub const DT_UNKNOWN: u8 = 0x0;
pub const DT_FIFO: u8 = 0x1;
pub const DT_CHR: u8 = 0x2;
pub const DT_DIR: u8 = 0x4;
pub const DT_BLK: u8 = 0x6;
pub const DT_REG: u8 = 0x8;
pub const DT_LNK: u8 = 0xa;
pub const DT_SOCK: u8 = 0xc;

/* PATH_MAX and MAXPATHLEN are often used and found with plenty of different
 * values.
 *
 * C conditions: #ifndef PATH_MAX / #ifndef MAXPATHLEN
 */
pub const PATH_MAX: usize = 4096;
pub const MAXPATHLEN: usize = PATH_MAX;

/* flags for mmap
 *
 * C condition: #ifndef MAP_FAILED
 */
pub const MAP_FAILED: *mut core::ffi::c_void = !0usize as *mut core::ffi::c_void;

/* whence values for lseek() */
pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;

/* flags for reboot */
pub const RB_AUTOBOOT: i32 = LINUX_REBOOT_CMD_RESTART;
pub const RB_HALT_SYSTEM: i32 = LINUX_REBOOT_CMD_HALT;
pub const RB_ENABLE_CAD: i32 = LINUX_REBOOT_CMD_CAD_ON;
pub const RB_DISABLE_CAD: i32 = LINUX_REBOOT_CMD_CAD_OFF;
pub const RB_POWER_OFF: i32 = LINUX_REBOOT_CMD_POWER_OFF;
pub const RB_SW_SUSPEND: i32 = LINUX_REBOOT_CMD_SW_SUSPEND;
pub const RB_KEXEC: i32 = LINUX_REBOOT_CMD_KEXEC;

/* Macros used on waitpid()'s return status */
#[inline]
pub const fn WEXITSTATUS(status: i32) -> i32 {
    (status & 0xff00) >> 8
}

#[inline]
pub const fn WIFEXITED(status: i32) -> bool {
    (status & 0x7f) == 0
}

#[inline]
pub const fn WTERMSIG(status: i32) -> i32 {
    status & 0x7f
}

#[inline]
pub const fn WIFSIGNALED(status: i32) -> bool {
    status - 1 < 0xff
}

/* standard exit() codes */
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

/* for getdents64() */
#[repr(C)]
pub struct linux_dirent64 {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [core::ffi::c_char; 0],
}

/* The format of the struct as returned by the libc to the application, which
 * significantly differs from the format returned by the stat() syscall flavours.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub union stat_st_atime {
    pub st_atime: time_t,
    pub st_atim: timespec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union stat_st_mtime {
    pub st_mtime: time_t,
    pub st_mtim: timespec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union stat_st_ctime {
    pub st_ctime: time_t,
    pub st_ctim: timespec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: dev_t,         /* ID of device containing file */
    pub st_ino: ino_t,         /* inode number */
    pub st_mode: mode_t,       /* protection */
    pub st_nlink: nlink_t,     /* number of hard links */
    pub st_uid: uid_t,         /* user ID of owner */
    pub st_gid: gid_t,         /* group ID of owner */
    pub st_rdev: dev_t,        /* device ID (if special file) */
    pub st_size: off_t,        /* total size, in bytes */
    pub st_blksize: blksize_t, /* blocksize for file system I/O */
    pub st_blocks: blkcnt_t,   /* number of 512B blocks allocated */
    pub st_atime: stat_st_atime, /* time of last access */
    pub st_mtime: stat_st_mtime, /* time of last modification */
    pub st_ctime: stat_st_ctime, /* time of last status change */
}

pub type clockid_t = __kernel_clockid_t;
pub type timer_t = i32;

/* C condition: #ifndef container_of */
#[macro_export]
macro_rules! container_of {
    ($ptr:expr, $type:ty, $field:tt) => {{
        let __field_ptr = $ptr;
        (__field_ptr as *const u8)
            .wrapping_sub(::core::mem::offset_of!($type, $field))
            as *mut $type
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
