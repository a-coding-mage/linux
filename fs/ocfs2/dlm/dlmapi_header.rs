/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * dlmapi.h
 *
 * externally exported dlm interfaces
 *
 * Copyright (C) 2004 Oracle.  All rights reserved.
 */

// C forward declarations.
#[repr(C)]
pub struct dlm_lock {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dlm_ctxt {
    _private: [u8; 0],
}

/* NOTE: changes made to this enum should be reflected in dlmdebug.c */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dlm_status {
    DLM_NORMAL = 0,           /*  0: request in progress */
    DLM_GRANTED,              /*  1: request granted */
    DLM_DENIED,               /*  2: request denied */
    DLM_DENIED_NOLOCKS,       /*  3: request denied, out of system resources */
    DLM_WORKING,              /*  4: async request in progress */
    DLM_BLOCKED,              /*  5: lock request blocked */
    DLM_BLOCKED_ORPHAN,       /*  6: lock request blocked by a orphan lock*/
    DLM_DENIED_GRACE_PERIOD,  /*  7: topological change in progress */
    DLM_SYSERR,               /*  8: system error */
    DLM_NOSUPPORT,            /*  9: unsupported */
    DLM_CANCELGRANT,          /* 10: can't cancel convert: already granted */
    DLM_IVLOCKID,             /* 11: bad lockid */
    DLM_SYNC,                 /* 12: synchronous request granted */
    DLM_BADTYPE,              /* 13: bad resource type */
    DLM_BADRESOURCE,          /* 14: bad resource handle */
    DLM_MAXHANDLES,           /* 15: no more resource handles */
    DLM_NOCLINFO,             /* 16: can't contact cluster manager */
    DLM_NOLOCKMGR,            /* 17: can't contact lock manager */
    DLM_NOPURGED,             /* 18: can't contact purge daemon */
    DLM_BADARGS,              /* 19: bad api args */
    DLM_VOID,                 /* 20: no status */
    DLM_NOTQUEUED,            /* 21: NOQUEUE was specified and request failed */
    DLM_IVBUFLEN,             /* 22: invalid resource name length */
    DLM_CVTUNGRANT,           /* 23: attempted to convert ungranted lock */
    DLM_BADPARAM,             /* 24: invalid lock mode specified */
    DLM_VALNOTVALID,          /* 25: value block has been invalidated */
    DLM_REJECTED,             /* 26: request rejected, unrecognized client */
    DLM_ABORT,                /* 27: blocked lock request cancelled */
    DLM_CANCEL,               /* 28: conversion request cancelled */
    DLM_IVRESHANDLE,          /* 29: invalid resource handle */
    DLM_DEADLOCK,             /* 30: deadlock recovery refused this request */
    DLM_DENIED_NOASTS,        /* 31: failed to allocate AST */
    DLM_FORWARD,              /* 32: request must wait for primary's response */
    DLM_TIMEOUT,              /* 33: timeout value for lock has expired */
    DLM_IVGROUPID,            /* 34: invalid group specification */
    DLM_VERS_CONFLICT,        /* 35: version conflicts prevent request handling */
    DLM_BAD_DEVICE_PATH,      /* 36: Locks device does not exist or path wrong */
    DLM_NO_DEVICE_PERMISSION, /* 37: Client has insufficient pers for device */
    DLM_NO_CONTROL_DEVICE,    /* 38: Cannot set options on opened device */
    DLM_RECOVERING,           /* 39: extension, allows caller to fail a lock request if it is being recovered */
    DLM_MIGRATING,            /* 40: extension, allows caller to fail a lock request if it is being migrated */
    DLM_MAXSTATS,             /* 41: upper limit for return code validation */
}

/* for pretty-printing dlm_status error names */
extern "C" {
    pub fn dlm_errname(err: dlm_status) -> *const std::ffi::c_char;
}

/* Eventually the DLM will use standard errno values, but in the meantime this lets us track dlm errors as they bubble up. */
#[macro_export]
macro_rules! dlm_error {
    ($st:expr) => {{
        if ($st) != $crate::dlm_status::DLM_RECOVERING &&
           ($st) != $crate::dlm_status::DLM_MIGRATING &&
           ($st) != $crate::dlm_status::DLM_FORWARD {
            unsafe { mlog(ML_ERROR, b"dlm status = %s\\n\\0".as_ptr() as _, dlm_errname($st)); }
        }
    }};
}

pub const DLM_LKSB_UNUSED1: u32 = 0x01;
pub const DLM_LKSB_PUT_LVB: u32 = 0x02;
pub const DLM_LKSB_GET_LVB: u32 = 0x04;
pub const DLM_LKSB_UNUSED2: u32 = 0x08;
pub const DLM_LKSB_UNUSED3: u32 = 0x10;
pub const DLM_LKSB_UNUSED4: u32 = 0x20;
pub const DLM_LKSB_UNUSED5: u32 = 0x40;
pub const DLM_LKSB_UNUSED6: u32 = 0x80;
pub const DLM_LVB_LEN: usize = 64;

/* Callers are only allowed access to the lvb and status members of this struct. */
#[repr(C)]
pub struct dlm_lockstatus {
    pub status: dlm_status,
    pub flags: u32,
    pub lockid: *mut dlm_lock,
    pub lvb: [std::ffi::c_char; DLM_LVB_LEN],
}

pub const LKM_IVMODE: i32 = -1;
pub const LKM_NLMODE: i32 = 0;
pub const LKM_CRMODE: i32 = 1;
pub const LKM_CWMODE: i32 = 2;
pub const LKM_PRMODE: i32 = 3;
pub const LKM_PWMODE: i32 = 4;
pub const LKM_EXMODE: i32 = 5;
pub const LKM_MAXMODE: i32 = 5;
pub const LKM_MODEMASK: i32 = 0xff;

pub const LKM_ORPHAN: u32 = 0x00000010;
pub const LKM_PARENTABLE: u32 = 0x00000020;
pub const LKM_BLOCK: u32 = 0x00000040;
pub const LKM_LOCAL: u32 = 0x00000080;
pub const LKM_VALBLK: u32 = 0x00000100;
pub const LKM_NOQUEUE: u32 = 0x00000200;
pub const LKM_CONVERT: u32 = 0x00000400;
pub const LKM_NODLCKWT: u32 = 0x00000800;
pub const LKM_UNLOCK: u32 = 0x00001000;
pub const LKM_CANCEL: u32 = 0x00002000;
pub const LKM_DEQALL: u32 = 0x00004000;
pub const LKM_INVVALBLK: u32 = 0x00008000;
pub const LKM_SYNCSTS: u32 = 0x00010000;
pub const LKM_TIMEOUT: u32 = 0x00020000;
pub const LKM_SNGLDLCK: u32 = 0x00040000;
pub const LKM_FINDLOCAL: u32 = 0x00080000;
pub const LKM_PROC_OWNED: u32 = 0x00100000;
pub const LKM_XID: u32 = 0x00200000;
pub const LKM_XID_CONFLICT: u32 = 0x00400000;
pub const LKM_FORCE: u32 = 0x00800000;
pub const LKM_REVVALBLK: u32 = 0x01000000;
pub const LKM_UNUSED1: u32 = 0x00000001;
pub const LKM_UNUSED2: u32 = 0x00000002;
pub const LKM_UNUSED3: u32 = 0x00000004;
pub const LKM_UNUSED4: u32 = 0x00000008;
pub const LKM_UNUSED5: u32 = 0x02000000;
pub const LKM_UNUSED6: u32 = 0x04000000;
pub const LKM_UNUSED7: u32 = 0x08000000;
pub const LKM_MIGRATION: u32 = 0x10000000;
pub const LKM_PUT_LVB: u32 = 0x20000000;
pub const LKM_GET_LVB: u32 = 0x40000000;
pub const LKM_RECOVERY: u32 = 0x80000000;

pub type dlm_astlockfunc_t = unsafe extern "C" fn(*mut std::ffi::c_void);
pub type dlm_bastlockfunc_t = unsafe extern "C" fn(*mut std::ffi::c_void, i32);
pub type dlm_astunlockfunc_t = unsafe extern "C" fn(*mut std::ffi::c_void, dlm_status);

extern "C" {
    pub fn dlmlock(dlm: *mut dlm_ctxt, mode: i32, lksb: *mut dlm_lockstatus,
                   flags: i32, name: *const std::ffi::c_char, namelen: i32,
                   ast: Option<dlm_astlockfunc_t>, data: *mut std::ffi::c_void,
                   bast: Option<dlm_bastlockfunc_t>) -> dlm_status;
    pub fn dlmunlock(dlm: *mut dlm_ctxt, lksb: *mut dlm_lockstatus, flags: i32,
                     unlockast: Option<dlm_astunlockfunc_t>, data: *mut std::ffi::c_void) -> dlm_status;
}

#[repr(C)]
pub struct dlm_protocol_version { pub pv_major: u8, pub pv_minor: u8 }

extern "C" {
    pub fn dlm_register_domain(domain: *const std::ffi::c_char, key: u32,
                               fs_proto: *mut dlm_protocol_version) -> *mut dlm_ctxt;
    pub fn dlm_unregister_domain(dlm: *mut dlm_ctxt);
    pub fn dlm_print_one_lock(lockid: *mut dlm_lock);
}

pub type dlm_eviction_func = unsafe extern "C" fn(i32, *mut std::ffi::c_void);

#[repr(C)]
pub struct dlm_eviction_cb {
    pub ec_item: list_head,
    pub ec_func: Option<dlm_eviction_func>,
    pub ec_data: *mut std::ffi::c_void,
}

extern "C" {
    pub fn dlm_setup_eviction_cb(cb: *mut dlm_eviction_cb, f: Option<dlm_eviction_func>, data: *mut std::ffi::c_void);
    pub fn dlm_register_eviction_cb(dlm: *mut dlm_ctxt, cb: *mut dlm_eviction_cb);
    pub fn dlm_unregister_eviction_cb(cb: *mut dlm_eviction_cb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
