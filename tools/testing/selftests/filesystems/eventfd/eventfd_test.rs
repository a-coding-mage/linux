// SPDX-License-Identifier: GPL-2.0

// C dependency intent:
// _GNU_SOURCE, errno.h, fcntl.h, asm/unistd.h, linux/time_types.h, unistd.h,
// assert.h, signal.h, pthread.h, sys/epoll.h, sys/eventfd.h,
// and "kselftest_harness.h".

use std::ffi::{CStr, CString};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::raw::{c_int, c_long, c_uint, c_void};
use std::os::unix::io::FromRawFd;
use std::ptr;

const EVENTFD_TEST_ITERATIONS: u64 = 100000;

const F_GETFL: c_int = 3;
const F_GETFD: c_int = 1;
const FD_CLOEXEC: c_int = 1;
const O_RDWR: c_int = 0o2;
const O_CLOEXEC: c_int = 0o2000000;
const O_NONBLOCK: c_int = 0o4000;
const EFD_CLOEXEC: c_int = O_CLOEXEC;
const EFD_NONBLOCK: c_int = O_NONBLOCK;
const EFD_SEMAPHORE: c_int = 1;
const EINVAL: c_int = 22;
const EAGAIN: c_int = 11;
const SYS_EVENTFD2: c_long = 290;

#[repr(C)]
struct error {
    code: c_int,
    msg: [u8; 512],
}

extern "C" {
    fn syscall(number: c_long, ...) -> c_long;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn __errno_location() -> *mut c_int;
}

fn errno() -> c_int {
    unsafe { *__errno_location() }
}

fn error_set(err: *mut error, code: c_int, args: fmt::Arguments<'_>) -> c_int {
    if code == 0 || err.is_null() || unsafe { (*err).code } != 0 {
        return code;
    }

    let rendered = fmt::format(args);

    unsafe {
        (*err).code = code;
        let bytes = rendered.as_bytes();
        assert!(bytes.len() < (*err).msg.len());
        ptr::write_bytes((*err).msg.as_mut_ptr(), 0, (*err).msg.len());
        ptr::copy_nonoverlapping(bytes.as_ptr(), (*err).msg.as_mut_ptr(), bytes.len());
    }

    code
}

macro_rules! error_set {
    ($err:expr, $code:expr, $($arg:tt)*) => {
        error_set($err, $code, format_args!($($arg)*))
    };
}

#[inline]
fn sys_eventfd2(count: c_uint, flags: c_int) -> c_int {
    unsafe { syscall(SYS_EVENTFD2, count, flags) as c_int }
}

fn eventfd_check_flag_rdwr() {
    let fd: c_int;
    let flags: c_int;

    fd = sys_eventfd2(0, 0);
    assert!(fd >= 0);

    flags = unsafe { fcntl(fd, F_GETFL) };
    // The kernel automatically adds the O_RDWR flag.
    assert_eq!(flags, O_RDWR);

    unsafe {
        close(fd);
    }
}

fn eventfd_check_flag_cloexec() {
    let fd: c_int;
    let flags: c_int;

    fd = sys_eventfd2(0, EFD_CLOEXEC);
    assert!(fd >= 0);

    flags = unsafe { fcntl(fd, F_GETFD) };
    assert!(flags > -1);
    assert_eq!(flags, FD_CLOEXEC);

    unsafe {
        close(fd);
    }
}

fn eventfd_check_flag_nonblock() {
    let fd: c_int;
    let flags: c_int;

    fd = sys_eventfd2(0, EFD_NONBLOCK);
    assert!(fd >= 0);

    flags = unsafe { fcntl(fd, F_GETFL) };
    assert!(flags > -1);
    assert_eq!(flags & EFD_NONBLOCK, EFD_NONBLOCK);
    assert_eq!(flags & O_RDWR, O_RDWR);

    unsafe {
        close(fd);
    }
}

