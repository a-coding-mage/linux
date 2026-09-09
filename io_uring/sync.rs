// SPDX-License-Identifier: GPL-2.0
// Translated from the corresponding Linux kernel C implementation.

// Dependencies supplied by the surrounding kernel/io_uring translation unit.

use core::ffi::c_int;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_sqe {
    pub addr: u64,
    pub off: i64,
    pub len: u64,
    pub buf_index: u16,
    pub splice_fd_in: i32,
    pub sync_range_flags: u32,
    pub fsync_flags: u32,
    pub rw_flags: u32,
}

#[repr(C)]
pub struct io_kiocb {
    pub file: *mut file,
    pub flags: u32,
}

#[repr(C)]
pub struct io_sync {
    pub file: *mut file,
    pub len: i64,
    pub off: i64,
    pub flags: c_int,
    pub mode: c_int,
}

pub const REQ_F_FORCE_ASYNC: u32 = 1 << 0;
pub const IO_URING_F_NONBLOCK: u32 = 1 << 0;
pub const IORING_FSYNC_DATASYNC: c_int = 1;
pub const IOU_COMPLETE: c_int = 0;
pub const EINVAL: c_int = 22;
pub const LLONG_MAX: i64 = i64::MAX;

extern "C" {
    fn sync_file_range(file: *mut file, off: i64, len: i64, flags: c_int) -> c_int;
    fn vfs_fsync_range(file: *mut file, start: i64, end: i64, datasync: c_int) -> c_int;
    fn vfs_fallocate(file: *mut file, mode: c_int, offset: i64, len: i64) -> c_int;
    fn fsnotify_modify(file: *mut file);
    fn io_req_set_res(req: *mut io_kiocb, res: c_int, flags: u32);
}

#[inline]
unsafe fn io_kiocb_to_cmd<T>(req: *mut io_kiocb) -> *mut T {
    req as *mut T
}

#[inline]
unsafe fn read_once<T: Copy>(value: *const T) -> T {
    core::ptr::read_volatile(value)
}

pub unsafe fn io_sfr_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int {
    let sync = &mut *io_kiocb_to_cmd::<io_sync>(req);

    if (*sqe).addr != 0 || (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 {
        return -EINVAL;
    }

    sync.off = read_once(&(*sqe).off);
    sync.len = read_once(&(*sqe).len) as i64;
    sync.flags = read_once(&(*sqe).sync_range_flags) as c_int;
    (*req).flags |= REQ_F_FORCE_ASYNC;

    0
}

pub unsafe fn io_sync_file_range(req: *mut io_kiocb, issue_flags: u32) -> c_int {
    let sync = &mut *io_kiocb_to_cmd::<io_sync>(req);
    // sync_file_range always requires a blocking context.
    let _ = issue_flags & IO_URING_F_NONBLOCK;

    let ret = sync_file_range((*req).file, sync.off, sync.len, sync.flags);
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

pub unsafe fn io_fsync_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int {
    let sync = &mut *io_kiocb_to_cmd::<io_sync>(req);

    if (*sqe).addr != 0 || (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 {
        return -EINVAL;
    }

    sync.flags = read_once(&(*sqe).fsync_flags) as c_int;
    if (sync.flags & !IORING_FSYNC_DATASYNC) != 0 {
        return -EINVAL;
    }
    sync.off = read_once(&(*sqe).off);
    if sync.off < 0 {
        return -EINVAL;
    }
    sync.len = read_once(&(*sqe).len) as i64;
    (*req).flags |= REQ_F_FORCE_ASYNC;
    0
}

pub unsafe fn io_fsync(req: *mut io_kiocb, issue_flags: u32) -> c_int {
    let sync = &mut *io_kiocb_to_cmd::<io_sync>(req);
    let end = sync.off.wrapping_add(sync.len);
    // fsync always requires a blocking context.
    let _ = issue_flags & IO_URING_F_NONBLOCK;

    let ret = vfs_fsync_range((*req).file, sync.off, if end > 0 { end } else { LLONG_MAX },
                              sync.flags & IORING_FSYNC_DATASYNC);
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

pub unsafe fn io_fallocate_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int {
    let sync = &mut *io_kiocb_to_cmd::<io_sync>(req);

    if (*sqe).buf_index != 0 || (*sqe).rw_flags != 0 || (*sqe).splice_fd_in != 0 {
        return -EINVAL;
    }
    sync.off = read_once(&(*sqe).off);
    sync.len = read_once(&(*sqe).addr) as i64;
    sync.mode = read_once(&(*sqe).len) as c_int;
    (*req).flags |= REQ_F_FORCE_ASYNC;
    0
}

pub unsafe fn io_fallocate(req: *mut io_kiocb, issue_flags: u32) -> c_int {
    let sync = &mut *io_kiocb_to_cmd::<io_sync>(req);
    // fallocate always requiring blocking context.
    let _ = issue_flags & IO_URING_F_NONBLOCK;

    let ret = vfs_fallocate((*req).file, sync.mode, sync.off, sync.len);
    if ret >= 0 {
        fsnotify_modify((*req).file);
    }
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
