/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2004
 *   Portions Copyright (C) Christoph Hellwig, 2001-2002
 */

// Dependencies supplied by the surrounding translation unit:
// linux mutex/rwsem/slab/bitops/uuid and jfs_types/jfs_xtree/jfs_dtree.

/* JFS magic number */
pub const JFS_SUPER_MAGIC: u32 = 0x3153_464a; // "JFS1"

/* JFS-private inode information */
#[repr(C)]
pub struct jfs_inode_info {
    pub fileset: i32,
    pub mode2: u32,
    pub saved_uid: kuid_t,
    pub saved_gid: kgid_t,
    pub ixpxd: pxd_t,
    pub acl: dxd_t,
    pub ea: dxd_t,
    pub otime: time64_t,
    pub next_index: u32,
    pub acltype: i32,
    pub btorder: i16,
    pub btindex: i16,
    pub ipimap: *mut inode,
    pub cflag: usize,
    pub agstart: u64,
    pub bxflag: u16,
    pub pad: u8,
    pub active_ag: i8,
    pub blid: lid_t,
    pub atlhead: lid_t,
    pub atltail: lid_t,
    pub ag_lock: spinlock_t,
    pub anon_inode_list: list_head,
    pub rdwrlock: rw_semaphore,
    pub commit_mutex: mutex,
    pub xattr_sem: rw_semaphore,
    pub xtlid: lid_t,
    pub u: jfs_inode_union,
    // CONFIG_QUOTA: struct dquot __rcu *i_dquot[MAXQUOTAS];
    pub dev: u32,
    pub vfs_inode: inode,
}

#[repr(C)]
pub union jfs_inode_union {
    pub file: jfs_inode_file,
    pub dir: jfs_inode_dir,
    pub link: jfs_inode_link,
}

#[repr(C)]
pub struct jfs_inode_file {
    pub _xtroot: xtroot_t,
    pub _imap: *mut inomap,
}

#[repr(C)]
pub struct jfs_inode_dir {
    pub _table: [dir_table_slot; 12],
    pub _dtroot: dtroot_t,
}

#[repr(C)]
pub struct jfs_inode_link {
    pub _unused: [u8; 16],
    pub _dxd: dxd_t,
    pub inline_data: jfs_inline_union,
}

#[repr(C)]
pub union jfs_inline_union {
    pub fields: jfs_inline_fields,
    pub _inline_all: [u8; 256],
}

#[repr(C)]
pub struct jfs_inline_fields {
    pub _inline_sym: [u8; 128],
    pub _inline_ea: [u8; 128],
}

/* C field aliases: i_xtroot, i_imap, i_dirtable, i_dtroot, i_inline,
 * i_inline_ea, and i_inline_all. */

pub const IN_LAZYCOMMIT: u32 = 1;

#[repr(i32)]
pub enum cflags {
    COMMIT_Nolink,
    COMMIT_Inlineea,
    COMMIT_Freewmap,
    COMMIT_Dirty,
    COMMIT_Dirtable,
    COMMIT_Stale,
    COMMIT_Synclist,
}

#[repr(i32)]
pub enum commit_mutex_class {
    COMMIT_MUTEX_PARENT,
    COMMIT_MUTEX_CHILD,
    COMMIT_MUTEX_SECOND_PARENT,
    COMMIT_MUTEX_VICTIM,
}

#[repr(i32)]
pub enum rdwrlock_class {
    RDWRLOCK_NORMAL,
    RDWRLOCK_IMAP,
    RDWRLOCK_DMAP,
}

#[repr(C)]
pub struct jfs_sb_info {
    pub sb: *mut super_block,
    pub mntflag: usize,
    pub ipbmap: *mut inode,
    pub ipaimap: *mut inode,
    pub ipaimap2: *mut inode,
    pub ipimap: *mut inode,
    pub log: *mut jfs_log,
    pub log_list: list_head,
    pub bsize: i16,
    pub l2bsize: i16,
    pub nbperpage: i16,
    pub l2nbperpage: i16,
    pub l2niperblk: i16,
    pub logdev: dev_t,
    pub aggregate: u32,
    pub logpxd: pxd_t,
    pub fsckpxd: pxd_t,
    pub ait2: pxd_t,
    pub uuid: uuid_t,
    pub loguuid: uuid_t,
    pub commit_state: i32,
    pub gengen: u32,
    pub inostamp: u32,
    pub bmap: *mut bmap,
    pub nls_tab: *mut nls_table,
    pub direct_inode: *mut inode,
    pub state: u32,
    pub flag: usize,
    pub p_state: u32,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub umask: u32,
    pub minblks_trim: u32,
}

pub unsafe fn JFS_IP(inode: *mut inode) -> *mut jfs_inode_info {
    container_of(inode, jfs_inode_info, vfs_inode)
}

pub unsafe fn jfs_dirtable_inline(inode: *mut inode) -> i32 {
    if (*JFS_IP(inode)).next_index <= MAX_INLINE_DIRTABLE_ENTRY + 1 { 1 } else { 0 }
}

pub unsafe fn JFS_SBI(sb: *mut super_block) -> *mut jfs_sb_info {
    (*sb).s_fs_info
}

pub unsafe fn isReadOnly(inode: *mut inode) -> i32 {
    if !(*JFS_SBI((*inode).i_sb)).log.is_null() { 0 } else { 1 }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