fn eventfd_check_flag_cloexec_and_nonblock() {
    let fd: c_int;
    let mut flags: c_int;

    fd = sys_eventfd2(0, EFD_CLOEXEC | EFD_NONBLOCK);
    assert!(fd >= 0);

    flags = unsafe { fcntl(fd, F_GETFL) };
    assert!(flags > -1);
    assert_eq!(flags & EFD_NONBLOCK, EFD_NONBLOCK);
    assert_eq!(flags & O_RDWR, O_RDWR);

    flags = unsafe { fcntl(fd, F_GETFD) };
    assert!(flags > -1);
    assert_eq!(flags, FD_CLOEXEC);

    unsafe {
        close(fd);
    }
}

#[inline]
fn trim_newline(str_: &mut String) {
    if let Some(pos) = str_.rfind('\n') {
        str_.truncate(pos);
    }
}

fn verify_fdinfo(
    fd: c_int,
    err: *mut error,
    prefix: &str,
    prefix_len: usize,
    expect: fmt::Arguments<'_>,
) -> c_int {
    let mut buffer = fmt::format(expect);
    assert!(buffer.len() < 512);

    let path = format!("/proc/self/fdinfo/{}", fd);
    let c_path = CString::new(path.clone()).unwrap();
    let fdinfo_fd = unsafe { syscall(2, c_path.as_ptr(), 0) as c_int };
    if fdinfo_fd < 0 {
        return error_set!(err, -1, "fdinfo open failed for {}", fd);
    }

    let f = unsafe { File::from_raw_fd(fdinfo_fd) };
    let mut found: c_int = 0;

    for line_result in BufReader::new(f).lines() {
        let mut line = line_result.unwrap();
        line.push('\n');

        if !line.as_bytes().starts_with(&prefix.as_bytes()[..prefix_len]) {
            continue;
        }

        found = 1;

        let mut val = line[prefix_len..].to_string();
        let r = val.as_bytes().cmp(buffer.as_bytes());
        if r != std::cmp::Ordering::Equal {
            trim_newline(&mut line);
            trim_newline(&mut buffer);
            trim_newline(&mut val);
            error_set!(err, -1, "{} '{}' != '{}'", prefix, val, buffer);
        }
        break;
    }

    if found == 0 {
        return error_set!(err, -1, "{} not found for fd {}", prefix, fd);
    }

    0
}

fn eventfd_check_flag_semaphore() {
    let mut err = error {
        code: 0,
        msg: [0; 512],
    };
    let fd: c_int;
    let ret: c_int;

    fd = sys_eventfd2(0, EFD_SEMAPHORE);
    assert!(fd >= 0);

    let mut ret = unsafe { fcntl(fd, F_GETFL) };
    assert!(ret > -1);
    assert_eq!(ret & O_RDWR, O_RDWR);

    // The semaphore could only be obtained from fdinfo.
    ret = verify_fdinfo(fd, &mut err, "eventfd-semaphore: ", 19, format_args!("1\n"));
    if ret != 0 {
        let msg = unsafe { CStr::from_ptr(err.msg.as_ptr() as *const i8) }
            .to_string_lossy()
            .into_owned();
        eprintln!("eventfd semaphore flag check failed: {}", msg);
    }
    assert_eq!(ret, 0);

    unsafe {
        close(fd);
    }
}

/*
 * A write(2) fails with the error EINVAL if the size of the supplied buffer
 * is less than 8 bytes, or if an attempt is made to write the value
 * 0xffffffffffffffff.
 */
fn eventfd_check_write() {
    let mut value: u64 = 1;
    let mut size: isize;
    let fd: c_int;

    fd = sys_eventfd2(0, 0);
    assert!(fd >= 0);

    size = unsafe { write(fd, &value as *const u64 as *const c_void, std::mem::size_of::<c_int>()) };
    assert_eq!(size, -1);
    assert_eq!(errno(), EINVAL);

    size = unsafe { write(fd, &value as *const u64 as *const c_void, std::mem::size_of_val(&value)) };
    assert_eq!(size, std::mem::size_of_val(&value) as isize);

    value = -1i64 as u64;
    size = unsafe { write(fd, &value as *const u64 as *const c_void, std::mem::size_of_val(&value)) };
    assert_eq!(size, -1);
    assert_eq!(errno(), EINVAL);

    unsafe {
        close(fd);
    }
}

/*
 * A read(2) fails with the error EINVAL if the size of the supplied buffer is
 * less than 8 bytes.
 */
