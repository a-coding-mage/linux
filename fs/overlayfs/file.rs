// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017 Red Hat, Inc.
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

unsafe fn ovl_whatisit(inode: *mut inode, realinode: *mut inode) -> c_char {
    if realinode != ovl_inode_upper(inode) { b'l' as c_char }
    else if ovl_has_upperdata(inode) { b'u' as c_char }
    else { b'm' as c_char }
}

unsafe fn ovl_open_realfile(file: *const file, realpath: *const path) -> *mut file {
    let realinode = d_inode((*realpath).dentry);
    let inode = file_inode(file);
    let mut real_idmap: *mut mnt_idmap;
    let realfile: *mut file;
    let mut flags = (*file).f_flags | OVL_OPEN_FLAGS;
    let mut acc_mode = ACC_MODE(flags);
    let mut err: c_int;
    if flags & O_APPEND != 0 { acc_mode |= MAY_APPEND; }
    with_ovl_creds((*inode).i_sb, {
        real_idmap = mnt_idmap((*realpath).mnt);
        err = inode_permission(real_idmap, realinode, MAY_OPEN | acc_mode);
        if err != 0 { realfile = ERR_PTR(err); }
        else {
            if !inode_owner_or_capable(real_idmap, realinode) { flags &= !O_NOATIME; }
            realfile = backing_file_open(file, flags, realpath, current_cred());
        }
    });
    pr_debug!("open(%p[%pD2/%c], 0%o) -> (%p, 0%o)\n", file, file,
              ovl_whatisit(inode, realinode), (*file).f_flags, realfile,
              if IS_ERR(realfile) { 0 } else { (*realfile).f_flags });
    realfile
}

const OVL_SETFL_MASK: c_uint = O_APPEND | O_NONBLOCK | O_NDELAY | O_DIRECT;

unsafe fn ovl_change_flags(file: *mut file, mut flags: c_uint) -> c_int {
    let inode = file_inode(file);
    flags &= OVL_SETFL_MASK;
    if ((flags ^ (*file).f_flags) & O_APPEND) != 0 && IS_APPEND(inode) { return -EPERM; }
    if flags & O_DIRECT != 0 && (*file).f_mode & FMODE_CAN_ODIRECT == 0 { return -EINVAL; }
    if let Some(check_flags) = (*(*file).f_op).check_flags {
        let err = check_flags(flags); if err != 0 { return err; }
    }
    spin_lock(&mut (*file).f_lock);
    (*file).f_flags = ((*file).f_flags & !OVL_SETFL_MASK) | flags;
    (*file).f_iocb_flags = iocb_flags(file);
    spin_unlock(&mut (*file).f_lock);
    0
}

#[repr(C)]
pub struct ovl_file { pub realfile: *mut file, pub upperfile: *mut file }

pub unsafe fn ovl_file_alloc(realfile: *mut file) -> *mut ovl_file {
    let of = kzalloc_obj::<ovl_file>();
    if of.is_null() { return core::ptr::null_mut(); }
    (*of).realfile = realfile; of
}

pub unsafe fn ovl_file_free(of: *mut ovl_file) {
    fput((*of).realfile); if !(*of).upperfile.is_null() { fput((*of).upperfile); } kfree(of);
}

unsafe fn ovl_is_real_file(realfile: *const file, realpath: *const path) -> bool {
    file_inode(realfile) == d_inode((*realpath).dentry)
}

unsafe fn ovl_real_file_path(file: *const file, realpath: *const path) -> *mut file {
    let of = (*file).private_data as *mut ovl_file; let mut realfile = (*of).realfile;
    if WARN_ON_ONCE((*realpath).dentry.is_null()) { return ERR_PTR(-EIO); }
    if !ovl_is_real_file(realfile, realpath) {
        let mut upperfile = READ_ONCE((*of).upperfile);
        if upperfile.is_null() {
            upperfile = ovl_open_realfile(file, realpath);
            if IS_ERR(upperfile) { return upperfile; }
            let old = cmpxchg_release(&mut (*of).upperfile, core::ptr::null_mut(), upperfile);
            if !old.is_null() { fput(upperfile); upperfile = old; }
        }
        if WARN_ON_ONCE(!ovl_is_real_file(upperfile, realpath)) { return ERR_PTR(-EIO); }
        realfile = upperfile;
    }
    if ((*file).f_flags ^ (*realfile).f_flags) & !OVL_OPEN_FLAGS != 0 {
        let err = ovl_change_flags(realfile, (*file).f_flags); if err != 0 { return ERR_PTR(err); }
    }
    realfile
}

