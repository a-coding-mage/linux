/* SPDX-License-Identifier: GPL-2.0 */
// Translated from trace/events/timestamp.h.
// The C tracepoint declarations below are represented as Rust data layouts and
// assignment/formatting helpers; tracepoint registration is supplied externally.

use core::ffi::c_char;

// I_CTIME_QUERIED and the kernel inode/timespec64 definitions are supplied by
// the corresponding kernel dependencies.
extern "C" {
    static I_CTIME_QUERIED: u32;
}

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
    pub i_ino: u64,
    pub i_generation: u32,
}

#[repr(C)]
pub struct super_block {
    pub s_dev: dev_t,
}

#[repr(C)]
pub struct timespec64 {
    pub tv_sec: time64_t,
    pub tv_nsec: i64,
}

pub type dev_t = u64;
pub type time64_t = i64;

#[repr(C)]
pub struct CtimeEntry {
    pub ino: u64,
    pub ctime_s: time64_t,
    pub dev: dev_t,
    pub ctime_ns: u32,
    pub gen: u32,
}

#[repr(C)]
pub struct CtimeNsXchgEntry {
    pub ino: u64,
    pub dev: dev_t,
    pub gen: u32,
    pub old: u32,
    pub new: u32,
    pub cur: u32,
}

#[repr(C)]
pub struct FillMgCmtimeEntry {
    pub ino: u64,
    pub ctime_s: time64_t,
    pub mtime_s: time64_t,
    pub dev: dev_t,
    pub ctime_ns: u32,
    pub mtime_ns: u32,
    pub gen: u32,
}

pub const CTIME_QUERIED_FLAGS: &[(u32, &str)] = &[(unsafe { I_CTIME_QUERIED }, "Q")];

#[inline]
pub unsafe fn ctime_fast_assign(inode: *const inode, ctime: *const timespec64) -> CtimeEntry {
    CtimeEntry {
        dev: (*(*inode).i_sb).s_dev,
        ino: (*inode).i_ino,
        gen: (*inode).i_generation,
        ctime_s: (*ctime).tv_sec,
        ctime_ns: (*ctime).tv_nsec as u32,
    }
}

#[inline]
pub unsafe fn ctime_ns_xchg_fast_assign(
    inode: *const inode,
    old: u32,
    new: u32,
    cur: u32,
) -> CtimeNsXchgEntry {
    CtimeNsXchgEntry {
        dev: (*(*inode).i_sb).s_dev,
        ino: (*inode).i_ino,
        gen: (*inode).i_generation,
        old,
        new,
        cur,
    }
}

#[inline]
pub unsafe fn fill_mg_cmtime_fast_assign(
    inode: *const inode,
    ctime: *const timespec64,
    mtime: *const timespec64,
) -> FillMgCmtimeEntry {
    FillMgCmtimeEntry {
        dev: (*(*inode).i_sb).s_dev,
        ino: (*inode).i_ino,
        gen: (*inode).i_generation,
        ctime_s: (*ctime).tv_sec,
        mtime_s: (*mtime).tv_sec,
        ctime_ns: (*ctime).tv_nsec as u32,
        mtime_ns: (*mtime).tv_nsec as u32,
    }
}

// Event classes/events declared by the C tracepoint macros:
// ctime; inode_set_ctime_to_ts; ctime_xchg_skip; ctime_ns_xchg; fill_mg_cmtime.
// Their TP_PROTO signatures are respectively:
// (*mut inode, *mut timespec64), (*mut inode, u32, u32, u32), and
// (*mut inode, *mut timespec64, *mut timespec64).


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
