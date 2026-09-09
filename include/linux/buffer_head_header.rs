/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/buffer_head.h. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bh_state_bits {
    BH_Uptodate,
    BH_Dirty,
    BH_Lock,
    BH_Req,
    BH_Mapped,
    BH_New,
    BH_Async_Read,
    BH_Async_Write,
    BH_Delay,
    BH_Boundary,
    BH_Write_EIO,
    BH_Unwritten,
    BH_Quiet,
    BH_Meta,
    BH_Prio,
    BH_Defer_Completion,
    BH_Migrate,
    BH_PrivateStart,
}

pub const MAX_BUF_PER_PAGE: usize = PAGE_SIZE / 512;

#[repr(C)]
pub union buffer_head_page_or_folio {
    pub b_page: *mut page,
    pub b_folio: *mut folio,
}

#[repr(C)]
pub struct buffer_head {
    pub b_state: ::core::ffi::c_ulong,
    pub b_this_page: *mut buffer_head,
    pub page_or_folio: buffer_head_page_or_folio,
    pub b_blocknr: sector_t,
    pub b_size: usize,
    pub b_data: *mut ::core::ffi::c_char,
    pub b_bdev: *mut block_device,
    pub b_private: *mut ::core::ffi::c_void,
    pub b_assoc_buffers: list_head,
    pub b_mmb: *mut mapping_metadata_bhs,
    pub b_count: atomic_t,
    pub b_uptodate_lock: spinlock_t,
}

#[inline(always)]
pub unsafe fn set_buffer_dirty(bh: *mut buffer_head) { if test_bit(BH_Dirty as _, &(*bh).b_state) == 0 { set_bit(BH_Dirty as _, &mut (*bh).b_state); } }
#[inline(always)] pub unsafe fn clear_buffer_dirty(bh: *mut buffer_head) { clear_bit(BH_Dirty as _, &mut (*bh).b_state); }
#[inline(always)] pub unsafe fn buffer_dirty(bh: *const buffer_head) -> i32 { test_bit(BH_Dirty as _, &(*bh).b_state) }
#[inline(always)] pub unsafe fn test_set_buffer_dirty(bh: *mut buffer_head) -> i32 { test_and_set_bit(BH_Dirty as _, &mut (*bh).b_state) }
#[inline(always)] pub unsafe fn test_clear_buffer_dirty(bh: *mut buffer_head) -> i32 { test_and_clear_bit(BH_Dirty as _, &mut (*bh).b_state) }

macro_rules! define_buffer_helpers {
    ($bit:ident, $set:ident, $clear:ident, $test:ident) => {
        #[inline(always)] pub unsafe fn $set(bh: *mut buffer_head) { if test_bit($bit as _, &(*bh).b_state) == 0 { set_bit($bit as _, &mut (*bh).b_state); } }
        #[inline(always)] pub unsafe fn $clear(bh: *mut buffer_head) { clear_bit($bit as _, &mut (*bh).b_state); }
        #[inline(always)] pub unsafe fn $test(bh: *const buffer_head) -> i32 { test_bit($bit as _, &(*bh).b_state) }
    };
}

define_buffer_helpers!(BH_Lock, set_buffer_locked, clear_buffer_locked, buffer_locked);
define_buffer_helpers!(BH_Req, set_buffer_req, clear_buffer_req, buffer_req);
define_buffer_helpers!(BH_Mapped, set_buffer_mapped, clear_buffer_mapped, buffer_mapped);
define_buffer_helpers!(BH_New, set_buffer_new, clear_buffer_new, buffer_new);
define_buffer_helpers!(BH_Async_Read, set_buffer_async_read, clear_buffer_async_read, buffer_async_read);
define_buffer_helpers!(BH_Async_Write, set_buffer_async_write, clear_buffer_async_write, buffer_async_write);
define_buffer_helpers!(BH_Delay, set_buffer_delay, clear_buffer_delay, buffer_delay);
define_buffer_helpers!(BH_Boundary, set_buffer_boundary, clear_buffer_boundary, buffer_boundary);
define_buffer_helpers!(BH_Write_EIO, set_buffer_write_io_error, clear_buffer_write_io_error, buffer_write_io_error);
define_buffer_helpers!(BH_Unwritten, set_buffer_unwritten, clear_buffer_unwritten, buffer_unwritten);
define_buffer_helpers!(BH_Meta, set_buffer_meta, clear_buffer_meta, buffer_meta);
define_buffer_helpers!(BH_Prio, set_buffer_prio, clear_buffer_prio, buffer_prio);
define_buffer_helpers!(BH_Defer_Completion, set_buffer_defer_completion, clear_buffer_defer_completion, buffer_defer_completion);

