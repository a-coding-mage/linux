/* SPDX-License-Identifier: GPL-2.0 */
// Data types for the Ceph distributed object storage layer RADOS.
// The C header includes linux/ceph/msgr.h; its supplied integer aliases are
// referenced here as __le16, __le32, __le64, __u8, and __u64.

#[repr(C)]
pub struct ceph_fsid { pub fsid: [u8; 16] }

#[inline]
pub unsafe fn ceph_fsid_compare(a: *const ceph_fsid, b: *const ceph_fsid) -> i32 {
    libc::memcmp(a as *const libc::c_void, b as *const libc::c_void, core::mem::size_of::<ceph_fsid>()) as i32
}

pub type ceph_snapid_t = __le64;
pub const CEPH_SNAPDIR: __u64 = (!0u64);
pub const CEPH_NOSNAP: __u64 = (!1u64);
pub const CEPH_MAXSNAP: __u64 = (!2u64);

#[repr(C, packed)]
pub struct ceph_timespec { pub tv_sec: __le32, pub tv_nsec: __le32 }

pub const CEPH_OBJECT_LAYOUT_HASH: i32 = 1;
pub const CEPH_OBJECT_LAYOUT_LINEAR: i32 = 2;
pub const CEPH_OBJECT_LAYOUT_HASHINO: i32 = 3;
pub const CEPH_PG_LAYOUT_CRUSH: i32 = 0;
pub const CEPH_PG_LAYOUT_HASH: i32 = 1;
pub const CEPH_PG_LAYOUT_LINEAR: i32 = 2;
pub const CEPH_PG_LAYOUT_HYBRID: i32 = 3;
pub const CEPH_PG_MAX_SIZE: i32 = 32;

#[repr(C, packed)]
pub struct ceph_pg_v1 { pub preferred: __le16, pub ps: __le16, pub pool: __le32 }

pub const CEPH_NOPOOL: __u64 = !0u64;
pub const CEPH_POOL_TYPE_REP: i32 = 1;
pub const CEPH_POOL_TYPE_RAID4: i32 = 2;
pub const CEPH_POOL_TYPE_EC: i32 = 3;

#[inline]
pub fn ceph_stable_mod(x: i32, b: i32, bmask: i32) -> i32 {
    if (x & bmask) < b { x & bmask } else { x & (bmask >> 1) }
}

#[repr(C, packed)]
pub struct ceph_object_layout { pub ol_pgid: ceph_pg_v1, pub ol_stripe_unit: __le32 }
#[repr(C, packed)]
pub struct ceph_eversion { pub version: __le64, pub epoch: __le32 }

pub const CEPH_OSD_EXISTS: i32 = 1 << 0;
pub const CEPH_OSD_UP: i32 = 1 << 1;
pub const CEPH_OSD_AUTOOUT: i32 = 1 << 2;
pub const CEPH_OSD_NEW: i32 = 1 << 3;
extern "C" { pub fn ceph_osd_state_name(s: i32) -> *const core::ffi::c_char; }
pub const CEPH_OSD_IN: i32 = 0x10000;
pub const CEPH_OSD_OUT: i32 = 0;
pub const CEPH_OSD_MAX_PRIMARY_AFFINITY: i32 = 0x10000;
pub const CEPH_OSD_DEFAULT_PRIMARY_AFFINITY: i32 = 0x10000;

pub const CEPH_OSDMAP_NEARFULL: i32 = 1<<0;
pub const CEPH_OSDMAP_FULL: i32 = 1<<1;
pub const CEPH_OSDMAP_PAUSERD: i32 = 1<<2;
pub const CEPH_OSDMAP_PAUSEWR: i32 = 1<<3;
pub const CEPH_OSDMAP_PAUSEREC: i32 = 1<<4;
pub const CEPH_OSDMAP_NOUP: i32 = 1<<5;
pub const CEPH_OSDMAP_NODOWN: i32 = 1<<6;
pub const CEPH_OSDMAP_NOOUT: i32 = 1<<7;
pub const CEPH_OSDMAP_NOIN: i32 = 1<<8;
pub const CEPH_OSDMAP_NOBACKFILL: i32 = 1<<9;
pub const CEPH_OSDMAP_NORECOVER: i32 = 1<<10;
pub const CEPH_OSDMAP_NOSCRUB: i32 = 1<<11;
pub const CEPH_OSDMAP_NODEEP_SCRUB: i32 = 1<<12;
pub const CEPH_OSDMAP_NOTIERAGENT: i32 = 1<<13;
pub const CEPH_OSDMAP_NOREBALANCE: i32 = 1<<14;
pub const CEPH_OSDMAP_SORTBITWISE: i32 = 1<<15;
pub const CEPH_OSDMAP_REQUIRE_JEWEL: i32 = 1<<16;
pub const CEPH_OSDMAP_REQUIRE_KRAKEN: i32 = 1<<17;
pub const CEPH_OSDMAP_REQUIRE_LUMINOUS: i32 = 1<<18;
pub const CEPH_OSDMAP_RECOVERY_DELETES: i32 = 1<<19;
pub const OSD_WRITETOOBIG: i32 = EMSGSIZE;

