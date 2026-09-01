// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>
// C dependencies: stdlib.h, errno.h, unistd.h, signal.h, sys/epoll.h,
// "mainloop.h", "log.h"

use core::ffi::{c_int, c_uint, c_void};
use core::mem;

pub type mainloop_callback_t = Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>;

type sig_atomic_t = c_int;

const EPOLLIN: u32 = 0x001;
const EPOLL_CTL_ADD: c_int = 1;
const EPOLL_CTL_DEL: c_int = 2;
const EINTR: c_int = 4;
const MAX_EVENTS: usize = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub union epoll_data_t {
    pub ptr: *mut c_void,
    pub fd: c_int,
    pub u32_: u32,
    pub u64_: u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct epoll_event {
    pub events: u32,
    pub data: epoll_data_t,
}

#[repr(C)]
struct mainloop_data {
    cb: mainloop_callback_t,
    data: *mut c_void,
    fd: c_int,
}

static mut epfd: c_int = -1;
static mut exit_mainloop: sig_atomic_t = 0;

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn epoll_create(size: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;
    fn __errno_location() -> *mut c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainloop(timeout: c_uint) -> c_int {
    let mut i: c_int;
    let mut nfds: c_int;
    let mut events: [epoll_event; MAX_EVENTS] = mem::zeroed();
    let mut md: *mut mainloop_data;

    if epfd < 0 {
        return -1;
    }

    loop {
        nfds = epoll_wait(
            epfd,
            events.as_mut_ptr(),
            MAX_EVENTS as c_int,
            timeout as c_int,
        );

        if exit_mainloop != 0 || nfds == 0 {
            return 0;
        }

        if nfds < 0 {
            if *__errno_location() == EINTR {
                continue;
            }
            return -1;
        }

        i = 0;
        while i < nfds {
            md = events[i as usize].data.ptr as *mut mainloop_data;

            if ((*md).cb.unwrap())((*md).fd, (*md).data) > 0 {
                return 0;
            }

            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainloop_add(
    fd: c_int,
    cb: mainloop_callback_t,
    data: *mut c_void,
) -> c_int {
    let mut ev = epoll_event {
        events: EPOLLIN,
        data: epoll_data_t {
            ptr: core::ptr::null_mut(),
        },
    };

    let md: *mut mainloop_data;

    md = malloc(mem::size_of::<mainloop_data>()) as *mut mainloop_data;
    if md.is_null() {
        return -1;
    }

    (*md).data = data;
    (*md).cb = cb;
    (*md).fd = fd;

    ev.data.ptr = md as *mut c_void;

    if epoll_ctl(epfd, EPOLL_CTL_ADD, fd, &mut ev) < 0 {
        free(md as *mut c_void);
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainloop_del(fd: c_int) -> c_int {
    if epoll_ctl(epfd, EPOLL_CTL_DEL, fd, core::ptr::null_mut()) < 0 {
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainloop_init() -> c_int {
    epfd = epoll_create(2);
    if epfd < 0 {
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainloop_exit() {
    exit_mainloop = 1;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainloop_fini() {
    close(epfd);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
