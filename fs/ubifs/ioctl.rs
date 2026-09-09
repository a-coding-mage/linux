// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file is part of UBIFS.
 *
 * Copyright (C) 2006-2008 Nokia Corporation.
 * Copyright (C) 2006, 2007 University of Szeged, Hungary
 *
 * Authors: Zoltan Sogor
 *          Artem Bityutskiy (Битюцкий Артём)
 *          Adrian Hunter
 */

/* This file implements EXT2-compatible extended attribute ioctl() calls */

// Dependency declarations are supplied by the surrounding UBIFS/kernel translation.

/* Need to be kept consistent with checked flags in ioctl2ubifs() */
const UBIFS_SETTABLE_IOCTL_FLAGS: u32 =
    FS_COMPR_FL | FS_SYNC_FL | FS_APPEND_FL | FS_IMMUTABLE_FL | FS_DIRSYNC_FL;

/* Need to be kept consistent with checked flags in ubifs2ioctl() */
const UBIFS_GETTABLE_IOCTL_FLAGS: u32 = UBIFS_SETTABLE_IOCTL_FLAGS | FS_ENCRYPT_FL;

/**
 * ubifs_set_inode_flags - set VFS inode flags.
 * @inode: VFS inode to set flags for
 *
 * This function propagates flags from UBIFS inode object to VFS inode object.
 */
pub unsafe fn ubifs_set_inode_flags(inode: *mut inode) {
    let flags = (*ubifs_inode(inode)).flags;

    (*inode).i_flags &= !(S_SYNC | S_APPEND | S_IMMUTABLE | S_DIRSYNC | S_ENCRYPTED);
    if flags & UBIFS_SYNC_FL != 0 {
        (*inode).i_flags |= S_SYNC;
    }
    if flags & UBIFS_APPEND_FL != 0 {
        (*inode).i_flags |= S_APPEND;
    }
    if flags & UBIFS_IMMUTABLE_FL != 0 {
        (*inode).i_flags |= S_IMMUTABLE;
    }
    if flags & UBIFS_DIRSYNC_FL != 0 {
        (*inode).i_flags |= S_DIRSYNC;
    }
    if flags & UBIFS_CRYPT_FL != 0 {
        (*inode).i_flags |= S_ENCRYPTED;
    }
}

/*
 * ioctl2ubifs - convert ioctl inode flags to UBIFS inode flags.
 * @ioctl_flags: flags to convert
 *
 * This function converts ioctl flags (@FS_COMPR_FL, etc) to UBIFS inode flags
 * (@UBIFS_COMPR_FL, etc).
 */
unsafe fn ioctl2ubifs(ioctl_flags: i32) -> i32 {
    let mut ubifs_flags = 0;
    if ioctl_flags & FS_COMPR_FL as i32 != 0 { ubifs_flags |= UBIFS_COMPR_FL as i32; }
    if ioctl_flags & FS_SYNC_FL as i32 != 0 { ubifs_flags |= UBIFS_SYNC_FL as i32; }
    if ioctl_flags & FS_APPEND_FL as i32 != 0 { ubifs_flags |= UBIFS_APPEND_FL as i32; }
    if ioctl_flags & FS_IMMUTABLE_FL as i32 != 0 { ubifs_flags |= UBIFS_IMMUTABLE_FL as i32; }
    if ioctl_flags & FS_DIRSYNC_FL as i32 != 0 { ubifs_flags |= UBIFS_DIRSYNC_FL as i32; }
    ubifs_flags
}

/*
 * ubifs2ioctl - convert UBIFS inode flags to ioctl inode flags.
 * @ubifs_flags: flags to convert
 *
 * This function converts UBIFS inode flags (@UBIFS_COMPR_FL, etc) to ioctl
 * flags (@FS_COMPR_FL, etc).
 */
unsafe fn ubifs2ioctl(ubifs_flags: i32) -> i32 {
    let mut ioctl_flags = 0;
    if ubifs_flags & UBIFS_COMPR_FL as i32 != 0 { ioctl_flags |= FS_COMPR_FL as i32; }
    if ubifs_flags & UBIFS_SYNC_FL as i32 != 0 { ioctl_flags |= FS_SYNC_FL as i32; }
    if ubifs_flags & UBIFS_APPEND_FL as i32 != 0 { ioctl_flags |= FS_APPEND_FL as i32; }
    if ubifs_flags & UBIFS_IMMUTABLE_FL as i32 != 0 { ioctl_flags |= FS_IMMUTABLE_FL as i32; }
    if ubifs_flags & UBIFS_DIRSYNC_FL as i32 != 0 { ioctl_flags |= FS_DIRSYNC_FL as i32; }
    if ubifs_flags & UBIFS_CRYPT_FL as i32 != 0 { ioctl_flags |= FS_ENCRYPT_FL as i32; }
    ioctl_flags
}

