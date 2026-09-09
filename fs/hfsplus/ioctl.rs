// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hfsplus/ioctl.c
 *
 * Copyright (C) 2003
 * Ethan Benson <erbenson@alaska.net>
 * partially derived from linux/fs/ext2/ioctl.c
 * Copyright (C) 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 *
 * hfsplus ioctls
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * "Blessing" an HFS+ filesystem writes metadata to the superblock informing
 * the platform firmware which file to boot from
 */
unsafe fn hfsplus_ioctl_bless(file: *mut file, _user_flags: *mut i32) -> i32 {
    let dentry: *mut dentry = (*file).f_path.dentry;
    let inode: *mut inode = d_inode(dentry);
    let sbi: *mut hfsplus_sb_info = HFSPLUS_SB((*inode).i_sb);
    let vh: *mut hfsplus_vh = (*sbi).s_vhdr;
    let bvh: *mut hfsplus_vh = (*sbi).s_backup_vhdr;
    let cnid: u32 = (*dentry).d_fsdata as usize as u32;

    if capable(CAP_SYS_ADMIN) == 0 {
        return -EPERM;
    }

    mutex_lock(&mut (*sbi).vh_mutex);

    /* Directory containing the bootable system */
    (*vh).finder_info[0] = cpu_to_be32(d_parent_ino(dentry));
    (*bvh).finder_info[0] = (*vh).finder_info[0];

    /*
     * Bootloader. Just using the inode here breaks in the case of
     * hard links - the firmware wants the ID of the hard link file,
     * but the inode points at the indirect inode
     */
    (*vh).finder_info[1] = cpu_to_be32(cnid);
    (*bvh).finder_info[1] = (*vh).finder_info[1];

    /* Per spec, the OS X system folder - same as finder_info[0] here */
    (*vh).finder_info[5] = cpu_to_be32(d_parent_ino(dentry));
    (*bvh).finder_info[5] = (*vh).finder_info[5];

    mutex_unlock(&mut (*sbi).vh_mutex);
    0
}

unsafe fn hfsplus_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize {
    let argp: *mut core::ffi::c_void = arg as *mut core::ffi::c_void;

    match cmd {
        HFSPLUS_IOC_BLESS => hfsplus_ioctl_bless(file, argp as *mut i32) as isize,
        _ => -ENOTTY as isize,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
