/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Function declarations and data structures related to the splice
 * implementation.
 *
 * Copyright (C) 2007 Jens Axboe <jens.axboe@oracle.com>
 */

// Dependency: declarations from linux/pipe_fs_i.h are supplied elsewhere.

/* Flags passed in from splice/tee/vmsplice */
pub const SPLICE_F_MOVE: u32 = 0x01; /* move pages instead of copying */
pub const SPLICE_F_NONBLOCK: u32 = 0x02; /* don't block on the pipe splicing */
pub const SPLICE_F_MORE: u32 = 0x04; /* expect more data */
pub const SPLICE_F_GIFT: u32 = 0x08; /* pages passed in are a gift */
pub const SPLICE_F_ALL: u32 =
    SPLICE_F_MOVE | SPLICE_F_NONBLOCK | SPLICE_F_MORE | SPLICE_F_GIFT;

#[repr(C)]
pub union splice_desc_u {
    pub userptr: *mut core::ffi::c_void, /* memory to write to */
    pub file: *mut file,                 /* file to read/write */
    pub data: *mut core::ffi::c_void,    /* cookie */
}

/* Passed to the actors */
#[repr(C)]
pub struct splice_desc {
    pub total_len: usize, /* remaining length */
    pub len: u32,         /* current length */
    pub flags: u32,       /* splice flags */
    /* actor() private data */
    pub u: splice_desc_u,
    pub splice_eof: Option<unsafe extern "C" fn(sd: *mut splice_desc)>,
    pub pos: i64,                    /* file position */
    pub opos: *mut i64,              /* sendfile: output position */
    pub num_spliced: usize,          /* number of bytes already spliced */
    pub need_wakeup: bool,           /* need to wake up writer */
}

#[repr(C)]
pub struct partial_page {
    pub offset: u32,
    pub len: u32,
    pub private: usize,
}

/* Passed to splice_to_pipe */
#[repr(C)]
pub struct splice_pipe_desc {
    pub pages: *mut *mut page, /* page map */
    pub partial: *mut partial_page, /* pages[] may not be contig */
    pub nr_pages: i32,         /* number of populated pages in map */
    pub nr_pages_max: u32,     /* pages[] & partial[] arrays size */
    pub ops: *const pipe_buf_operations, /* ops associated with output pipe */
    pub spd_release:
        Option<unsafe extern "C" fn(*mut splice_pipe_desc, u32)>,
}

pub type splice_actor = unsafe extern "C" fn(
    *mut pipe_inode_info,
    *mut pipe_buffer,
    *mut splice_desc,
) -> i32;
pub type splice_direct_actor = unsafe extern "C" fn(*mut pipe_inode_info, *mut splice_desc) -> i32;

unsafe extern "C" {
    pub fn splice_from_pipe(
        pipe: *mut pipe_inode_info,
        out: *mut file,
        ppos: *mut i64,
        len: usize,
        flags: u32,
        actor: splice_actor,
    ) -> isize;
    pub fn __splice_from_pipe(
        pipe: *mut pipe_inode_info,
        sd: *mut splice_desc,
        actor: splice_actor,
    ) -> isize;
    pub fn splice_to_pipe(pipe: *mut pipe_inode_info, spd: *mut splice_pipe_desc) -> isize;
    pub fn add_to_pipe(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) -> isize;
    pub fn vfs_splice_read(
        input: *mut file,
        ppos: *mut i64,
        pipe: *mut pipe_inode_info,
        len: usize,
        flags: u32,
    ) -> isize;
    pub fn splice_direct_to_actor(
        file: *mut file,
        sd: *mut splice_desc,
        actor: splice_direct_actor,
    ) -> isize;
    pub fn do_splice(
        input: *mut file,
        off_in: *mut i64,
        output: *mut file,
        off_out: *mut i64,
        len: usize,
        flags: u32,
    ) -> isize;
    pub fn do_splice_direct(
        input: *mut file,
        ppos: *mut i64,
        output: *mut file,
        opos: *mut i64,
        len: usize,
        flags: u32,
    ) -> isize;
    pub fn splice_file_range(
        input: *mut file,
        ppos: *mut i64,
        output: *mut file,
        opos: *mut i64,
        len: usize,
    ) -> isize;

    pub fn do_tee(input: *mut file, output: *mut file, len: usize, flags: u32) -> isize;
    pub fn splice_to_socket(
        pipe: *mut pipe_inode_info,
        out: *mut file,
        ppos: *mut i64,
        len: usize,
        flags: u32,
    ) -> isize;

    pub fn splice_grow_spd(pipe: *const pipe_inode_info, spd: *mut splice_pipe_desc) -> i32;
    pub fn splice_shrink_spd(spd: *mut splice_pipe_desc);
}

pub unsafe fn splice_copy_file_range(
    input: *mut file,
    pos_in: i64,
    output: *mut file,
    pos_out: i64,
    len: usize,
) -> isize {
    let mut input_pos = pos_in;
    let mut output_pos = pos_out;
    splice_file_range(input, &mut input_pos, output, &mut output_pos, len)
}

unsafe extern "C" {
    pub static page_cache_pipe_buf_ops: pipe_buf_operations;
    pub static default_pipe_buf_ops: pipe_buf_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