fn eventfd_check_read() {
    let mut value: u64 = 0;
    let mut size: isize;
    let fd: c_int;

    fd = sys_eventfd2(1, 0);
    assert!(fd >= 0);

    size = unsafe { read(fd, &mut value as *mut u64 as *mut c_void, std::mem::size_of::<c_int>()) };
    assert_eq!(size, -1);
    assert_eq!(errno(), EINVAL);

    size = unsafe { read(fd, &mut value as *mut u64 as *mut c_void, std::mem::size_of_val(&value)) };
    assert_eq!(size, std::mem::size_of_val(&value) as isize);
    assert_eq!(value, 1);

    unsafe {
        close(fd);
    }
}

/*
 * If EFD_SEMAPHORE was not specified and the eventfd counter has a nonzero
 * value, then a read(2) returns 8 bytes containing that value, and the
 * counter's value is reset to zero.
 * If the eventfd counter is zero at the time of the call to read(2), then the
 * call fails with the error EAGAIN if the file descriptor has been made nonblocking.
 */
fn eventfd_check_read_with_nonsemaphore() {
    let mut value: u64;
    let mut size: isize;
    let fd: c_int;
    let mut i: c_int;

    fd = sys_eventfd2(0, EFD_NONBLOCK);
    assert!(fd >= 0);

    value = 1;
    i = 0;
    while (i as u64) < EVENTFD_TEST_ITERATIONS {
        size = unsafe { write(fd, &value as *const u64 as *const c_void, std::mem::size_of_val(&value)) };
        assert_eq!(size, std::mem::size_of_val(&value) as isize);
        i += 1;
    }

    size = unsafe { read(fd, &mut value as *mut u64 as *mut c_void, std::mem::size_of_val(&value)) };
    assert_eq!(size, std::mem::size_of::<u64>() as isize);
    assert_eq!(value, EVENTFD_TEST_ITERATIONS);

    size = unsafe { read(fd, &mut value as *mut u64 as *mut c_void, std::mem::size_of_val(&value)) };
    assert_eq!(size, -1);
    assert_eq!(errno(), EAGAIN);

    unsafe {
        close(fd);
    }
}

/*
 * If EFD_SEMAPHORE was specified and the eventfd counter has a nonzero value,
 * then a read(2) returns 8 bytes containing the value 1, and the counter's
 * value is decremented by 1.
 * If the eventfd counter is zero at the time of the call to read(2), then the
 * call fails with the error EAGAIN if the file descriptor has been made nonblocking.
 */
fn eventfd_check_read_with_semaphore() {
    let mut value: u64;
    let mut size: isize;
    let fd: c_int;
    let mut i: c_int;

    fd = sys_eventfd2(0, EFD_SEMAPHORE | EFD_NONBLOCK);
    assert!(fd >= 0);

    value = 1;
    i = 0;
    while (i as u64) < EVENTFD_TEST_ITERATIONS {
        size = unsafe { write(fd, &value as *const u64 as *const c_void, std::mem::size_of_val(&value)) };
        assert_eq!(size, std::mem::size_of_val(&value) as isize);
        i += 1;
    }

    i = 0;
    while (i as u64) < EVENTFD_TEST_ITERATIONS {
        size = unsafe { read(fd, &mut value as *mut u64 as *mut c_void, std::mem::size_of_val(&value)) };
        assert_eq!(size, std::mem::size_of_val(&value) as isize);
        assert_eq!(value, 1);
        i += 1;
    }

    size = unsafe { read(fd, &mut value as *mut u64 as *mut c_void, std::mem::size_of_val(&value)) };
    assert_eq!(size, -1);
    assert_eq!(errno(), EAGAIN);

    unsafe {
        close(fd);
    }
}

fn main() {
    eventfd_check_flag_rdwr();
    eventfd_check_flag_cloexec();
    eventfd_check_flag_nonblock();
    eventfd_check_flag_cloexec_and_nonblock();
    eventfd_check_flag_semaphore();
    eventfd_check_write();
    eventfd_check_read();
    eventfd_check_read_with_nonsemaphore();
    eventfd_check_read_with_semaphore();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
