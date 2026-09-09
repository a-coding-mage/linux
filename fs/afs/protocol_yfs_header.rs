/* SPDX-License-Identifier: GPL-2.0-or-later */
/* YFS protocol bits
 *
 * Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

pub const YFS_FS_SERVICE: u32 = 2500;
pub const YFS_CM_SERVICE: u32 = 2501;
pub const YFSCBMAX: u32 = 1024;

#[repr(i32)]
pub enum YFS_CM_Operations {
    YFSCBProbe = 206,
    YFSCBGetLock = 207,
    YFSCBXStatsVersion = 209,
    YFSCBGetXStats = 210,
    YFSCBInitCallBackState3 = 213,
    YFSCBProbeUuid = 214,
    YFSCBGetServerPrefs = 215,
    YFSCBGetCellServDV = 216,
    YFSCBGetLocalCell = 217,
    YFSCBGetCacheConfig = 218,
    YFSCBGetCellByNum = 65537,
    YFSCBTellMeAboutYourself = 65538,
    YFSCBCallBack = 64204,
}

#[repr(i32)]
pub enum YFS_FS_Operations {
    YFSFETCHACL = 64131,
    YFSFETCHSTATUS = 64132,
    YFSSTOREACL = 64134,
    YFSSTORESTATUS = 64135,
    YFSREMOVEFILE = 64136,
    YFSCREATEFILE = 64137,
    YFSRENAME = 64138,
    YFSSYMLINK = 64139,
    YFSLINK = 64140,
    YFSMAKEDIR = 64141,
    YFSREMOVEDIR = 64142,
    YFSGETVOLUMESTATUS = 64149,
    YFSSETVOLUMESTATUS = 64150,
    YFSSETLOCK = 64156,
    YFSEXTENDLOCK = 64157,
    YFSRELEASELOCK = 64158,
    YFSLOOKUP = 64161,
    YFSFLUSHCPS = 64165,
    YFSFETCHOPAQUEACL = 64168,
    YFSWHOAMI = 64170,
    YFSREMOVEACL = 64171,
    YFSREMOVEFILE2 = 64173,
    YFSSTOREOPAQUEACL2 = 64174,
    YFSRENAME_REPLACE = 64176,
    YFSRENAME_NOREPLACE = 64177,
    YFSRENAME_EXCHANGE = 64187,
    YFSINLINEBULKSTATUS = 64536,
    YFSFETCHDATA64 = 64537,
    YFSSTOREDATA64 = 64538,
    YFSUPDATESYMLINK = 64540,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_u64 {
    pub msw: u32,
    pub lsw: u32,
}

#[inline]
pub fn xdr_to_u64(x: yfs_xdr_u64) -> u64 {
    ((u32::from_be(x.msw) as u64) << 32) | (u32::from_be(x.lsw) as u64)
}

#[inline]
pub fn u64_to_xdr(x: u64) -> yfs_xdr_u64 {
    yfs_xdr_u64 { msw: ((x >> 32) as u32).to_be(), lsw: (x as u32).to_be() }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_vnode {
    pub lo: yfs_xdr_u64,
    pub hi: u32,
    pub unique: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSFid {
    pub volume: yfs_xdr_u64,
    pub vnode: yfs_xdr_vnode,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSFetchStatus {
    pub r#type: u32,
    pub nlink: u32,
    pub size: yfs_xdr_u64,
    pub data_version: yfs_xdr_u64,
    pub author: yfs_xdr_u64,
    pub owner: yfs_xdr_u64,
    pub group: yfs_xdr_u64,
    pub mode: u32,
    pub caller_access: u32,
    pub anon_access: u32,
    pub parent: yfs_xdr_vnode,
    pub data_access_protocol: u32,
    pub mtime_client: yfs_xdr_u64,
    pub mtime_server: yfs_xdr_u64,
    pub lock_count: u32,
    pub abort_code: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSCallBack {
    pub version: u32,
    pub expiration_time: yfs_xdr_u64,
    pub r#type: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSStoreStatus {
    pub mask: u32,
    pub mode: u32,
    pub mtime_client: yfs_xdr_u64,
    pub owner: yfs_xdr_u64,
    pub group: yfs_xdr_u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_RPCFlags { pub rpc_flags: u32 }

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSVolSync {
    pub vol_creation_date: yfs_xdr_u64,
    pub vol_update_date: yfs_xdr_u64,
    pub max_quota: yfs_xdr_u64,
    pub blocks_in_use: yfs_xdr_u64,
    pub blocks_avail: yfs_xdr_u64,
}

#[repr(i32)]
pub enum yfs_volume_type { yfs_volume_type_ro = 0, yfs_volume_type_rw = 1 }

pub const yfs_FVSOnline: u32 = 0x1;
pub const yfs_FVSInservice: u32 = 0x2;
pub const yfs_FVSBlessed: u32 = 0x4;
pub const yfs_FVSNeedsSalvage: u32 = 0x8;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSFetchVolumeStatus {
    pub vid: yfs_xdr_u64,
    pub parent_id: yfs_xdr_u64,
    pub flags: u32,
    pub r#type: u32,
    pub max_quota: yfs_xdr_u64,
    pub blocks_in_use: yfs_xdr_u64,
    pub part_blocks_avail: yfs_xdr_u64,
    pub part_max_blocks: yfs_xdr_u64,
    pub vol_copy_date: yfs_xdr_u64,
    pub vol_backup_date: yfs_xdr_u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSStoreVolumeStatus {
    pub mask: u32,
    pub min_quota: yfs_xdr_u64,
    pub max_quota: yfs_xdr_u64,
    pub file_quota: yfs_xdr_u64,
}

#[repr(i32)]
pub enum yfs_lock_type {
    yfs_LockNone = -1,
    yfs_LockRead = 0,
    yfs_LockWrite = 1,
    yfs_LockExtend = 2,
    yfs_LockRelease = 3,
    yfs_LockMandatoryRead = 0x100,
    yfs_LockMandatoryWrite = 0x101,
    yfs_LockMandatoryExtend = 0x102,
}

/* RXYFS Viced Capability Flags */
pub const YFS_VICED_CAPABILITY_ERRORTRANS: u32 = 0x0001;
pub const YFS_VICED_CAPABILITY_64BITFILES: u32 = 0x0002;
pub const YFS_VICED_CAPABILITY_WRITELOCKACL: u32 = 0x0004;
pub const YFS_VICED_CAPABILITY_SANEACLS: u32 = 0x0008;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
