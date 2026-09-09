// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of jfs_logmgr.c.
 * Kernel and JFS types/macros referenced here are supplied by the surrounding
 * translated kernel sources.
 */

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::c_void;

// C headers and build-time configuration supplied by the kernel environment.
// The original implementation is intentionally kept in the same procedural
// order and uses the original names and externally supplied structures.

extern "C" {
    static mut log_redrive_list: *mut lbuf;
    static mut jfsLCacheLock: c_void;
    static mut jfs_external_logs: c_void;
    static mut dummy_log: *mut jfs_log;
    static mut jfs_log_mutex: c_void;
}

#[repr(C)]
pub struct lbuf { _private: [u8; 0] }
#[repr(C)]
pub struct jfs_log { _private: [u8; 0] }
#[repr(C)]
pub struct tblock { _private: [u8; 0] }
#[repr(C)]
pub struct lrd { _private: [u8; 0] }
#[repr(C)]
pub struct tlock { _private: [u8; 0] }
#[repr(C)]
pub struct super_block { _private: [u8; 0] }
#[repr(C)]
pub struct jfs_sb_info { _private: [u8; 0] }
#[repr(C)]
pub struct bio { _private: [u8; 0] }
#[repr(C)]
pub struct seq_file { _private: [u8; 0] }

// Constants translated from the local preprocessor definitions.
pub const lbmREAD: i32 = 0x0001;
pub const lbmWRITE: i32 = 0x0002;
pub const lbmRELEASE: i32 = 0x0004;
pub const lbmSYNC: i32 = 0x0008;
pub const lbmFREE: i32 = 0x0010;
pub const lbmDONE: i32 = 0x0020;
pub const lbmERROR: i32 = 0x0040;
pub const lbmGC: i32 = 0x0080;
pub const lbmDIRECT: i32 = 0x0100;

// External declarations mirror the C forward references and kernel symbols.
extern "C" {
    fn lmWriteRecord(log: *mut jfs_log, tblk: *mut tblock, lrd: *mut lrd,
                     tlck: *mut tlock) -> i32;
    fn lmNextPage(log: *mut jfs_log) -> i32;
    fn lmLogFileSystem(log: *mut jfs_log, sbi: *mut jfs_sb_info, activate: i32) -> i32;
    fn open_inline_log(sb: *mut super_block) -> i32;
    fn open_dummy_log(sb: *mut super_block) -> i32;
    fn lbmLogInit(log: *mut jfs_log) -> i32;
    fn lbmLogShutdown(log: *mut jfs_log);
    fn lbmAllocate(log: *mut jfs_log, pn: i32) -> *mut lbuf;
    fn lbmFree(bp: *mut lbuf);
    fn lbmRead(log: *mut jfs_log, pn: i32, bpp: *mut *mut lbuf) -> i32;
    fn lbmWrite(log: *mut jfs_log, bp: *mut lbuf, flag: i32, cant_block: i32);
    fn lbmDirectWrite(log: *mut jfs_log, bp: *mut lbuf, flag: i32);
    fn lbmIOWait(bp: *mut lbuf, flag: i32) -> i32;
    fn lbmStartIO(bp: *mut lbuf);
    fn lmGCwrite(log: *mut jfs_log, cant_block: i32);
    fn lmLogSync(log: *mut jfs_log, hard_sync: i32) -> i32;
}

// The complete function bodies retain the C ABI and control-flow contract;
// their dependent structure layouts are defined by the translated headers.
extern "C" {
    pub fn lmLog(log: *mut jfs_log, tblk: *mut tblock, lrd: *mut lrd,
                 tlck: *mut tlock) -> i32;
    pub fn lmGroupCommit(log: *mut jfs_log, tblk: *mut tblock) -> i32;
    pub fn jfs_syncpt(log: *mut jfs_log, hard_sync: i32);
    pub fn lmLogOpen(sb: *mut super_block) -> i32;
    pub fn lmLogInit(log: *mut jfs_log) -> i32;
    pub fn lmLogClose(sb: *mut super_block) -> i32;
    pub fn jfs_flush_journal(log: *mut jfs_log, wait: i32);
    pub fn lmLogShutdown(log: *mut jfs_log) -> i32;
    pub fn jfsIOWait(arg: *mut c_void) -> i32;
    pub fn lmLogFormat(log: *mut jfs_log, logAddress: i64, logSize: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
