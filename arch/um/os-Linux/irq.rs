// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017 - Cambridge Greys Ltd
 * Copyright (C) 2011 - 2014 Cisco Systems Inc
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C dependencies supplied by the surrounding userspace implementation.

use core::ffi::c_void;

const MAX_EPOLL_EVENTS: usize = 64;

const EPOLLIN: i32 = 0x001;
const EPOLLPRI: i32 = 0x002;
const EPOLLOUT: i32 = 0x004;
const EPOLLERR: i32 = 0x008;
const EPOLLHUP: i32 = 0x010;
const EPOLLRDHUP: i32 = 0x2000;
const EPOLLET: i32 = 1 << 31;
const EPOLL_CTL_ADD: i32 = 1;
const EPOLL_CTL_DEL: i32 = 2;
const EPOLL_CTL_MOD: i32 = 3;
const SIGIO: i32 = 29;
const SIG_IGN: usize = 1;
const EINTR: i32 = 4;
const EEXIST: i32 = 17;

#[repr(C)]
pub union EpollData {
    pub ptr: *mut c_void,
    pub fd: i32,
    pub u32: u32,
    pub u64: u64,
}

#[repr(C)]
pub struct EpollEvent {
    pub events: u32,
    pub data: EpollData,
}

#[repr(C)]
pub enum UmIrqType {}

// These enum values are supplied by irq_user.h.
extern "C" {
    pub static mut errno: i32;
    pub fn epoll_create(size: i32) -> i32;
    pub fn epoll_wait(epfd: i32, events: *mut EpollEvent, maxevents: i32, timeout: i32) -> i32;
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut EpollEvent) -> i32;
    pub fn strerror(errnum: i32) -> *const i8;
    pub fn signal(signum: i32, handler: usize) -> usize;
    pub fn printk(fmt: *const i8, ...);
    pub fn os_close_file(fd: i32);
}

extern "C" {
    pub const IRQ_READ: UmIrqType;
    pub const IRQ_WRITE: UmIrqType;
}

static mut EPOLLFd: i32 = -1;
static mut EPOLL_EVENTS: [EpollEvent; MAX_EPOLL_EVENTS] = unsafe { core::mem::zeroed() };

pub unsafe extern "C" fn os_epoll_get_data_pointer(index: i32) -> *mut c_void {
    EPOLL_EVENTS[index as usize].data.ptr
}

pub unsafe extern "C" fn os_epoll_triggered(index: i32, events: i32) -> i32 {
    (EPOLL_EVENTS[index as usize].events as i32) & events
}

pub unsafe extern "C" fn os_event_mask(irq_type: UmIrqType) -> i32 {
    if core::ptr::eq(&irq_type, &IRQ_READ) {
        return EPOLLIN | EPOLLPRI | EPOLLERR | EPOLLHUP | EPOLLRDHUP;
    }
    if core::ptr::eq(&irq_type, &IRQ_WRITE) {
        return EPOLLOUT;
    }
    0
}

pub unsafe extern "C" fn os_setup_epoll() -> i32 {
    EPOLLFd = epoll_create(MAX_EPOLL_EVENTS as i32);
    EPOLLFd
}

pub unsafe extern "C" fn os_waiting_for_events_epoll() -> i32 {
    let n = epoll_wait(EPOLLFd, EPOLL_EVENTS.as_mut_ptr(), MAX_EPOLL_EVENTS as i32, 0);
    if n < 0 {
        let err = -errno;
        if errno != EINTR {
            printk(core::ptr::null());
        }
        return err;
    }
    n
}

pub unsafe extern "C" fn os_add_epoll_fd(events: i32, fd: i32, data: *mut c_void) -> i32 {
    let mut event = EpollEvent {
        events: (events | EPOLLET) as u32,
        data: EpollData { ptr: data },
    };
    let mut result = epoll_ctl(EPOLLFd, EPOLL_CTL_ADD, fd, &mut event);
    if result != 0 && errno == EEXIST {
        result = os_mod_epoll_fd(events, fd, data);
    }
    result
}

pub unsafe extern "C" fn os_mod_epoll_fd(events: i32, fd: i32, data: *mut c_void) -> i32 {
    let mut event = EpollEvent {
        events: events as u32,
        data: EpollData { ptr: data },
    };
    epoll_ctl(EPOLLFd, EPOLL_CTL_MOD, fd, &mut event)
}

pub unsafe extern "C" fn os_del_epoll_fd(fd: i32) -> i32 {
    let mut event = core::mem::MaybeUninit::<EpollEvent>::uninit();
    epoll_ctl(EPOLLFd, EPOLL_CTL_DEL, fd, event.as_mut_ptr())
}

pub unsafe extern "C" fn os_set_ioignore() {
    signal(SIGIO, SIG_IGN);
}

pub unsafe extern "C" fn os_close_epoll_fd() {
    os_close_file(EPOLLFd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
