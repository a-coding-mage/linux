// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux kernel headers are intentionally left external.

use core::ffi::{c_char, c_int, c_void};

pub unsafe fn kernel_read_file(
    file: *mut file,
    offset: loff_t,
    buf: *mut *mut c_void,
    buf_size: usize,
    file_size: *mut usize,
    id: kernel_read_file_id,
) -> isize {
    let mut i_size: loff_t;
    let mut pos: loff_t;
    let mut copied: isize;
    let mut allocated: *mut c_void = core::ptr::null_mut();
    let whole_file: bool;
    let mut ret: c_int;

    if offset != 0 && ((*buf).is_null() || file_size.is_null()) {
        return -EINVAL as isize;
    }

    if !S_ISREG((*file_inode(file)).i_mode) {
        return -EINVAL as isize;
    }

    ret = deny_write_access(file);
    if ret != 0 {
        return ret as isize;
    }

    i_size = i_size_read(file_inode(file));
    if i_size <= 0 {
        ret = -EINVAL;
        allow_write_access(file);
        return ret as isize;
    }
    /* The file is too big for sane activities. */
    if i_size > SSIZE_MAX as loff_t {
        ret = -EFBIG;
        allow_write_access(file);
        return ret as isize;
    }
    /* The entire file cannot be read in one buffer. */
    if file_size.is_null() && offset == 0 && i_size > buf_size as loff_t {
        ret = -EFBIG;
        allow_write_access(file);
        return ret as isize;
    }

    whole_file = offset == 0 && i_size <= buf_size as loff_t;
    ret = security_kernel_read_file(file, id, whole_file);
    if ret != 0 {
        allow_write_access(file);
        return ret as isize;
    }

    if !file_size.is_null() {
        *file_size = i_size as usize;
    }

    if (*buf).is_null() {
        *buf = allocated = vmalloc(i_size as usize);
    }
    if (*buf).is_null() {
        ret = -ENOMEM;
        allow_write_access(file);
        return ret as isize;
    }

    pos = offset;
    copied = 0;
    while copied < buf_size as isize {
        let mut bytes: isize;
        let wanted = core::cmp::min(buf_size - copied as usize, (i_size - pos) as usize);

        bytes = kernel_read(file, (*buf as *mut u8).add(copied as usize) as *mut c_void, wanted, &mut pos);
        if bytes < 0 {
            ret = bytes as c_int;
            break;
        }

        if bytes == 0 {
            ret = 0;
            break;
        }
        copied += bytes;
        ret = 0;
    }

    if ret == 0 && whole_file {
        if pos != i_size {
            ret = -EIO;
        } else {
            ret = security_kernel_post_read_file(file, *buf, i_size as usize, id);
        }
    }

    if ret < 0 && !allocated.is_null() {
        vfree(*buf);
        *buf = core::ptr::null_mut();
    }

    allow_write_access(file);
    if ret == 0 { copied } else { ret as isize }
}

pub unsafe fn kernel_read_file_from_path(
    path: *const c_char, offset: loff_t, buf: *mut *mut c_void,
    buf_size: usize, file_size: *mut usize, id: kernel_read_file_id,
) -> isize {
    if path.is_null() || *path == 0 { return -EINVAL as isize; }
    let file = filp_open(path, O_RDONLY, 0);
    if IS_ERR(file) { return PTR_ERR(file) as isize; }
    let ret = kernel_read_file(file, offset, buf, buf_size, file_size, id);
    fput(file);
    ret
}

pub unsafe fn kernel_read_file_from_path_initns(
    path: *const c_char, offset: loff_t, buf: *mut *mut c_void,
    buf_size: usize, file_size: *mut usize, id: kernel_read_file_id,
) -> isize {
    if path.is_null() || *path == 0 { return -EINVAL as isize; }
    // scoped_with_init_fs() changes the active filesystem namespace for this call.
    let file = filp_open(path, O_RDONLY, 0);
    if IS_ERR(file) { return PTR_ERR(file) as isize; }
    let ret = kernel_read_file(file, offset, buf, buf_size, file_size, id);
    fput(file);
    ret
}

pub unsafe fn kernel_read_file_from_fd(
    fd: c_int, offset: loff_t, buf: *mut *mut c_void,
    buf_size: usize, file_size: *mut usize, id: kernel_read_file_id,
) -> isize {
    let f = fdget(fd);
    if fd_empty(f) || ((*fd_file(f)).f_mode & FMODE_READ) == 0 { return -EBADF as isize; }
    kernel_read_file(fd_file(f), offset, buf, buf_size, file_size, id)
}

// External kernel types, constants, and functions are provided by dependencies.
type file = core::ffi::c_void;
type loff_t = i64;
type kernel_read_file_id = c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
