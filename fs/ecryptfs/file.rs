// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * eCryptfs: Linux filesystem encryption layer
 *
 * Copyright (C) 1997-2004 Erez Zadok
 * Copyright (C) 2001-2004 Stony Brook University
 * Copyright (C) 2004-2007 International Business Machines Corp.
 *   Author(s): Michael A. Halcrow <mhalcrow@us.ibm.com>
 *             Michael C. Thompson <mcthomps@us.ibm.com>
 */

// Linux kernel headers and ecryptfs_kernel.h supply the external types,
// constants, globals, macros, and functions referenced below.

unsafe fn ecryptfs_read_update_atime(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp;
    let rc = generic_file_read_iter(iocb, to);
    if rc >= 0 {
        let path = ecryptfs_lower_path((*file).f_path.dentry);
        touch_atime(&path);
    }
    rc
}

unsafe fn ecryptfs_splice_read_update_atime(
    input: *mut file,
    ppos: *mut loff_t,
    pipe: *mut pipe_inode_info,
    len: usize,
    flags: c_uint,
) -> ssize_t {
    let rc = filemap_splice_read(input, ppos, pipe, len, flags);
    if rc >= 0 {
        let path = ecryptfs_lower_path((*input).f_path.dentry);
        touch_atime(&path);
    }
    rc
}

#[repr(C)]
struct ecryptfs_getdents_callback {
    ctx: dir_context,
    caller: *mut dir_context,
    sb: *mut super_block,
    filldir_called: c_int,
    entries_written: c_int,
}

unsafe fn ecryptfs_filldir(
    ctx: *mut dir_context,
    lower_name: *const c_char,
    lower_namelen: c_int,
    offset: loff_t,
    ino: u64,
    d_type: c_uint,
) -> bool {
    let buf = container_of!(ctx, ecryptfs_getdents_callback, ctx);
    let mut name: *mut c_char = core::ptr::null_mut();
    let mut name_size: usize = 0;
    (*buf).filldir_called += 1;
    let err = ecryptfs_decode_and_decrypt_filename(
        &mut name, &mut name_size, (*buf).sb, lower_name, lower_namelen,
    );
    if err != 0 {
        if err != -EINVAL {
            ecryptfs_printk!(KERN_DEBUG,
                "%s: Error attempting to decode and decrypt filename [%s]; rc = [%d]\\n",
                __func__, lower_name, err);
            return false;
        }
        // Mask -EINVAL errors; these are most likely plaintext lower names.
        return true;
    }
    (*buf).caller.as_mut().unwrap().pos = (*buf).ctx.pos;
    let res = dir_emit((*buf).caller, name, name_size, ino, d_type);
    kfree(name);
    if res {
        (*buf).entries_written += 1;
    }
    res
}

unsafe fn ecryptfs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let inode = file_inode(file);
    let mut buf = ecryptfs_getdents_callback {
        ctx: dir_context { actor: Some(ecryptfs_filldir), pos: 0 },
        caller: ctx,
        sb: (*inode).i_sb,
        filldir_called: 0,
        entries_written: 0,
    };
    let lower_file = ecryptfs_file_to_lower(file);
    let rc = iterate_dir(lower_file, &mut buf.ctx);
    (*ctx).pos = buf.ctx.pos;
    if rc >= 0 && (buf.entries_written != 0 || buf.filldir_called == 0) {
        fsstack_copy_attr_atime(inode, file_inode(lower_file));
    }
    rc
}

static mut ecryptfs_file_info_cache: *mut kmem_cache = core::ptr::null_mut();

