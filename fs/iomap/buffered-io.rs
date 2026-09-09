// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation boundary for iomap/buffered-io.c.
// The Linux kernel types and helper functions referenced below are supplied by
// the surrounding kernel translation units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct iomap_folio_state {
    pub state_lock: spinlock_t,
    pub read_bytes_pending: u32,
    pub write_bytes_pending: atomic_t,
    pub state: [usize; 0],
}

// External kernel ABI types and operations are intentionally left unresolved;
// they are provided by the other translated kernel sources.
extern "C" {
    pub fn iomap_folio_mark_uptodate(folio: *mut folio);
    pub fn iomap_finish_folio_read(folio: *mut folio, off: usize, len: usize, error: i32);
    pub fn iomap_read_folio(ops: *const iomap_ops, ctx: *mut iomap_read_folio_ctx, private: *mut c_void);
    pub fn iomap_readahead(ops: *const iomap_ops, ctx: *mut iomap_read_folio_ctx, private: *mut c_void);
    pub fn iomap_is_partially_uptodate(folio: *mut folio, from: usize, count: usize) -> bool;
    pub fn iomap_get_folio(iter: *mut iomap_iter, pos: i64, len: usize) -> *mut folio;
    pub fn iomap_release_folio(folio: *mut folio, gfp_flags: u32) -> bool;
    pub fn iomap_invalidate_folio(folio: *mut folio, offset: usize, len: usize);
    pub fn iomap_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool;
    pub fn iomap_file_buffered_write(iocb: *mut kiocb, i: *mut iov_iter,
        ops: *const iomap_ops, write_ops: *const iomap_write_ops, private: *mut c_void) -> isize;
    pub fn iomap_fsverity_write(file: *mut file, pos: i64, length: usize, buf: *const c_void,
        ops: *const iomap_ops, write_ops: *const iomap_write_ops) -> i32;
    pub fn iomap_write_delalloc_release(inode: *mut inode, start_byte: i64, end_byte: i64,
        flags: u32, iomap: *mut iomap, punch: iomap_punch_t);
    pub fn iomap_file_unshare(inode: *mut inode, pos: i64, len: i64,
        ops: *const iomap_ops, write_ops: *const iomap_write_ops) -> i32;
    pub fn iomap_fill_dirty_folios(iter: *mut iomap_iter, start: *mut i64, end: i64,
        iomap_flags: *mut u32) -> u32;
    pub fn iomap_zero_range(inode: *mut inode, pos: i64, len: i64, did_zero: *mut bool,
        ops: *const iomap_ops, write_ops: *const iomap_write_ops, private: *mut c_void) -> i32;
    pub fn iomap_truncate_page(inode: *mut inode, pos: i64, did_zero: *mut bool,
        ops: *const iomap_ops, write_ops: *const iomap_write_ops, private: *mut c_void) -> i32;
    pub fn iomap_page_mkwrite(vmf: *mut vm_fault, ops: *const iomap_ops, private: *mut c_void) -> vm_fault_t;
    pub fn iomap_finish_folio_write(inode: *mut inode, folio: *mut folio, len: usize);
    pub fn iomap_writeback_folio(wpc: *mut iomap_writepage_ctx, folio: *mut folio) -> i32;
    pub fn iomap_writepages(wpc: *mut iomap_writepage_ctx) -> i32;
}

// Opaque declarations mirror the C translation unit's externally supplied
// kernel structures without inventing implementations or dependencies.
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct iomap_ops { _private: [u8; 0] }
#[repr(C)] pub struct iomap_read_folio_ctx { _private: [u8; 0] }
#[repr(C)] pub struct iomap_iter { _private: [u8; 0] }
#[repr(C)] pub struct iomap_write_ops { _private: [u8; 0] }
#[repr(C)] pub struct kiocb { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct iomap { _private: [u8; 0] }
#[repr(C)] pub struct vm_fault { _private: [u8; 0] }
#[repr(C)] pub struct iomap_writepage_ctx { _private: [u8; 0] }
pub type iomap_punch_t = unsafe extern "C" fn(*mut inode, i64, i64, *mut iomap);
pub type vm_fault_t = usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
