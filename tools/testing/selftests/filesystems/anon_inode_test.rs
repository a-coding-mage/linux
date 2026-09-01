// SPDX-License-Identifier: GPL-2.0
// C source used _GNU_SOURCE and __SANE_USERSPACE_TYPES__ before including:
// <fcntl.h>, <stdio.h>, <sys/stat.h>, "kselftest_harness.h", and "wrappers.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

type uid_t = c_uint;
type gid_t = c_uint;
type mode_t = c_uint;

const AT_EMPTY_PATH: c_int = 0x1000;
const EACCES: c_int = 13;
const ENXIO: c_int = 6;
const EOPNOTSUPP: c_int = 95;

unsafe extern "C" {
    fn close(fd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn execveat(
        dirfd: c_int,
        pathname: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
        flags: c_int,
    ) -> c_int;
    fn fchmod(fd: c_int, mode: mode_t) -> c_int;
    fn fchown(fd: c_int, owner: uid_t, group: gid_t) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;

    // From "wrappers.h".
    fn sys_fsopen(fs_name: *const c_char, flags: c_uint) -> c_int;

    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn anon_inode_no_chown() {
    let fd_context: c_int;

    fd_context = unsafe { sys_fsopen(c"tmpfs".as_ptr(), 0) };
    assert!(fd_context >= 0);

    assert!(unsafe { fchown(fd_context, 1234, 5678) } < 0);
    assert_eq!(unsafe { errno() }, EOPNOTSUPP);

    assert_eq!(unsafe { close(fd_context) }, 0);
}

unsafe fn anon_inode_no_chmod() {
    let fd_context: c_int;

    fd_context = unsafe { sys_fsopen(c"tmpfs".as_ptr(), 0) };
    assert!(fd_context >= 0);

    assert!(unsafe { fchmod(fd_context, 0o777) } < 0);
    assert_eq!(unsafe { errno() }, EOPNOTSUPP);

    assert_eq!(unsafe { close(fd_context) }, 0);
}

unsafe fn anon_inode_no_exec() {
    let fd_context: c_int;

    fd_context = unsafe { sys_fsopen(c"tmpfs".as_ptr(), 0) };
    assert!(fd_context >= 0);

    let empty_argv: [*mut c_char; 1] = [ptr::null_mut()];
    let empty_envp: [*mut c_char; 1] = [ptr::null_mut()];

    assert!(
        unsafe {
            execveat(
                fd_context,
                c"".as_ptr(),
                empty_argv.as_ptr(),
                empty_envp.as_ptr(),
                AT_EMPTY_PATH,
            )
        } < 0
    );
    assert_eq!(unsafe { errno() }, EACCES);

    assert_eq!(unsafe { close(fd_context) }, 0);
}

unsafe fn anon_inode_no_open() {
    let mut fd_context: c_int;

    fd_context = unsafe { sys_fsopen(c"tmpfs".as_ptr(), 0) };
    assert!(fd_context >= 0);

    assert!(unsafe { dup2(fd_context, 500) } >= 0);
    assert_eq!(unsafe { close(fd_context) }, 0);
    fd_context = 500;

    assert!(unsafe { open(c"/proc/self/fd/500".as_ptr(), 0) } < 0);
    assert_eq!(unsafe { errno() }, ENXIO);

    assert_eq!(unsafe { close(fd_context) }, 0);
}

fn main() {
    unsafe {
        anon_inode_no_chown();
        anon_inode_no_chmod();
        anon_inode_no_exec();
        anon_inode_no_open();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