unsafe fn ovl_real_file(file: *const file) -> *mut file {
    let dentry = file_dentry(file); let mut realpath = core::mem::zeroed();
    if d_is_dir(dentry) { let f = ovl_dir_real_file(file, false); if WARN_ON_ONCE(f.is_null()) { return ERR_PTR(-EIO); } return f; }
    let err = ovl_verify_lowerdata(dentry); if err != 0 { return ERR_PTR(err); }
    ovl_path_realdata(dentry, &mut realpath); ovl_real_file_path(file, &realpath)
}

unsafe fn ovl_open(inode: *mut inode, file: *mut file) -> c_int {
    let dentry = file_dentry(file); let mut realpath = core::mem::zeroed();
    let err = ovl_verify_lowerdata(dentry); if err != 0 { return err; }
    let err = ovl_maybe_copy_up(dentry, (*file).f_flags); if err != 0 { return err; }
    (*file).f_flags &= !(O_CREAT | O_EXCL | O_NOCTTY | O_TRUNC);
    ovl_path_realdata(dentry, &mut realpath); if realpath.dentry.is_null() { return -EIO; }
    let realfile = ovl_open_realfile(file, &realpath); if IS_ERR(realfile) { return PTR_ERR(realfile); }
    let of = ovl_file_alloc(realfile); if of.is_null() { fput(realfile); return -ENOMEM; }
    (*file).private_data = of as *mut c_void; 0
}

unsafe fn ovl_release(_inode: *mut inode, file: *mut file) -> c_int { ovl_file_free((*file).private_data as *mut ovl_file); 0 }

unsafe fn ovl_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    if offset == 0 { if whence == SEEK_CUR { return (*file).f_pos; } if whence == SEEK_SET { return vfs_setpos(file, 0, 0); } }
    let realfile = ovl_real_file(file); if IS_ERR(realfile) { return PTR_ERR(realfile); }
    let inode = file_inode(file); ovl_inode_lock(inode); (*realfile).f_pos = (*file).f_pos;
    let ret = with_ovl_creds((*inode).i_sb, vfs_llseek(realfile, offset, whence));
    (*file).f_pos = (*realfile).f_pos; ovl_inode_unlock(inode); ret
}

unsafe fn ovl_file_modified(file: *mut file) { ovl_copyattr(file_inode(file)); }
unsafe fn ovl_file_end_write(iocb: *mut kiocb, _ret: ssize_t) { ovl_file_modified((*iocb).ki_filp); }
unsafe fn ovl_file_accessed(file: *mut file) {
    if (*file).f_flags & O_NOATIME != 0 { return; }
    let inode = file_inode(file); let upperinode = ovl_inode_upper(inode); if upperinode.is_null() { return; }
    let ctime = inode_get_ctime(inode); let uctime = inode_get_ctime(upperinode);
    let mtime = inode_get_mtime(inode); let umtime = inode_get_mtime(upperinode);
    if !timespec64_equal(&mtime, &umtime) || !timespec64_equal(&ctime, &uctime) {
        inode_set_mtime_to_ts(inode, inode_get_mtime(upperinode)); inode_set_ctime_to_ts(inode, uctime);
    }
    touch_atime(&(*file).f_path);
}

// The remaining operations retain the kernel call structure and external interfaces.
unsafe fn ovl_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t {
    if !iov_iter_count(iter) { return 0; }
    let file = (*iocb).ki_filp; let realfile = ovl_real_file(file); if IS_ERR(realfile) { return PTR_ERR(realfile); }
    let ctx = backing_file_ctx { cred: ovl_creds((*file_inode(file)).i_sb), accessed: Some(ovl_file_accessed), ..core::mem::zeroed() };
    backing_file_read_iter(realfile, iter, iocb, (*iocb).ki_flags, &ctx)
}

unsafe fn ovl_write_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t {
    if !iov_iter_count(iter) { return 0; }
    let file = (*iocb).ki_filp; let inode = file_inode(file); inode_lock(inode); ovl_copyattr(inode);
    let realfile = ovl_real_file(file); let mut ret = PTR_ERR(realfile); if !IS_ERR(realfile) {
        let mut flags = (*iocb).ki_flags; if !ovl_should_sync(OVL_FS((*inode).i_sb)) { flags &= !(IOCB_DSYNC | IOCB_SYNC); }
        let ctx = backing_file_ctx { cred: ovl_creds((*inode).i_sb), end_write: Some(ovl_file_end_write), ..core::mem::zeroed() };
        ret = backing_file_write_iter(realfile, iter, iocb, flags, &ctx);
    }
    inode_unlock(inode); ret
}

