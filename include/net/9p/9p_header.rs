/* SPDX-License-Identifier: GPL-2.0-only */
/* 9P protocol definitions. */

use core::ffi::c_char;

pub type p9_debug_flags = u32;
pub const P9_DEBUG_ERROR: p9_debug_flags = 1 << 0;
pub const P9_DEBUG_9P: p9_debug_flags = 1 << 2;
pub const P9_DEBUG_VFS: p9_debug_flags = 1 << 3;
pub const P9_DEBUG_CONV: p9_debug_flags = 1 << 4;
pub const P9_DEBUG_MUX: p9_debug_flags = 1 << 5;
pub const P9_DEBUG_TRANS: p9_debug_flags = 1 << 6;
pub const P9_DEBUG_SLABS: p9_debug_flags = 1 << 7;
pub const P9_DEBUG_FCALL: p9_debug_flags = 1 << 8;
pub const P9_DEBUG_FID: p9_debug_flags = 1 << 9;
pub const P9_DEBUG_PKT: p9_debug_flags = 1 << 10;
pub const P9_DEBUG_FSC: p9_debug_flags = 1 << 11;
pub const P9_DEBUG_VPKT: p9_debug_flags = 1 << 12;
pub const P9_DEBUG_CACHE: p9_debug_flags = 1 << 13;
pub const P9_DEBUG_MMAP: p9_debug_flags = 1 << 14;

#[cfg(CONFIG_NET_9P_DEBUG)]
extern "C" {
    pub static mut p9_debug_level: u32;
    pub fn _p9_debug(level: p9_debug_flags, func: *const c_char, fmt: *const c_char, ...);
}

pub type p9_msg_t = u8;
pub const P9_TLERROR: p9_msg_t = 6;
pub const P9_RLERROR: p9_msg_t = 7;
pub const P9_TSTATFS: p9_msg_t = 8;
pub const P9_RSTATFS: p9_msg_t = 9;
pub const P9_TLOPEN: p9_msg_t = 12;
pub const P9_RLOPEN: p9_msg_t = 13;
pub const P9_TLCREATE: p9_msg_t = 14;
pub const P9_RLCREATE: p9_msg_t = 15;
pub const P9_TSYMLINK: p9_msg_t = 16;
pub const P9_RSYMLINK: p9_msg_t = 17;
pub const P9_TMKNOD: p9_msg_t = 18;
pub const P9_RMKNOD: p9_msg_t = 19;
pub const P9_TRENAME: p9_msg_t = 20;
pub const P9_RRENAME: p9_msg_t = 21;
pub const P9_TREADLINK: p9_msg_t = 22;
pub const P9_RREADLINK: p9_msg_t = 23;
pub const P9_TGETATTR: p9_msg_t = 24;
pub const P9_RGETATTR: p9_msg_t = 25;
pub const P9_TSETATTR: p9_msg_t = 26;
pub const P9_RSETATTR: p9_msg_t = 27;
pub const P9_TXATTRWALK: p9_msg_t = 30;
pub const P9_RXATTRWALK: p9_msg_t = 31;
pub const P9_TXATTRCREATE: p9_msg_t = 32;
pub const P9_RXATTRCREATE: p9_msg_t = 33;
pub const P9_TREADDIR: p9_msg_t = 40;
pub const P9_RREADDIR: p9_msg_t = 41;
pub const P9_TFSYNC: p9_msg_t = 50;
pub const P9_RFSYNC: p9_msg_t = 51;
pub const P9_TLOCK: p9_msg_t = 52;
pub const P9_RLOCK: p9_msg_t = 53;
pub const P9_TGETLOCK: p9_msg_t = 54;
pub const P9_RGETLOCK: p9_msg_t = 55;
pub const P9_TLINK: p9_msg_t = 70;
pub const P9_RLINK: p9_msg_t = 71;
pub const P9_TMKDIR: p9_msg_t = 72;
pub const P9_RMKDIR: p9_msg_t = 73;
pub const P9_TRENAMEAT: p9_msg_t = 74;
pub const P9_RRENAMEAT: p9_msg_t = 75;
pub const P9_TUNLINKAT: p9_msg_t = 76;
pub const P9_RUNLINKAT: p9_msg_t = 77;
pub const P9_TVERSION: p9_msg_t = 100;
pub const P9_RVERSION: p9_msg_t = 101;
pub const P9_TAUTH: p9_msg_t = 102;
pub const P9_RAUTH: p9_msg_t = 103;
pub const P9_TATTACH: p9_msg_t = 104;
pub const P9_RATTACH: p9_msg_t = 105;
pub const P9_TERROR: p9_msg_t = 106;
pub const P9_RERROR: p9_msg_t = 107;
pub const P9_TFLUSH: p9_msg_t = 108;
pub const P9_RFLUSH: p9_msg_t = 109;
pub const P9_TWALK: p9_msg_t = 110;
pub const P9_RWALK: p9_msg_t = 111;
pub const P9_TOPEN: p9_msg_t = 112;
pub const P9_ROPEN: p9_msg_t = 113;
pub const P9_TCREATE: p9_msg_t = 114;
pub const P9_RCREATE: p9_msg_t = 115;
pub const P9_TREAD: p9_msg_t = 116;
pub const P9_RREAD: p9_msg_t = 117;
pub const P9_TWRITE: p9_msg_t = 118;
pub const P9_RWRITE: p9_msg_t = 119;
pub const P9_TCLUNK: p9_msg_t = 120;
pub const P9_RCLUNK: p9_msg_t = 121;
pub const P9_TREMOVE: p9_msg_t = 122;
pub const P9_RREMOVE: p9_msg_t = 123;
pub const P9_TSTAT: p9_msg_t = 124;
pub const P9_RSTAT: p9_msg_t = 125;
pub const P9_TWSTAT: p9_msg_t = 126;
pub const P9_RWSTAT: p9_msg_t = 127;

