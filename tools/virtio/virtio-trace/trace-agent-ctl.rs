// SPDX-License-Identifier: GPL-2.0-only
/*
 * Controller of read/write threads for virtio-trace
 *
 * Copyright (C) 2012 Hitachi, Ltd.
 * Created by Yoshihiro Yunomae <yoshihiro.yunomae.ez@hitachi.com>
 *            Masami Hiramatsu <masami.hiramatsu.pt@hitachi.com>
 */

// C dependencies from:
// <fcntl.h>, <poll.h>, <signal.h>, <stdio.h>, <stdlib.h>, <unistd.h>,
// and "trace-agent.h".

use core::ffi::{c_char, c_int, c_short, c_ulong, c_void};

const HOST_MSG_SIZE: usize = 256;
const EVENT_WAIT_MSEC: c_int = 100;

const O_RDONLY: c_int = 0;
const POLLIN: c_short = 0x001;
const SIGINT: c_int = 2;
const SIGQUIT: c_int = 3;
const SIGTERM: c_int = 15;
const EXIT_FAILURE: c_int = 1;

type SigAtomicT = c_int;
type SSizeT = isize;
type SizeT = usize;
type NFdsT = c_ulong;

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
pub struct pthread_cond_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut cond_wakeup: pthread_cond_t;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn poll(fds: *mut pollfd, nfds: NFdsT, timeout: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: SizeT) -> SSizeT;
    fn exit(status: c_int) -> !;
    fn signal(
        signum: c_int,
        handler: Option<unsafe extern "C" fn(c_int)>,
    ) -> Option<unsafe extern "C" fn(c_int)>;
    fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> c_int;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

static mut global_signal_val: SigAtomicT = 0;
pub static mut global_sig_receive: bool = false; /* default false */
pub static mut global_run_operation: bool = false; /* default false*/

/* Handle SIGTERM/SIGINT/SIGQUIT to exit */
unsafe extern "C" fn signal_handler(sig: c_int) {
    global_signal_val = sig;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rw_ctl_init(ctl_path: *const c_char) -> c_int {
    let ctl_fd: c_int;

    ctl_fd = open(ctl_path, O_RDONLY);
    if ctl_fd == -1 {
        pr_err(c"Cannot open ctl_fd\n".as_ptr());
        exit(EXIT_FAILURE);
    }

    ctl_fd
}

unsafe fn wait_order(ctl_fd: c_int) -> c_int {
    let mut poll_fd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut ret: c_int = 0;

    while !global_sig_receive {
        poll_fd.fd = ctl_fd;
        poll_fd.events = POLLIN;

        ret = poll(&mut poll_fd, 1, EVENT_WAIT_MSEC);

        if global_signal_val != 0 {
            global_sig_receive = true;
            pr_info(c"Receive interrupt %d\n".as_ptr(), global_signal_val);

            /* Wakes rw-threads when they are sleeping */
            if !global_run_operation {
                pthread_cond_broadcast(&raw mut cond_wakeup);
            }

            ret = -1;
            break;
        }

        if ret < 0 {
            pr_err(c"Polling error\n".as_ptr());
            exit(EXIT_FAILURE);
        }

        if ret != 0 {
            break;
        }
    }

    ret
}

/*
 * control read/write threads by handling global_run_operation
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rw_ctl_loop(ctl_fd: c_int) -> *mut c_void {
    let mut rlen: SSizeT;
    let mut buf: [c_char; HOST_MSG_SIZE] = [0; HOST_MSG_SIZE];
    let mut ret: c_int;

    /* Setup signal handlers */
    signal(SIGTERM, Some(signal_handler));
    signal(SIGINT, Some(signal_handler));
    signal(SIGQUIT, Some(signal_handler));

    while !global_sig_receive {
        ret = wait_order(ctl_fd);
        if ret < 0 {
            break;
        }

        rlen = read(ctl_fd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf));
        if rlen < 0 {
            pr_err(c"read data error in ctl thread\n".as_ptr());
            exit(EXIT_FAILURE);
        }

        if rlen == 2 && buf[0] == b'1' as c_char {
            /*
             * If host writes '1' to a control path,
             * this controller wakes all read/write threads.
             */
            global_run_operation = true;
            pthread_cond_broadcast(&raw mut cond_wakeup);
            pr_debug(c"Wake up all read/write threads\n".as_ptr());
        } else if rlen == 2 && buf[0] == b'0' as c_char {
            /*
             * If host writes '0' to a control path, read/write
             * threads will wait for notification from Host.
             */
            global_run_operation = false;
            pr_debug(c"Stop all read/write threads\n".as_ptr());
        } else {
            pr_info(c"Invalid host notification: %s\n".as_ptr(), buf.as_ptr());
        }
    }

    core::ptr::null_mut()
}
