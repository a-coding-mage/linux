/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) International Business Machines Corp., 2000-2004 */
/* Portions Copyright (C) Christoph Hellwig, 2001-2002 */

// Dependencies supplied by the surrounding translation unit:
// linux::uuid, jfs_filsys, and jfs_lock.

pub const LOGPSIZE: usize = 4096;
pub const L2LOGPSIZE: u32 = 12;
pub const LOGPAGES: usize = 16;

pub const LOGSUPER_B: u32 = 1;
pub const LOGSTART_B: u32 = 2;
pub const LOGMAGIC: u32 = 0x87654321;
pub const LOGVERSION: u32 = 1;
pub const MAX_ACTIVE: usize = 128;

#[repr(C)]
pub struct logsuper {
    pub magic: __le32,
    pub version: __le32,
    pub serial: __le32,
    pub size: __le32,
    pub bsize: __le32,
    pub l2bsize: __le32,
    pub flag: __le32,
    pub state: __le32,
    pub end: __le32,
    pub uuid: uuid_t,
    pub label: [::std::os::raw::c_char; 16],
    pub active: [logsuper_active; MAX_ACTIVE],
}

#[repr(C)]
pub struct logsuper_active { pub uuid: uuid_t }

pub const LOGMOUNT: u32 = 0;
pub const LOGREDONE: u32 = 1;
pub const LOGWRAP: u32 = 2;
pub const LOGREADERR: u32 = 3;

#[repr(C)]
pub struct logpage_header { pub page: __le32, pub rsrvd: __le16, pub eor: __le16 }
#[repr(C)]
pub struct logpage_trailer { pub page: __le32, pub rsrvd: __le16, pub eor: __le16 }
#[repr(C)]
pub struct logpage {
    pub h: logpage_header,
    pub data: [__le32; LOGPSIZE / 4 - 4],
    pub t: logpage_trailer,
}
pub const LOGPHDRSIZE: usize = 8;
pub const LOGPTLRSIZE: usize = 8;

pub const LOG_COMMIT: u16 = 0x8000;
pub const LOG_SYNCPT: u16 = 0x4000;
pub const LOG_MOUNT: u16 = 0x2000;
pub const LOG_REDOPAGE: u16 = 0x0800;
pub const LOG_NOREDOPAGE: u16 = 0x0080;
pub const LOG_NOREDOINOEXT: u16 = 0x0040;
pub const LOG_UPDATEMAP: u16 = 0x0008;
pub const LOG_NOREDOFILE: u16 = 0x0001;
pub const LOG_INODE: u16 = 0x0001;
pub const LOG_XTREE: u16 = 0x0002;
pub const LOG_DTREE: u16 = 0x0004;
pub const LOG_BTROOT: u16 = 0x0010;
pub const LOG_EA: u16 = 0x0020;
pub const LOG_ACL: u16 = 0x0040;
pub const LOG_DATA: u16 = 0x0080;
pub const LOG_NEW: u16 = 0x0100;
pub const LOG_EXTEND: u16 = 0x0200;
pub const LOG_RELOCATE: u16 = 0x0400;
pub const LOG_DIR_XTREE: u16 = 0x0800;
pub const LOG_ALLOCXADLIST: u16 = 0x0080;
pub const LOG_ALLOCPXDLIST: u16 = 0x0040;
pub const LOG_ALLOCXAD: u16 = 0x0020;
pub const LOG_ALLOCPXD: u16 = 0x0010;
pub const LOG_FREEXADLIST: u16 = 0x0008;
pub const LOG_FREEPXDLIST: u16 = 0x0004;
pub const LOG_FREEXAD: u16 = 0x0002;
pub const LOG_FREEPXD: u16 = 0x0001;

#[repr(C)]
pub struct lrd {
    pub logtid: __le32, pub backchain: __le32, pub type_: __le16, pub length: __le16,
    pub aggregate: __le32, pub log: lrd_log,
}
#[repr(C)]
pub union lrd_log {
    pub redopage: lrd_redopage, pub noredopage: lrd_noredopage, pub updatemap: lrd_updatemap,
    pub noredoinoext: lrd_noredoinoext, pub syncpt: lrd_syncpt, pub freextent: lrd_freextent,
    pub noredofile: lrd_noredofile, pub newpage: lrd_newpage,
}
#[repr(C)] pub struct lrd_redopage { pub fileset: __le32, pub inode: __le32, pub type_: __le16, pub l2linesize: __le16, pub pxd: pxd_t }
#[repr(C)] pub struct lrd_noredopage { pub fileset: __le32, pub inode: __le32, pub type_: __le16, pub rsrvd: __le16, pub pxd: pxd_t }
#[repr(C)] pub struct lrd_updatemap { pub fileset: __le32, pub inode: __le32, pub type_: __le16, pub nxd: __le16, pub pxd: pxd_t }
#[repr(C)] pub struct lrd_noredoinoext { pub fileset: __le32, pub iagnum: __le32, pub inoext_idx: __le32, pub pxd: pxd_t }
#[repr(C)] pub struct lrd_syncpt { pub sync: __le32 }
#[repr(C)] pub struct lrd_freextent { pub type_: __le32, pub nextent: __le32 }
#[repr(C)] pub struct lrd_noredofile { pub fileset: __le32, pub inode: __le32 }
#[repr(C)] pub struct lrd_newpage { pub fileset: __le32, pub inode: __le32, pub type_: __le32, pub pxd: pxd_t }
pub const LOGRDSIZE: usize = core::mem::size_of::<lrd>();

