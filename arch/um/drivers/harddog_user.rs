// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct dog_data {
    stdin_fd: c_int,
    stdout_fd: c_int,
    close_me: [c_int; 2],
}

extern "C" {
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    static mut errno: c_int;

    fn os_pipe(fds: *mut c_int, close_on_exec: c_int, stream: c_int) -> c_int;
    fn printk(format: *const c_char, ...);
    fn os_getpid() -> c_int;
    fn run_helper(
        pre_exec: Option<unsafe extern "C" fn(*mut c_void)>,
        arg: *mut c_void,
        argv: *mut *mut c_char,
    ) -> c_int;
    fn helper_wait(pid: c_int);
}

unsafe extern "C" fn pre_exec(d: *mut c_void) {
    let data = &*(d as *mut dog_data);

    dup2(data.stdin_fd, 0);
    dup2(data.stdout_fd, 1);
    dup2(data.stdout_fd, 2);
    close(data.stdin_fd);
    close(data.stdout_fd);
    close(data.close_me[0]);
    close(data.close_me[1]);
}

#[no_mangle]
pub unsafe extern "C" fn start_watchdog(
    in_fd_ret: *mut c_int,
    out_fd_ret: *mut c_int,
    sock: *mut c_char,
) -> c_int {
    let mut data: dog_data = core::mem::zeroed();
    let mut in_fds = [0 as c_int; 2];
    let mut out_fds = [0 as c_int; 2];
    let mut pid: c_int;
    let mut n: isize;
    let mut err: c_int;
    let mut pid_buf = [0 as c_char; 8];
    let mut c: c_char = 0;
    let mut pid_args: [*mut c_char; 4] = [
        b"/usr/bin/uml_watchdog\0".as_ptr() as *mut c_char,
        b"-pid\0".as_ptr() as *mut c_char,
        pid_buf.as_mut_ptr(),
        core::ptr::null_mut(),
    ];
    let mut mconsole_args: [*mut c_char; 4] = [
        b"/usr/bin/uml_watchdog\0".as_ptr() as *mut c_char,
        b"-mconsole\0".as_ptr() as *mut c_char,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    ];
    let mut args: *mut *mut c_char = core::ptr::null_mut();

    err = os_pipe(in_fds.as_mut_ptr(), 1, 0);
    if err < 0 {
        printk(b"harddog_open - os_pipe failed, err = %d\n\0".as_ptr() as *const c_char, -err);
        return err;
    }

    err = os_pipe(out_fds.as_mut_ptr(), 1, 0);
    if err < 0 {
        printk(b"harddog_open - os_pipe failed, err = %d\n\0".as_ptr() as *const c_char, -err);
        close(in_fds[0]);
        close(in_fds[1]);
        return err;
    }

    data.stdin_fd = out_fds[0];
    data.stdout_fd = in_fds[1];
    data.close_me[0] = out_fds[1];
    data.close_me[1] = in_fds[0];

    if !sock.is_null() {
        mconsole_args[2] = sock;
        args = mconsole_args.as_mut_ptr();
    } else {
        // XXX The os_getpid() is not SMP correct
        sprintf(pid_buf.as_mut_ptr(), b"%d\0".as_ptr() as *const c_char, os_getpid());
        args = pid_args.as_mut_ptr();
    }

    pid = run_helper(Some(pre_exec), &mut data as *mut _ as *mut c_void, args);

    close(out_fds[0]);
    close(in_fds[1]);

    if pid < 0 {
        err = -pid;
        printk(b"harddog_open - run_helper failed, errno = %d\n\0".as_ptr() as *const c_char, -err);
        close(out_fds[0]);
        close(out_fds[1]);
        return err;
    }

    n = read(in_fds[0], &mut c as *mut c_char as *mut c_void, core::mem::size_of::<c_char>());
    if n == 0 {
        printk(b"harddog_open - EOF on watchdog pipe\n\0".as_ptr() as *const c_char);
        helper_wait(pid);
        err = -5; // -EIO
        close(out_fds[0]);
        close(out_fds[1]);
        return err;
    } else if n < 0 {
        printk(b"harddog_open - read of watchdog pipe failed, err = %d\n\0".as_ptr() as *const c_char, errno);
        helper_wait(pid);
        err = n as c_int;
        close(out_fds[0]);
        close(out_fds[1]);
        return err;
    }
    *in_fd_ret = in_fds[0];
    *out_fd_ret = out_fds[1];
    0
}

#[no_mangle]
pub unsafe extern "C" fn stop_watchdog(in_fd: c_int, out_fd: c_int) {
    close(in_fd);
    close(out_fd);
}

#[no_mangle]
pub unsafe extern "C" fn ping_watchdog(fd: c_int) -> c_int {
    let c: c_char = b'\n' as c_char;
    let n = write(fd, &c as *const c_char as *const c_void, core::mem::size_of::<c_char>());
    if n != core::mem::size_of::<c_char>() as isize {
        printk(b"ping_watchdog - write failed, ret = %d, err = %d\n\0".as_ptr() as *const c_char, n as c_int, errno);
        if n < 0 {
            return n as c_int;
        }
        return -5; // -EIO
    }
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
