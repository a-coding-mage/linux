/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Common helpers for stackable filesystems and backing files.
 *
 * Copyright (C) 2023 CTERA Networks.
 */

// Dependencies supplied by the corresponding Linux headers:
// linux/file.h, linux/uio.h, and linux/fs.h.

#[repr(C)]
pub struct backing_file_ctx {
    pub cred: *const cred,
    pub accessed: Option<unsafe extern "C" fn(file: *mut file)>,
    pub end_write: Option<unsafe extern "C" fn(iocb: *mut kiocb, _: isize)>,
}

extern "C" {
    pub fn backing_file_open(
        user_file: *const file,
        flags: i32,
        real_path: *const path,
        cred: *const cred,
    ) -> *mut file;

    pub fn backing_tmpfile_open(
        user_file: *const file,
        flags: i32,
        real_parentpath: *const path,
        mode: umode_t,
        cred: *const cred,
    ) -> *mut file;

    pub fn backing_file_read_iter(
        file: *mut file,
        iter: *mut iov_iter,
        iocb: *mut kiocb,
        flags: i32,
        ctx: *mut backing_file_ctx,
    ) -> isize;

    pub fn backing_file_write_iter(
        file: *mut file,
        iter: *mut iov_iter,
        iocb: *mut kiocb,
        flags: i32,
        ctx: *mut backing_file_ctx,
    ) -> isize;

    pub fn backing_file_splice_read(
        input: *mut file,
        iocb: *mut kiocb,
        pipe: *mut pipe_inode_info,
        len: usize,
        flags: u32,
        ctx: *mut backing_file_ctx,
    ) -> isize;

    pub fn backing_file_splice_write(
        pipe: *mut pipe_inode_info,
        output: *mut file,
        iocb: *mut kiocb,
        len: usize,
        flags: u32,
        ctx: *mut backing_file_ctx,
    ) -> isize;

    pub fn backing_file_mmap(
        file: *mut file,
        vma: *mut vm_area_struct,
        ctx: *mut backing_file_ctx,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
