// SPDX-License-Identifier: GPL-2.0
// Kernel headers and local headers from the C source provide the external
// types, constants, macros, and functions referenced below.

use core::ffi::c_int;

#[repr(C)]
pub struct io_epoll {
    pub file: *mut file,
    pub epfd: c_int,
    pub op: c_int,
    pub fd: c_int,
    pub event: epoll_event,
}

#[repr(C)]
pub struct io_epoll_wait {
    pub file: *mut file,
    pub maxevents: c_int,
    pub events: *mut epoll_event,
}

// External declarations supplied by the included kernel and io_uring headers.
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct epoll_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_kiocb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    pub buf_index: u16,
    pub splice_fd_in: u16,
    pub fd: i32,
    pub len: u32,
    pub off: u64,
    pub addr: u64,
    pub rw_flags: u32,
}

#[repr(C)]
pub struct epoll_key {
    pub file: *mut file,
    pub fd: c_int,
}

unsafe extern "C" {
    fn io_kiocb_to_cmd_epoll(req: *mut io_kiocb) -> *mut io_epoll;
    fn io_kiocb_to_cmd_epoll_wait(req: *mut io_kiocb) -> *mut io_epoll_wait;
    fn read_once_i32(ptr: *const i32) -> i32;
    fn read_once_u32(ptr: *const u32) -> u32;
    fn read_once_u64(ptr: *const u64) -> u64;
    fn copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn ep_op_has_event(op: c_int) -> bool;
    fn do_epoll_ctl_file(
        file: *mut file,
        op: c_int,
        key: *mut epoll_key,
        event: *mut epoll_event,
        force_nonblock: bool,
    ) -> c_int;
    fn epoll_sendevents(req_file: *mut file, events: *mut epoll_event, maxevents: c_int) -> c_int;
    fn req_set_fail(req: *mut io_kiocb);
    fn io_req_set_res(req: *mut io_kiocb, res: c_int, flags: u32);
    fn fd_file(fd: c_int) -> *mut file;
    fn fd_empty(fd: c_int) -> bool;
}

const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const EBADF: c_int = 9;
const EAGAIN: c_int = 11;
const IO_URING_F_NONBLOCK: u32 = 1 << 0;
const IOU_COMPLETE: c_int = 0;

pub unsafe fn io_epoll_ctl_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int {
    let epoll = &mut *io_kiocb_to_cmd_epoll(req);

    if (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 {
        return -EINVAL;
    }

    epoll.epfd = read_once_i32(&(*sqe).fd);
    epoll.op = read_once_u32(&(*sqe).len) as c_int;
    epoll.fd = read_once_u64(&(*sqe).off) as c_int;

    if ep_op_has_event(epoll.op) {
        let ev = read_once_u64(&(*sqe).addr) as *mut epoll_event;
        if copy_from_user(
            &mut epoll.event as *mut epoll_event as *mut core::ffi::c_void,
            ev as *const core::ffi::c_void,
            core::mem::size_of::<epoll_event>(),
        ) != 0 {
            return -EFAULT;
        }
    }

    0
}

pub unsafe fn io_epoll_ctl(req: *mut io_kiocb, issue_flags: u32) -> c_int {
    let ie = &mut *io_kiocb_to_cmd_epoll(req);
    let force_nonblock = issue_flags & IO_URING_F_NONBLOCK != 0;
    let mut key: epoll_key;

    if fd_empty(ie.epfd) {
        return -EBADF;
    }
    if fd_empty(ie.fd) {
        return -EBADF;
    }

    key = epoll_key { file: fd_file(ie.fd), fd: ie.fd };
    let ret = do_epoll_ctl_file(fd_file(ie.epfd), ie.op, &mut key, &mut ie.event, force_nonblock);
    if force_nonblock && ret == -EAGAIN {
        return -EAGAIN;
    }

    if ret < 0 {
        req_set_fail(req);
    }
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

pub unsafe fn io_epoll_wait_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int {
    let iew = &mut *io_kiocb_to_cmd_epoll_wait(req);

    if (*sqe).off != 0 || (*sqe).rw_flags != 0 || (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 {
        return -EINVAL;
    }

    iew.maxevents = read_once_u32(&(*sqe).len) as c_int;
    iew.events = read_once_u64(&(*sqe).addr) as *mut epoll_event;
    0
}

pub unsafe fn io_epoll_wait(req: *mut io_kiocb, _issue_flags: u32) -> c_int {
    let iew = &mut *io_kiocb_to_cmd_epoll_wait(req);
    let ret = epoll_sendevents((*req).file, iew.events, iew.maxevents);
    if ret == 0 {
        return -EAGAIN;
    }
    if ret < 0 {
        req_set_fail(req);
    }
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