pub type p9_open_mode_t = u32;
pub const P9_OREAD: p9_open_mode_t = 0x00;
pub const P9_OWRITE: p9_open_mode_t = 0x01;
pub const P9_ORDWR: p9_open_mode_t = 0x02;
pub const P9_OEXEC: p9_open_mode_t = 0x03;
pub const P9_OTRUNC: p9_open_mode_t = 0x10;
pub const P9_OREXEC: p9_open_mode_t = 0x20;
pub const P9_ORCLOSE: p9_open_mode_t = 0x40;
pub const P9_OAPPEND: p9_open_mode_t = 0x80;
pub const P9_OEXCL: p9_open_mode_t = 0x1000;
pub const P9L_MODE_MASK: p9_open_mode_t = 0x1fff;
pub const P9L_DIRECT: p9_open_mode_t = 0x2000;
pub const P9L_NOWRITECACHE: p9_open_mode_t = 0x4000;
pub const P9L_LOOSE: p9_open_mode_t = 0x8000;

pub type p9_perm_t = u32;
pub const P9_DMDIR: p9_perm_t = 0x80000000;
pub const P9_DMAPPEND: p9_perm_t = 0x40000000;
pub const P9_DMEXCL: p9_perm_t = 0x20000000;
pub const P9_DMMOUNT: p9_perm_t = 0x10000000;
pub const P9_DMAUTH: p9_perm_t = 0x08000000;
pub const P9_DMTMP: p9_perm_t = 0x04000000;
pub const P9_DMSYMLINK: p9_perm_t = 0x02000000;
pub const P9_DMLINK: p9_perm_t = 0x01000000;
pub const P9_DMDEVICE: p9_perm_t = 0x00800000;
pub const P9_DMNAMEDPIPE: p9_perm_t = 0x00200000;
pub const P9_DMSOCKET: p9_perm_t = 0x00100000;
pub const P9_DMSETUID: p9_perm_t = 0x00080000;
pub const P9_DMSETGID: p9_perm_t = 0x00040000;
pub const P9_DMSETVTX: p9_perm_t = 0x00010000;

pub const P9_DOTL_RDONLY: u32 = 0o0000000;
pub const P9_DOTL_WRONLY: u32 = 0o0000001;
pub const P9_DOTL_RDWR: u32 = 0o0000002;
pub const P9_DOTL_NOACCESS: u32 = 0o0000003;
pub const P9_DOTL_CREATE: u32 = 0o0000100;
pub const P9_DOTL_EXCL: u32 = 0o0000200;
pub const P9_DOTL_NOCTTY: u32 = 0o0000400;
pub const P9_DOTL_TRUNC: u32 = 0o0001000;
pub const P9_DOTL_APPEND: u32 = 0o0002000;
pub const P9_DOTL_NONBLOCK: u32 = 0o0004000;
pub const P9_DOTL_DSYNC: u32 = 0o0010000;
pub const P9_DOTL_FASYNC: u32 = 0o0020000;
pub const P9_DOTL_DIRECT: u32 = 0o0040000;
pub const P9_DOTL_LARGEFILE: u32 = 0o0100000;
pub const P9_DOTL_DIRECTORY: u32 = 0o0200000;
pub const P9_DOTL_NOFOLLOW: u32 = 0o0400000;
pub const P9_DOTL_NOATIME: u32 = 0o1000000;
pub const P9_DOTL_CLOEXEC: u32 = 0o2000000;
pub const P9_DOTL_SYNC: u32 = 0o4000000;
pub const P9_DOTL_AT_REMOVEDIR: u32 = 0x200;
pub const P9_LOCK_TYPE_RDLCK: u8 = 0;
pub const P9_LOCK_TYPE_WRLCK: u8 = 1;
pub const P9_LOCK_TYPE_UNLCK: u8 = 2;

