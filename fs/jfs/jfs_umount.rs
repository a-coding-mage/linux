// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) International Business Machines Corp., 2000-2004
 */

/*
 *	jfs_umount.c
 *
 * note: file system in transition to aggregate/fileset:
 * (ref. jfs_mount.c)
 *
 * file system unmount is interpreted as mount of the single/only
 * fileset in the aggregate and, if unmount of the last fileset,
 * as unmount of the aggerate;
 */

// C header dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    pub i_mapping: *mut address_space,
}

#[repr(C)]
pub struct address_space {
    _private: [u8; 0],
}

#[repr(C)]
pub struct jfs_log {
    _private: [u8; 0],
}

#[repr(C)]
pub struct jfs_sb_info {
    pub ipbmap: *mut inode,
    pub ipimap: *mut inode,
    pub ipaimap: *mut inode,
    pub ipaimap2: *mut inode,
    pub log: *mut jfs_log,
    pub direct_inode: *mut inode,
}

pub const FM_CLEAN: i32 = 0; // External filesystem constant.

extern "C" {
    pub fn JFS_SBI(sb: *mut super_block) -> *mut jfs_sb_info;
    pub fn jfs_info(fmt: *const u8, ...);
    pub fn jfs_flush_journal(log: *mut jfs_log, n: i32);
    pub fn LOG_LOCK(log: *mut jfs_log);
    pub fn LOG_UNLOCK(log: *mut jfs_log);
    pub fn diUnmount(ip: *mut inode, flag: i32);
    pub fn diFreeSpecial(ip: *mut inode);
    pub fn dbUnmount(ip: *mut inode, flag: i32);
    pub fn filemap_write_and_wait(mapping: *mut address_space) -> i32;
    pub fn updateSuper(sb: *mut super_block, state: i32);
    pub fn lmLogClose(sb: *mut super_block) -> i32;
    pub fn dbSync(ip: *mut inode);
    pub fn diSync(ip: *mut inode);
}

/*
 * NAME: jfs_umount(vfsp, flags, crp)
 *
 * FUNCTION: vfs_umount()
 *
 * PARAMETERS: vfsp - virtual file system pointer
 *             flags - unmount for shutdown
 *             crp - credential
 *
 * RETURN : EBUSY - device has open files
 */
pub unsafe fn jfs_umount(sb: *mut super_block) -> i32 {
    let sbi = JFS_SBI(sb);
    let ipbmap = (*sbi).ipbmap;
    let ipimap = (*sbi).ipimap;
    let ipaimap = (*sbi).ipaimap;
    let ipaimap2 = (*sbi).ipaimap2;
    let log = (*sbi).log;
    let mut rc: i32 = 0;

    jfs_info(b"UnMount JFS: sb:0x%p\0".as_ptr(), sb);

    /* update superblock and close log, if mounted read-write and recovery enabled */
    if !log.is_null() {
        /* Wait for outstanding transactions to be written to log. */
        jfs_flush_journal(log, 2);
    }

    /* Hold log lock so write_special_inodes cannot see a NULL inode pointer. */
    if !log.is_null() {
        LOG_LOCK(log);
    }

    /* close fileset inode allocation map (aka fileset inode) */
    diUnmount(ipimap, 0);
    diFreeSpecial(ipimap);
    (*sbi).ipimap = core::ptr::null_mut();

    /* close secondary aggregate inode allocation map */
    if !ipaimap2.is_null() {
        diUnmount(ipaimap2, 0);
        diFreeSpecial(ipaimap2);
        (*sbi).ipaimap2 = core::ptr::null_mut();
    }

    /* close aggregate inode allocation map */
    diUnmount(ipaimap, 0);
    diFreeSpecial(ipaimap);
    (*sbi).ipaimap = core::ptr::null_mut();

    /* close aggregate block allocation map */
    dbUnmount(ipbmap, 0);
    diFreeSpecial(ipbmap);
    (*sbi).ipbmap = core::ptr::null_mut();

    /* Make sure all metadata makes it to disk before marking the superblock clean. */
    filemap_write_and_wait((*(*sbi).direct_inode).i_mapping);

    if !log.is_null() {
        LOG_UNLOCK(log);
    }

    /* log is NULL if read-only mount */
    if !log.is_null() {
        updateSuper(sb, FM_CLEAN);
        rc = lmLogClose(sb);
    }

    jfs_info(b"UnMount JFS Complete: rc = %d\0".as_ptr(), rc);
    rc
}

pub unsafe fn jfs_umount_rw(sb: *mut super_block) -> i32 {
    let sbi = JFS_SBI(sb);
    let log = (*sbi).log;

    if log.is_null() {
        return 0;
    }

    /* close log: remove file system from log active file system list. */
    jfs_flush_journal(log, 2);

    /* Make sure all metadata makes it to disk. */
    dbSync((*sbi).ipbmap);
    diSync((*sbi).ipimap);

    /* We cannot mark the superblock clean before everything is flushed to disk. */
    filemap_write_and_wait((*(*sbi).direct_inode).i_mapping);

    updateSuper(sb, FM_CLEAN);
    lmLogClose(sb)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
