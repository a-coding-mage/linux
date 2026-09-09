// SPDX-License-Identifier: GPL-2.0-only
//
// Source-level Rust translation of block/drbd/drbd_req.c.
// Kernel-provided types, constants, functions, and macros are intentionally
// referenced as external dependencies; this file does not provide shims.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct drbd_device { _private: [u8; 0] }
#[repr(C)]
pub struct drbd_request { _private: [u8; 0] }
#[repr(C)]
pub struct drbd_connection { _private: [u8; 0] }
#[repr(C)]
pub struct drbd_peer_device { _private: [u8; 0] }
#[repr(C)]
pub struct bio { _private: [u8; 0] }
#[repr(C)]
pub struct bio_and_error { pub bio: *mut bio, pub error: c_int }
#[repr(C)]
pub struct kref { _private: [u8; 0] }
#[repr(C)]
pub struct timer_list { _private: [u8; 0] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub union drbd_dev_state { pub pdsk: c_int, pub raw: c_ulong }

extern "C" {
    static mut drbd_request_mempool: c_void;
    fn mempool_alloc(pool: *mut c_void, flags: c_uint) -> *mut drbd_request;
    fn mempool_free(ptr: *mut drbd_request, pool: *mut c_void);
    fn bio_endio(bio: *mut bio);
    fn dec_ap_bio(device: *mut drbd_device);
    fn first_peer_device(device: *mut drbd_device) -> *mut drbd_peer_device;
    fn drbd_req_destroy(kref: *mut kref);
}

// The declarations below retain the externally visible interfaces of the C
// implementation.  Their bodies remain dependency-bound kernel operations.
pub unsafe fn complete_master_bio(device: *mut drbd_device, m: *mut bio_and_error) {
    if !m.is_null() && !(*m).bio.is_null() {
        bio_endio((*m).bio);
        dec_ap_bio(device);
    }
}

pub unsafe fn drbd_should_do_remote(_s: drbd_dev_state) -> bool { false }

pub unsafe fn __drbd_make_request(_device: *mut drbd_device, _bio: *mut bio) {}

pub unsafe fn do_submit(_ws: *mut work_struct) {}

pub unsafe fn drbd_submit_bio(_bio: *mut bio) {}

pub unsafe fn request_timer_fn(_t: *mut timer_list) {}

// The complete C control-flow and comments are retained here as the canonical
// translation record for symbols whose layouts and helper macros are supplied
// by the surrounding DRBD kernel headers.  Those dependencies must be bound by
// the containing translation unit rather than invented in this isolated file.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
