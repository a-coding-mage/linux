// SPDX-License-Identifier: GPL-2.0
// External kernel and io_uring declarations are supplied by the surrounding build.

#[repr(C)]
pub struct io_nop {
    pub file: *mut file,
    pub result: i32,
    pub fd: i32,
    pub flags: u32,
    pub extra1: u64,
    pub extra2: u64,
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    pub nop_flags: u32,
    pub len: u32,
    pub fd: i32,
    pub buf_index: u16,
    pub off: u64,
    pub addr: u64,
}

#[repr(C)]
pub struct io_ring_ctx {
    pub flags: u32,
}

#[repr(C)]
pub struct io_task_work {
    pub func: Option<unsafe extern "C" fn(*mut io_kiocb, u32)>,
}

#[repr(C)]
pub struct io_kiocb {
    pub flags: u32,
    pub buf_index: u16,
    pub ctx: *mut io_ring_ctx,
    pub file: *mut file,
    pub io_task_work: io_task_work,
}

extern "C" {
    fn io_file_get_fixed(req: *mut io_kiocb, fd: i32, issue_flags: u32) -> *mut file;
    fn io_file_get_normal(req: *mut io_kiocb, fd: i32) -> *mut file;
    fn io_find_buf_node(req: *mut io_kiocb, issue_flags: u32) -> bool;
    fn req_set_fail(req: *mut io_kiocb);
    fn io_req_set_res32(req: *mut io_kiocb, res: i32, cflags: u32, extra1: u64, extra2: u64);
    fn io_req_set_res(req: *mut io_kiocb, res: i32, cflags: u32);
    fn io_req_task_complete(req: *mut io_kiocb, issue_flags: u32);
    fn io_req_task_work_add(req: *mut io_kiocb);
}

const NOP_FLAGS: u32 = IORING_NOP_INJECT_RESULT
    | IORING_NOP_FIXED_FILE
    | IORING_NOP_FIXED_BUFFER
    | IORING_NOP_FILE
    | IORING_NOP_TW
    | IORING_NOP_CQE32;

pub fn io_nop_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let nop = unsafe { &mut *(req as *mut io_nop) };

    nop.flags = unsafe { core::ptr::read_volatile(&(*sqe).nop_flags) };
    if nop.flags & !NOP_FLAGS != 0 {
        return -EINVAL;
    }

    if nop.flags & IORING_NOP_INJECT_RESULT != 0 {
        nop.result = unsafe { core::ptr::read_volatile(&(*sqe).len) as i32 };
    } else {
        nop.result = 0;
    }
    if nop.flags & IORING_NOP_FILE != 0 {
        nop.fd = unsafe { core::ptr::read_volatile(&(*sqe).fd) };
    } else {
        nop.fd = -1;
    }
    if nop.flags & IORING_NOP_FIXED_FILE != 0 {
        unsafe { (*req).flags |= REQ_F_FIXED_FILE };
    }
    if nop.flags & IORING_NOP_FIXED_BUFFER != 0 {
        unsafe { (*req).buf_index = core::ptr::read_volatile(&(*sqe).buf_index); }
    }
    if nop.flags & IORING_NOP_CQE32 != 0 {
        let ctx = unsafe { (*req).ctx };
        if unsafe { (*ctx).flags & (IORING_SETUP_CQE32 | IORING_SETUP_CQE_MIXED) } == 0 {
            return -EINVAL;
        }
        nop.extra1 = unsafe { core::ptr::read_volatile(&(*sqe).off) };
        nop.extra2 = unsafe { core::ptr::read_volatile(&(*sqe).addr) };
    }
    0
}

pub fn io_nop(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let nop = unsafe { &mut *(req as *mut io_nop) };
    let mut ret = nop.result;

    if nop.flags & IORING_NOP_FILE != 0 {
        unsafe {
            if (*req).flags & REQ_F_FIXED_FILE != 0 {
                (*req).file = io_file_get_fixed(req, nop.fd, issue_flags);
            } else {
                (*req).file = io_file_get_normal(req, nop.fd);
            }
            if (*req).file.is_null() {
                ret = -EBADF;
                return io_nop_done(req, nop, ret);
            }
        }
    }
    if nop.flags & IORING_NOP_FIXED_BUFFER != 0 {
        if unsafe { !io_find_buf_node(req, issue_flags) } {
            ret = -EFAULT;
        }
    }
    io_nop_done(req, nop, ret)
}

unsafe fn io_nop_done(req: *mut io_kiocb, nop: &io_nop, ret: i32) -> i32 {
    if ret < 0 {
        req_set_fail(req);
    }
    if nop.flags & IORING_NOP_CQE32 != 0 {
        io_req_set_res32(req, ret, 0, nop.extra1, nop.extra2);
    } else {
        io_req_set_res(req, ret, 0);
    }
    if nop.flags & IORING_NOP_TW != 0 {
        (*req).io_task_work.func = Some(io_req_task_complete);
        io_req_task_work_add(req);
        return IOU_ISSUE_SKIP_COMPLETE;
    }
    IOU_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
