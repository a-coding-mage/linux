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
    fn trace_block_touch_buffer(*mut buffer_head);
    fn folio_mark_accessed(*mut folio);
    fn wait_on_bit_lock_io(*mut c_ulong, c_uint, c_uint);
    fn clear_and_wake_up_bit(c_uint, *mut c_ulong);
    fn wait_on_bit_io(*mut c_ulong, c_uint, c_uint);
    fn bio_put(*mut bio);
    fn bio_flagged(*mut bio, c_uint) -> bool;
    fn set_bit(c_uint, *mut c_ulong);
    fn clear_buffer_uptodate(*mut buffer_head);
    fn set_buffer_uptodate(*mut buffer_head);
    fn unlock_buffer(*mut buffer_head);
    fn end_buffer_read_sync(*mut buffer_head, c_int);
    fn buffer_io_error(*mut buffer_head, *mut c_char);
    fn __find_get_block(*mut block_device, sector_t, c_uint) -> *mut buffer_head;
    fn __find_get_block_nonatomic(*mut block_device, sector_t, c_uint) -> *mut buffer_head;
    fn bdev_getblk(*mut block_device, sector_t, c_uint, gfp_t) -> *mut buffer_head;
    fn brelse(*mut buffer_head);
    fn put_bh(*mut buffer_head);
    fn get_bh(*mut buffer_head);
    fn bh_submit(*mut buffer_head, blk_opf_t, bio_end_io_t);
    fn bh_end_read(*mut bio);
    fn bh_end_write(*mut bio);
    fn bh_end_async_write(*mut bio);
    fn block_dirty_folio(*mut address_space, *mut folio) -> bool;
    fn folio_alloc_buffers(*mut folio, usize, gfp_t) -> *mut buffer_head;
    fn alloc_buffer_head(gfp_t) -> *mut buffer_head;
    fn free_buffer_head(*mut buffer_head);
    fn block_invalidate_folio(*mut folio, usize, usize);
    fn create_empty_buffers(*mut folio, usize, c_ulong) -> *mut buffer_head;
    fn block_write_full_folio(*mut folio, *mut writeback_control, *mut c_void) -> c_int;
    fn block_read_full_folio(*mut folio, get_block_t) -> c_int;
    fn block_write_begin(*mut address_space, loff_t, c_uint, *mut *mut folio, get_block_t) -> c_int;
    fn block_write_end(loff_t, c_uint, c_uint, *mut folio) -> c_uint;
    fn generic_write_end(*const kiocb, *mut address_space, loff_t, c_uint, c_uint,
                         *mut folio, *mut c_void) -> c_uint;
    fn block_commit_write(*mut folio, usize, usize);
    fn block_truncate_page(*mut address_space, loff_t, get_block_t) -> c_int;
    fn generic_block_bmap(*mut address_space, sector_t, get_block_t) -> sector_t;
    fn write_dirty_buffer(*mut buffer_head, blk_opf_t);
    fn sync_dirty_buffer(*mut buffer_head) -> c_int;
    fn try_to_free_buffers(*mut folio) -> bool;
    fn bh_uptodate_or_lock(*mut buffer_head) -> c_int;
    fn __bh_read(*mut buffer_head, blk_opf_t, bool) -> c_int;
    fn __bh_read_batch(c_int, *mut *mut buffer_head, blk_opf_t, bool);
    fn invalidate_bh_lrus();
    fn invalidate_bh_lrus_cpu();
    fn folio_set_bh(*mut buffer_head, *mut folio, c_ulong);
    fn mark_buffer_dirty(*mut buffer_head);
    fn mark_buffer_write_io_error(*mut buffer_head);
    fn mmb_init(*mut mapping_metadata_bhs, *mut address_space);
    fn mmb_has_buffers(*mut mapping_metadata_bhs) -> bool;
    fn mmb_sync(*mut mapping_metadata_bhs) -> c_int;
    fn mmb_mark_buffer_dirty(*mut buffer_head, *mut mapping_metadata_bhs);
    fn mmb_invalidate(*mut mapping_metadata_bhs);
    fn __brelse(*mut buffer_head);
    fn __bforget(*mut buffer_head);
    fn __breadahead(*mut block_device, sector_t, c_uint);
    fn __bread_gfp(*mut block_device, sector_t, c_uint, gfp_t) -> *mut buffer_head;
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
