// SPDX-License-Identifier: GPL-2.0-only
/*
 * Common helpers for stackable filesystems and backing files.
 *
 * Forked from fs/overlayfs/file.c.
 *
 * Copyright (C) 2017 Red Hat, Inc.
 * Copyright (C) 2023 CTERA Networks.
 */

// Kernel dependencies supplied by the surrounding Rust translation unit.

pub unsafe fn backing_file_open(
    user_file: *const file,
    flags: c_int,
    real_path: *const path,
    cred: *const cred,
) -> *mut file {
    let user_path = file_user_path(user_file);
    let mut f: *mut file;
    let mut error: c_int;

    f = alloc_empty_backing_file(flags, cred, user_file);
    if is_err(f) {
        return f;
    }

    path_get(user_path);
    backing_file_set_user_path(f, user_path);
    error = vfs_open(real_path, f);
    if error != 0 {
        fput(f);
        f = err_ptr(error);
    }

    f
}

pub unsafe fn backing_tmpfile_open(
    user_file: *const file,
    flags: c_int,
    real_parentpath: *const path,
    mode: umode_t,
    cred: *const cred,
) -> *mut file {
    let real_idmap = mnt_idmap((*real_parentpath).mnt);
    let user_path = &(*user_file).f_path as *const path;
    let mut f: *mut file;
    let mut error: c_int;

    f = alloc_empty_backing_file(flags, cred, user_file);
    if is_err(f) {
        return f;
    }

    path_get(user_path);
    backing_file_set_user_path(f, user_path);
    error = vfs_tmpfile(real_idmap, real_parentpath, f, mode);
    if error != 0 {
        fput(f);
        f = err_ptr(error);
    }
    f
}

#[repr(C)]
pub struct backing_aio {
    pub iocb: kiocb,
    pub ref_: refcount_t,
    pub orig_iocb: *mut kiocb,
    /* used for aio completion */
    pub end_write: Option<unsafe extern "C" fn(*mut kiocb, ssize_t)>,
    pub work: work_struct,
    pub res: c_long,
}

static mut backing_aio_cachep: *mut kmem_cache = core::ptr::null_mut();

const BACKING_IOCB_MASK: c_int = IOCB_NOWAIT | IOCB_HIPRI | IOCB_DSYNC | IOCB_SYNC | IOCB_APPEND;

unsafe fn iocb_to_rw_flags(flags: c_int) -> rwf_t {
    (flags & BACKING_IOCB_MASK) as rwf_t
}

unsafe fn backing_aio_put(aio: *mut backing_aio) {
    if refcount_dec_and_test(&mut (*aio).ref_) {
        fput((*aio).iocb.ki_filp);
        kmem_cache_free(backing_aio_cachep, aio as *mut c_void);
    }
}

unsafe fn backing_aio_cleanup(aio: *mut backing_aio, res: c_long) {
    let iocb = &mut (*aio).iocb;
    let orig_iocb = (*aio).orig_iocb;

    (*orig_iocb).ki_pos = iocb.ki_pos;
    if let Some(end_write) = (*aio).end_write {
        end_write(orig_iocb, res);
    }

    backing_aio_put(aio);
}

unsafe extern "C" fn backing_aio_rw_complete(iocb: *mut kiocb, res: c_long) {
    let aio = container_of!(iocb, backing_aio, iocb);
    let orig_iocb = (*aio).orig_iocb;

    if (*iocb).ki_flags & IOCB_WRITE != 0 {
        kiocb_end_write(iocb);
    }

    backing_aio_cleanup(aio, res);
    ((*orig_iocb).ki_complete.unwrap())(orig_iocb, res);
}

unsafe extern "C" fn backing_aio_complete_work(work: *mut work_struct) {
    let aio = container_of!(work, backing_aio, work);

    backing_aio_rw_complete(&mut (*aio).iocb, (*aio).res);
}