#[inline(always)] pub unsafe fn set_buffer_uptodate(bh: *mut buffer_head) { if test_bit(BH_Uptodate as _, &(*bh).b_state) != 0 { return; } smp_mb__before_atomic(); set_bit(BH_Uptodate as _, &mut (*bh).b_state); }
#[inline(always)] pub unsafe fn clear_buffer_uptodate(bh: *mut buffer_head) { clear_bit(BH_Uptodate as _, &mut (*bh).b_state); }
#[inline(always)] pub unsafe fn buffer_uptodate(bh: *const buffer_head) -> i32 { test_bit_acquire(BH_Uptodate as _, &(*bh).b_state) }

#[inline] pub unsafe fn bh_offset(bh: *const buffer_head) -> ::core::ffi::c_ulong { (*bh).b_data as ::core::ffi::c_ulong & (page_size((*bh).page_or_folio.b_page) - 1) }

pub unsafe fn page_buffers(page: *mut page) -> *mut buffer_head { BUG_ON(!PagePrivate(page)); page_private(page) as *mut buffer_head }
pub unsafe fn folio_buffers(folio: *mut folio) -> *mut buffer_head { folio_get_private(folio) as *mut buffer_head }

extern "C" {
    pub fn buffer_check_dirty_writeback(folio: *mut folio, dirty: *mut bool, writeback: *mut bool);
    pub fn mark_buffer_dirty(bh: *mut buffer_head);
    pub fn mark_buffer_write_io_error(bh: *mut buffer_head);
    pub fn touch_buffer(bh: *mut buffer_head);
    pub fn folio_set_bh(bh: *mut buffer_head, folio: *mut folio, offset: ::core::ffi::c_ulong);
    pub fn folio_alloc_buffers(folio: *mut folio, size: ::core::ffi::c_ulong, gfp: gfp_t) -> *mut buffer_head;
    pub fn alloc_page_buffers(page: *mut page, size: ::core::ffi::c_ulong) -> *mut buffer_head;
    pub fn create_empty_buffers(folio: *mut folio, blocksize: ::core::ffi::c_ulong, b_state: ::core::ffi::c_ulong) -> *mut buffer_head;
    pub fn end_buffer_read_sync(bh: *mut buffer_head, uptodate: i32);
    pub fn bio_endio_bh(bio: *mut bio, bhp: *mut *mut buffer_head) -> bool;
    pub fn bh_end_read(bio: *mut bio);
    pub fn bh_end_write(bio: *mut bio);
    pub fn bh_end_async_write(bio: *mut bio);
    pub fn mmb_mark_buffer_dirty(bh: *mut buffer_head, mmb: *mut mapping_metadata_bhs);
    pub fn clean_bdev_aliases(bdev: *mut block_device, block: sector_t, len: sector_t);
    pub fn __wait_on_buffer(bh: *mut buffer_head);
    pub fn bh_waitq_head(bh: *mut buffer_head) -> *mut wait_queue_head_t;
    pub fn __find_get_block(bdev: *mut block_device, block: sector_t, size: ::core::ffi::c_uint) -> *mut buffer_head;
    pub fn __find_get_block_nonatomic(bdev: *mut block_device, block: sector_t, size: ::core::ffi::c_uint) -> *mut buffer_head;
    pub fn bdev_getblk(bdev: *mut block_device, block: sector_t, size: ::core::ffi::c_uint, gfp: gfp_t) -> *mut buffer_head;
    pub fn __brelse(bh: *mut buffer_head);
    pub fn __bforget(bh: *mut buffer_head);
    pub fn __breadahead(bdev: *mut block_device, block: sector_t, size: ::core::ffi::c_uint);
    pub fn __bread_gfp(bdev: *mut block_device, block: sector_t, size: ::core::ffi::c_uint, gfp: gfp_t) -> *mut buffer_head;
    pub fn alloc_buffer_head(gfp: gfp_t) -> *mut buffer_head;
    pub fn free_buffer_head(bh: *mut buffer_head);
    pub fn unlock_buffer(bh: *mut buffer_head);
    pub fn __lock_buffer(bh: *mut buffer_head);
    pub fn sync_dirty_buffer(bh: *mut buffer_head) -> i32;
    pub fn __sync_dirty_buffer(bh: *mut buffer_head, op_flags: blk_opf_t) -> i32;
    pub fn write_dirty_buffer(bh: *mut buffer_head, op_flags: blk_opf_t);
    pub fn bh_submit(bh: *mut buffer_head, op_flags: blk_opf_t, end_io: bio_end_io_t);
}

#[inline] pub unsafe fn clean_bdev_bh_alias(bh: *mut buffer_head) { clean_bdev_aliases((*bh).b_bdev, (*bh).b_blocknr, 1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
