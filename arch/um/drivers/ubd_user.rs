// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2016 Anton Ivanov (aivanov@brocade.com)
 * Copyright (C) 2000, 2001, 2002 Jeff Dike (jdike@karaya.com)
 * Copyright (C) 2001 Ridgerun,Inc (glonnon@ridgerun.com)
 */

// C dependencies supplied by the surrounding repository:
// <stddef.h>, <unistd.h>, <errno.h>, <sched.h>, <signal.h>, <string.h>,
// <netinet/in.h>, <sys/time.h>, <sys/socket.h>, <sys/mman.h>, <sys/param.h>,
// <endian.h>, <byteswap.h>, "ubd.h", <os.h>, and <poll.h>.

use core::ffi::{c_int, c_short, c_void};

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
pub struct os_helper_thread {
    _private: [u8; 0],
}

extern "C" {
    static mut kernel_fd: c_int;

    fn os_pipe(fds: *mut c_int, close_on_exec: c_int, nonblocking: c_int) -> c_int;
    fn printk(fmt: *const u8, ...);
    fn os_set_fd_block(fd: c_int, block: c_int) -> c_int;
    fn os_run_helper_thread(
        td_out: *mut *mut os_helper_thread,
        thread: unsafe extern "C" fn(*mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn os_close_file(fd: c_int);
    fn io_thread(arg: *mut c_void) -> c_int;
    fn poll(fds: *mut pollfd, nfds: usize, timeout: c_int) -> c_int;
}

const POLLIN: c_short = 0x0001;
const POLLOUT: c_short = 0x0004;

static mut kernel_pollfd: pollfd = pollfd {
    fd: 0,
    events: 0,
    revents: 0,
};

pub unsafe fn start_io_thread(
    td_out: *mut *mut os_helper_thread,
    fd_out: *mut c_int,
) -> c_int {
    let mut fds = [0 as c_int; 2];
    let mut err: c_int;

    err = os_pipe(fds.as_mut_ptr(), 1, 1);
    if err < 0 {
        printk(b"start_io_thread - os_pipe failed, err = %d\n\0".as_ptr(), -err);
        return err;
    }

    kernel_fd = fds[0];
    kernel_pollfd.fd = kernel_fd;
    kernel_pollfd.events = POLLIN;
    *fd_out = fds[1];

    err = os_set_fd_block(*fd_out, 0);
    err |= os_set_fd_block(kernel_fd, 0);
    if err != 0 {
        printk(b"start_io_thread - failed to set nonblocking I/O.\n\0".as_ptr());
        os_close_file(fds[0]);
        os_close_file(fds[1]);
        kernel_fd = -1;
        *fd_out = -1;
        return err;
    }

    err = os_run_helper_thread(td_out, io_thread, core::ptr::null_mut());
    if err < 0 {
        printk(b"%s - failed to run helper thread, err = %d\n\0".as_ptr(),
               b"start_io_thread\0".as_ptr(), -err);
        os_close_file(fds[0]);
        os_close_file(fds[1]);
        kernel_fd = -1;
        *fd_out = -1;
        return err;
    }

    0
}

pub unsafe fn ubd_read_poll(timeout: c_int) -> c_int {
    kernel_pollfd.events = POLLIN;
    poll(&mut kernel_pollfd, 1, timeout)
}

pub unsafe fn ubd_write_poll(timeout: c_int) -> c_int {
    kernel_pollfd.events = POLLOUT;
    poll(&mut kernel_pollfd, 1, timeout)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
