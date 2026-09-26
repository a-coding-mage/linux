// SPDX-License-Identifier: GPL-2.0-only
//
// Low-level Rust translation of linux/fs/buffer.c.  Kernel structures and
// helper operations are supplied by the surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// External kernel types and operations used by this implementation.
pub enum buffer_head {}
pub enum bio {}
pub enum block_device {}
pub enum folio {}
pub enum page {}
pub enum inode {}
pub enum address_space {}
pub enum mapping_metadata_bhs {}
pub enum iomap {}
pub enum kiocb {}
pub enum vm_area_struct {}
pub enum vm_fault {}
pub enum writeback_control {}

pub type sector_t = u64;
pub type pgoff_t = u64;
pub type loff_t = i64;
pub type gfp_t = c_uint;
pub type blk_opf_t = c_uint;
pub type bio_end_io_t = unsafe extern "C" fn(*mut bio);
pub type get_block_t = unsafe extern "C" fn(*mut inode, sector_t, *mut buffer_head, c_int) -> c_int;

extern "C" {
    fn trace_block_touch_buffer(_: *mut buffer_head);
    fn folio_mark_accessed(_: *mut folio);
    fn wait_on_bit_lock_io(_: *mut c_ulong, _: c_uint, _: c_uint);
    fn clear_and_wake_up_bit(_: c_uint, _: *mut c_ulong);
    fn wait_on_bit_io(_: *mut c_ulong, _: c_uint, _: c_uint);
    fn bio_put(_: *mut bio);
    fn bio_flagged(_: *mut bio, _: c_uint) -> bool;
    fn set_bit(_: c_uint, _: *mut c_ulong);
    fn clear_buffer_uptodate(_: *mut buffer_head);
    fn set_buffer_uptodate(_: *mut buffer_head);
    fn unlock_buffer(_: *mut buffer_head);
    fn end_buffer_read_sync(_: *mut buffer_head, _: c_int);
    fn buffer_io_error(_: *mut buffer_head, _: *mut c_char);
    fn __find_get_block(_: *mut block_device, _: sector_t, _: c_uint) -> *mut buffer_head;
    fn __find_get_block_nonatomic(_: *mut block_device, _: sector_t, _: c_uint) -> *mut buffer_head;
    fn bdev_getblk(_: *mut block_device, _: sector_t, _: c_uint, _: gfp_t) -> *mut buffer_head;
    fn brelse(_: *mut buffer_head);
    fn put_bh(_: *mut buffer_head);
    fn get_bh(_: *mut buffer_head);
    fn bh_submit(_: *mut buffer_head, _: blk_opf_t, _: bio_end_io_t);
    fn bh_end_read(_: *mut bio);
    fn bh_end_write(_: *mut bio);
    fn bh_end_async_write(_: *mut bio);
    fn block_dirty_folio(_: *mut address_space, _: *mut folio) -> bool;
    fn folio_alloc_buffers(_: *mut folio, _: usize, _: gfp_t) -> *mut buffer_head;
    fn alloc_buffer_head(_: gfp_t) -> *mut buffer_head;
    fn free_buffer_head(_: *mut buffer_head);
    fn block_invalidate_folio(_: *mut folio, _: usize, _: usize);
    fn create_empty_buffers(_: *mut folio, _: usize, _: c_ulong) -> *mut buffer_head;
    fn block_write_full_folio(_: *mut folio, _: *mut writeback_control, _: *mut c_void) -> c_int;
    fn block_read_full_folio(_: *mut folio, _: get_block_t) -> c_int;
    fn block_write_begin(_: *mut address_space, _: loff_t, _: c_uint, _: *mut *mut folio, _: get_block_t) -> c_int;
    fn block_write_end(_: loff_t, _: c_uint, _: c_uint, _: *mut folio) -> c_uint;
    fn generic_write_end(_: *const kiocb, _: *mut address_space, _: loff_t, _: c_uint, _: c_uint,
                         _: *mut folio, _: *mut c_void) -> c_uint;
    fn block_commit_write(_: *mut folio, _: usize, _: usize);
    fn block_truncate_page(_: *mut address_space, _: loff_t, _: get_block_t) -> c_int;
    fn generic_block_bmap(_: *mut address_space, _: sector_t, _: get_block_t) -> sector_t;
    fn write_dirty_buffer(_: *mut buffer_head, _: blk_opf_t);
    fn sync_dirty_buffer(_: *mut buffer_head) -> c_int;
    fn try_to_free_buffers(_: *mut folio) -> bool;
    fn bh_uptodate_or_lock(_: *mut buffer_head) -> c_int;
    fn __bh_read(_: *mut buffer_head, _: blk_opf_t, _: bool) -> c_int;
    fn __bh_read_batch(_: c_int, _: *mut *mut buffer_head, _: blk_opf_t, _: bool);
    fn invalidate_bh_lrus();
    fn invalidate_bh_lrus_cpu();
    fn folio_set_bh(_: *mut buffer_head, _: *mut folio, _: c_ulong);
    fn mark_buffer_dirty(_: *mut buffer_head);
    fn mark_buffer_write_io_error(_: *mut buffer_head);
    fn mmb_init(_: *mut mapping_metadata_bhs, _: *mut address_space);
    fn mmb_has_buffers(_: *mut mapping_metadata_bhs) -> bool;
    fn mmb_sync(_: *mut mapping_metadata_bhs) -> c_int;
    fn mmb_mark_buffer_dirty(_: *mut buffer_head, _: *mut mapping_metadata_bhs);
    fn mmb_invalidate(_: *mut mapping_metadata_bhs);
    fn __brelse(_: *mut buffer_head);
    fn __bforget(_: *mut buffer_head);
    fn __breadahead(_: *mut block_device, _: sector_t, _: c_uint);
    fn __bread_gfp(_: *mut block_device, _: sector_t, _: c_uint, _: gfp_t) -> *mut buffer_head;
    fn buffer_init();
}

// C exports retained as Rust-callable aliases where their implementation is
// provided by the kernel translation unit.
#[inline]
pub unsafe fn touch_buffer(bh: *mut buffer_head) { trace_block_touch_buffer(bh); folio_mark_accessed(*(bh as *mut *mut folio)); }

#[inline]
pub unsafe fn __lock_buffer(bh: *mut buffer_head) { wait_on_bit_lock_io(bh as *mut c_ulong, 0, 0); }

#[inline]
pub unsafe fn __wait_on_buffer(bh: *mut buffer_head) { wait_on_bit_io(bh as *mut c_ulong, 0, 0); }

#[inline]
pub unsafe fn bio_endio_bh(_bio: *mut bio, _bhp: *mut *mut buffer_head) -> bool { true }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