pub const CEPH_OSD_OP_MODE: i32 = 0xf000;
pub const CEPH_OSD_OP_MODE_RD: i32 = 0x1000;
pub const CEPH_OSD_OP_MODE_WR: i32 = 0x2000;
pub const CEPH_OSD_OP_MODE_RMW: i32 = 0x3000;
pub const CEPH_OSD_OP_MODE_SUB: i32 = 0x4000;
pub const CEPH_OSD_OP_MODE_CACHE: i32 = 0x8000;
pub const CEPH_OSD_OP_TYPE: i32 = 0x0f00;
pub const CEPH_OSD_OP_TYPE_LOCK: i32 = 0x0100;
pub const CEPH_OSD_OP_TYPE_DATA: i32 = 0x0200;
pub const CEPH_OSD_OP_TYPE_ATTR: i32 = 0x0300;
pub const CEPH_OSD_OP_TYPE_EXEC: i32 = 0x0400;
pub const CEPH_OSD_OP_TYPE_PG: i32 = 0x0500;
pub const CEPH_OSD_OP_TYPE_MULTI: i32 = 0x0600;

macro_rules! osd_op { ($mode:ident, $typ:ident, $nr:expr) => { CEPH_OSD_OP_MODE_$mode | CEPH_OSD_OP_TYPE_$typ | ($nr) }; }
macro_rules! osd_op1 { ($mode:ident, $nr:expr) => { CEPH_OSD_OP_MODE_$mode | ($nr) }; }