pub type p9_qid_t = u8;
pub const P9_QTDIR: p9_qid_t = 0x80;
pub const P9_QTAPPEND: p9_qid_t = 0x40;
pub const P9_QTEXCL: p9_qid_t = 0x20;
pub const P9_QTMOUNT: p9_qid_t = 0x10;
pub const P9_QTAUTH: p9_qid_t = 0x08;
pub const P9_QTTMP: p9_qid_t = 0x04;
pub const P9_QTSYMLINK: p9_qid_t = 0x02;
pub const P9_QTLINK: p9_qid_t = 0x01;
pub const P9_QTFILE: p9_qid_t = 0x00;

pub const P9_NOTAG: u16 = !0;
pub const P9_NOFID: u32 = !0;
pub const P9_MAXWELEM: u32 = 16;
pub const P9_HDRSZ: u32 = 7;
pub const P9_IOHDRSZ: u32 = 24;
pub const P9_READDIRHDRSZ: u32 = 24;
pub const P9_ZC_HDR_SZ: u32 = 4096;
pub const P9_ERRMAX: u32 = 128;

#[repr(C)]
pub struct p9_qid { pub type_: u8, pub version: u32, pub path: u64 }

#[repr(C)]
pub struct p9_wstat {
    pub size: u16, pub type_: u16, pub dev: u32, pub qid: p9_qid, pub mode: u32,
    pub atime: u32, pub mtime: u32, pub length: u64,
    pub name: *const c_char, pub uid: *const c_char, pub gid: *const c_char,
    pub muid: *const c_char, pub extension: *mut c_char,
    pub n_uid: kuid_t, pub n_gid: kgid_t, pub n_muid: kuid_t,
}

#[repr(C)]
pub struct p9_stat_dotl {
    pub st_result_mask: u64, pub qid: p9_qid, pub st_mode: u32,
    pub st_uid: kuid_t, pub st_gid: kgid_t, pub st_nlink: u64, pub st_rdev: u64,
    pub st_size: u64, pub st_blksize: u64, pub st_blocks: u64,
    pub st_atime_sec: u64, pub st_atime_nsec: u64, pub st_mtime_sec: u64,
    pub st_mtime_nsec: u64, pub st_ctime_sec: u64, pub st_ctime_nsec: u64,
    pub st_btime_sec: u64, pub st_btime_nsec: u64, pub st_gen: u64,
    pub st_data_version: u64,
}

pub const P9_STATS_MODE: u64 = 0x00000001;
pub const P9_STATS_NLINK: u64 = 0x00000002;
pub const P9_STATS_UID: u64 = 0x00000004;
pub const P9_STATS_GID: u64 = 0x00000008;
pub const P9_STATS_RDEV: u64 = 0x00000010;
pub const P9_STATS_ATIME: u64 = 0x00000020;
pub const P9_STATS_MTIME: u64 = 0x00000040;
pub const P9_STATS_CTIME: u64 = 0x00000080;
pub const P9_STATS_INO: u64 = 0x00000100;
pub const P9_STATS_SIZE: u64 = 0x00000200;
pub const P9_STATS_BLOCKS: u64 = 0x00000400;
pub const P9_STATS_BTIME: u64 = 0x00000800;
pub const P9_STATS_GEN: u64 = 0x00001000;
pub const P9_STATS_DATA_VERSION: u64 = 0x00002000;
pub const P9_STATS_BASIC: u64 = 0x000007ff;
pub const P9_STATS_ALL: u64 = 0x00003fff;

#[repr(C)]
pub struct p9_iattr_dotl {
    pub valid: u32, pub mode: u32, pub uid: kuid_t, pub gid: kgid_t,
    pub size: u64, pub atime_sec: u64, pub atime_nsec: u64,
    pub mtime_sec: u64, pub mtime_nsec: u64,
}

pub const P9_LOCK_SUCCESS: u32 = 0;
pub const P9_LOCK_BLOCKED: u32 = 1;
pub const P9_LOCK_ERROR: u32 = 2;
pub const P9_LOCK_GRACE: u32 = 3;
pub const P9_LOCK_FLAGS_BLOCK: u32 = 1;
pub const P9_LOCK_FLAGS_RECLAIM: u32 = 2;

#[repr(C)]
pub struct p9_flock {
    pub type_: u8, pub flags: u32, pub start: u64, pub length: u64,
    pub proc_id: u32, pub client_id: *mut c_char,
}

#[repr(C)]
pub struct p9_getlock {
    pub type_: u8, pub start: u64, pub length: u64, pub proc_id: u32,
    pub client_id: *mut c_char,
}

#[repr(C)]
pub struct p9_rstatfs {
    pub type_: u32, pub bsize: u32, pub blocks: u64, pub bfree: u64,
    pub bavail: u64, pub files: u64, pub ffree: u64, pub fsid: u64,
    pub namelen: u32,
}

#[repr(C)]
pub struct p9_fcall {
    pub size: u32, pub id: u8, pub tag: u16, pub offset: usize, pub capacity: usize,
    pub cache: *mut kmem_cache, pub sdata: *mut u8, pub zc: bool,
}

extern "C" {
    pub fn p9_errstr2errno(errstr: *mut c_char, len: i32) -> i32;
    pub fn p9_error_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
