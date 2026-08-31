/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */
/*
 * Lightweight directory reading library.
 */

use libc::{
    c_int, c_long, c_uchar, c_ushort, c_void, mode_t, off64_t, size_t, ssize_t, stat, DT_DIR,
    DT_UNKNOWN, NAME_MAX, SEEK_SET,
};

// C header dependency intent:
// <dirent.h>, <fcntl.h>, <stdlib.h>, <unistd.h>, <sys/stat.h>,
// <sys/syscall.h>, and <linux/limits.h>.

// Mirrors the fallback SYS_getdents64 selection used when the C headers do not
// provide SYS_getdents64.
#[cfg(any(target_arch = "x86_64", target_arch = "arm"))]
pub const SYS_GETDENTS64_FALLBACK: c_long = 217;
#[cfg(any(target_arch = "x86", target_arch = "s390x", target_arch = "sh"))]
pub const SYS_GETDENTS64_FALLBACK: c_long = 220;
#[cfg(target_arch = "alpha")]
pub const SYS_GETDENTS64_FALLBACK: c_long = 377;
#[cfg(target_arch = "mips")]
pub const SYS_GETDENTS64_FALLBACK: c_long = 308;
#[cfg(any(target_arch = "powerpc64", target_arch = "powerpc"))]
pub const SYS_GETDENTS64_FALLBACK: c_long = 202;
#[cfg(any(target_arch = "sparc64", target_arch = "sparc"))]
pub const SYS_GETDENTS64_FALLBACK: c_long = 154;
#[cfg(target_arch = "xtensa")]
pub const SYS_GETDENTS64_FALLBACK: c_long = 60;
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "s390x",
    target_arch = "sh",
    target_arch = "alpha",
    target_arch = "mips",
    target_arch = "powerpc64",
    target_arch = "powerpc",
    target_arch = "sparc64",
    target_arch = "sparc",
    target_arch = "xtensa"
)))]
pub const SYS_GETDENTS64_FALLBACK: c_long = 61;

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn lseek(fd: c_int, offset: off64_t, whence: c_int) -> off64_t;
    fn fstatat(dirfd: c_int, pathname: *const i8, statbuf: *mut stat, flags: c_int) -> c_int;
}

#[inline]
pub unsafe fn perf_getdents64(fd: c_int, dirp: *mut c_void, count: size_t) -> ssize_t {
    #[cfg(feature = "memory_sanitizer")]
    {
        unsafe {
            core::ptr::write_bytes(dirp, 0, count);
        }
    }

    unsafe { syscall(libc::SYS_getdents64 as c_long, fd, dirp, count) as ssize_t }
}

#[repr(C)]
pub struct io_dirent64 {
    pub d_ino: u64,                    /* 64-bit inode number */
    pub d_off: off64_t,                /* 64-bit offset to next structure */
    pub d_reclen: c_ushort,            /* Size of this dirent */
    pub d_type: c_uchar,               /* File type */
    pub d_name: [i8; NAME_MAX as usize + 1], /* Filename (null-terminated) */
}

#[repr(C)]
pub struct io_dir {
    pub dirfd: c_int,
    pub available_bytes: ssize_t,
    pub next: *mut io_dirent64,
    pub buff: [io_dirent64; 4],
}

#[inline]
pub unsafe fn io_dir__init(iod: *mut io_dir, dirfd: c_int) {
    unsafe {
        (*iod).dirfd = dirfd;
        (*iod).available_bytes = 0;
    }
}

#[inline]
pub unsafe fn io_dir__rewinddir(iod: *mut io_dir) {
    unsafe {
        lseek((*iod).dirfd, 0, SEEK_SET);
        (*iod).available_bytes = 0;
    }
}

#[inline]
pub unsafe fn io_dir__readdir(iod: *mut io_dir) -> *mut io_dirent64 {
    let entry: *mut io_dirent64;

    unsafe {
        if (*iod).available_bytes <= 0 {
            let rc: ssize_t = perf_getdents64(
                (*iod).dirfd,
                (*iod).buff.as_mut_ptr() as *mut c_void,
                core::mem::size_of_val(&(*iod).buff) as size_t,
            );

            if rc <= 0 {
                return core::ptr::null_mut();
            }
            (*iod).available_bytes = rc;
            (*iod).next = (*iod).buff.as_mut_ptr();
        }
        entry = (*iod).next;
        (*iod).next = (entry as *mut i8).add((*entry).d_reclen as usize) as *mut io_dirent64;
        (*iod).available_bytes -= (*entry).d_reclen as ssize_t;
        entry
    }
}

#[inline]
pub unsafe fn io_dir__is_dir(iod: *const io_dir, dent: *mut io_dirent64) -> bool {
    unsafe {
        if (*dent).d_type == DT_UNKNOWN {
            let mut st: stat = core::mem::zeroed();

            if fstatat((*iod).dirfd, (*dent).d_name.as_ptr(), &mut st, 0) != 0 {
                return false;
            }

            if s_isdir(st.st_mode) {
                (*dent).d_type = DT_DIR;
                return true;
            }
        }
        (*dent).d_type == DT_DIR
    }
}

#[inline]
pub const fn s_isdir(mode: mode_t) -> bool {
    (mode & libc::S_IFMT) == libc::S_IFDIR
}