unsafe extern "C" fn backing_aio_queue_completion(iocb: *mut kiocb, res: c_long) {
    let aio = container_of!(iocb, backing_aio, iocb);

    /*
     * Punt to a work queue to serialize updates of mtime/size.
     */
    (*aio).res = res;
    INIT_WORK(&mut (*aio).work, backing_aio_complete_work);
    queue_work(
        (*file_inode((*aio).orig_iocb).ki_filp).i_sb.s_dio_done_wq,
        &mut (*aio).work,
    );
}

unsafe fn backing_aio_init_wq(iocb: *mut kiocb) -> c_int {
    let sb = (*file_inode((*iocb).ki_filp)).i_sb;

    if !sb.s_dio_done_wq.is_null() {
        return 0;
    }

    sb_init_dio_done_wq(sb)
}

unsafe fn do_backing_file_read_iter(
    file: *mut file,
    iter: *mut iov_iter,
    iocb: *mut kiocb,
    flags: c_int,
) -> c_int {
    let mut aio: *mut backing_aio = core::ptr::null_mut();
    let mut ret: c_int;

    if is_sync_kiocb(iocb) {
        let rwf = iocb_to_rw_flags(flags);
        return vfs_iter_read(file, iter, &mut (*iocb).ki_pos, rwf);
    }

    aio = kmem_cache_zalloc(backing_aio_cachep, GFP_KERNEL) as *mut backing_aio;
    if aio.is_null() {
        return -ENOMEM;
    }

    (*aio).orig_iocb = iocb;
    kiocb_clone(&mut (*aio).iocb, iocb, get_file(file));
    (*aio).iocb.ki_complete = Some(backing_aio_rw_complete);
    refcount_set(&mut (*aio).ref_, 2);
    ret = vfs_iocb_iter_read(file, &mut (*aio).iocb, iter);
    backing_aio_put(aio);
    if ret != -EIOCBQUEUED {
        backing_aio_cleanup(aio, ret as c_long);
    }
    ret
}

pub unsafe fn backing_file_read_iter(
    file: *mut file,
    iter: *mut iov_iter,
    iocb: *mut kiocb,
    flags: c_int,
    ctx: *mut backing_file_ctx,
) -> ssize_t {
    let ret: ssize_t;

    if WARN_ON_ONCE((*file).f_mode & FMODE_BACKING == 0) {
        return -EIO as ssize_t;
    }
    if iov_iter_count(iter) == 0 {
        return 0;
    }
    if (*iocb).ki_flags & IOCB_DIRECT != 0 && (*file).f_mode & FMODE_CAN_ODIRECT == 0 {
        return -EINVAL as ssize_t;
    }

    scoped_with_creds!((*ctx).cred, {
        ret = do_backing_file_read_iter(file, iter, iocb, flags) as ssize_t;
    });

    if let Some(accessed) = (*ctx).accessed {
        accessed((*iocb).ki_filp);
    }
    ret
}

unsafe fn do_backing_file_write_iter(
    file: *mut file,
    iter: *mut iov_iter,
    iocb: *mut kiocb,
    flags: c_int,
    end_write: Option<unsafe extern "C" fn(*mut kiocb, ssize_t)>,
) -> c_int {
    let aio: *mut backing_aio;
    let mut ret: c_int;

    if is_sync_kiocb(iocb) {
        let rwf = iocb_to_rw_flags(flags);
        ret = vfs_iter_write(file, iter, &mut (*iocb).ki_pos, rwf);
        if let Some(end_write) = end_write {
            end_write(iocb, ret as ssize_t);
        }
        return ret;
    }

    ret = backing_aio_init_wq(iocb);
    if ret != 0 {
        return ret;
    }
    aio = kmem_cache_zalloc(backing_aio_cachep, GFP_KERNEL) as *mut backing_aio;
    if aio.is_null() {
        return -ENOMEM;
    }

    (*aio).orig_iocb = iocb;
    (*aio).end_write = end_write;
    kiocb_clone(&mut (*aio).iocb, iocb, get_file(file));
    (*aio).iocb.ki_flags = flags;
    (*aio).iocb.ki_complete = Some(backing_aio_queue_completion);
    refcount_set(&mut (*aio).ref_, 2);
    ret = vfs_iocb_iter_write(file, &mut (*aio).iocb, iter);
    backing_aio_put(aio);
    if ret != -EIOCBQUEUED {
        backing_aio_cleanup(aio, ret as c_long);
    }
    ret
}

