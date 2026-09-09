// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/jfs/ioctl.c
 *
 * Copyright (C) 2006 Herbert Poetzl
 * adapted from Remy Card's ext2/ioctl.c
 */

// Kernel headers and JFS headers from the original translation unit provide
// the types, constants, macros, and external functions referenced below.

#[repr(C)]
struct JfsMap {
    jfs_flag: libc::c_long,
    ext2_flag: libc::c_long,
}

static mut JFS_MAP: [JfsMap; 8] = [
    JfsMap { jfs_flag: JFS_NOATIME_FL as libc::c_long, ext2_flag: FS_NOATIME_FL as libc::c_long },
    JfsMap { jfs_flag: JFS_DIRSYNC_FL as libc::c_long, ext2_flag: FS_DIRSYNC_FL as libc::c_long },
    JfsMap { jfs_flag: JFS_SYNC_FL as libc::c_long, ext2_flag: FS_SYNC_FL as libc::c_long },
    JfsMap { jfs_flag: JFS_SECRM_FL as libc::c_long, ext2_flag: FS_SECRM_FL as libc::c_long },
    JfsMap { jfs_flag: JFS_UNRM_FL as libc::c_long, ext2_flag: FS_UNRM_FL as libc::c_long },
    JfsMap { jfs_flag: JFS_APPEND_FL as libc::c_long, ext2_flag: FS_APPEND_FL as libc::c_long },
    JfsMap { jfs_flag: JFS_IMMUTABLE_FL as libc::c_long, ext2_flag: FS_IMMUTABLE_FL as libc::c_long },
    JfsMap { jfs_flag: 0, ext2_flag: 0 },
];

unsafe fn jfs_map_ext2(flags: libc::c_ulong, from: libc::c_int) -> libc::c_long {
    let mut index: usize = 0;
    let mut mapped: libc::c_long = 0;

    while JFS_MAP[index].jfs_flag != 0 {
        if from != 0 {
            if (JFS_MAP[index].ext2_flag as libc::c_ulong) & flags != 0 {
                mapped |= JFS_MAP[index].jfs_flag;
            }
        } else if (JFS_MAP[index].jfs_flag as libc::c_ulong) & flags != 0 {
            mapped |= JFS_MAP[index].ext2_flag;
        }
        index += 1;
    }
    mapped
}

pub unsafe fn jfs_fileattr_get(
    dentry: *mut struct_dentry,
    fa: *mut struct_file_kattr,
) -> libc::c_int {
    let jfs_inode = JFS_IP(d_inode(dentry));
    let flags = (*jfs_inode).mode2 & JFS_FL_USER_VISIBLE;

    if d_is_special(dentry) {
        return -ENOTTY;
    }

    fileattr_fill_flags(fa, jfs_map_ext2(flags as libc::c_ulong, 0));
    0
}

pub unsafe fn jfs_fileattr_set(
    idmap: *mut struct_mnt_idmap,
    dentry: *mut struct_dentry,
    fa: *mut struct_file_kattr,
) -> libc::c_int {
    let _ = idmap;
    let inode = d_inode(dentry);
    let jfs_inode = JFS_IP(inode);
    let mut flags: libc::c_uint;

    if d_is_special(dentry) {
        return -ENOTTY;
    }
    if fileattr_has_fsx(fa) {
        return -EOPNOTSUPP;
    }

    flags = jfs_map_ext2((*fa).flags as libc::c_ulong, 1) as libc::c_uint;
    if !S_ISDIR((*inode).i_mode) {
        flags &= !JFS_DIRSYNC_FL;
    }

    /* Is it quota file? Do not allow user to mess with it */
    if IS_NOQUOTA(inode) {
        return -EPERM;
    }

    flags &= JFS_FL_USER_MODIFIABLE;
    flags |= (*jfs_inode).mode2 & !JFS_FL_USER_MODIFIABLE;
    (*jfs_inode).mode2 = flags;

    jfs_set_inode_flags(inode);
    inode_set_ctime_current(inode);
    mark_inode_dirty(inode);
    0
}

pub unsafe fn jfs_ioctl(
    filp: *mut struct_file,
    cmd: libc::c_uint,
    arg: libc::c_ulong,
) -> libc::c_long {
    let inode = file_inode(filp);

    match cmd {
        FITRIM => {
            let sb = (*inode).i_sb;
            let mut range: struct_fstrim_range = core::mem::zeroed();
            let mut ret: i64 = 0;

            if !capable(CAP_SYS_ADMIN) {
                return -EPERM as libc::c_long;
            }
            if bdev_max_discard_sectors((*sb).s_bdev) == 0 {
                jfs_warn("FITRIM not supported on device");
                return -EOPNOTSUPP as libc::c_long;
            }
            if copy_from_user(&mut range as *mut _ as *mut core::ffi::c_void,
                              arg as *const core::ffi::c_void,
                              core::mem::size_of::<struct_fstrim_range>()) != 0 {
                return -EFAULT as libc::c_long;
            }

            range.minlen = core::cmp::max(
                range.minlen,
                bdev_discard_granularity((*sb).s_bdev),
            );
            ret = jfs_ioc_trim(inode, &mut range);
            if ret < 0 {
                return ret as libc::c_long;
            }
            if copy_to_user(arg as *mut core::ffi::c_void,
                            &range as *const _ as *const core::ffi::c_void,
                            core::mem::size_of::<struct_fstrim_range>()) != 0 {
                return -EFAULT as libc::c_long;
            }
            0
        }
        _ => -ENOTTY as libc::c_long,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
