// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/ext2/file.c
 *
 * Copyright (C) 1992, 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 *
 *  from
 *
 *  linux/fs/minix/file.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  ext2 fs regular file handling primitives
 *
 *  64-bit file support on 64-bit platforms by Jakub Jelinek
 * 	(jj@sunsite.ms.mff.cuni.cz)
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

/*
 * Called when filp is released. This happens when all file descriptors
 * for a single struct file are closed. Note that different open() calls
 * for the same file yield different struct file structures.
 */
unsafe fn ext2_release_file(inode: *mut inode, filp: *mut file) -> c_int {
    if (*filp).f_mode & FMODE_WRITE != 0 {
        mutex_lock(&mut (*EXT2_I(inode)).truncate_mutex);
        ext2_discard_reservation(inode);
        mutex_unlock(&mut (*EXT2_I(inode)).truncate_mutex);
    }
    0
}

unsafe fn ext2_dio_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp;
    let inode = (*(*file).f_mapping).host;
    let ret: ssize_t;

    trace_ext2_dio_read_begin(iocb, to, 0);
    inode_lock_shared(inode);
    ret = iomap_dio_rw(iocb, to, &ext2_iomap_ops, core::ptr::null_mut(), 0,
                       core::ptr::null_mut(), 0);
    inode_unlock_shared(inode);
    trace_ext2_dio_read_end(iocb, to, ret);

    ret
}

unsafe fn ext2_dio_write_end_io(iocb: *mut kiocb, size: ssize_t,
                                error: c_int, _flags: c_uint) -> c_int {
    let mut pos = (*iocb).ki_pos;
    let inode = file_inode((*iocb).ki_filp);

    if error != 0 {
        trace_ext2_dio_write_endio(iocb, size, error);
        return error;
    }

    /*
     * If we are extending the file, we have to update i_size here before
     * page cache gets invalidated in iomap_dio_rw(). This prevents racing
     * buffered reads from zeroing out too much from page cache pages.
     * Note that all extending writes always happens synchronously with
     * inode lock held by ext2_dio_write_iter(). So it is safe to update
     * inode size here for extending file writes.
     */
    pos += size;
    if pos > i_size_read(inode) {
        i_size_write(inode, pos);
        mark_inode_dirty(inode);
    }
    trace_ext2_dio_write_endio(iocb, size, error);
    error
}

static mut ext2_dio_write_ops: iomap_dio_ops = iomap_dio_ops {
    end_io: Some(ext2_dio_write_end_io),
};

unsafe fn ext2_dio_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp;
    let inode = (*(*file).f_mapping).host;
    let mut ret: ssize_t;
    let mut flags: c_uint = 0;
    let blocksize = (*(*inode).i_sb).s_blocksize as c_ulong;
    let offset = (*iocb).ki_pos;
    let count = iov_iter_count(from);
    let mut status: ssize_t = 0;

    trace_ext2_dio_write_begin(iocb, from, 0);
    inode_lock(inode);
    ret = generic_write_checks(iocb, from);
    if ret <= 0 {
        inode_unlock(inode);
        trace_ext2_dio_write_end(iocb, from, ret);
        return ret;
    }

    ret = kiocb_modified(iocb);
    if ret != 0 {
        inode_unlock(inode);
        trace_ext2_dio_write_end(iocb, from, ret);
        return ret;
    }

    /* use IOMAP_DIO_FORCE_WAIT for unaligned or extending writes */
    if (*iocb).ki_pos + iov_iter_count(from) > i_size_read(inode)
        || (((*iocb).ki_pos | iov_iter_alignment(from)) as c_ulong) % blocksize != 0 {
        flags |= IOMAP_DIO_FORCE_WAIT;
    }

    ret = iomap_dio_rw(iocb, from, &ext2_iomap_ops, &ext2_dio_write_ops,
                       flags, core::ptr::null_mut(), 0);

    /* ENOTBLK is magic return value for fallback to buffered-io */
    if ret == -ENOTBLK { ret = 0; }

    if ret < 0 && ret != -EIOCBQUEUED {
        ext2_write_failed((*inode).i_mapping, offset + count);
    }

    /* handle case for partial write and for fallback to buffered write */
    if ret >= 0 && iov_iter_count(from) != 0 {
        let pos = (*iocb).ki_pos;
        (*iocb).ki_flags &= !IOCB_DIRECT;
        status = generic_perform_write(iocb, from);
        if status < 0 {
            inode_unlock(inode);
            trace_ext2_dio_write_end(iocb, from, status);
            return status;
        }

        ret += status;
        let endbyte = pos + status - 1;
        let ret2 = filemap_write_and_wait_range((*inode).i_mapping, pos, endbyte);
        if ret2 == 0 {
            invalidate_mapping_pages((*inode).i_mapping, pos >> PAGE_SHIFT,
                                     endbyte >> PAGE_SHIFT);
            if ret > 0 { ret = generic_write_sync(iocb, ret); }
        } else {
            ret = ret2 as ssize_t;
        }
    }

    inode_unlock(inode);
    if status != 0 { trace_ext2_dio_write_buff_end(iocb, from, status); }
    trace_ext2_dio_write_end(iocb, from, ret);
    ret
}

unsafe fn ext2_file_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    if (*iocb).ki_flags & IOCB_DIRECT != 0 { ext2_dio_read_iter(iocb, to) }
    else { generic_file_read_iter(iocb, to) }
}

unsafe fn ext2_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    if (*iocb).ki_flags & IOCB_DIRECT != 0 { ext2_dio_write_iter(iocb, from) }
    else { generic_file_write_iter(iocb, from) }
}

unsafe fn ext2_file_open(inode: *mut inode, filp: *mut file) -> c_int {
    (*filp).f_mode |= FMODE_CAN_ODIRECT;
    dquot_file_open(inode, filp)
}

#[no_mangle]
pub static ext2_file_operations: file_operations = file_operations {
    llseek: Some(generic_file_llseek),
    read_iter: Some(ext2_file_read_iter),
    write_iter: Some(ext2_file_write_iter),
    unlocked_ioctl: Some(ext2_ioctl),
    #[cfg(CONFIG_COMPAT)]
    compat_ioctl: Some(ext2_compat_ioctl),
    mmap_prepare: Some(generic_file_mmap_prepare),
    open: Some(ext2_file_open),
    release: Some(ext2_release_file),
    fsync: Some(simple_fsync),
    get_unmapped_area: Some(thp_get_unmapped_area),
    splice_read: Some(filemap_splice_read),
    splice_write: Some(iter_file_splice_write),
    setlease: Some(generic_setlease),
};

#[no_mangle]
pub static ext2_file_inode_operations: inode_operations = inode_operations {
    listxattr: Some(ext2_listxattr),
    getattr: Some(ext2_getattr),
    setattr: Some(ext2_setattr),
    get_inode_acl: Some(ext2_get_acl),
    set_acl: Some(ext2_set_acl),
    fiemap: Some(ext2_fiemap),
    fileattr_get: Some(ext2_fileattr_get),
    fileattr_set: Some(ext2_fileattr_set),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
