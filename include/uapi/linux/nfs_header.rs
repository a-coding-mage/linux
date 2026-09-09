/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * NFS protocol definitions
 *
 * This file contains constants mostly for Version 2 of the protocol,
 * but also has a couple of NFSv3 bits in (notably the error codes).
 */

pub const NFS_PROGRAM: u32 = 100003;
pub const NFS_PORT: u32 = 2049;
pub const NFS_RDMA_PORT: u32 = 20049;
pub const NFS_MAXDATA: u32 = 8192;
pub const NFS_MAXPATHLEN: u32 = 1024;
pub const NFS_MAXNAMLEN: u32 = 255;
pub const NFS_MAXGROUPS: u32 = 16;
pub const NFS_FHSIZE: u32 = 32;
pub const NFS_COOKIESIZE: u32 = 4;
pub const NFS_FIFO_DEV: i32 = -1;
pub const NFSMODE_FMT: u32 = 0o170000;
pub const NFSMODE_DIR: u32 = 0o040000;
pub const NFSMODE_CHR: u32 = 0o020000;
pub const NFSMODE_BLK: u32 = 0o060000;
pub const NFSMODE_REG: u32 = 0o100000;
pub const NFSMODE_LNK: u32 = 0o120000;
pub const NFSMODE_SOCK: u32 = 0o140000;
pub const NFSMODE_FIFO: u32 = 0o010000;

pub const NFS_MNT_PROGRAM: u32 = 100005;
pub const NFS_MNT_VERSION: u32 = 1;
pub const NFS_MNT3_VERSION: u32 = 3;

pub const NFS_PIPE_DIRNAME: &str = "nfs";

/*
 * NFS stats. The good thing with these values is that NFSv3 errors are
 * a superset of NFSv2 errors (with the exception of NFSERR_WFLUSH which
 * no-one uses anyway), so we can happily mix code as long as we make sure
 * no NFSv3 errors are returned to NFSv2 clients.
 * Error codes that have a `--' in the v2 column are not part of the
 * standard, but seem to be widely used nevertheless.
 */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfs_stat {
    NFS_OK = 0,
    NFSERR_PERM = 1,
    NFSERR_NOENT = 2,
    NFSERR_IO = 5,
    NFSERR_NXIO = 6,
    NFSERR_ACCES = 13,
    NFSERR_EXIST = 17,
    NFSERR_XDEV = 18,
    NFSERR_NODEV = 19,
    NFSERR_NOTDIR = 20,
    NFSERR_ISDIR = 21,
    NFSERR_INVAL = 22,
    NFSERR_FBIG = 27,
    NFSERR_NOSPC = 28,
    NFSERR_ROFS = 30,
    NFSERR_MLINK = 31,
    NFSERR_NAMETOOLONG = 63,
    NFSERR_NOTEMPTY = 66,
    NFSERR_DQUOT = 69,
    NFSERR_STALE = 70,
    NFSERR_REMOTE = 71,
    NFSERR_WFLUSH = 99,
    NFSERR_BADHANDLE = 10001,
    NFSERR_NOT_SYNC = 10002,
    NFSERR_BAD_COOKIE = 10003,
    NFSERR_NOTSUPP = 10004,
    NFSERR_TOOSMALL = 10005,
    NFSERR_SERVERFAULT = 10006,
    NFSERR_BADTYPE = 10007,
    NFSERR_JUKEBOX = 10008,
    NFSERR_SAME = 10009,
    NFSERR_DENIED = 10010,
    NFSERR_EXPIRED = 10011,
    NFSERR_LOCKED = 10012,
    NFSERR_GRACE = 10013,
    NFSERR_FHEXPIRED = 10014,
    NFSERR_SHARE_DENIED = 10015,
    NFSERR_WRONGSEC = 10016,
    NFSERR_CLID_INUSE = 10017,
    NFSERR_RESOURCE = 10018,
    NFSERR_MOVED = 10019,
    NFSERR_NOFILEHANDLE = 10020,
    NFSERR_MINOR_VERS_MISMATCH = 10021,
    NFSERR_STALE_CLIENTID = 10022,
    NFSERR_STALE_STATEID = 10023,
    NFSERR_OLD_STATEID = 10024,
    NFSERR_BAD_STATEID = 10025,
    NFSERR_BAD_SEQID = 10026,
    NFSERR_NOT_SAME = 10027,
    NFSERR_LOCK_RANGE = 10028,
    NFSERR_SYMLINK = 10029,
    NFSERR_RESTOREFH = 10030,
    NFSERR_LEASE_MOVED = 10031,
    NFSERR_ATTRNOTSUPP = 10032,
    NFSERR_NO_GRACE = 10033,
    NFSERR_RECLAIM_BAD = 10034,
    NFSERR_RECLAIM_CONFLICT = 10035,
    NFSERR_BAD_XDR = 10036,
    NFSERR_LOCKS_HELD = 10037,
    NFSERR_OPENMODE = 10038,
    NFSERR_BADOWNER = 10039,
    NFSERR_BADCHAR = 10040,
    NFSERR_BADNAME = 10041,
    NFSERR_BAD_RANGE = 10042,
    NFSERR_LOCK_NOTSUPP = 10043,
    NFSERR_OP_ILLEGAL = 10044,
    NFSERR_DEADLOCK = 10045,
    NFSERR_FILE_OPEN = 10046,
    NFSERR_ADMIN_REVOKED = 10047,
    NFSERR_CB_PATH_DOWN = 10048,
}

/* NFSv2 file types - beware, these are not the same in NFSv3 */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfs_ftype {
    NFNON = 0,
    NFREG = 1,
    NFDIR = 2,
    NFBLK = 3,
    NFCHR = 4,
    NFLNK = 5,
    NFSOCK = 6,
    NFBAD = 7,
    NFFIFO = 8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
