/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rolling buffer of folios
 *
 * Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependency intent from <linux/folio_queue.h> and <linux/uio.h> is preserved
// through the externally supplied types and functions referenced below.

/*
 * Rolling buffer.  Whilst the buffer is live and in use, folios and folio
 * queue segments can be added to one end by one thread and removed from the
 * other end by another thread.  The buffer isn't allowed to be empty; it must
 * always have at least one folio_queue in it so that neither side has to
 * modify both queue pointers.
 *
 * The iterator in the buffer is extended as buffers are inserted.  It can be
 * snapshotted to use a segment of the buffer.
 */
#[repr(C)]
pub struct rolling_buffer {
    pub head: *mut folio_queue, /* Producer's insertion point */
    pub tail: *mut folio_queue, /* Consumer's removal point */
    pub iter: iov_iter,         /* Iterator tracking what's left in the buffer */
    pub next_head_slot: u8,     /* Next slot in ->head */
    pub first_tail_slot: u8,    /* First slot in ->tail */
}

/*
 * Snapshot of a rolling buffer.
 */
#[repr(C)]
pub struct rolling_buffer_snapshot {
    pub curr_folioq: *mut folio_queue, /* Queue segment in which current folio resides */
    pub curr_slot: ::core::ffi::c_uchar, /* Folio currently being read */
    pub curr_order: ::core::ffi::c_uchar, /* Order of folio */
}

/* Marks to store per-folio in the internal folio_queue structs. */
pub const ROLLBUF_MARK_1: u32 = 1u32 << 0;
pub const ROLLBUF_MARK_2: u32 = 1u32 << 1;

unsafe extern "C" {
    pub fn rolling_buffer_init(
        roll: *mut rolling_buffer,
        rreq_id: ::core::ffi::c_uint,
        direction: ::core::ffi::c_uint,
        gfp: gfp_t,
    ) -> ::core::ffi::c_int;
    pub fn rolling_buffer_make_space(
        roll: *mut rolling_buffer,
        gfp: gfp_t,
    ) -> ::core::ffi::c_int;
    pub fn rolling_buffer_load_from_ra(
        roll: *mut rolling_buffer,
        ractl: *mut readahead_control,
        put_batch: *mut folio_batch,
    ) -> isize;
    pub fn rolling_buffer_append(
        roll: *mut rolling_buffer,
        folio: *mut folio,
        flags: ::core::ffi::c_uint,
        gfp: gfp_t,
    ) -> isize;
    pub fn rolling_buffer_delete_spent(roll: *mut rolling_buffer) -> *mut folio_queue;
    pub fn rolling_buffer_clear(roll: *mut rolling_buffer);
    pub fn iov_iter_advance(iter: *mut iov_iter, amount: usize);
}

#[inline]
pub unsafe fn rolling_buffer_advance(roll: *mut rolling_buffer, amount: usize) {
    unsafe {
        iov_iter_advance(&mut (*roll).iter, amount);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
