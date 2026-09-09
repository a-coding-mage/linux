// SPDX-License-Identifier: GPL-2.0
/*
 * FUSE passthrough to backing file.
 *
 * Copyright (c) 2023 CTERA Networks.
 */

use core::ffi::{c_int, c_uint, c_void};

// Declarations supplied by fuse_i.h and the kernel headers.
#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
    pub f_flags: c_uint,
    pub f_path: path,
}
#[repr(C)] pub struct inode;
#[repr(C)] pub struct iov_iter;
#[repr(C)] pub struct pipe_inode_info;
#[repr(C)] pub struct vm_area_struct { pub vm_start: usize, pub vm_end: usize }
#[repr(C)] pub struct fuse_conn;
#[repr(C)] pub struct fuse_backing { pub file: *mut file, pub cred: *mut cred }
#[repr(C)] pub struct cred;
#[repr(C)] pub struct fuse_mount { pub fc: *mut fuse_conn }
#[repr(C)] pub struct fuse_file {
    pub private_data: *mut c_void,
    pub fm: *mut fuse_mount,
    pub passthrough: *mut file,
    pub cred: *mut cred,
}
#[repr(C)] pub struct path;
#[repr(C)] pub struct kiocb {
    pub ki_filp: *mut file,
    pub ki_pos: i64,
    pub ki_flags: c_uint,
}
#[repr(C)] pub struct backing_file_ctx {
    pub cred: *mut cred,
    pub accessed: Option<unsafe extern "C" fn(*mut file)>,
    pub end_write: Option<unsafe extern "C" fn(*mut kiocb, isize)>,
}

extern "C" {
    fn fuse_invalidate_atime(inode: *mut inode);
    fn fuse_write_update_attr(inode: *mut inode, pos: i64, ret: isize);
    fn iov_iter_count(iter: *mut iov_iter) -> usize;
    fn backing_file_read_iter(file: *mut file, iter: *mut iov_iter, iocb: *mut kiocb,
                              flags: c_uint, ctx: *mut backing_file_ctx) -> isize;
    fn backing_file_write_iter(file: *mut file, iter: *mut iov_iter, iocb: *mut kiocb,
                               flags: c_uint, ctx: *mut backing_file_ctx) -> isize;
    fn backing_file_splice_read(file: *mut file, iocb: *mut kiocb, pipe: *mut pipe_inode_info,
                                len: usize, flags: c_uint, ctx: *mut backing_file_ctx) -> isize;
    fn backing_file_splice_write(pipe: *mut pipe_inode_info, file: *mut file, iocb: *mut kiocb,
                                 len: usize, flags: c_uint, ctx: *mut backing_file_ctx) -> isize;
    fn backing_file_mmap(file: *mut file, vma: *mut vm_area_struct,
                         ctx: *mut backing_file_ctx) -> isize;
    fn inode_lock(inode: *mut inode);
    fn inode_unlock(inode: *mut inode);
    fn init_sync_kiocb(iocb: *mut kiocb, file: *mut file);
    fn fuse_file_passthrough(ff: *mut fuse_file) -> *mut file;
    fn fuse_backing_lookup(fc: *mut fuse_conn, id: c_int) -> *mut fuse_backing;
    fn backing_file_open(file: *mut file, flags: c_uint, path: *mut path,
                         cred: *mut cred) -> *mut file;
    fn fuse_backing_put(fb: *mut fuse_backing);
    fn get_cred(cred: *mut cred) -> *mut cred;
    fn fput(file: *mut file);
    fn put_cred(cred: *mut cred);
    fn pr_debug(fmt: *const u8, ...);
}

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;

#[inline]
unsafe fn file_inode(file: *mut file) -> *mut inode { file as *mut inode }

unsafe extern "C" fn fuse_file_accessed(file: *mut file) {
    let inode = file_inode(file);
    fuse_invalidate_atime(inode);
}

unsafe extern "C" fn fuse_passthrough_end_write(iocb: *mut kiocb, ret: isize) {
    let inode = file_inode((*iocb).ki_filp);
    fuse_write_update_attr(inode, (*iocb).ki_pos, ret);
}

