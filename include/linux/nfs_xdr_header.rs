/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of linux/nfs_xdr.h.  Types supplied by the surrounding
// kernel translation are intentionally left as external dependencies.

pub const NFS_MAX_FILE_IO_SIZE: u32 = 1048576;
pub const NFS_DEF_FILE_IO_SIZE: u32 = 4096;
pub const NFS_MIN_FILE_IO_SIZE: u32 = 1024;
pub const NFS_BITMASK_SZ: usize = 3;
pub const NFS_AUX_UNCACHEABLE_FILE_DATA: u32 = 1 << 0;

#[repr(C)] pub struct nfs4_string { pub len: u32, pub data: *mut i8 }
#[repr(C)] pub struct nfs_fsid { pub major: u64, pub minor: u64 }
#[inline] pub unsafe fn nfs_fsid_equal(a: *const nfs_fsid, b: *const nfs_fsid) -> i32 {
    ((*a).major == (*b).major && (*a).minor == (*b).minor) as i32
}
#[repr(C)] pub struct nfs4_threshold { pub bm: u32, pub l_type: u32, pub rd_sz: u64, pub wr_sz: u64, pub rd_io_sz: u64, pub wr_io_sz: u64 }

#[repr(C)] pub union nfs_fattr_du { pub nfs2: nfs_fattr_du_nfs2, pub nfs3: nfs_fattr_du_nfs3 }
#[repr(C)] pub struct nfs_fattr_du_nfs2 { pub blocksize: u32, pub blocks: u32 }
#[repr(C)] pub struct nfs_fattr_du_nfs3 { pub used: u64 }
#[repr(C)] pub struct nfs_fattr {
    pub valid: u64, pub mode: umode_t, pub nlink: u32, pub uid: kuid_t, pub gid: kgid_t, pub rdev: dev_t, pub size: u64,
    pub du: nfs_fattr_du, pub fsid: nfs_fsid, pub fileid: u64, pub mounted_on_fileid: u64,
    pub atime: timespec64, pub mtime: timespec64, pub ctime: timespec64, pub btime: timespec64,
    pub aux_flags: u32, pub change_attr: u64, pub pre_change_attr: u64, pub pre_size: u64,
    pub pre_mtime: timespec64, pub pre_ctime: timespec64, pub time_start: c_ulong, pub gencount: c_ulong,
    pub owner_name: *mut nfs4_string, pub group_name: *mut nfs4_string, pub mdsthreshold: *mut nfs4_threshold, pub label: *mut nfs4_label,
}

pub const NFS_ATTR_FATTR_TYPE: u64 = 1<<0; pub const NFS_ATTR_FATTR_MODE: u64 = 1<<1;
pub const NFS_ATTR_FATTR_NLINK: u64 = 1<<2; pub const NFS_ATTR_FATTR_OWNER: u64 = 1<<3;
pub const NFS_ATTR_FATTR_GROUP: u64 = 1<<4; pub const NFS_ATTR_FATTR_RDEV: u64 = 1<<5;
pub const NFS_ATTR_FATTR_SIZE: u64 = 1<<6; pub const NFS_ATTR_FATTR_PRESIZE: u64 = 1<<7;
pub const NFS_ATTR_FATTR_BLOCKS_USED: u64 = 1<<8; pub const NFS_ATTR_FATTR_SPACE_USED: u64 = 1<<9;
pub const NFS_ATTR_FATTR_FSID: u64 = 1<<10; pub const NFS_ATTR_FATTR_FILEID: u64 = 1<<11;
pub const NFS_ATTR_FATTR_ATIME: u64 = 1<<12; pub const NFS_ATTR_FATTR_MTIME: u64 = 1<<13;
pub const NFS_ATTR_FATTR_CTIME: u64 = 1<<14; pub const NFS_ATTR_FATTR_PREMTIME: u64 = 1<<15;
pub const NFS_ATTR_FATTR_PRECTIME: u64 = 1<<16; pub const NFS_ATTR_FATTR_CHANGE: u64 = 1<<17;
pub const NFS_ATTR_FATTR_PRECHANGE: u64 = 1<<18; pub const NFS_ATTR_FATTR_V4_LOCATIONS: u64 = 1<<19;
pub const NFS_ATTR_FATTR_V4_REFERRAL: u64 = 1<<20; pub const NFS_ATTR_FATTR_MOUNTPOINT: u64 = 1<<21;
pub const NFS_ATTR_FATTR_MOUNTED_ON_FILEID: u64 = 1<<22; pub const NFS_ATTR_FATTR_OWNER_NAME: u64 = 1<<23;
pub const NFS_ATTR_FATTR_GROUP_NAME: u64 = 1<<24; pub const NFS_ATTR_FATTR_V4_SECURITY_LABEL: u64 = 1<<25;
pub const NFS_ATTR_FATTR_BTIME: u64 = 1<<26; pub const NFS_ATTR_FATTR_UNCACHEABLE_FILE_DATA: u64 = 1<<27;
pub const NFS_ATTR_FATTR: u64 = NFS_ATTR_FATTR_TYPE|NFS_ATTR_FATTR_MODE|NFS_ATTR_FATTR_NLINK|NFS_ATTR_FATTR_OWNER|NFS_ATTR_FATTR_GROUP|NFS_ATTR_FATTR_RDEV|NFS_ATTR_FATTR_SIZE|NFS_ATTR_FATTR_FSID|NFS_ATTR_FATTR_FILEID|NFS_ATTR_FATTR_ATIME|NFS_ATTR_FATTR_MTIME|NFS_ATTR_FATTR_CTIME|NFS_ATTR_FATTR_CHANGE;
pub const NFS_ATTR_FATTR_V2: u64 = NFS_ATTR_FATTR | NFS_ATTR_FATTR_BLOCKS_USED;
pub const NFS_ATTR_FATTR_V3: u64 = NFS_ATTR_FATTR | NFS_ATTR_FATTR_SPACE_USED;
pub const NFS_ATTR_FATTR_V4: u64 = NFS_ATTR_FATTR | NFS_ATTR_FATTR_SPACE_USED | NFS_ATTR_FATTR_BTIME | NFS_ATTR_FATTR_V4_SECURITY_LABEL | NFS_ATTR_FATTR_UNCACHEABLE_FILE_DATA;
pub const NFS_MAX_LAYOUT_TYPES: usize = 8;