unsafe fn setflags(inode: *mut inode, flags: i32) -> i32 {
    let mut err: i32;
    let release: i32;
    let ui = ubifs_inode(inode);
    let c = (*(*inode).i_sb).s_fs_info as *mut ubifs_info;
    let mut req = ubifs_budget_req {
        dirtied_ino: 1,
        dirtied_ino_d: ALIGN((*ui).data_len, 8),
    };

    err = ubifs_budget_space(c, &mut req);
    if err != 0 { return err; }

    mutex_lock(&mut (*ui).ui_mutex);
    (*ui).flags &= !ioctl2ubifs(UBIFS_SETTABLE_IOCTL_FLAGS as i32);
    (*ui).flags |= ioctl2ubifs(flags);
    ubifs_set_inode_flags(inode);
    inode_set_ctime_current(inode);
    release = (*ui).dirty;
    mark_inode_dirty_sync(inode);
    mutex_unlock(&mut (*ui).ui_mutex);

    if release != 0 { ubifs_release_budget(c, &mut req); }
    if IS_SYNC(inode) { err = write_inode_now(inode, 1); }
    err
}

pub unsafe fn ubifs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> i32 {
    let inode = d_inode(dentry);
    let flags = ubifs2ioctl((*ubifs_inode(inode)).flags);
    if d_is_special(dentry) { return -ENOTTY; }
    dbg_gen!("get flags: %#x, i_flags %#x", flags, (*inode).i_flags);
    fileattr_fill_flags(fa, flags);
    0
}

pub unsafe fn ubifs_fileattr_set(
    _idmap: *mut mnt_idmap, dentry: *mut dentry, fa: *mut file_kattr,
) -> i32 {
    let inode = d_inode(dentry);
    let mut flags = (*fa).flags;
    if d_is_special(dentry) { return -ENOTTY; }
    if fileattr_has_fsx(fa) { return -EOPNOTSUPP; }
    if flags & !(UBIFS_GETTABLE_IOCTL_FLAGS as i32) != 0 { return -EOPNOTSUPP; }
    flags &= UBIFS_SETTABLE_IOCTL_FLAGS as i32;
    if !S_ISDIR((*inode).i_mode) { flags &= !(FS_DIRSYNC_FL as i32); }
    dbg_gen!("set flags: %#x, i_flags %#x", flags, (*inode).i_flags);
    setflags(inode, flags)
}

pub unsafe fn ubifs_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize {
    let inode = file_inode(file);
    match cmd {
        FS_IOC_SET_ENCRYPTION_POLICY => {
            let c = (*(*inode).i_sb).s_fs_info as *mut ubifs_info;
            let err = ubifs_enable_encryption(c);
            if err != 0 { return err as isize; }
            fscrypt_ioctl_set_policy(file, arg as *const core::ffi::c_void)
        }
        FS_IOC_GET_ENCRYPTION_POLICY => fscrypt_ioctl_get_policy(file, arg as *mut core::ffi::c_void),
        FS_IOC_GET_ENCRYPTION_POLICY_EX => fscrypt_ioctl_get_policy_ex(file, arg as *mut core::ffi::c_void),
        FS_IOC_ADD_ENCRYPTION_KEY => fscrypt_ioctl_add_key(file, arg as *mut core::ffi::c_void),
        FS_IOC_REMOVE_ENCRYPTION_KEY => fscrypt_ioctl_remove_key(file, arg as *mut core::ffi::c_void),
        FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS => fscrypt_ioctl_remove_key_all_users(file, arg as *mut core::ffi::c_void),
        FS_IOC_GET_ENCRYPTION_KEY_STATUS => fscrypt_ioctl_get_key_status(file, arg as *mut core::ffi::c_void),
        FS_IOC_GET_ENCRYPTION_NONCE => fscrypt_ioctl_get_nonce(file, arg as *mut core::ffi::c_void),
        _ => -ENOTTY as isize,
    }
}

#[cfg(CONFIG_COMPAT)]
pub unsafe fn ubifs_compat_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize {
    match cmd {
        FS_IOC_SET_ENCRYPTION_POLICY | FS_IOC_GET_ENCRYPTION_POLICY |
        FS_IOC_GET_ENCRYPTION_POLICY_EX | FS_IOC_ADD_ENCRYPTION_KEY |
        FS_IOC_REMOVE_ENCRYPTION_KEY | FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS |
        FS_IOC_GET_ENCRYPTION_KEY_STATUS | FS_IOC_GET_ENCRYPTION_NONCE => {}
        _ => return -ENOIOCTLCMD as isize,
    }
    ubifs_ioctl(file, cmd, compat_ptr(arg) as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
