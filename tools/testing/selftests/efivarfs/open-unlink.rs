// SPDX-License-Identifier: GPL-2.0
//
// C dependencies translated from:
// errno.h, stdio.h, stdint.h, stdlib.h, unistd.h, sys/ioctl.h,
// sys/types.h, sys/stat.h, fcntl.h, linux/fs.h

use libc::{
    c_char, c_int, c_ulong, c_uint, c_void, close, fprintf, ioctl, open, perror, read, stderr,
    unlink, write, EXIT_FAILURE, EXIT_SUCCESS, O_CREAT, O_RDONLY, O_WRONLY,
};

const FS_IOC_GETFLAGS: c_ulong = 0x80086601;
const FS_IOC_SETFLAGS: c_ulong = 0x40086602;
const FS_IMMUTABLE_FL: c_uint = 0x00000010;

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
}

unsafe fn set_errno(error: c_int) {
    *__errno_location() = error;
}

unsafe fn get_errno() -> c_int {
    *__errno_location()
}

unsafe fn set_immutable(path: *const c_char, immutable: c_int) -> c_int {
    let mut flags: c_uint = 0;
    let fd: c_int;
    let mut rc: c_int;
    let error: c_int;

    fd = open(path, O_RDONLY);
    if fd < 0 {
        return fd;
    }

    rc = ioctl(
        fd,
        FS_IOC_GETFLAGS,
        &mut flags as *mut c_uint as *mut c_void,
    );
    if rc < 0 {
        error = get_errno();
        close(fd);
        set_errno(error);
        return rc;
    }

    if immutable != 0 {
        flags |= FS_IMMUTABLE_FL;
    } else {
        flags &= !FS_IMMUTABLE_FL;
    }

    rc = ioctl(
        fd,
        FS_IOC_SETFLAGS,
        &mut flags as *mut c_uint as *mut c_void,
    );
    error = get_errno();
    close(fd);
    set_errno(error);
    rc
}

unsafe fn get_immutable(path: *const c_char) -> c_int {
    let mut flags: c_uint = 0;
    let fd: c_int;
    let rc: c_int;
    let error: c_int;

    fd = open(path, O_RDONLY);
    if fd < 0 {
        return fd;
    }

    rc = ioctl(
        fd,
        FS_IOC_GETFLAGS,
        &mut flags as *mut c_uint as *mut c_void,
    );
    if rc < 0 {
        error = get_errno();
        close(fd);
        set_errno(error);
        return rc;
    }
    close(fd);
    if flags & FS_IMMUTABLE_FL != 0 {
        return 1;
    }
    0
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let path: *const c_char;
    let mut buf: [c_char; 5] = [0; 5];
    let mut fd: c_int;
    let mut rc: isize;

    if argc < 2 {
        fprintf(
            stderr,
            b"usage: %s <path>\n\0".as_ptr() as *const c_char,
            *argv.offset(0),
        );
        return EXIT_FAILURE;
    }

    path = *argv.offset(1);

    /* attributes: EFI_VARIABLE_NON_VOLATILE |
     *		EFI_VARIABLE_BOOTSERVICE_ACCESS |
     *		EFI_VARIABLE_RUNTIME_ACCESS
     */
    *(buf.as_mut_ptr() as *mut u32) = 0x7;
    buf[4] = 0;

    /* create a test variable */
    fd = open(path, O_WRONLY | O_CREAT, 0o600);
    if fd < 0 {
        perror(b"open(O_WRONLY)\0".as_ptr() as *const c_char);
        return EXIT_FAILURE;
    }

    rc = write(
        fd,
        buf.as_ptr() as *const c_void,
        core::mem::size_of_val(&buf),
    );
    if rc != core::mem::size_of_val(&buf) as isize {
        perror(b"write\0".as_ptr() as *const c_char);
        return EXIT_FAILURE;
    }

    close(fd);

    rc = get_immutable(path) as isize;
    if rc < 0 {
        perror(b"ioctl(FS_IOC_GETFLAGS)\0".as_ptr() as *const c_char);
        return EXIT_FAILURE;
    } else if rc != 0 {
        rc = set_immutable(path, 0) as isize;
        if rc < 0 {
            perror(b"ioctl(FS_IOC_SETFLAGS)\0".as_ptr() as *const c_char);
            return EXIT_FAILURE;
        }
    }

    fd = open(path, O_RDONLY);
    if fd < 0 {
        perror(b"open\0".as_ptr() as *const c_char);
        return EXIT_FAILURE;
    }

    if unlink(path) < 0 {
        perror(b"unlink\0".as_ptr() as *const c_char);
        return EXIT_FAILURE;
    }

    rc = read(
        fd,
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    );
    if rc > 0 {
        fprintf(
            stderr,
            b"reading from an unlinked variable shouldn't be possible\n\0".as_ptr()
                as *const c_char,
        );
        return EXIT_FAILURE;
    }

    EXIT_SUCCESS
}

fn main() {
    let mut args: Vec<std::ffi::CString> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap())
        .collect();
    let mut argv: Vec<*mut c_char> = args
        .iter_mut()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .collect();
    let rc = unsafe { main_impl(argv.len() as c_int, argv.as_mut_ptr()) };
    std::process::exit(rc);
}