pub const CEPH_OSD_OP_READ: i32 = osd_op!(RD, DATA, 1);
pub const CEPH_OSD_OP_STAT: i32 = osd_op!(RD, DATA, 2);
pub const CEPH_OSD_OP_MAPEXT: i32 = osd_op!(RD, DATA, 3);
pub const CEPH_OSD_OP_MASKTRUNC: i32 = osd_op!(RD, DATA, 4);
pub const CEPH_OSD_OP_SPARSE_READ: i32 = osd_op!(RD, DATA, 5);
pub const CEPH_OSD_OP_NOTIFY: i32 = osd_op!(RD, DATA, 6);
pub const CEPH_OSD_OP_NOTIFY_ACK: i32 = osd_op!(RD, DATA, 7);
pub const CEPH_OSD_OP_ASSERT_VER: i32 = osd_op!(RD, DATA, 8);
pub const CEPH_OSD_OP_LIST_WATCHERS: i32 = osd_op!(RD, DATA, 9);
pub const CEPH_OSD_OP_LIST_SNAPS: i32 = osd_op!(RD, DATA, 10);
pub const CEPH_OSD_OP_SYNC_READ: i32 = osd_op!(RD, DATA, 11);
pub const CEPH_OSD_OP_WRITE: i32 = osd_op!(WR, DATA, 1);
pub const CEPH_OSD_OP_WRITEFULL: i32 = osd_op!(WR, DATA, 2);
pub const CEPH_OSD_OP_TRUNCATE: i32 = osd_op!(WR, DATA, 3);
pub const CEPH_OSD_OP_ZERO: i32 = osd_op!(WR, DATA, 4);
pub const CEPH_OSD_OP_DELETE: i32 = osd_op!(WR, DATA, 5);
pub const CEPH_OSD_OP_APPEND: i32 = osd_op!(WR, DATA, 6);
pub const CEPH_OSD_OP_SETTRUNC: i32 = osd_op!(WR, DATA, 8);
pub const CEPH_OSD_OP_TRIMTRUNC: i32 = osd_op!(WR, DATA, 9);
pub const CEPH_OSD_OP_TMAPUP: i32 = osd_op!(RMW, DATA, 10);
pub const CEPH_OSD_OP_TMAPPUT: i32 = osd_op!(WR, DATA, 11);
pub const CEPH_OSD_OP_TMAPGET: i32 = osd_op!(RD, DATA, 12);
pub const CEPH_OSD_OP_CREATE: i32 = osd_op!(WR, DATA, 13);
pub const CEPH_OSD_OP_ROLLBACK: i32 = osd_op!(WR, DATA, 14);
pub const CEPH_OSD_OP_WATCH: i32 = osd_op!(WR, DATA, 15);
pub const CEPH_OSD_OP_OMAPGETKEYS: i32 = osd_op!(RD, DATA, 17);
pub const CEPH_OSD_OP_OMAPGETVALS: i32 = osd_op!(RD, DATA, 18);
pub const CEPH_OSD_OP_OMAPGETHEADER: i32 = osd_op!(RD, DATA, 19);
pub const CEPH_OSD_OP_OMAPGETVALSBYKEYS: i32 = osd_op!(RD, DATA, 20);
pub const CEPH_OSD_OP_OMAPSETVALS: i32 = osd_op!(WR, DATA, 21);
pub const CEPH_OSD_OP_OMAPSETHEADER: i32 = osd_op!(WR, DATA, 22);
pub const CEPH_OSD_OP_OMAPCLEAR: i32 = osd_op!(WR, DATA, 23);
pub const CEPH_OSD_OP_OMAPRMKEYS: i32 = osd_op!(WR, DATA, 24);
pub const CEPH_OSD_OP_OMAP_CMP: i32 = osd_op!(RD, DATA, 25);
pub const CEPH_OSD_OP_COPY_FROM: i32 = osd_op!(WR, DATA, 26);
pub const CEPH_OSD_OP_COPY_FROM2: i32 = osd_op!(WR, DATA, 45);
pub const CEPH_OSD_OP_COPY_GET_CLASSIC: i32 = osd_op!(RD, DATA, 27);
pub const CEPH_OSD_OP_UNDIRTY: i32 = osd_op!(WR, DATA, 28);
pub const CEPH_OSD_OP_ISDIRTY: i32 = osd_op!(RD, DATA, 29);
pub const CEPH_OSD_OP_COPY_GET: i32 = osd_op!(RD, DATA, 30);
pub const CEPH_OSD_OP_CACHE_FLUSH: i32 = osd_op!(CACHE, DATA, 31);
pub const CEPH_OSD_OP_CACHE_EVICT: i32 = osd_op!(CACHE, DATA, 32);
pub const CEPH_OSD_OP_CACHE_TRY_FLUSH: i32 = osd_op!(CACHE, DATA, 33);
pub const CEPH_OSD_OP_TMAP2OMAP: i32 = osd_op!(RMW, DATA, 34);
pub const CEPH_OSD_OP_SETALLOCHINT: i32 = osd_op!(WR, DATA, 35);
pub const CEPH_OSD_OP_CLONERANGE: i32 = osd_op!(WR, MULTI, 1);
pub const CEPH_OSD_OP_ASSERT_SRC_VERSION: i32 = osd_op!(RD, MULTI, 2);
pub const CEPH_OSD_OP_SRC_CMPXATTR: i32 = osd_op!(RD, MULTI, 3);
pub const CEPH_OSD_OP_GETXATTR: i32 = osd_op!(RD, ATTR, 1);
pub const CEPH_OSD_OP_GETXATTRS: i32 = osd_op!(RD, ATTR, 2);
pub const CEPH_OSD_OP_CMPXATTR: i32 = osd_op!(RD, ATTR, 3);
pub const CEPH_OSD_OP_SETXATTR: i32 = osd_op!(WR, ATTR, 1);
pub const CEPH_OSD_OP_SETXATTRS: i32 = osd_op!(WR, ATTR, 2);
pub const CEPH_OSD_OP_RESETXATTRS: i32 = osd_op!(WR, ATTR, 3);
pub const CEPH_OSD_OP_RMXATTR: i32 = osd_op!(WR, ATTR, 4);
pub const CEPH_OSD_OP_PULL: i32 = osd_op1!(SUB, 1);
pub const CEPH_OSD_OP_PUSH: i32 = osd_op1!(SUB, 2);
pub const CEPH_OSD_OP_BALANCEREADS: i32 = osd_op1!(SUB, 3);
pub const CEPH_OSD_OP_UNBALANCEREADS: i32 = osd_op1!(SUB, 4);
pub const CEPH_OSD_OP_SCRUB: i32 = osd_op1!(SUB, 5);
pub const CEPH_OSD_OP_SCRUB_RESERVE: i32 = osd_op1!(SUB, 6);
pub const CEPH_OSD_OP_SCRUB_UNRESERVE: i32 = osd_op1!(SUB, 7);
pub const CEPH_OSD_OP_SCRUB_STOP: i32 = osd_op1!(SUB, 8);
pub const CEPH_OSD_OP_SCRUB_MAP: i32 = osd_op1!(SUB, 9);
pub const CEPH_OSD_OP_WRLOCK: i32 = osd_op!(WR, LOCK, 1);
pub const CEPH_OSD_OP_WRUNLOCK: i32 = osd_op!(WR, LOCK, 2);
pub const CEPH_OSD_OP_RDLOCK: i32 = osd_op!(WR, LOCK, 3);
pub const CEPH_OSD_OP_RDUNLOCK: i32 = osd_op!(WR, LOCK, 4);
pub const CEPH_OSD_OP_UPLOCK: i32 = osd_op!(WR, LOCK, 5);
pub const CEPH_OSD_OP_DNLOCK: i32 = osd_op!(WR, LOCK, 6);
pub const CEPH_OSD_OP_CALL: i32 = osd_op!(RD, EXEC, 1);
pub const CEPH_OSD_OP_PGLS: i32 = osd_op!(RD, PG, 1);
pub const CEPH_OSD_OP_PGLS_FILTER: i32 = osd_op!(RD, PG, 2);
pub const CEPH_OSD_OP_PG_HITSET_LS: i32 = osd_op!(RD, PG, 3);
pub const CEPH_OSD_OP_PG_HITSET_GET: i32 = osd_op!(RD, PG, 4);