unsafe fn read_or_initialize_metadata(dentry: *mut dentry) -> c_int {
    let inode = d_inode(dentry);
    let mount_crypt_stat = &mut ecryptfs_superblock_to_private((*inode).i_sb).mount_crypt_stat;
    let crypt_stat = &mut ecryptfs_inode_to_private(inode).crypt_stat;
    mutex_lock(&mut crypt_stat.cs_mutex);
    let mut rc;
    if crypt_stat.flags & ECRYPTFS_POLICY_APPLIED != 0
        && crypt_stat.flags & ECRYPTFS_KEY_VALID != 0 {
        rc = 0;
    } else {
        rc = ecryptfs_read_metadata(dentry);
        if rc == 0 {
        } else if mount_crypt_stat.flags & ECRYPTFS_PLAINTEXT_PASSTHROUGH_ENABLED != 0 {
            crypt_stat.flags &= !(ECRYPTFS_I_SIZE_INITIALIZED | ECRYPTFS_ENCRYPTED);
            rc = 0;
        } else if mount_crypt_stat.flags & ECRYPTFS_XATTR_METADATA_ENABLED == 0
            && i_size_read(ecryptfs_inode_to_lower(inode)) == 0 {
            rc = ecryptfs_initialize_file(dentry, inode);
        } else {
            rc = -EIO;
        }
    }
    mutex_unlock(&mut crypt_stat.cs_mutex);
    rc
}

unsafe fn ecryptfs_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let lower_file = ecryptfs_file_to_lower(file);
    if !can_mmap_file(lower_file) { return -ENODEV; }
    generic_file_mmap(file, vma)
}

unsafe fn ecryptfs_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut rc: c_int = 0;
    let dentry = (*file).f_path.dentry;
    let file_info = kmem_cache_zalloc(ecryptfs_file_info_cache, GFP_KERNEL);
    ecryptfs_set_file_private(file, file_info);
    if file_info.is_null() { rc = -ENOMEM; return rc; }
    let crypt_stat = &mut ecryptfs_inode_to_private(inode).crypt_stat;
    mutex_lock(&mut crypt_stat.cs_mutex);
    if crypt_stat.flags & ECRYPTFS_POLICY_APPLIED == 0 {
        crypt_stat.flags |= ECRYPTFS_POLICY_APPLIED | ECRYPTFS_ENCRYPTED;
    }
    mutex_unlock(&mut crypt_stat.cs_mutex);
    rc = ecryptfs_get_lower_file(dentry, inode);
    if rc != 0 { kmem_cache_free(ecryptfs_file_info_cache, ecryptfs_file_to_private(file)); return rc; }
    if ((*ecryptfs_inode_to_private(inode).lower_file).f_flags & O_ACCMODE) == O_RDONLY
        && (*file).f_flags & O_ACCMODE != O_RDONLY {
        ecryptfs_put_lower_file(inode);
        kmem_cache_free(ecryptfs_file_info_cache, ecryptfs_file_to_private(file));
        return -EPERM;
    }
    ecryptfs_set_file_lower(file, ecryptfs_inode_to_private(inode).lower_file);
    rc = read_or_initialize_metadata(dentry);
    if rc != 0 { ecryptfs_put_lower_file(inode); kmem_cache_free(ecryptfs_file_info_cache, ecryptfs_file_to_private(file)); }
    rc
}

unsafe fn ecryptfs_dir_open(inode: *mut inode, file: *mut file) -> c_int {
    let dentry = (*file).f_path.dentry;
    let file_info = kmem_cache_zalloc(ecryptfs_file_info_cache, GFP_KERNEL);
    ecryptfs_set_file_private(file, file_info);
    if file_info.is_null() { return -ENOMEM; }
    let path = ecryptfs_lower_path(dentry);
    let lower_file = dentry_open(&path, (*file).f_flags, current_cred());
    if IS_ERR(lower_file) { kmem_cache_free(ecryptfs_file_info_cache, file_info); return PTR_ERR(lower_file); }
    ecryptfs_set_file_lower(file, lower_file);
    0
}

unsafe fn ecryptfs_flush(file: *mut file, td: fl_owner_t) -> c_int {
    let lower_file = ecryptfs_file_to_lower(file);
    if (*(*lower_file).f_op).flush.is_some() {
        filemap_write_and_wait((*file).f_mapping);
        return ((*(*lower_file).f_op).flush.unwrap())(lower_file, td);
    }
    0
}

