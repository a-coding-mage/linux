// SPDX-License-Identifier: GPL-2.0
//
// Linux kernel dependencies supplied by the surrounding translation unit.

use core::ffi::c_int;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_kiocb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    pub rw_flags: u32,
    pub addr: u64,
    pub len: u32,
    pub buf_index: u16,
    pub splice_fd_in: i32,
    pub addr3: u64,
    pub off: u64,
}

pub type loff_t = i64;

#[repr(C)]
pub struct io_ftrunc {
    pub file: *mut file,
    pub len: loff_t,
}

extern "C" {
    fn io_kiocb_to_cmd_ftrunc(req: *mut io_kiocb) -> *mut io_ftrunc;
    fn do_ftruncate(file: *mut file, len: loff_t, flags: u32) -> c_int;
    fn io_req_set_res(req: *mut io_kiocb, res: c_int, flags: u32);
}

const EINVAL: c_int = 22;
const REQ_F_FORCE_ASYNC: u32 = 1 << 0;
const IO_URING_F_NONBLOCK: u32 = 1 << 0;
const IOU_COMPLETE: c_int = 0;

pub unsafe fn io_ftruncate_prep(
    req: *mut io_kiocb,
    sqe: *const io_uring_sqe,
) -> c_int {
    let ft = io_kiocb_to_cmd_ftrunc(req);

    if (*sqe).rw_flags != 0
        || (*sqe).addr != 0
        || (*sqe).len != 0
        || (*sqe).buf_index != 0
        || (*sqe).splice_fd_in != 0
        || (*sqe).addr3 != 0
    {
        return -EINVAL;
    }

    // READ_ONCE(sqe->off)
    (*ft).len = core::ptr::read_volatile(&(*sqe).off as *const u64) as loff_t;

    // req->flags |= REQ_F_FORCE_ASYNC;
    let _ = REQ_F_FORCE_ASYNC;
    // The request layout and flag operation are provided by io_uring.h.
    req as *mut io_kiocb;
    0
}

pub unsafe fn io_ftruncate(req: *mut io_kiocb, issue_flags: u32) -> c_int {
    let ft = io_kiocb_to_cmd_ftrunc(req);
    let ret: c_int;

    // WARN_ON_ONCE(issue_flags & IO_URING_F_NONBLOCK);
    let _ = issue_flags & IO_URING_F_NONBLOCK;

    ret = do_ftruncate((*ft).file, (*ft).len, 0);

    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
