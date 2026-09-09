// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext2/ioctl.c
 *
 * Copyright (C) 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 */

// Dependencies supplied by the surrounding kernel/ext2 translation.

pub unsafe fn ext2_fileattr_get(
    dentry: *mut dentry,
    fa: *mut file_kattr,
) -> i32 {
    let ei: *mut ext2_inode_info = EXT2_I(d_inode(dentry));

    fileattr_fill_flags(fa, (*ei).i_flags & EXT2_FL_USER_VISIBLE);

    0
}

pub unsafe fn ext2_fileattr_set(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    fa: *mut file_kattr,
) -> i32 {
    let inode: *mut inode = d_inode(dentry);
    let ei: *mut ext2_inode_info = EXT2_I(inode);

    if fileattr_has_fsx(fa) {
        return -EOPNOTSUPP;
    }

    /* Is it quota file? Do not allow user to mess with it */
    if IS_NOQUOTA(inode) {
        return -EPERM;
    }

    (*ei).i_flags = ((*ei).i_flags & !EXT2_FL_USER_MODIFIABLE)
        | ((*fa).flags & EXT2_FL_USER_MODIFIABLE);

    ext2_set_inode_flags(inode);
    inode_set_ctime_current(inode);
    mark_inode_dirty(inode);

    0
}

pub unsafe fn ext2_ioctl(
    filp: *mut file,
    cmd: u32,
    arg: c_ulong,
) -> c_long {
    let inode: *mut inode = file_inode(filp);
    let ei: *mut ext2_inode_info = EXT2_I(inode);
    let mut rsv_window_size: u16;
    let mut ret: i32;

    ext2_debug!("cmd = %u, arg = %lu\n", cmd, arg);

    match cmd {
        EXT2_IOC_GETVERSION => {
            return put_user((*inode).i_generation, arg as *mut i32);
        }
        EXT2_IOC_SETVERSION => {
            let mut generation: u32 = 0;

            if !inode_owner_or_capable(&nop_mnt_idmap, inode) {
                return -EPERM as c_long;
            }
            ret = mnt_want_write_file(filp);
            if ret != 0 {
                return ret as c_long;
            }
            if get_user(&mut generation, arg as *mut i32) != 0 {
                ret = -EFAULT;
                mnt_drop_write_file(filp);
                return ret as c_long;
            }

            inode_lock(inode);
            inode_set_ctime_current(inode);
            (*inode).i_generation = generation;
            inode_unlock(inode);

            mark_inode_dirty(inode);
            mnt_drop_write_file(filp);
            return ret as c_long;
        }
        EXT2_IOC_GETRSVSZ => {
            if test_opt((*inode).i_sb, RESERVATION)
                && S_ISREG((*inode).i_mode)
                && !(*ei).i_block_alloc_info.is_null()
            {
                rsv_window_size = (*(*ei).i_block_alloc_info).rsv_window_node.rsv_goal_size;
                return put_user(rsv_window_size, arg as *mut i32);
            }
            return -ENOTTY as c_long;
        }
        EXT2_IOC_SETRSVSZ => {
            if !test_opt((*inode).i_sb, RESERVATION) || !S_ISREG((*inode).i_mode) {
                return -ENOTTY as c_long;
            }

            if !inode_owner_or_capable(&nop_mnt_idmap, inode) {
                return -EACCES as c_long;
            }

            if get_user(&mut rsv_window_size, arg as *mut i32) != 0 {
                return -EFAULT as c_long;
            }

            ret = mnt_want_write_file(filp);
            if ret != 0 {
                return ret as c_long;
            }

            if rsv_window_size > EXT2_MAX_RESERVE_BLOCKS {
                rsv_window_size = EXT2_MAX_RESERVE_BLOCKS;
            }

            /*
             * need to allocate reservation structure for this inode
             * before set the window size
             */
            /*
             * XXX What lock should protect the rsv_goal_size?
             * Accessed in ext2_get_block only.  ext3 uses i_truncate.
             */
            mutex_lock(&mut (*ei).truncate_mutex);
            if (*ei).i_block_alloc_info.is_null() {
                ext2_init_block_alloc_info(inode);
            }

            if !(*ei).i_block_alloc_info.is_null() {
                let rsv: *mut ext2_reserve_window_node =
                    &mut (*(*ei).i_block_alloc_info).rsv_window_node;
                (*rsv).rsv_goal_size = rsv_window_size;
            } else {
                ret = -ENOMEM;
            }

            mutex_unlock(&mut (*ei).truncate_mutex);
            mnt_drop_write_file(filp);
            return ret as c_long;
        }
        _ => -ENOTTY as c_long,
    }
}

// CONFIG_COMPAT
#[cfg(CONFIG_COMPAT)]
pub unsafe fn ext2_compat_ioctl(
    file: *mut file,
    mut cmd: u32,
    arg: c_ulong,
) -> c_long {
    /* These are just misnamed, they actually get/put from/to user an int */
    match cmd {
        EXT2_IOC32_GETVERSION => cmd = EXT2_IOC_GETVERSION,
        EXT2_IOC32_SETVERSION => cmd = EXT2_IOC_SETVERSION,
        _ => return -ENOIOCTLCMD as c_long,
    }
    ext2_ioctl(file, cmd, compat_ptr(arg) as c_ulong)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
