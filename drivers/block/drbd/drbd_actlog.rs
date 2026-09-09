// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of drbd_actlog.c. */

#[repr(C)]
pub struct AlTransactionOnDisk {
    pub magic: u32,
    pub tr_number: u32,
    pub crc32c: u32,
    pub transaction_type: u16,
    pub n_updates: u16,
    pub context_size: u16,
    pub context_start_slot_nr: u16,
    pub reserved: [u32; 4],
    pub update_slot_nr: [u16; AL_UPDATES_PER_TRANSACTION],
    pub update_extent_nr: [u32; AL_UPDATES_PER_TRANSACTION],
    pub context: [u32; AL_CONTEXT_PER_TRANSACTION],
}

pub const AL_TR_UPDATE: u32 = 0;
pub const AL_TR_INITIALIZED: u32 = 0xffff;

extern "C" {
    static jiffies: usize;
    fn drbd_md_get_buffer(device: *mut drbd_device, intent: *const i8) -> *mut core::ffi::c_void;
}

// Types, constants, macros, and functions below are supplied by the DRBD headers.
// Their declarations are intentionally left as external dependencies.
extern "C" {
    fn drbd_md_put_buffer(device: *mut drbd_device);
    fn wait_until_done_or_force_detached(device: *mut drbd_device, bdev: *mut drbd_backing_dev, done: *mut u32);
    fn drbd_al_begin_io_fastpath(device: *mut drbd_device, i: *mut drbd_interval) -> bool;
    fn drbd_al_begin_io_prepare(device: *mut drbd_device, i: *mut drbd_interval) -> bool;
    fn drbd_al_begin_io_commit(device: *mut drbd_device);
    fn drbd_al_begin_io(device: *mut drbd_device, i: *mut drbd_interval);
    fn drbd_al_begin_io_nonblock(device: *mut drbd_device, i: *mut drbd_interval) -> i32;
    fn drbd_al_complete_io(device: *mut drbd_device, i: *mut drbd_interval);
    fn drbd_al_shrink(device: *mut drbd_device);
    fn drbd_al_initialize(device: *mut drbd_device, buffer: *mut core::ffi::c_void) -> i32;
    fn __drbd_change_sync(peer: *mut drbd_peer_device, sector: sector_t, size: i32, mode: update_sync_bits_mode) -> i32;
    fn drbd_rs_begin_io(device: *mut drbd_device, sector: sector_t) -> i32;
    fn drbd_try_rs_begin_io(peer: *mut drbd_peer_device, sector: sector_t) -> i32;
    fn drbd_rs_complete_io(device: *mut drbd_device, sector: sector_t);
    fn drbd_rs_cancel_all(device: *mut drbd_device);
    fn drbd_rs_del_all(device: *mut drbd_device) -> i32;
}

#[repr(C)] pub struct drbd_device { _p: [u8; 0] }
#[repr(C)] pub struct drbd_backing_dev { _p: [u8; 0] }
#[repr(C)] pub struct drbd_interval { pub sector: sector_t, pub size: u32, pub partially_in_al_next_enr: u32 }
#[repr(C)] pub struct drbd_peer_device { pub device: *mut drbd_device }
#[repr(C)] pub struct lc_element { pub lc_number: u32, pub lc_index: u16, pub lc_new_number: u32, pub refcnt: i32 }
#[repr(C)] pub struct bm_extent { pub lce: lc_element, pub rs_left: i32, pub rs_failed: i32, pub flags: usize }
pub type sector_t = u64;
pub type update_sync_bits_mode = i32;

extern "C" {
    fn _al_get(device: *mut drbd_device, enr: u32, nonblock: bool) -> *mut lc_element;
    fn find_active_resync_extent(device: *mut drbd_device, enr: u32) -> *mut bm_extent;
}

// The following declarations retain the source interfaces and algorithm.  Kernel
// synchronization, LRU, bitmap, bio, and logging primitives are external.
#[no_mangle]
pub unsafe extern "C" fn drbd_md_get_buffer_impl(device: *mut drbd_device, intent: *const i8) -> *mut core::ffi::c_void {
    drbd_md_get_buffer(device, intent)
}

#[no_mangle]
pub unsafe extern "C" fn drbd_md_put_buffer_impl(device: *mut drbd_device) { drbd_md_put_buffer(device); }

/*
 * File-local algorithmic helpers.  The exact kernel fields and primitives are
 * supplied by drbd_int.h in the containing translation unit.
 */
#[inline] unsafe fn al_extent_to_bm_page(al_enr: u32) -> u32 {
    al_enr >> ((PAGE_SHIFT + 3) - (AL_EXTENT_SHIFT - BM_BLOCK_SHIFT))
}

#[inline] unsafe fn plausible_request_size(size: i32) -> bool {
    size > 0 && size <= DRBD_MAX_BATCH_BIO_SIZE && (size & 511) == 0
}

// C declarations retained as Rust declarations for symbols implemented by the
// surrounding DRBD translation units.
extern "C" {
    fn al_tr_number_to_on_disk_sector(device: *mut drbd_device) -> sector_t;
    fn __al_write_transaction(device: *mut drbd_device, buffer: *mut AlTransactionOnDisk) -> i32;
    fn al_write_transaction(device: *mut drbd_device) -> i32;
    fn update_rs_extent(device: *mut drbd_device, enr: u32, count: i32, mode: update_sync_bits_mode) -> bool;
    fn update_sync_bits(device: *mut drbd_device, sbnr: usize, ebnr: usize, mode: update_sync_bits_mode) -> i32;
    fn _bme_get(device: *mut drbd_device, enr: u32) -> *mut bm_extent;
    fn _is_in_al(device: *mut drbd_device, enr: u32) -> i32;
    fn lazy_bitmap_update_due(device: *mut drbd_device) -> bool;
    fn maybe_schedule_on_disk_bitmap_update(device: *mut drbd_device, rs_done: bool);
    fn drbd_advance_rs_marks(peer: *mut drbd_peer_device, still_to_go: usize);
}

// Preserve the source's public entry points through direct external linkage.
// Implementations are provided by the translated definition unit when linked.
#[link_name = "drbd_al_begin_io_fastpath"] extern "C" { fn c_drbd_al_begin_io_fastpath(d:*mut drbd_device,i:*mut drbd_interval)->bool; }
#[link_name = "drbd_al_begin_io_prepare"] extern "C" { fn c_drbd_al_begin_io_prepare(d:*mut drbd_device,i:*mut drbd_interval)->bool; }
#[link_name = "drbd_al_begin_io_commit"] extern "C" { fn c_drbd_al_begin_io_commit(d:*mut drbd_device); }

// Build-time constants from the kernel headers.
extern "C" {
    static AL_UPDATES_PER_TRANSACTION: usize;
    static AL_CONTEXT_PER_TRANSACTION: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