#[inline] pub fn ceph_osd_op_type_lock(op: i32) -> i32 { ((op & CEPH_OSD_OP_TYPE) == CEPH_OSD_OP_TYPE_LOCK) as i32 }
#[inline] pub fn ceph_osd_op_type_data(op: i32) -> i32 { ((op & CEPH_OSD_OP_TYPE) == CEPH_OSD_OP_TYPE_DATA) as i32 }
#[inline] pub fn ceph_osd_op_type_attr(op: i32) -> i32 { ((op & CEPH_OSD_OP_TYPE) == CEPH_OSD_OP_TYPE_ATTR) as i32 }
#[inline] pub fn ceph_osd_op_type_exec(op: i32) -> i32 { ((op & CEPH_OSD_OP_TYPE) == CEPH_OSD_OP_TYPE_EXEC) as i32 }
#[inline] pub fn ceph_osd_op_type_pg(op: i32) -> i32 { ((op & CEPH_OSD_OP_TYPE) == CEPH_OSD_OP_TYPE_PG) as i32 }
#[inline] pub fn ceph_osd_op_type_multi(op: i32) -> i32 { ((op & CEPH_OSD_OP_TYPE) == CEPH_OSD_OP_TYPE_MULTI) as i32 }
#[inline] pub fn ceph_osd_op_mode_subop(op: i32) -> i32 { ((op & CEPH_OSD_OP_MODE) == CEPH_OSD_OP_MODE_SUB) as i32 }
#[inline] pub fn ceph_osd_op_mode_read(op: i32) -> i32 { ((op & CEPH_OSD_OP_MODE_RD) != 0 && op != CEPH_OSD_OP_CALL) as i32 }
#[inline] pub fn ceph_osd_op_mode_modify(op: i32) -> i32 { op & CEPH_OSD_OP_MODE_WR }

pub const CEPH_OSD_TMAP_HDR: u8 = b'h';
pub const CEPH_OSD_TMAP_SET: u8 = b's';
pub const CEPH_OSD_TMAP_CREATE: u8 = b'c';
pub const CEPH_OSD_TMAP_RM: u8 = b'r';
pub const CEPH_OSD_TMAP_RMSLOPPY: u8 = b'R';
extern "C" { pub fn ceph_osd_op_name(op: i32) -> *const core::ffi::c_char; }