unsafe fn ovl_splice_read(input: *mut file, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: usize, flags: c_uint) -> ssize_t {
    let realfile = ovl_real_file(input); if IS_ERR(realfile) { return PTR_ERR(realfile); }
    let mut iocb: kiocb = core::mem::zeroed(); init_sync_kiocb(&mut iocb, input); iocb.ki_pos = *ppos;
    let ctx = backing_file_ctx { cred: ovl_creds((*file_inode(input)).i_sb), accessed: Some(ovl_file_accessed), ..core::mem::zeroed() };
    let ret = backing_file_splice_read(realfile, &mut iocb, pipe, len, flags, &ctx); *ppos = iocb.ki_pos; ret
}

unsafe fn ovl_splice_write(pipe: *mut pipe_inode_info, out: *mut file, ppos: *mut loff_t, len: usize, flags: c_uint) -> ssize_t {
    let inode = file_inode(out); inode_lock(inode); ovl_copyattr(inode);
    let realfile = ovl_real_file(out); let mut ret = PTR_ERR(realfile);
    if !IS_ERR(realfile) {
        let mut iocb: kiocb = core::mem::zeroed(); init_sync_kiocb(&mut iocb, out); iocb.ki_pos = *ppos;
        let ctx = backing_file_ctx { cred: ovl_creds((*inode).i_sb), end_write: Some(ovl_file_end_write), ..core::mem::zeroed() };
        ret = backing_file_splice_write(pipe, realfile, &mut iocb, len, flags, &ctx); *ppos = iocb.ki_pos;
    }
    inode_unlock(inode); ret
}

unsafe fn ovl_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int {
    let ret = ovl_sync_status(OVL_FS((*file_inode(file)).i_sb)); if ret <= 0 { return ret; }
    let dentry = file_dentry(file); let ty = ovl_path_type(dentry);
    if !OVL_TYPE_UPPER(ty) || datasync != 0 && OVL_TYPE_MERGE(ty) { return 0; }
    let mut upperpath: path = core::mem::zeroed(); ovl_path_upper(dentry, &mut upperpath);
    let upperfile = ovl_real_file_path(file, &upperpath); if IS_ERR(upperfile) { return PTR_ERR(upperfile); }
    with_ovl_creds((*file_inode(file)).i_sb, vfs_fsync_range(upperfile, start, end, datasync))
}

unsafe fn ovl_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let of = (*file).private_data as *mut ovl_file;
    let ctx = backing_file_ctx { cred: ovl_creds((*file_inode(file)).i_sb), accessed: Some(ovl_file_accessed), ..core::mem::zeroed() };
    backing_file_mmap((*of).realfile, vma, &ctx)
}

unsafe fn ovl_fallocate(file: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_int {
    let inode = file_inode(file); inode_lock(inode); ovl_copyattr(inode);
    let mut ret = file_remove_privs(file); if ret == 0 { let realfile = ovl_real_file(file); ret = PTR_ERR(realfile); if !IS_ERR(realfile) { ret = with_ovl_creds((*inode).i_sb, vfs_fallocate(realfile, mode, offset, len)); ovl_file_modified(file); } }
    inode_unlock(inode); ret
}

unsafe fn ovl_fadvise(file: *mut file, offset: loff_t, len: loff_t, advice: c_int) -> c_int { let f = ovl_real_file(file); if IS_ERR(f) { return PTR_ERR(f); } with_ovl_creds((*file_inode(file)).i_sb, vfs_fadvise(f, offset, len, advice)) }

#[repr(C)] pub enum ovl_copyop { OVL_COPY, OVL_CLONE, OVL_DEDUPE }

unsafe fn ovl_copy_file_range(a: *mut file, ia: loff_t, b: *mut file, ib: loff_t, len: usize, flags: c_uint) -> ssize_t {
    let ra = ovl_real_file(a); if IS_ERR(ra) { return PTR_ERR(ra); } let rb = ovl_real_file(b); if IS_ERR(rb) { return PTR_ERR(rb); }
    with_ovl_creds((*file_inode(b)).i_sb, vfs_copy_file_range(ra, ia, rb, ib, len, flags))
}

unsafe fn ovl_flush(file: *mut file, id: fl_owner_t) -> c_int { let f = ovl_real_file(file); if IS_ERR(f) { return PTR_ERR(f); } if let Some(flush) = (*(*f).f_op).flush { with_ovl_creds((*file_inode(file)).i_sb, flush(f, id)) } else { 0 } }

#[no_mangle] pub static ovl_file_operations: file_operations = file_operations { open: Some(ovl_open), release: Some(ovl_release), llseek: Some(ovl_llseek), read_iter: Some(ovl_read_iter), write_iter: Some(ovl_write_iter), fsync: Some(ovl_fsync), mmap: Some(ovl_mmap), fallocate: Some(ovl_fallocate), fadvise: Some(ovl_fadvise), flush: Some(ovl_flush), splice_read: Some(ovl_splice_read), splice_write: Some(ovl_splice_write), ..unsafe { core::mem::zeroed() } };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