unsafe fn ecryptfs_release(inode: *mut inode, file: *mut file) -> c_int {
    ecryptfs_put_lower_file(inode);
    kmem_cache_free(ecryptfs_file_info_cache, ecryptfs_file_to_private(file));
    0
}

unsafe fn ecryptfs_dir_release(_inode: *mut inode, file: *mut file) -> c_int {
    fput(ecryptfs_file_to_lower(file));
    kmem_cache_free(ecryptfs_file_info_cache, ecryptfs_file_to_private(file));
    0
}

unsafe fn ecryptfs_dir_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    vfs_llseek(ecryptfs_file_to_lower(file), offset, whence)
}

unsafe fn ecryptfs_fsync(file: *mut file, _start: loff_t, _end: loff_t, datasync: c_int) -> c_int {
    let rc = file_write_and_wait(file);
    if rc != 0 { return rc; }
    vfs_fsync(ecryptfs_file_to_lower(file), datasync)
}

unsafe fn ecryptfs_fasync(fd: c_int, file: *mut file, flag: c_int) -> c_int {
    let lower_file = ecryptfs_file_to_lower(file);
    if (*(*lower_file).f_op).fasync.is_some() { ((*(*lower_file).f_op).fasync.unwrap())(fd, lower_file, flag) } else { 0 }
}

unsafe fn ecryptfs_unlocked_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let lower_file = ecryptfs_file_to_lower(file);
    if (*(*lower_file).f_op).unlocked_ioctl.is_none() { return -ENOTTY; }
    match cmd {
        FITRIM | FS_IOC_GETFLAGS | FS_IOC_SETFLAGS | FS_IOC_GETVERSION | FS_IOC_SETVERSION => {
            let rc = ((*(*lower_file).f_op).unlocked_ioctl.unwrap())(lower_file, cmd, arg);
            fsstack_copy_attr_all(file_inode(file), file_inode(lower_file)); rc
        }
        _ => -ENOTTY,
    }
}

#[cfg(CONFIG_COMPAT)]
unsafe fn ecryptfs_compat_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let lower_file = ecryptfs_file_to_lower(file);
    if (*(*lower_file).f_op).compat_ioctl.is_none() { return -ENOIOCTLCMD; }
    match cmd {
        FITRIM | FS_IOC32_GETFLAGS | FS_IOC32_SETFLAGS | FS_IOC32_GETVERSION | FS_IOC32_SETVERSION => {
            let rc = ((*(*lower_file).f_op).compat_ioctl.unwrap())(lower_file, cmd, arg);
            fsstack_copy_attr_all(file_inode(file), file_inode(lower_file)); rc
        }
        _ => -ENOIOCTLCMD,
    }
}

#[no_mangle]
pub static ecryptfs_dir_fops: file_operations = file_operations {
    iterate_shared: Some(ecryptfs_readdir), read: Some(generic_read_dir),
    unlocked_ioctl: Some(ecryptfs_unlocked_ioctl),
    #[cfg(CONFIG_COMPAT)] compat_ioctl: Some(ecryptfs_compat_ioctl),
    open: Some(ecryptfs_dir_open), release: Some(ecryptfs_dir_release),
    fsync: Some(ecryptfs_fsync), llseek: Some(ecryptfs_dir_llseek),
};

#[no_mangle]
pub static ecryptfs_main_fops: file_operations = file_operations {
    llseek: Some(generic_file_llseek), read_iter: Some(ecryptfs_read_update_atime),
    write_iter: Some(generic_file_write_iter), unlocked_ioctl: Some(ecryptfs_unlocked_ioctl),
    #[cfg(CONFIG_COMPAT)] compat_ioctl: Some(ecryptfs_compat_ioctl),
    mmap: Some(ecryptfs_mmap), open: Some(ecryptfs_open), flush: Some(ecryptfs_flush),
    release: Some(ecryptfs_release), fsync: Some(ecryptfs_fsync), fasync: Some(ecryptfs_fasync),
    splice_read: Some(ecryptfs_splice_read_update_atime),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