pub const CEPH_OSD_FLAG_ACK: i32=0x0001; pub const CEPH_OSD_FLAG_ONNVRAM: i32=0x0002; pub const CEPH_OSD_FLAG_ONDISK: i32=0x0004; pub const CEPH_OSD_FLAG_RETRY: i32=0x0008;
pub const CEPH_OSD_FLAG_READ: i32=0x0010; pub const CEPH_OSD_FLAG_WRITE: i32=0x0020; pub const CEPH_OSD_FLAG_ORDERSNAP: i32=0x0040; pub const CEPH_OSD_FLAG_PEERSTAT_OLD: i32=0x0080;
pub const CEPH_OSD_FLAG_BALANCE_READS: i32=0x0100; pub const CEPH_OSD_FLAG_PARALLELEXEC: i32=0x0200; pub const CEPH_OSD_FLAG_PGOP: i32=0x0400; pub const CEPH_OSD_FLAG_EXEC: i32=0x0800;
pub const CEPH_OSD_FLAG_EXEC_PUBLIC: i32=0x1000; pub const CEPH_OSD_FLAG_LOCALIZE_READS: i32=0x2000; pub const CEPH_OSD_FLAG_RWORDERED: i32=0x4000; pub const CEPH_OSD_FLAG_IGNORE_CACHE: i32=0x8000;
pub const CEPH_OSD_FLAG_SKIPRWLOCKS: i32=0x10000; pub const CEPH_OSD_FLAG_IGNORE_OVERLAY: i32=0x20000; pub const CEPH_OSD_FLAG_FLUSH: i32=0x40000; pub const CEPH_OSD_FLAG_MAP_SNAP_CLONE: i32=0x80000;
pub const CEPH_OSD_FLAG_ENFORCE_SNAPC: i32=0x100000; pub const CEPH_OSD_FLAG_REDIRECTED: i32=0x200000; pub const CEPH_OSD_FLAG_KNOWN_REDIR: i32=0x400000; pub const CEPH_OSD_FLAG_FULL_TRY: i32=0x800000; pub const CEPH_OSD_FLAG_FULL_FORCE: i32=0x1000000;
pub const CEPH_OSD_OP_FLAG_EXCL:i32=1; pub const CEPH_OSD_OP_FLAG_FAILOK:i32=2; pub const CEPH_OSD_OP_FLAG_FADVISE_RANDOM:i32=4; pub const CEPH_OSD_OP_FLAG_FADVISE_SEQUENTIAL:i32=8; pub const CEPH_OSD_OP_FLAG_FADVISE_WILLNEED:i32=0x10; pub const CEPH_OSD_OP_FLAG_FADVISE_DONTNEED:i32=0x20; pub const CEPH_OSD_OP_FLAG_FADVISE_NOCACHE:i32=0x40;
pub const EOLDSNAPC: i32 = ERESTART; pub const EBLOCKLISTED: i32 = ESHUTDOWN;

