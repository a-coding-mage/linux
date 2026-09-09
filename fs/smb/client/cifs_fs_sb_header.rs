/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *
 *   Copyright (c) International Business Machines  Corp., 2002,2004
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *
 */
/* C dependencies: linux/rbtree.h and linux/backing-dev.h. */

pub const CIFS_MOUNT_NO_PERM: u32 = 1; /* do not do client vfs_perm check */
pub const CIFS_MOUNT_SET_UID: u32 = 2; /* set current's euid in create etc. */
pub const CIFS_MOUNT_SERVER_INUM: u32 = 4; /* inode numbers from uniqueid from server */
pub const CIFS_MOUNT_DIRECT_IO: u32 = 8; /* do not write nor read through page cache */
pub const CIFS_MOUNT_NO_XATTR: u32 = 0x10; /* if set - disable xattr support */
pub const CIFS_MOUNT_MAP_SPECIAL_CHR: u32 = 0x20; /* remap illegal chars in filenames */
pub const CIFS_MOUNT_POSIX_PATHS: u32 = 0x40; /* Negotiate posix pathnames if possible */
pub const CIFS_MOUNT_UNX_EMUL: u32 = 0x80; /* Network compat with SFUnix emulation */
pub const CIFS_MOUNT_NO_BRL: u32 = 0x100; /* No sending byte range locks to srv */
pub const CIFS_MOUNT_CIFS_ACL: u32 = 0x200; /* send ACL requests to non-POSIX srv */
pub const CIFS_MOUNT_OVERR_UID: u32 = 0x400; /* override uid returned from server */
pub const CIFS_MOUNT_OVERR_GID: u32 = 0x800; /* override gid returned from server */
pub const CIFS_MOUNT_DYNPERM: u32 = 0x1000; /* allow in-memory only mode setting */
pub const CIFS_MOUNT_NOPOSIXBRL: u32 = 0x2000; /* mandatory not posix byte range lock */
pub const CIFS_MOUNT_NOSSYNC: u32 = 0x4000; /* don't do slow SMBflush on every sync */
pub const CIFS_MOUNT_FSCACHE: u32 = 0x8000; /* local caching enabled */
pub const CIFS_MOUNT_MF_SYMLINKS: u32 = 0x10000; /* Minshall+French Symlinks enabled */
pub const CIFS_MOUNT_MULTIUSER: u32 = 0x20000; /* multiuser mount */
pub const CIFS_MOUNT_STRICT_IO: u32 = 0x40000; /* strict cache mode */
pub const CIFS_MOUNT_RWPIDFORWARD: u32 = 0x80000; /* use pid forwarding for rw */
pub const CIFS_MOUNT_POSIXACL: u32 = 0x100000; /* mirror of SB_POSIXACL in mnt_cifs_flags */
pub const CIFS_MOUNT_CIFS_BACKUPUID: u32 = 0x200000; /* backup intent bit for a user */
pub const CIFS_MOUNT_CIFS_BACKUPGID: u32 = 0x400000; /* backup intent bit for a group */
pub const CIFS_MOUNT_MAP_SFM_CHR: u32 = 0x800000; /* SFM/MAC mapping for illegal chars */
pub const CIFS_MOUNT_USE_PREFIX_PATH: u32 = 0x1000000; /* make subpath with unaccessible
                                                          * root mountable
                                                          */
pub const CIFS_MOUNT_UID_FROM_ACL: u32 = 0x2000000; /* try to get UID via special SID */
pub const CIFS_MOUNT_NO_HANDLE_CACHE: u32 = 0x4000000; /* disable caching dir handles */
pub const CIFS_MOUNT_NO_DFS: u32 = 0x8000000; /* disable DFS resolving */
pub const CIFS_MOUNT_MODE_FROM_SID: u32 = 0x10000000; /* retrieve mode from special ACE */
pub const CIFS_MOUNT_RO_CACHE: u32 = 0x20000000; /* assumes share will not change */
pub const CIFS_MOUNT_RW_CACHE: u32 = 0x40000000; /* assumes only client accessing */
pub const CIFS_MOUNT_SHUTDOWN: u32 = 0x80000000;

#[repr(C)]
pub struct cifs_sb_info {
    pub tlink_tree: rb_root,
    pub tcon_sb_link: list_head,
    pub tlink_tree_lock: spinlock_t,
    pub master_tlink: *mut tcon_link,
    pub local_nls: *mut nls_table,
    pub ctx: *mut smb3_fs_context,
    pub active: atomic_t,
    pub mnt_cifs_flags: atomic_t,
    pub outstanding_rreq: atomic_t, /* nr of rreqs not yet fully deinitialized */
    pub prune_tlinks: delayed_work,
    pub rcu: rcu_head,

    /* only used when CIFS_MOUNT_USE_PREFIX_PATH is set */
    pub prepath: *mut ::std::os::raw::c_char,

    /*
     * Indicate whether serverino option was turned off later
     * (cifs_autodisable_serverino) in order to match new mounts.
     */
    pub mnt_cifs_serverino_autodisabled: bool,
    /*
     * Available once the mount has completed.
     */
    pub root: *mut dentry,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