#[repr(C)] pub struct nfs_fsinfo { pub fattr:*mut nfs_fattr, pub rtmax:u32, pub rtpref:u32, pub rtmult:u32, pub wtmax:u32, pub wtpref:u32, pub wtmult:u32, pub dtpref:u32, pub maxfilesize:u64, pub time_delta:timespec64, pub lease_time:u32, pub nlayouttypes:u32, pub layouttype:[u32;NFS_MAX_LAYOUT_TYPES], pub blksize:u32, pub clone_blksize:u32, pub change_attr_type:nfs4_change_attr_type, pub xattr_support:u32 }
#[repr(C)] pub struct nfs_fsstat { pub fattr:*mut nfs_fattr, pub tbytes:u64, pub fbytes:u64, pub abytes:u64, pub tfiles:u64, pub ffiles:u64, pub afiles:u64 }
#[repr(C)] pub struct nfs2_fsstat { pub tsize:u32, pub bsize:u32, pub blocks:u32, pub bfree:u32, pub bavail:u32 }
#[repr(C)] pub struct nfs_pathconf { pub fattr:*mut nfs_fattr, pub max_link:u32, pub max_namelen:u32, pub case_insensitive:bool, pub case_preserving:bool }
#[repr(C)] pub struct nfs4_change_info { pub atomic:u32, pub before:u64, pub after:u64 }

pub const PNFS_LAYOUT_MAXSIZE: usize = 4096; pub const PNFS_LAYOUTSTATS_MAXSIZE: usize = 384;
pub const NFS42_LAYOUTERROR_MAX: usize = 5; pub const NFS4_ACL_TRUNC: i32 = 1;
pub const NFS_PAGEVEC_SIZE: usize = 8; pub const MAX_BIND_CONN_TO_SESSION_RETRIES: i32 = 3;
pub const SECINFO_STYLE_CURRENT_FH: i32 = 0; pub const SECINFO_STYLE_PARENT: i32 = 1;

#[repr(C)] pub struct nfs_write_verifier { pub data:[i8;8] }
#[repr(C)] pub struct nfs_writeverf { pub verifier:nfs_write_verifier, pub committed:nfs3_stable_how }
#[repr(C)] pub struct nfs_page_array { pub pagevec:*mut *mut page, pub npages:u32, pub page_array:[*mut page;NFS_PAGEVEC_SIZE] }
pub const NFS_IOHDR_ERROR: u32=0; pub const NFS_IOHDR_EOF:u32=1; pub const NFS_IOHDR_REDO:u32=2; pub const NFS_IOHDR_STAT:u32=3; pub const NFS_IOHDR_RESEND_PNFS:u32=4; pub const NFS_IOHDR_RESEND_MDS:u32=5; pub const NFS_IOHDR_UNSTABLE_WRITES:u32=6; pub const NFS_IOHDR_ODIRECT:u32=7;

// The remaining declarations are represented with their C-compatible forward
// interfaces; concrete kernel types and feature-gated structures are supplied
// by the corresponding translated NFS headers.
extern "C" {
    pub static nfs_v2_clientops: nfs_rpc_ops;
    pub static nfs_v3_clientops: nfs_rpc_ops;
    pub static nfs_v4_clientops: nfs_rpc_ops;
    pub static nfs_version2: rpc_version;
    pub static nfs_version3: rpc_version;
    pub static nfs_version4: rpc_version;
    pub static nfsacl_version3: rpc_version;
    pub static nfsacl_program: rpc_program;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