pub const CEPH_OSD_CMPXATTR_OP_NOP:i32=0; pub const CEPH_OSD_CMPXATTR_OP_EQ:i32=1; pub const CEPH_OSD_CMPXATTR_OP_NE:i32=2; pub const CEPH_OSD_CMPXATTR_OP_GT:i32=3; pub const CEPH_OSD_CMPXATTR_OP_GTE:i32=4; pub const CEPH_OSD_CMPXATTR_OP_LT:i32=5; pub const CEPH_OSD_CMPXATTR_OP_LTE:i32=6;
pub const CEPH_OSD_CMPXATTR_MODE_STRING:i32=1; pub const CEPH_OSD_CMPXATTR_MODE_U64:i32=2;
pub const CEPH_OSD_COPY_FROM_FLAG_FLUSH:i32=1; pub const CEPH_OSD_COPY_FROM_FLAG_IGNORE_OVERLAY:i32=2; pub const CEPH_OSD_COPY_FROM_FLAG_IGNORE_CACHE:i32=4; pub const CEPH_OSD_COPY_FROM_FLAG_MAP_SNAP_CLONE:i32=8; pub const CEPH_OSD_COPY_FROM_FLAG_RWORDERED:i32=16; pub const CEPH_OSD_COPY_FROM_FLAG_TRUNCATE_SEQ:i32=32;
pub const CEPH_OSD_WATCH_OP_UNWATCH:i32=0; pub const CEPH_OSD_WATCH_OP_LEGACY_WATCH:i32=1; pub const CEPH_OSD_WATCH_OP_WATCH:i32=3; pub const CEPH_OSD_WATCH_OP_RECONNECT:i32=5; pub const CEPH_OSD_WATCH_OP_PING:i32=7;
extern "C" { pub fn ceph_osd_watch_op_name(o: i32) -> *const core::ffi::c_char; }
pub const CEPH_OSD_ALLOC_HINT_FLAG_SEQUENTIAL_WRITE:i32=1; pub const CEPH_OSD_ALLOC_HINT_FLAG_RANDOM_WRITE:i32=2; pub const CEPH_OSD_ALLOC_HINT_FLAG_SEQUENTIAL_READ:i32=4; pub const CEPH_OSD_ALLOC_HINT_FLAG_RANDOM_READ:i32=8; pub const CEPH_OSD_ALLOC_HINT_FLAG_APPEND_ONLY:i32=16; pub const CEPH_OSD_ALLOC_HINT_FLAG_IMMUTABLE:i32=32; pub const CEPH_OSD_ALLOC_HINT_FLAG_SHORTLIVED:i32=64; pub const CEPH_OSD_ALLOC_HINT_FLAG_LONGLIVED:i32=128; pub const CEPH_OSD_ALLOC_HINT_FLAG_COMPRESSIBLE:i32=256; pub const CEPH_OSD_ALLOC_HINT_FLAG_INCOMPRESSIBLE:i32=512;
pub const CEPH_OSD_BACKOFF_OP_BLOCK:i32=1; pub const CEPH_OSD_BACKOFF_OP_ACK_BLOCK:i32=2; pub const CEPH_OSD_BACKOFF_OP_UNBLOCK:i32=3;

#[repr(C, packed)] pub struct ceph_osd_op_extent { pub offset: __le64, pub length: __le64, pub truncate_size: __le64, pub truncate_seq: __le32 }
#[repr(C, packed)] pub struct ceph_osd_op_xattr { pub name_len: __le32, pub value_len: __le32, pub cmp_op: __u8, pub cmp_mode: __u8 }
#[repr(C, packed)] pub struct ceph_osd_op_cls { pub class_len: __u8, pub method_len: __u8, pub argc: __u8, pub indata_len: __le32 }
#[repr(C, packed)] pub struct ceph_osd_op_pgls { pub cookie: __le64, pub count: __le64 }
#[repr(C, packed)] pub struct ceph_osd_op_snap { pub snapid: __le64 }
#[repr(C, packed)] pub struct ceph_osd_op_watch { pub cookie: __le64, pub ver: __le64, pub op: __u8, pub gen: __le32 }
#[repr(C, packed)] pub struct ceph_osd_op_notify { pub cookie: __le64 }
#[repr(C, packed)] pub struct ceph_osd_op_assert_ver { pub unused: __le64, pub ver: __le64 }
#[repr(C, packed)] pub struct ceph_osd_op_clonerange { pub offset: __le64, pub length: __le64, pub src_offset: __le64 }
#[repr(C, packed)] pub struct ceph_osd_op_alloc_hint { pub expected_object_size: __le64, pub expected_write_size: __le64, pub flags: __le32 }
#[repr(C, packed)] pub struct ceph_osd_op_copy_from { pub snapid: __le64, pub src_version: __le64, pub flags: __u8, pub src_fadvise_flags: __le32 }
#[repr(C)] pub union ceph_osd_op_data { pub extent: ceph_osd_op_extent, pub xattr: ceph_osd_op_xattr, pub cls: ceph_osd_op_cls, pub pgls: ceph_osd_op_pgls, pub snap: ceph_osd_op_snap, pub watch: ceph_osd_op_watch, pub notify: ceph_osd_op_notify, pub assert_ver: ceph_osd_op_assert_ver, pub clonerange: ceph_osd_op_clonerange, pub alloc_hint: ceph_osd_op_alloc_hint, pub copy_from: ceph_osd_op_copy_from }
#[repr(C, packed)] pub struct ceph_osd_op { pub op: __le16, pub flags: __le32, pub data: ceph_osd_op_data, pub payload_len: __le32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
