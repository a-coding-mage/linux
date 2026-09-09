/* SPDX-License-Identifier: GPL-2.0-or-later */
/* AFS common types
 *
 * Copyright (C) 2002, 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependency intent from <linux/in.h> is preserved through the referenced types.

pub const AFS_MAXCELLNAME: usize = 253;
pub const AFS_MAXVOLNAME: usize = 64;
pub const AFS_MAXNSERVERS: usize = 8;
pub const AFS_NMAXNSERVERS: usize = 13;
pub const AFS_MAXTYPES: usize = 3;
pub const AFSNAMEMAX: usize = 256;
pub const AFSPATHMAX: usize = 1024;
pub const AFSOPAQUEMAX: usize = 1024;

pub const AFS_VL_MAX_LIFESPAN: i32 = 120;
pub const AFS_PROBE_MAX_LIFESPAN: i32 = 30;

pub type afs_volid_t = u64;
pub type afs_vnodeid_t = u64;
pub type afs_dataversion_t = u64;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_voltype_t {
    AFSVL_RWVOL,
    AFSVL_ROVOL,
    AFSVL_BACKVOL,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_file_type_t {
    AFS_FTYPE_INVALID = 0,
    AFS_FTYPE_FILE = 1,
    AFS_FTYPE_DIR = 2,
    AFS_FTYPE_SYMLINK = 3,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_lock_type_t {
    AFS_LOCK_READ = 0,
    AFS_LOCK_WRITE = 1,
}

pub const AFS_LOCKWAIT: i32 = 5 * 60;

#[repr(C)]
pub struct afs_fid {
    pub vid: afs_volid_t,
    pub vnode: afs_vnodeid_t,
    pub vnode_hi: u32,
    pub unique: u32,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_callback_type_t {
    AFSCM_CB_UNTYPED = 0,
    AFSCM_CB_EXCLUSIVE = 1,
    AFSCM_CB_SHARED = 2,
    AFSCM_CB_DROPPED = 3,
}

#[repr(C)]
pub struct afs_callback {
    pub expires_at: time64_t,
}

#[repr(C)]
pub struct afs_callback_break {
    pub fid: afs_fid,
}

pub const AFSCBMAX: usize = 50;

#[repr(C)]
pub struct afs_uuid {
    pub time_low: __be32,
    pub time_mid: __be16,
    pub time_hi_and_version: __be16,
    pub clock_seq_hi_and_reserved: __s8,
    pub clock_seq_low: __s8,
    pub node: [__s8; 6],
}

#[repr(C)]
pub struct afs_volume_info_servers {
    pub addr: in_addr,
}

#[repr(C)]
pub struct afs_volume_info {
    pub vid: afs_volid_t,
    pub type_: afs_voltype_t,
    pub type_vids: [afs_volid_t; 5],
    pub nservers: usize,
    pub servers: [afs_volume_info_servers; 8],
}

pub type afs_access_t = u32;
pub const AFS_ACE_READ: u32 = 0x00000001;
pub const AFS_ACE_WRITE: u32 = 0x00000002;
pub const AFS_ACE_INSERT: u32 = 0x00000004;
pub const AFS_ACE_LOOKUP: u32 = 0x00000008;
pub const AFS_ACE_DELETE: u32 = 0x00000010;
pub const AFS_ACE_LOCK: u32 = 0x00000020;
pub const AFS_ACE_ADMINISTER: u32 = 0x00000040;
pub const AFS_ACE_USER_A: u32 = 0x01000000;
pub const AFS_ACE_USER_B: u32 = 0x02000000;
pub const AFS_ACE_USER_C: u32 = 0x04000000;
pub const AFS_ACE_USER_D: u32 = 0x08000000;
pub const AFS_ACE_USER_E: u32 = 0x10000000;
pub const AFS_ACE_USER_F: u32 = 0x20000000;
pub const AFS_ACE_USER_G: u32 = 0x40000000;
pub const AFS_ACE_USER_H: u32 = 0x80000000;

#[repr(C)]
pub struct afs_file_status {
    pub size: u64,
    pub data_version: afs_dataversion_t,
    pub mtime_client: timespec64,
    pub mtime_server: timespec64,
    pub author: i64,
    pub owner: i64,
    pub group: i64,
    pub caller_access: afs_access_t,
    pub anon_access: afs_access_t,
    pub mode: umode_t,
    pub type_: afs_file_type_t,
    pub nlink: u32,
    pub lock_count: i32,
    pub abort_code: u32,
}

#[repr(C)]
pub struct afs_status_cb {
    pub status: afs_file_status,
    pub callback: afs_callback,
    pub have_status: bool,
    pub have_cb: bool,
    pub have_error: bool,
}

pub const AFS_SET_MTIME: u32 = 0x01;
pub const AFS_SET_OWNER: u32 = 0x02;
pub const AFS_SET_GROUP: u32 = 0x04;
pub const AFS_SET_MODE: u32 = 0x08;
pub const AFS_SET_SEG_SIZE: u32 = 0x10;

#[repr(C)]
pub struct afs_volsync {
    pub creation: time64_t,
    pub update: time64_t,
}

#[repr(C)]
pub struct afs_volume_status {
    pub vid: afs_volid_t,
    pub parent_id: afs_volid_t,
    pub online: u8,
    pub in_service: u8,
    pub blessed: u8,
    pub needs_salvage: u8,
    pub type_: u32,
    pub min_quota: u64,
    pub max_quota: u64,
    pub blocks_in_use: u64,
    pub part_blocks_avail: u64,
    pub part_max_blocks: u64,
    pub vol_copy_date: i64,
    pub vol_backup_date: i64,
}

pub const AFS_BLOCK_SIZE: usize = 1024;

#[repr(C)]
pub struct afs_uuid__xdr {
    pub time_low: __be32,
    pub time_mid: __be32,
    pub time_hi_and_version: __be32,
    pub clock_seq_hi_and_reserved: __be32,
    pub clock_seq_low: __be32,
    pub node: [__be32; 6],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