#[repr(C)] pub struct lvd { pub offset: __le16, pub length: __le16 }

#[repr(C)]
pub struct jfs_log {
    pub sb_list: list_head, pub journal_list: list_head, pub bdev_file: *mut file, pub serial: ::std::os::raw::c_int,
    pub base: s64, pub size: ::std::os::raw::c_int, pub l2bsize: ::std::os::raw::c_int, pub flag: ::std::os::raw::c_ulong,
    pub lbuf_free: *mut lbuf, pub free_wait: wait_queue_head_t, pub logtid: ::std::os::raw::c_int,
    pub page: ::std::os::raw::c_int, pub eor: ::std::os::raw::c_int, pub bp: *mut lbuf, pub loglock: mutex,
    pub nextsync: ::std::os::raw::c_int, pub active: ::std::os::raw::c_int, pub syncwait: wait_queue_head_t,
    pub cflag: uint, pub cqueue: list_head, pub flush_tblk: *mut tblock, pub gcrtc: ::std::os::raw::c_int,
    pub gclrt: *mut tblock, pub gclock: spinlock_t, pub logsize: ::std::os::raw::c_int, pub lsn: ::std::os::raw::c_int,
    pub clsn: ::std::os::raw::c_int, pub syncpt: ::std::os::raw::c_int, pub sync: ::std::os::raw::c_int,
    pub synclist: list_head, pub synclock: spinlock_t, pub wqueue: *mut lbuf, pub count: ::std::os::raw::c_int,
    pub uuid: uuid_t, pub no_integrity: ::std::os::raw::c_int,
}

pub const log_INLINELOG: u32 = 1; pub const log_SYNCBARRIER: u32 = 2; pub const log_QUIESCE: u32 = 3; pub const log_FLUSH: u32 = 4;
pub const logGC_PAGEOUT: u32 = 0x00000001;
pub const tblkGC_QUEUE: u16 = 0x0001; pub const tblkGC_READY: u16 = 0x0002; pub const tblkGC_COMMIT: u16 = 0x0004; pub const tblkGC_COMMITTED: u16 = 0x0008; pub const tblkGC_EOP: u16 = 0x0010; pub const tblkGC_FREE: u16 = 0x0020; pub const tblkGC_LEADER: u16 = 0x0040; pub const tblkGC_ERROR: u16 = 0x0080; pub const tblkGC_LAZY: u16 = 0x0100; pub const tblkGC_UNLOCKED: u16 = 0x0200;

#[repr(C)] pub struct lbuf {
    pub l_log: *mut jfs_log, pub l_flag: uint, pub l_wqnext: *mut lbuf, pub l_freelist: *mut lbuf,
    pub l_pn: ::std::os::raw::c_int, pub l_eor: ::std::os::raw::c_int, pub l_ceor: ::std::os::raw::c_int,
    pub l_blkno: s64, pub l_ldata: caddr_t, pub l_page: *mut page, pub l_offset: uint, pub l_ioevent: wait_queue_head_t,
}
pub type l_redrive_next = l_freelist;
#[repr(C)] pub struct logsyncblk { pub xflag: u16, pub flag: u16, pub lid: lid_t, pub lsn: s32, pub synclist: list_head }

#[macro_export] macro_rules! LOG_LOCK_INIT { ($log:expr) => { mutex_init(&mut (*$log).loglock) }; }
#[macro_export] macro_rules! LOG_LOCK { ($log:expr) => { mutex_lock(&mut (*$log).loglock) }; }
#[macro_export] macro_rules! LOG_UNLOCK { ($log:expr) => { mutex_unlock(&mut (*$log).loglock) }; }
#[macro_export] macro_rules! LOGSYNC_LOCK_INIT { ($log:expr) => { spin_lock_init(&mut (*$log).synclock) }; }
#[macro_export] macro_rules! LOGSYNC_LOCK { ($log:expr, $flags:expr) => { spin_lock_irqsave(&mut (*$log).synclock, $flags) }; }
#[macro_export] macro_rules! LOGSYNC_UNLOCK { ($log:expr, $flags:expr) => { spin_unlock_irqrestore(&mut (*$log).synclock, $flags) }; }
#[macro_export] macro_rules! logdiff { ($diff:ident, $lsn:expr, $log:expr) => {{ $diff = ($lsn) - (*$log).syncpt; if $diff < 0 { $diff += (*$log).logsize; } }}; }

extern "C" {
    pub fn lmLogOpen(sb: *mut super_block) -> ::std::os::raw::c_int;
    pub fn lmLogClose(sb: *mut super_block) -> ::std::os::raw::c_int;
    pub fn lmLogShutdown(log: *mut jfs_log) -> ::std::os::raw::c_int;
    pub fn lmLogInit(log: *mut jfs_log) -> ::std::os::raw::c_int;
    pub fn lmLogFormat(log: *mut jfs_log, logAddress: s64, logSize: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn lmGroupCommit(log: *mut jfs_log, tblk: *mut tblock) -> ::std::os::raw::c_int;
    pub fn jfsIOWait(arg: *mut ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn jfs_flush_journal(log: *mut jfs_log, wait: ::std::os::raw::c_int);
    pub fn jfs_syncpt(log: *mut jfs_log, hard_sync: ::std::os::raw::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
