// SPDX-License-Identifier: GPL-2.0
// Translated from advise.c. Kernel headers and local dependencies are supplied
// by the surrounding build; their include intent is preserved here.

use core::ffi::c_int;

// linux/kernel.h, linux/errno.h, linux/fs.h, linux/file.h, linux/mm.h,
// linux/slab.h, linux/namei.h, linux/io_uring.h,
// uapi/linux/fadvise.h, uapi/linux/io_uring.h, io_uring.h, advise.h

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
    pub buf_index: u16,
    pub splice_fd_in: u32,
    pub addr: u64,
    pub off: u64,
    pub len: u32,
    pub fadvise_advice: u32,
}

#[repr(C)]
pub struct io_fadvise {
    pub file: *mut file,
    pub offset: u64,
    pub len: u64,
    pub advice: u32,
}

#[repr(C)]
pub struct io_madvise {
    pub file: *mut file,
    pub addr: u64,
    pub len: u64,
    pub advice: u32,
}

extern "C" {
    static mut current: *mut core::ffi::c_void;

    fn do_madvise(
        mm: *mut core::ffi::c_void,
        start: u64,
        len_in: u64,
        behavior: u32,
    ) -> c_int;
    fn vfs_fadvise(
        file: *mut file,
        offset: u64,
        len_in: u64,
        advice: u32,
    ) -> c_int;
    fn io_req_set_res(req: *mut io_kiocb, res: c_int, flags: u32);
    fn req_set_fail(req: *mut io_kiocb);
}

// These constants and macros are provided by the kernel headers/local io_uring
// implementation: EINVAL, EOPNOTSUPP, REQ_F_FORCE_ASYNC, IO_URING_F_NONBLOCK,
// IOU_COMPLETE, POSIX_FADV_NORMAL, POSIX_FADV_RANDOM, POSIX_FADV_SEQUENTIAL,
// READ_ONCE, WARN_ON_ONCE, and io_kiocb_to_cmd.

pub unsafe fn io_madvise_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int {
    // CONFIG_ADVISE_SYSCALLS && CONFIG_MMU
    let ma: *mut io_madvise = io_kiocb_to_cmd!(req, io_madvise);

    if (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 {
        return -EINVAL;
    }

    (*ma).addr = READ_ONCE!((*sqe).addr);
    (*ma).len = READ_ONCE!((*sqe).off);
    if (*ma).len == 0 {
        (*ma).len = READ_ONCE!((*sqe).len);
    }
    (*ma).advice = READ_ONCE!((*sqe).fadvise_advice);
    (*req).flags |= REQ_F_FORCE_ASYNC;
    0
}

pub unsafe fn io_madvise(req: *mut io_kiocb, issue_flags: u32) -> c_int {
    // CONFIG_ADVISE_SYSCALLS && CONFIG_MMU
    let ma: *mut io_madvise = io_kiocb_to_cmd!(req, io_madvise);
    let ret: c_int;

    WARN_ON_ONCE!(issue_flags & IO_URING_F_NONBLOCK);

    ret = do_madvise((*current).cast(), (*ma).addr, (*ma).len, (*ma).advice);
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

unsafe fn io_fadvise_force_async(fa: *mut io_fadvise) -> bool {
    match (*fa).advice {
        POSIX_FADV_NORMAL | POSIX_FADV_RANDOM | POSIX_FADV_SEQUENTIAL => false,
        _ => true,
    }
}

pub unsafe fn io_fadvise_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int {
    let fa: *mut io_fadvise = io_kiocb_to_cmd!(req, io_fadvise);

    if (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 {
        return -EINVAL;
    }

    (*fa).offset = READ_ONCE!((*sqe).off);
    (*fa).len = READ_ONCE!((*sqe).addr);
    if (*fa).len == 0 {
        (*fa).len = READ_ONCE!((*sqe).len);
    }
    (*fa).advice = READ_ONCE!((*sqe).fadvise_advice);
    if io_fadvise_force_async(fa) {
        (*req).flags |= REQ_F_FORCE_ASYNC;
    }
    0
}

pub unsafe fn io_fadvise(req: *mut io_kiocb, issue_flags: u32) -> c_int {
    let fa: *mut io_fadvise = io_kiocb_to_cmd!(req, io_fadvise);
    let ret: c_int;

    WARN_ON_ONCE!(issue_flags & IO_URING_F_NONBLOCK != 0 && io_fadvise_force_async(fa));

    ret = vfs_fadvise((*req).file, (*fa).offset, (*fa).len, (*fa).advice);
    if ret < 0 {
        req_set_fail(req);
    }
    io_req_set_res(req, ret, 0);
    IOU_COMPLETE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
