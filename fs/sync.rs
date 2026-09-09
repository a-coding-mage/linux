// SPDX-License-Identifier: GPL-2.0
/* High-level sync()-related operations */

// Kernel headers and internal dependencies are supplied by other translation units.

const VALID_FLAGS: u32 = SYNC_FILE_RANGE_WAIT_BEFORE | SYNC_FILE_RANGE_WRITE |
    SYNC_FILE_RANGE_WAIT_AFTER;

pub unsafe fn sync_filesystem(sb: *mut super_block) -> i32 {
    let mut ret: i32 = 0;
    WARN_ON(!rwsem_is_locked(&(*sb).s_umount));
    if sb_rdonly(sb) { return 0; }
    writeback_inodes_sb(sb, WB_REASON_SYNC);
    if let Some(sync_fs) = (*(*sb).s_op).sync_fs {
        ret = sync_fs(sb, 0);
        if ret != 0 { return ret; }
    }
    ret = sync_blockdev_nowait((*sb).s_bdev);
    if ret != 0 { return ret; }
    sync_inodes_sb(sb);
    if let Some(sync_fs) = (*(*sb).s_op).sync_fs {
        ret = sync_fs(sb, 1);
        if ret != 0 { return ret; }
    }
    sync_blockdev((*sb).s_bdev)
}

pub unsafe fn sync_inodes_one_sb(sb: *mut super_block, _arg: *mut core::ffi::c_void) {
    if !sb_rdonly(sb) { sync_inodes_sb(sb); }
}

pub unsafe fn sync_fs_one_sb(sb: *mut super_block, arg: *mut core::ffi::c_void) {
    if !sb_rdonly(sb) && ((*sb).s_iflags & SB_I_SKIP_SYNC) == 0 {
        if let Some(sync_fs) = (*(*sb).s_op).sync_fs {
            sync_fs(sb, *(arg as *mut i32));
        }
    }
}

pub unsafe fn ksys_sync() {
    let mut nowait: i32 = 0;
    let mut wait: i32 = 1;
    wakeup_flusher_threads(WB_REASON_SYNC);
    iterate_supers(Some(sync_inodes_one_sb), core::ptr::null_mut());
    iterate_supers(Some(sync_fs_one_sb), &mut nowait as *mut _ as *mut _);
    iterate_supers(Some(sync_fs_one_sb), &mut wait as *mut _ as *mut _);
    sync_bdevs(false);
    sync_bdevs(true);
}

pub unsafe fn sync() -> i32 { ksys_sync(); 0 }

unsafe fn do_sync_work(work: *mut work_struct) {
    let mut nowait: i32 = 0;
    let mut wait: i32 = 1;
    iterate_supers(Some(sync_inodes_one_sb), core::ptr::null_mut());
    iterate_supers(Some(sync_fs_one_sb), &mut nowait as *mut _ as *mut _);
    sync_bdevs(false);
    iterate_supers(Some(sync_inodes_one_sb), core::ptr::null_mut());
    iterate_supers(Some(sync_fs_one_sb), &mut wait as *mut _ as *mut _);
    sync_bdevs(false);
    printk(c"Emergency Sync complete\n".as_ptr());
    kfree(work as *mut _);
}

pub unsafe fn emergency_sync() {
    let work = kmalloc_obj::<work_struct>(GFP_ATOMIC);
    if !work.is_null() {
        INIT_WORK(work, Some(do_sync_work));
        schedule_work(work);
    }
}

pub unsafe fn syncfs(fd: i32) -> i32 {
    let f = CLASS_fd(fd);
    if fd_empty(f) { return -EBADF; }
    let sb = (*(*fd_file(f)).f_path.dentry).d_sb;
    down_read(&(*sb).s_umount);
    let ret = sync_filesystem(sb);
    up_read(&(*sb).s_umount);
    let ret2 = errseq_check_and_advance(&mut (*sb).s_wb_err, &mut (*fd_file(f)).f_sb_err);
    if ret != 0 { ret } else { ret2 }
}

pub unsafe fn vfs_fsync_range(file: *mut file, start: loff_t, end: loff_t, datasync: i32) -> i32 {
    let inode = (*(*file).f_mapping).host;
    let fsync = (*(*file).f_op).fsync;
    if fsync.is_none() { return -EINVAL; }
    if datasync == 0 { sync_lazytime(inode); }
    fsync.unwrap()(file, start, end, datasync)
}

pub unsafe fn vfs_fsync(file: *mut file, datasync: i32) -> i32 {
    vfs_fsync_range(file, 0, LLONG_MAX, datasync)
}

unsafe fn do_fsync(fd: u32, datasync: i32) -> i32 {
    let f = CLASS_fd(fd);
    if fd_empty(f) { return -EBADF; }
    vfs_fsync(fd_file(f), datasync)
}

pub unsafe fn fsync(fd: u32) -> i32 { do_fsync(fd, 0) }
pub unsafe fn fdatasync(fd: u32) -> i32 { do_fsync(fd, 1) }

pub unsafe fn sync_file_range(file: *mut file, offset: loff_t, mut nbytes: loff_t, flags: u32) -> i32 {
    let mut ret: i32;
    if flags & !VALID_FLAGS != 0 { return -EINVAL; }
    let mut endbyte = offset.wrapping_add(nbytes);
    if offset < 0 || endbyte < 0 || endbyte < offset { return -EINVAL; }
    if core::mem::size_of::<pgoff_t>() == 4 {
        if offset >= (0x100000000u64 << PAGE_SHIFT) as loff_t { return 0; }
        if endbyte >= (0x100000000u64 << PAGE_SHIFT) as loff_t { nbytes = 0; }
    }
    if nbytes == 0 { endbyte = LLONG_MAX; } else { endbyte -= 1; }
    let i_mode = (*file_inode(file)).i_mode;
    if !S_ISREG(i_mode) && !S_ISBLK(i_mode) && !S_ISDIR(i_mode) { return -ESPIPE; }
    let mapping = (*file).f_mapping;
    ret = 0;
    if flags & SYNC_FILE_RANGE_WAIT_BEFORE != 0 {
        ret = file_fdatawait_range(file, offset, endbyte); if ret < 0 { return ret; }
    }
    if flags & SYNC_FILE_RANGE_WRITE != 0 {
        if flags & SYNC_FILE_RANGE_WRITE_AND_WAIT == SYNC_FILE_RANGE_WRITE_AND_WAIT {
            ret = filemap_fdatawrite_range(mapping, offset, endbyte);
        } else { ret = filemap_flush_range(mapping, offset, endbyte); }
        if ret < 0 { return ret; }
    }
    if flags & SYNC_FILE_RANGE_WAIT_AFTER != 0 { ret = file_fdatawait_range(file, offset, endbyte); }
    ret
}

pub unsafe fn ksys_sync_file_range(fd: i32, offset: loff_t, nbytes: loff_t, flags: u32) -> i32 {
    let f = CLASS_fd(fd);
    if fd_empty(f) { return -EBADF; }
    sync_file_range(fd_file(f), offset, nbytes, flags)
}

pub unsafe fn sync_file_range_syscall(fd: i32, offset: loff_t, nbytes: loff_t, flags: u32) -> i32 {
    ksys_sync_file_range(fd, offset, nbytes, flags)
}

pub unsafe fn sync_file_range2(fd: i32, flags: u32, offset: loff_t, nbytes: loff_t) -> i32 {
    ksys_sync_file_range(fd, offset, nbytes, flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