pub unsafe extern "C" fn fuse_passthrough_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize {
    let file = (*iocb).ki_filp;
    let ff = (*file).private_data as *mut fuse_file;
    let backing_file = fuse_file_passthrough(ff);
    let count = iov_iter_count(iter);
    let mut ctx = backing_file_ctx { cred: (*ff).cred, accessed: Some(fuse_file_accessed), end_write: None };
    if count == 0 { return 0; }
    backing_file_read_iter(backing_file, iter, iocb, (*iocb).ki_flags, &mut ctx)
}

pub unsafe extern "C" fn fuse_passthrough_write_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize {
    let file = (*iocb).ki_filp;
    let inode = file_inode(file);
    let ff = (*file).private_data as *mut fuse_file;
    let backing_file = fuse_file_passthrough(ff);
    let count = iov_iter_count(iter);
    let mut ctx = backing_file_ctx { cred: (*ff).cred, accessed: None, end_write: Some(fuse_passthrough_end_write) };
    if count == 0 { return 0; }
    inode_lock(inode);
    let ret = backing_file_write_iter(backing_file, iter, iocb, (*iocb).ki_flags, &mut ctx);
    inode_unlock(inode);
    ret
}

pub unsafe extern "C" fn fuse_passthrough_splice_read(input: *mut file, ppos: *mut i64,
    pipe: *mut pipe_inode_info, len: usize, flags: c_uint) -> isize {
    let ff = (*input).private_data as *mut fuse_file;
    let backing_file = fuse_file_passthrough(ff);
    let mut ctx = backing_file_ctx { cred: (*ff).cred, accessed: Some(fuse_file_accessed), end_write: None };
    let mut iocb = core::mem::zeroed::<kiocb>();
    init_sync_kiocb(&mut iocb, input);
    iocb.ki_pos = *ppos;
    let ret = backing_file_splice_read(backing_file, &mut iocb, pipe, len, flags, &mut ctx);
    *ppos = iocb.ki_pos;
    ret
}

pub unsafe extern "C" fn fuse_passthrough_splice_write(pipe: *mut pipe_inode_info,
    output: *mut file, ppos: *mut i64, len: usize, flags: c_uint) -> isize {
    let ff = (*output).private_data as *mut fuse_file;
    let backing_file = fuse_file_passthrough(ff);
    let inode = file_inode(output);
    let mut ctx = backing_file_ctx { cred: (*ff).cred, accessed: None, end_write: Some(fuse_passthrough_end_write) };
    let mut iocb = core::mem::zeroed::<kiocb>();
    inode_lock(inode);
    init_sync_kiocb(&mut iocb, output);
    iocb.ki_pos = *ppos;
    let ret = backing_file_splice_write(pipe, backing_file, &mut iocb, len, flags, &mut ctx);
    *ppos = iocb.ki_pos;
    inode_unlock(inode);
    ret
}

pub unsafe extern "C" fn fuse_passthrough_mmap(file: *mut file, vma: *mut vm_area_struct) -> isize {
    let ff = (*file).private_data as *mut fuse_file;
    let backing_file = fuse_file_passthrough(ff);
    let mut ctx = backing_file_ctx { cred: (*ff).cred, accessed: Some(fuse_file_accessed), end_write: None };
    backing_file_mmap(backing_file, vma, &mut ctx)
}

pub unsafe extern "C" fn fuse_passthrough_open(file: *mut file, backing_id: c_int) -> *mut fuse_backing {
    let ff = (*file).private_data as *mut fuse_file;
    let fc = (*(*ff).fm).fc;
    let mut fb: *mut fuse_backing = core::ptr::null_mut();
    let mut err = -EINVAL;
    if backing_id <= 0 { return err as isize as *mut fuse_backing; }
    err = -ENOENT;
    fb = fuse_backing_lookup(fc, backing_id);
    if fb.is_null() { return err as isize as *mut fuse_backing; }
    let backing_file = backing_file_open(file, (*file).f_flags, &mut (*fb).file.as_mut().unwrap().f_path, (*fb).cred);
    if (backing_file as isize) < 0 {
        fuse_backing_put(fb);
        return backing_file;
    }
    (*ff).passthrough = backing_file;
    (*ff).cred = get_cred((*fb).cred);
    fb
}

pub unsafe extern "C" fn fuse_passthrough_release(ff: *mut fuse_file, _fb: *mut fuse_backing) {
    fput((*ff).passthrough);
    (*ff).passthrough = core::ptr::null_mut();
    put_cred((*ff).cred);
    (*ff).cred = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
