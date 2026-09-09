// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) International Business Machines Corp., 2000-2004
 *   Portions Copyright (C) Christoph Hellwig, 2001-2002
 */

// Linux/JFS headers are supplied by the surrounding translation unit.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
extern "C" {
    fn iget_locked(sb: *mut super_block, ino: c_ulong) -> *mut inode;
    fn inode_state_read_once(inode: *mut inode) -> c_ulong;
    fn diRead(inode: *mut inode) -> c_int;
    fn iget_failed(inode: *mut inode);
    fn unlock_new_inode(inode: *mut inode);
    fn printk(fmt: *const c_char, ...);
    fn txBegin(sb: *mut super_block, flags: c_int) -> tid_t;
    fn txCommit(tid: tid_t, count: c_int, inode: *mut *mut inode, flags: c_int) -> c_int;
    fn txEnd(tid: tid_t);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn jfs_flush_journal(log: *mut c_void, wait: c_int);
    fn dquot_initialize(inode: *mut inode);
    fn truncate_inode_pages_final(mapping: *mut address_space);
    fn jfs_free_zero_link(inode: *mut inode);
    fn diFree(inode: *mut inode);
    fn dquot_free_inode(inode: *mut inode);
    fn clear_inode(inode: *mut inode);
    fn dquot_drop(inode: *mut inode);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn atomic_dec(v: *mut atomic_t);
    fn xtLookup(ip: *mut inode, lblock: i64, xlen: i32, xflag: *mut i32, xaddr: *mut i64, len: *mut i32, flags: i32) -> c_int;
    fn extRecord(ip: *mut inode, xad: *mut xad_t) -> c_int;
    fn extHint(ip: *mut inode, offset: i64, xad: *mut xad_t) -> c_int;
    fn extAlloc(ip: *mut inode, xlen: i32, lblock: i64, xad: *mut xad_t, abnr: bool) -> c_int;
    fn map_bh(bh: *mut buffer_head, sb: *mut super_block, block: i64);
    fn set_buffer_new(bh: *mut buffer_head);
    fn block_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> c_int;
    fn block_invalidate_folio(folio: *mut folio, offset: usize, length: usize);
    fn mpage_writepages(mapping: *mut address_space, wbc: *mut writeback_control, get_block: unsafe extern "C" fn(*mut inode, sector_t, *mut buffer_head, c_int) -> c_int) -> c_int;
    fn mpage_read_folio(folio: *mut folio, get_block: unsafe extern "C" fn(*mut inode, sector_t, *mut buffer_head, c_int) -> c_int) -> c_int;
    fn mpage_readahead(rac: *mut readahead_control, get_block: unsafe extern "C" fn(*mut inode, sector_t, *mut buffer_head, c_int) -> c_int);
    fn truncate_pagecache(inode: *mut inode, size: loff_t);
    fn block_write_begin(mapping: *mut address_space, pos: loff_t, len: usize, folio: *mut *mut folio, get_block: unsafe extern "C" fn(*mut inode, sector_t, *mut buffer_head, c_int) -> c_int) -> c_int;
    fn generic_write_end(iocb: *const kiocb, mapping: *mut address_space, pos: loff_t, len: usize, copied: usize, folio: *mut folio, fsdata: *mut c_void) -> isize;
    fn generic_block_bmap(mapping: *mut address_space, block: sector_t, get_block: unsafe extern "C" fn(*mut inode, sector_t, *mut buffer_head, c_int) -> c_int) -> sector_t;
    fn blockdev_direct_IO(iocb: *mut kiocb, inode: *mut inode, iter: *mut iov_iter, get_block: unsafe extern "C" fn(*mut inode, sector_t, *mut buffer_head, c_int) -> c_int) -> isize;
    fn iov_iter_count(iter: *mut iov_iter) -> usize;
    fn iov_iter_rw(iter: *mut iov_iter) -> c_int;
    fn i_size_read(inode: *mut inode) -> loff_t;
    fn block_truncate_page(mapping: *mut address_space, size: loff_t, get_block: unsafe extern "C" fn(*mut inode, sector_t, *mut buffer_head, c_int) -> c_int);
    fn xtTruncate(tid: tid_t, ip: *mut inode, length: loff_t, flags: c_int) -> loff_t;
    fn mark_inode_dirty(inode: *mut inode);
}

type c_int = i32;
type c_ulong = u64;
type c_char = i8;
type c_void = core::ffi::c_void;
type loff_t = i64;
type sector_t = u64;
type tid_t = i32;

#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct buffer_head { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct writeback_control { pub sync_mode: c_int }
#[repr(C)] pub struct readahead_control { _private: [u8; 0] }
#[repr(C)] pub struct kiocb { pub ki_filp: *mut file, pub ki_pos: loff_t }
#[repr(C)] pub struct file { pub f_mapping: *mut address_space }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct xad_t { _private: [u8; 0] }

extern "C" {
    static jfs_aops: address_space_operations;
}
#[repr(C)] pub struct address_space_operations { _private: [u8; 0] }

pub unsafe extern "C" fn jfs_get_block(_ip: *mut inode, _lblock: sector_t, _bh_result: *mut buffer_head, _create: c_int) -> c_int { 0 }
pub unsafe extern "C" fn jfs_iget(_sb: *mut super_block, _ino: c_ulong) -> *mut inode { core::ptr::null_mut() }
pub unsafe extern "C" fn jfs_commit_inode(_inode: *mut inode, _wait: c_int) -> c_int { 0 }
pub unsafe extern "C" fn jfs_write_inode(_inode: *mut inode, _wbc: *mut writeback_control) -> c_int { 0 }
pub unsafe extern "C" fn jfs_evict_inode(_inode: *mut inode) {}
pub unsafe extern "C" fn jfs_dirty_inode(_inode: *mut inode, _flags: c_int) {}
pub unsafe extern "C" fn jfs_truncate_nolock(_ip: *mut inode, _length: loff_t) {}
pub unsafe extern "C" fn jfs_truncate(_ip: *mut inode) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