pub unsafe fn backing_file_write_iter(
    file: *mut file,
    iter: *mut iov_iter,
    iocb: *mut kiocb,
    flags: c_int,
    ctx: *mut backing_file_ctx,
) -> ssize_t {
    let ret: ssize_t;

    if WARN_ON_ONCE((*file).f_mode & FMODE_BACKING == 0) {
        return -EIO as ssize_t;
    }
    if iov_iter_count(iter) == 0 {
        return 0;
    }
    ret = file_remove_privs((*iocb).ki_filp) as ssize_t;
    if ret != 0 {
        return ret;
    }
    if (*iocb).ki_flags & IOCB_DIRECT != 0 && (*file).f_mode & FMODE_CAN_ODIRECT == 0 {
        return -EINVAL as ssize_t;
    }

    scoped_with_creds!((*ctx).cred, {
        ret = do_backing_file_write_iter(file, iter, iocb, flags, (*ctx).end_write) as ssize_t;
    });
    ret
}

pub unsafe fn backing_file_splice_read(
    input: *mut file,
    iocb: *mut kiocb,
    pipe: *mut pipe_inode_info,
    len: size_t,
    flags: c_uint,
    ctx: *mut backing_file_ctx,
) -> ssize_t {
    let ret: ssize_t;
    if WARN_ON_ONCE((*input).f_mode & FMODE_BACKING == 0) {
        return -EIO as ssize_t;
    }
    scoped_with_creds!((*ctx).cred, {
        ret = vfs_splice_read(input, &mut (*iocb).ki_pos, pipe, len, flags);
    });
    if let Some(accessed) = (*ctx).accessed {
        accessed((*iocb).ki_filp);
    }
    ret
}

pub unsafe fn backing_file_splice_write(
    pipe: *mut pipe_inode_info,
    out: *mut file,
    iocb: *mut kiocb,
    len: size_t,
    flags: c_uint,
    ctx: *mut backing_file_ctx,
) -> ssize_t {
    let ret: ssize_t;
    if WARN_ON_ONCE((*out).f_mode & FMODE_BACKING == 0) {
        return -EIO as ssize_t;
    }
    if (*out).f_op.is_null() || (*(*out).f_op).splice_write.is_none() {
        return -EINVAL as ssize_t;
    }
    ret = file_remove_privs((*iocb).ki_filp) as ssize_t;
    if ret != 0 {
        return ret;
    }
    scoped_with_creds!((*ctx).cred, {
        file_start_write(out);
        ret = ((*(*out).f_op).splice_write.unwrap())(pipe, out, &mut (*iocb).ki_pos, len, flags);
        file_end_write(out);
    });
    if let Some(end_write) = (*ctx).end_write {
        end_write(iocb, ret);
    }
    ret
}

pub unsafe fn backing_file_mmap(
    file: *mut file,
    vma: *mut vm_area_struct,
    ctx: *mut backing_file_ctx,
) -> c_int {
    let user_file = (*vma).vm_file;
    let ret: c_int;
    if WARN_ON_ONCE((*file).f_mode & FMODE_BACKING == 0) {
        return -EIO;
    }
    if !can_mmap_file(file) {
        return -ENODEV;
    }
    vma_set_file(vma, file);
    scoped_with_creds!((*ctx).cred, {
        ret = security_mmap_backing_file(vma, file, user_file);
        if ret == 0 {
            ret = vfs_mmap((*vma).vm_file, vma);
        }
    });
    if let Some(accessed) = (*ctx).accessed {
        accessed(user_file);
    }
    ret
}

unsafe extern "C" fn backing_aio_init() -> c_int {
    backing_aio_cachep = KMEM_CACHE!(backing_aio, SLAB_HWCACHE_ALIGN);
    if backing_aio_cachep.is_null() {
        return -ENOMEM;
    }
    0
}

fs_initcall!(backing_aio_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
