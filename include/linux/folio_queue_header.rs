/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Queue of folios definitions
 *
 * Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * See Documentation/core-api/folio_queue.rst for a description of the API.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/folio_batch.h, linux/mm.h

/// Segment in a queue of running buffers.
#[repr(C)]
pub struct folio_queue {
    /// Folios in the queue segment.
    pub vec: folio_batch,
    /// Order of each folio.
    pub orders: [u8; FOLIO_BATCH_SIZE],
    /// Next queue segment or NULL.
    pub next: *mut folio_queue,
    /// Previous queue segment or NULL.
    pub prev: *mut folio_queue,
    /// 1-bit mark per folio.
    pub marks: ::core::ffi::c_ulong,
    /// Second 1-bit mark per folio.
    pub marks2: ::core::ffi::c_ulong,
    pub rreq_id: ::core::ffi::c_uint,
    pub debug_id: ::core::ffi::c_uint,
}

// C build-time condition: FOLIO_BATCH_SIZE must not exceed BITS_PER_LONG;
// otherwise `marks` is not big enough.

#[inline]
pub unsafe fn folioq_init(folioq: *mut folio_queue, rreq_id: ::core::ffi::c_uint) {
    folio_batch_init(&mut (*folioq).vec);
    (*folioq).next = core::ptr::null_mut();
    (*folioq).prev = core::ptr::null_mut();
    (*folioq).marks = 0;
    (*folioq).marks2 = 0;
    (*folioq).rreq_id = rreq_id;
    (*folioq).debug_id = 0;
}

#[inline]
pub unsafe fn folioq_nr_slots(_folioq: *const folio_queue) -> ::core::ffi::c_uint {
    FOLIO_BATCH_SIZE
}

#[inline]
pub unsafe fn folioq_count(folioq: *mut folio_queue) -> ::core::ffi::c_uint {
    folio_batch_count(&(*folioq).vec)
}

#[inline]
pub unsafe fn folioq_full(folioq: *mut folio_queue) -> bool {
    folioq_count(folioq) >= folioq_nr_slots(folioq)
}

#[inline]
pub unsafe fn folioq_is_marked(folioq: *const folio_queue, slot: ::core::ffi::c_uint) -> bool {
    test_bit(slot, &(*folioq).marks)
}

#[inline]
pub unsafe fn folioq_mark(folioq: *mut folio_queue, slot: ::core::ffi::c_uint) {
    set_bit(slot, &mut (*folioq).marks);
}

#[inline]
pub unsafe fn folioq_unmark(folioq: *mut folio_queue, slot: ::core::ffi::c_uint) {
    clear_bit(slot, &mut (*folioq).marks);
}

#[inline]
pub unsafe fn folioq_is_marked2(folioq: *const folio_queue, slot: ::core::ffi::c_uint) -> bool {
    test_bit(slot, &(*folioq).marks2)
}

#[inline]
pub unsafe fn folioq_mark2(folioq: *mut folio_queue, slot: ::core::ffi::c_uint) {
    set_bit(slot, &mut (*folioq).marks2);
}

#[inline]
pub unsafe fn folioq_unmark2(folioq: *mut folio_queue, slot: ::core::ffi::c_uint) {
    clear_bit(slot, &mut (*folioq).marks2);
}

#[inline]
pub unsafe fn folioq_append(folioq: *mut folio_queue, folio: *mut folio) -> ::core::ffi::c_uint {
    let slot = (*folioq).vec.nr;
    (*folioq).vec.nr += 1;
    (*folioq).vec.folios[slot as usize] = folio;
    (*folioq).orders[slot as usize] = folio_order(folio);
    slot
}

#[inline]
pub unsafe fn folioq_append_mark(folioq: *mut folio_queue, folio: *mut folio) -> ::core::ffi::c_uint {
    let slot = (*folioq).vec.nr;
    (*folioq).vec.nr += 1;
    (*folioq).vec.folios[slot as usize] = folio;
    (*folioq).orders[slot as usize] = folio_order(folio);
    folioq_mark(folioq, slot);
    slot
}

#[inline]
pub unsafe fn folioq_folio(folioq: *const folio_queue, slot: ::core::ffi::c_uint) -> *mut folio {
    (*folioq).vec.folios[slot as usize]
}

#[inline]
pub unsafe fn folioq_folio_order(folioq: *const folio_queue, slot: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    (*folioq).orders[slot as usize] as ::core::ffi::c_uint
}

#[inline]
pub unsafe fn folioq_folio_size(folioq: *const folio_queue, slot: ::core::ffi::c_uint) -> usize {
    PAGE_SIZE << folioq_folio_order(folioq, slot)
}

#[inline]
pub unsafe fn folioq_clear(folioq: *mut folio_queue, slot: ::core::ffi::c_uint) {
    (*folioq).vec.folios[slot as usize] = core::ptr::null_mut();
    folioq_unmark(folioq, slot);
    folioq_unmark2(folioq, slot);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
