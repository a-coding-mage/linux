/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/ordered-events.h.
// C dependencies from <linux/types.h> and other headers are expected to provide:
// list_head and perf_event.

#[repr(C)]
pub struct ordered_event {
    pub timestamp: u64,
    pub file_offset: u64,
    pub file_path: *const ::std::os::raw::c_char,
    pub event: *mut perf_event,
    pub list: list_head,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum oe_flush {
    OE_FLUSH__NONE,
    OE_FLUSH__FINAL,
    OE_FLUSH__ROUND,
    OE_FLUSH__HALF,
    OE_FLUSH__TOP,
    OE_FLUSH__TIME,
}

pub type ordered_events__deliver_t = Option<
    unsafe extern "C" fn(
        oe: *mut ordered_events,
        event: *mut ordered_event,
    ) -> ::std::os::raw::c_int,
>;

#[repr(C)]
pub struct ordered_events_buffer {
    pub list: list_head,
    // C flexible array member: struct ordered_event event[];
    pub event: [ordered_event; 0],
}

#[repr(C)]
pub struct ordered_events {
    pub last_flush: u64,
    pub next_flush: u64,
    pub max_timestamp: u64,
    pub max_alloc_size: u64,
    pub cur_alloc_size: u64,
    pub events: list_head,
    pub cache: list_head,
    pub to_free: list_head,
    pub buffer: *mut ordered_events_buffer,
    pub last: *mut ordered_event,
    pub deliver: ordered_events__deliver_t,
    pub buffer_idx: ::std::os::raw::c_int,
    pub nr_events: ::std::os::raw::c_uint,
    pub last_flush_type: oe_flush,
    pub nr_unordered_events: u32,
    pub copy_on_queue: bool,
    pub data: *mut ::std::os::raw::c_void,
}

extern "C" {
    pub fn ordered_events__queue(
        oe: *mut ordered_events,
        event: *mut perf_event,
        timestamp: u64,
        file_offset: u64,
        file_path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn ordered_events__delete(oe: *mut ordered_events, event: *mut ordered_event);

    pub fn ordered_events__flush(
        oe: *mut ordered_events,
        how: oe_flush,
    ) -> ::std::os::raw::c_int;

    pub fn ordered_events__flush_time(
        oe: *mut ordered_events,
        timestamp: u64,
    ) -> ::std::os::raw::c_int;

    pub fn ordered_events__init(
        oe: *mut ordered_events,
        deliver: ordered_events__deliver_t,
        data: *mut ::std::os::raw::c_void,
    );

    pub fn ordered_events__free(oe: *mut ordered_events);
    pub fn ordered_events__reinit(oe: *mut ordered_events);
    pub fn ordered_events__first_time(oe: *mut ordered_events) -> u64;
}

#[inline]
pub unsafe fn ordered_events__set_alloc_size(oe: *mut ordered_events, size: u64) {
    (*oe).max_alloc_size = size;
}

#[inline]
pub unsafe fn ordered_events__set_copy_on_queue(oe: *mut ordered_events, copy: bool) {
    (*oe).copy_on_queue = copy;
}

#[inline]
pub unsafe fn ordered_events__last_flush_time(oe: *mut ordered_events) -> u64 {
    (*oe).last_flush
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
