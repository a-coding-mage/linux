// SPDX-License-Identifier: GPL-2.0

use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};

type SizeT = usize;
type SsizeT = isize;
type PidT = c_int;

#[repr(C)]
pub struct SiginfoT {
    _private: [u8; 0],
}

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn _exit(status: c_int) -> !;
    fn clone(
        func: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        ...
    ) -> PidT;
    fn syscall(number: c_long, ...) -> c_long;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(fd: c_int, path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: SizeT) -> SsizeT;
    fn write(fd: c_int, buf: *const c_void, count: SizeT) -> SsizeT;
    fn wait(status: *mut c_int) -> PidT;
    fn getpid() -> PidT;
    fn __errno_location() -> *mut c_int;
    fn warn(format: *const c_char, ...);
    fn warnx(format: *const c_char, ...);
    fn err(status: c_int, format: *const c_char, ...) -> !;
    fn exit(status: c_int) -> !;
}

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const SIGCHLD: c_int = 17;
const CLONE_PIDFD: c_int = 0x0000_1000;
const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2_000_000;
const O_DIRECTORY: c_int = 0o2_000_00;
const STDOUT_FILENO: c_int = 1;
const EPERM: c_int = 1;
const NR_PIDFD_SEND_SIGNAL: c_long = -1;

unsafe extern "C" fn do_child(_args: *mut c_void) -> c_int {
    let format = b"%d\n\0";
    printf(format.as_ptr() as *const c_char, getpid());
    _exit(EXIT_SUCCESS);
}

unsafe fn pidfd_clone(flags: c_int, pidfd: *mut c_int) -> PidT {
    let stack_size: usize = 1024;
    let mut stack = [0i8; 1024];
    clone(
        do_child,
        stack.as_mut_ptr().add(stack_size) as *mut c_void,
        flags | SIGCHLD,
        std::ptr::null_mut::<c_void>(),
        pidfd,
    )
}

unsafe fn sys_pidfd_send_signal(
    pidfd: c_int,
    sig: c_int,
    info: *mut SiginfoT,
    flags: c_uint,
) -> c_int {
    syscall(NR_PIDFD_SEND_SIGNAL, pidfd, sig, info, flags) as c_int
}

unsafe fn pidfd_metadata_fd(pid: PidT, pidfd: c_int) -> c_int {
    let mut path = [0i8; 100];
    let prefix = b"/proc/\0";
    let mut pos = 0usize;
    for &byte in &prefix[..prefix.len() - 1] {
        path[pos] = byte as i8;
        pos += 1;
    }
    let mut n = pid;
    let mut digits = [0i8; 20];
    let mut count = 0usize;
    if n == 0 {
        digits[0] = b'0' as i8;
        count = 1;
    } else {
        while n > 0 {
            digits[count] = (b'0' as c_int + n % 10) as i8;
            n /= 10;
            count += 1;
        }
        for i in 0..count {
            path[pos + i] = digits[count - 1 - i];
        }
    }
    pos += count;
    path[pos] = 0;

    let procfd = open(path.as_ptr(), O_DIRECTORY | O_RDONLY | O_CLOEXEC);
    if procfd < 0 {
        warn(b"Failed to open %s\n\0".as_ptr() as *const c_char, path.as_ptr());
        return -1;
    }

    let ret = sys_pidfd_send_signal(pidfd, 0, std::ptr::null_mut(), 0);
    if ret < 0 {
        match *__errno_location() {
            EPERM => {}
            _ => {
                warn(b"Failed to signal process\n\0".as_ptr() as *const c_char);
                close(procfd);
                return -1;
            }
        }
    }
    procfd
}

pub unsafe fn main(_argc: c_int, _argv: *mut *mut c_char) -> ! {
    let mut pidfd = -1;
    let mut ret = EXIT_FAILURE;
    let mut buf = [0i8; 4096];

    let pid = pidfd_clone(CLONE_PIDFD, &mut pidfd);
    if pid < 0 {
        err(ret, b"CLONE_PIDFD\0".as_ptr() as *const c_char);
    }
    if pidfd == -1 {
        warnx(b"CLONE_PIDFD is not supported by the kernel\0".as_ptr() as *const c_char);
        wait(std::ptr::null_mut());
        exit(ret);
    }

    let procfd = pidfd_metadata_fd(pid, pidfd);
    close(pidfd);
    if procfd < 0 {
        wait(std::ptr::null_mut());
        exit(ret);
    }

    let statusfd = openat(procfd, b"status\0".as_ptr() as *const c_char, O_RDONLY | O_CLOEXEC);
    close(procfd);
    if statusfd < 0 {
        wait(std::ptr::null_mut());
        exit(ret);
    }

    let mut bytes = read(statusfd, buf.as_mut_ptr() as *mut c_void, buf.len());
    if bytes > 0 {
        bytes = write(statusfd - statusfd + STDOUT_FILENO, buf.as_ptr() as *const c_void, bytes as SizeT);
    }
    close(statusfd);
    ret = EXIT_SUCCESS;
    let _ = bytes;
    wait(std::ptr::null_mut());
    exit(ret);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
