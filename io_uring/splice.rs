// SPDX-License-Identifier: GPL-2.0
// Translated from splice.c. Kernel and local header dependencies are supplied
// by the surrounding repository.

#[repr(C)]
pub struct io_splice {
    pub file_out: *mut file,
    pub off_out: loff_t,
    pub off_in: loff_t,
    pub len: u64,
    pub splice_fd_in: i32,
    pub flags: u32,
    pub rsrc_node: *mut io_rsrc_node,
}

extern "C" {
    fn io_kiocb_to_cmd(req: *mut io_kiocb) -> *mut io_splice;
    fn io_file_get_normal(req: *mut io_kiocb, fd: i32) -> *mut file;
    fn io_ring_submit_lock(ctx: *mut io_ring_ctx, issue_flags: u32);
    fn io_rsrc_node_lookup(table: *mut file_table_data, fd: u32) -> *mut io_rsrc_node;
    fn io_slot_file(node: *mut io_rsrc_node) -> *mut file;
    fn io_ring_submit_unlock(ctx: *mut io_ring_ctx, issue_flags: u32);
    fn io_put_rsrc_node(ctx: *mut io_ring_ctx, node: *mut io_rsrc_node);
    fn do_tee(input: *mut file, output: *mut file, len: u64, flags: u32) -> isize;
    fn do_splice(
        input: *mut file,
        off_in: *mut loff_t,
        output: *mut file,
        off_out: *mut loff_t,
        len: u64,
        flags: u32,
    ) -> isize;
    fn fput(file: *mut file);
    fn req_set_fail(req: *mut io_kiocb);
    fn io_req_set_res(req: *mut io_kiocb, res: isize, flags: u32);
    fn warn_on_once(condition: bool);
}

// External kernel types and constants are declared by the included headers.
#[allow(non_camel_case_types)]
pub type loff_t = i64;
pub enum file {}
pub enum io_kiocb {}
pub enum io_uring_sqe {}
pub enum io_ring_ctx {}
pub enum io_rsrc_node {}
pub enum file_table_data {}

const SPLICE_F_FD_IN_FIXED: u32 = 1 << 31;
const REQ_F_FORCE_ASYNC: u32 = 1 << 0;
const REQ_F_NEED_CLEANUP: u32 = 1 << 1;
const IO_URING_F_NONBLOCK: u32 = 1 << 0;
const IOU_COMPLETE: i32 = 0;
const EINVAL: isize = 22;
const EBADF: isize = 9;

// SPLICE_F_ALL is provided by the kernel splice interface.
const SPLICE_F_ALL: u32 = 0xffff_ffff;

unsafe fn __io_splice_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let sp = io_kiocb_to_cmd(req);
    let valid_flags = SPLICE_F_FD_IN_FIXED | SPLICE_F_ALL;

    (*sp).len = (*(sqe as *const io_uring_sqe_fields)).len;
    (*sp).flags = (*(sqe as *const io_uring_sqe_fields)).splice_flags;
    if ((*sp).flags & !valid_flags) != 0 {
        return -(EINVAL as i32);
    }
    (*sp).splice_fd_in = (*(sqe as *const io_uring_sqe_fields)).splice_fd_in;
    (*sp).rsrc_node = core::ptr::null_mut();
    (*(req as *mut io_kiocb_fields)).flags |= REQ_F_FORCE_ASYNC;
    0
}

#[repr(C)]
struct io_uring_sqe_fields {
    _pad0: [u8; 24],
    off: loff_t,
    len: u64,
    _pad1: [u8; 8],
    splice_off_in: loff_t,
    splice_flags: u32,
    splice_fd_in: i32,
}

#[repr(C)]
struct io_kiocb_fields {
    flags: u32,
    _rest: [u8; 0],
}

pub unsafe fn io_tee_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let s = sqe as *const io_uring_sqe_fields;
    if (*s).splice_off_in != 0 || (*s).off != 0 {
        return -(EINVAL as i32);
    }
    __io_splice_prep(req, sqe)
}

pub unsafe fn io_splice_cleanup(req: *mut io_kiocb) {
    let sp = io_kiocb_to_cmd(req);
    if !(*sp).rsrc_node.is_null() {
        io_put_rsrc_node((*(req as *mut io_kiocb_context)).ctx, (*sp).rsrc_node);
    }
}

#[repr(C)] struct io_kiocb_context { _prefix: [u8; 0], ctx: *mut io_ring_ctx }

unsafe fn io_splice_get_file(req: *mut io_kiocb, issue_flags: u32) -> *mut file {
    let sp = io_kiocb_to_cmd(req);
    let ctx = (*(req as *mut io_kiocb_context)).ctx;
    if ((*sp).flags & SPLICE_F_FD_IN_FIXED) == 0 {
        return io_file_get_normal(req, (*sp).splice_fd_in);
    }
    io_ring_submit_lock(ctx, issue_flags);
    let node = io_rsrc_node_lookup(core::ptr::null_mut(), (*sp).splice_fd_in as u32);
    let mut file = core::ptr::null_mut();
    if !node.is_null() {
        (*sp).rsrc_node = node;
        file = io_slot_file(node);
        (*(req as *mut io_kiocb_fields)).flags |= REQ_F_NEED_CLEANUP;
    }
    io_ring_submit_unlock(ctx, issue_flags);
    file
}

pub unsafe fn io_tee(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let sp = io_kiocb_to_cmd(req);
    let out = (*sp).file_out;
    let flags = (*sp).flags & !SPLICE_F_FD_IN_FIXED;
    let input = io_splice_get_file(req, issue_flags);
    let mut ret: isize = 0;
    warn_on_once((issue_flags & IO_URING_F_NONBLOCK) != 0);
    if input.is_null() { ret = -EBADF; } else {
        if (*sp).len != 0 { ret = do_tee(input, out, (*sp).len, flags); }
        if ((*sp).flags & SPLICE_F_FD_IN_FIXED) == 0 { fput(input); }
    }
    if ret != (*sp).len as isize { req_set_fail(req); }
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

pub unsafe fn io_splice_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let sp = io_kiocb_to_cmd(req);
    let s = sqe as *const io_uring_sqe_fields;
    (*sp).off_in = (*s).splice_off_in;
    (*sp).off_out = (*s).off;
    __io_splice_prep(req, sqe)
}

pub unsafe fn io_splice(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let sp = io_kiocb_to_cmd(req);
    let out = (*sp).file_out;
    let flags = (*sp).flags & !SPLICE_F_FD_IN_FIXED;
    let input = io_splice_get_file(req, issue_flags);
    let mut ret: isize = 0;
    warn_on_once((issue_flags & IO_URING_F_NONBLOCK) != 0);
    if input.is_null() { ret = -EBADF; } else {
        let mut off_in = (*sp).off_in;
        let mut off_out = (*sp).off_out;
        let pi = if off_in == -1 { core::ptr::null_mut() } else { &mut off_in };
        let po = if off_out == -1 { core::ptr::null_mut() } else { &mut off_out };
        if (*sp).len != 0 { ret = do_splice(input, pi, out, po, (*sp).len, flags); }
        if ((*sp).flags & SPLICE_F_FD_IN_FIXED) == 0 { fput(input); }
    }
    if ret != (*sp).len as isize { req_set_fail(req); }
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
