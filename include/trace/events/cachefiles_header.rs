/* SPDX-License-Identifier: GPL-2.0-or-later */
/* CacheFiles tracepoints; Rust translation of trace/events/cachefiles.h. */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CachefilesObjRefTrace {
    GetIoreq,
    New,
    PutAllocFail,
    PutDetach,
    PutIoreq,
    SeeCleanCommit,
    SeeCleanDelete,
    SeeCleanDropTmp,
    SeeLookupCookie,
    SeeLookupFailed,
    SeeWithdrawCookie,
    SeeWithdrawal,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FscacheWhyObjectKilled {
    IsStale,
    IsWeird,
    Invalidated,
    NoSpace,
    WasRetired,
    WasCulled,
    VolumeIsWeird,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CachefilesCoherencyTrace {
    CheckAux,
    CheckContent,
    CheckDirty,
    CheckLen,
    CheckObjSize,
    CheckOk,
    CheckType,
    CheckXattr,
    SetFail,
    SetOk,
    VolCheckCmp,
    VolCheckOk,
    VolCheckResv,
    VolCheckXattr,
    VolSetFail,
    VolSetOk,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CachefilesTruncTrace { DioAdjust, ExpandTmpfile, Shrink }

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CachefilesPrepareReadTrace {
    ReadAfterEof, ReadFoundHole, ReadFoundPart, ReadHaveData,
    ReadNoData, ReadNoFile, ReadSeekError, ReadSeekNxio,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CachefilesErrorTrace {
    FallocateError, GetxattrError, LinkError, LookupError, MkdirError,
    NotifyChangeError, OpenError, ReadError, RemxattrError, RenameError,
    SeekError, SetxattrError, StatfsError, TmpfileError, TruncError,
    UnlinkError, WriteError,
}

pub const CACHEFILES_OBJ_KILL_TRACES: &[(FscacheWhyObjectKilled, &str)] = &[
    (FscacheWhyObjectKilled::IsStale, "stale"),
    (FscacheWhyObjectKilled::IsWeird, "weird"),
    (FscacheWhyObjectKilled::Invalidated, "inval"),
    (FscacheWhyObjectKilled::NoSpace, "no_space"),
    (FscacheWhyObjectKilled::WasRetired, "was_retired"),
    (FscacheWhyObjectKilled::WasCulled, "was_culled"),
    (FscacheWhyObjectKilled::VolumeIsWeird, "volume_weird"),
];

pub const CACHEFILES_OBJ_REF_TRACES: &[(CachefilesObjRefTrace, &str)] = &[
    (CachefilesObjRefTrace::GetIoreq, "GET ioreq"),
    (CachefilesObjRefTrace::New, "NEW obj"),
    (CachefilesObjRefTrace::PutAllocFail, "PUT alloc_fail"),
    (CachefilesObjRefTrace::PutDetach, "PUT detach"),
    (CachefilesObjRefTrace::PutIoreq, "PUT ioreq"),
    (CachefilesObjRefTrace::SeeCleanCommit, "SEE clean_commit"),
    (CachefilesObjRefTrace::SeeCleanDelete, "SEE clean_delete"),
    (CachefilesObjRefTrace::SeeCleanDropTmp, "SEE clean_drop_tmp"),
    (CachefilesObjRefTrace::SeeLookupCookie, "SEE lookup_cookie"),
    (CachefilesObjRefTrace::SeeLookupFailed, "SEE lookup_failed"),
    (CachefilesObjRefTrace::SeeWithdrawCookie, "SEE withdraw_cookie"),
    (CachefilesObjRefTrace::SeeWithdrawal, "SEE withdrawal"),
];

pub const CACHEFILES_COHERENCY_TRACES: &[(&str, &str)] = &[
    ("cachefiles_coherency_check_aux", "BAD aux "),
    ("cachefiles_coherency_check_content", "BAD cont"),
    ("cachefiles_coherency_check_dirty", "BAD dirt"),
    ("cachefiles_coherency_check_len", "BAD len "),
    ("cachefiles_coherency_check_objsize", "BAD osiz"),
    ("cachefiles_coherency_check_ok", "OK      "),
    ("cachefiles_coherency_check_type", "BAD type"),
    ("cachefiles_coherency_check_xattr", "BAD xatt"),
    ("cachefiles_coherency_set_fail", "SET fail"),
    ("cachefiles_coherency_set_ok", "SET ok  "),
    ("cachefiles_coherency_vol_check_cmp", "VOL BAD cmp "),
    ("cachefiles_coherency_vol_check_ok", "VOL OK      "),
    ("cachefiles_coherency_vol_check_resv", "VOL BAD resv"),
    ("cachefiles_coherency_vol_check_xattr", "VOL BAD xatt"),
    ("cachefiles_coherency_vol_set_fail", "VOL SET fail"),
    ("cachefiles_coherency_vol_set_ok", "VOL SET ok  "),
];

pub const CACHEFILES_TRUNC_TRACES: &[(&str, &str)] = &[
    ("cachefiles_trunc_dio_adjust", "DIOADJ"),
    ("cachefiles_trunc_expand_tmpfile", "EXPTMP"),
    ("cachefiles_trunc_shrink", "SHRINK"),
];

pub const CACHEFILES_PREPARE_READ_TRACES: &[(&str, &str)] = &[
    ("cachefiles_trace_read_after_eof", "after-eof "),
    ("cachefiles_trace_read_found_hole", "found-hole"),
    ("cachefiles_trace_read_found_part", "found-part"),
    ("cachefiles_trace_read_have_data", "have-data "),
    ("cachefiles_trace_read_no_data", "no-data   "),
    ("cachefiles_trace_read_no_file", "no-file   "),
    ("cachefiles_trace_read_seek_error", "seek-error"),
    ("cachefiles_trace_read_seek_nxio", "seek-enxio"),
];

pub const CACHEFILES_ERROR_TRACES: &[(&str, &str)] = &[
    ("cachefiles_trace_fallocate_error", "fallocate"),
    ("cachefiles_trace_getxattr_error", "getxattr"),
    ("cachefiles_trace_link_error", "link"),
    ("cachefiles_trace_lookup_error", "lookup"),
    ("cachefiles_trace_mkdir_error", "mkdir"),
    ("cachefiles_trace_notify_change_error", "notify_change"),
    ("cachefiles_trace_open_error", "open"),
    ("cachefiles_trace_read_error", "read"),
    ("cachefiles_trace_remxattr_error", "remxattr"),
    ("cachefiles_trace_rename_error", "rename"),
    ("cachefiles_trace_seek_error", "seek"),
    ("cachefiles_trace_setxattr_error", "setxattr"),
    ("cachefiles_trace_statfs_error", "statfs"),
    ("cachefiles_trace_tmpfile_error", "tmpfile"),
    ("cachefiles_trace_trunc_error", "trunc"),
    ("cachefiles_trace_unlink_error", "unlink"),
    ("cachefiles_trace_write_error", "write"),
];

/* External kernel types and enums are supplied by other translated files. */
pub enum CachefilesObject {}
pub enum CachefilesVolume {}
pub enum Dentry {}
pub enum Inode {}

#[repr(C)] pub struct CachefilesRefEntry { pub obj: u32, pub cookie: u32, pub why: CachefilesObjRefTrace, pub usage: i32 }
#[repr(C)] pub struct CachefilesLookupEntry { pub dino: u64, pub ino: u64, pub obj: u32, pub error: i16 }
#[repr(C)] pub struct CachefilesMkdirEntry { pub dir: u32, pub subdir: u32 }
#[repr(C)] pub struct CachefilesTmpfileEntry { pub obj: u32, pub backer: u32 }
#[repr(C)] pub struct CachefilesLinkEntry { pub obj: u32, pub backer: u32 }
#[repr(C)] pub struct CachefilesUnlinkEntry { pub obj: u32, pub ino: u32, pub why: FscacheWhyObjectKilled }
#[repr(C)] pub struct CachefilesRenameEntry { pub obj: u32, pub ino: u32, pub why: FscacheWhyObjectKilled }
#[repr(C)] pub struct CachefilesCoherencyEntry { pub obj: u32, pub why: CachefilesCoherencyTrace, pub content: u32, pub ino: u64, pub aux: u64, pub disk_aux: u64 }
#[repr(C)] pub struct CachefilesVolCoherencyEntry { pub vol: u32, pub why: CachefilesCoherencyTrace, pub ino: u64 }
#[repr(C)] pub struct CachefilesPrepReadEntry { pub obj: u32, pub flags: u16, pub source: u32, pub why: CachefilesPrepareReadTrace, pub len: usize, pub start: i64, pub netfs_inode: u32, pub cache_inode: u32 }
#[repr(C)] pub struct CachefilesReadEntry { pub obj: u32, pub backer: u32, pub len: usize, pub start: i64 }
#[repr(C)] pub struct CachefilesWriteEntry { pub obj: u32, pub backer: u32, pub len: usize, pub start: i64 }
#[repr(C)] pub struct CachefilesTruncEntry { pub obj: u32, pub backer: u32, pub why: CachefilesTruncTrace, pub from: i64, pub to: i64 }
#[repr(C)] pub struct CachefilesMarkEntry { pub inode: u64, pub obj: u32 }
#[repr(C)] pub struct CachefilesVfsErrorEntry { pub obj: u32, pub backer: u32, pub where_: CachefilesErrorTrace, pub error: i16 }
#[repr(C)] pub struct CachefilesIoErrorEntry { pub obj: u32, pub backer: u32, pub where_: CachefilesErrorTrace, pub error: i16 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
