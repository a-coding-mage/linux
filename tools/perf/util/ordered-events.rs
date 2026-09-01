// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/ordered-events.c.
// C include dependencies intentionally remain external to this isolated file:
// errno.h, inttypes.h, linux/list.h, linux/compiler.h, linux/string.h,
// ordered-events.h, session.h, asm/bug.h, debug.h, ui/progress.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u64 = u64;
type size_t = usize;

const ETIME: c_int = 62;
const ENOMEM: c_int = 12;
const ULLONG_MAX: u64 = u64::MAX;

const OE_FLUSH__NONE: oe_flush = 0;
const OE_FLUSH__FINAL: oe_flush = 1;
const OE_FLUSH__ROUND: oe_flush = 2;
const OE_FLUSH__HALF: oe_flush = 3;
const OE_FLUSH__TOP: oe_flush = 4;
const OE_FLUSH__TIME: oe_flush = 5;

type oe_flush = c_uint;
type ordered_events__deliver_t =
    Option<unsafe extern "C" fn(*mut ordered_events, *mut ordered_event) -> c_int>;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct ordered_event {
    pub list: list_head,
    pub timestamp: u64,
    pub file_offset: u64,
    pub file_path: *const c_char,
    pub event: *mut perf_event,
}

#[repr(C)]
pub struct ordered_events_buffer {
    pub list: list_head,
    pub event: [ordered_event; 0],
}

#[repr(C)]
pub struct ordered_events {
    pub events: list_head,
    pub cache: list_head,
    pub to_free: list_head,
    pub buffer: *mut ordered_events_buffer,
    pub buffer_idx: c_uint,
    pub max_alloc_size: u64,
    pub cur_alloc_size: u64,
    pub nr_events: c_uint,
    pub nr_unordered_events: c_uint,
    pub last: *mut ordered_event,
    pub max_timestamp: u64,
    pub next_flush: u64,
    pub last_flush: u64,
    pub last_flush_type: oe_flush,
    pub deliver: ordered_events__deliver_t,
    pub data: *mut c_void,
    pub copy_on_queue: bool,
}

#[repr(C)]
pub struct ui_progress {
    _private: [u8; 0],
}

extern "C" {
    static debug_ordered_events: c_int;

    fn eprintf(level: c_int, var: c_int, fmt: *const c_char, ...);
    fn pr_oe_time(timestamp: u64, fmt: *const c_char, ...);
    fn pr_oe_time2(timestamp: u64, fmt: *const c_char, ...);

    fn memdup(src: *const c_void, len: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(ptr: *mut c_void, value: c_int, num: size_t) -> *mut c_void;

    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_move(list: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;

    fn session_done() -> bool;
    fn ui_progress__init(prog: *mut ui_progress, total: c_uint, title: *const c_char);
    fn ui_progress__update(prog: *mut ui_progress, advance: u64);
    fn ui_progress__finish();

    fn WARN_ONCE(condition: bool, fmt: *const c_char, ...) -> bool;
}

unsafe fn container_of_ordered_event(ptr: *mut list_head) -> *mut ordered_event {
    (ptr as *mut u8).sub(core::mem::offset_of!(ordered_event, list)) as *mut ordered_event
}

unsafe fn container_of_ordered_events_buffer(ptr: *mut list_head) -> *mut ordered_events_buffer {
    (ptr as *mut u8).sub(core::mem::offset_of!(ordered_events_buffer, list))
        as *mut ordered_events_buffer
}

unsafe fn ordered_events_buffer_event(buffer: *mut ordered_events_buffer, idx: c_uint) -> *mut ordered_event {
    ((*buffer).event.as_ptr() as *mut ordered_event).add(idx as usize)
}

unsafe fn pr_N(n: c_int, fmt: *const c_char) {
    eprintf(n, debug_ordered_events, fmt);
}

unsafe fn pr(fmt: *const c_char) {
    pr_N(1, fmt);
}

unsafe fn queue_event(oe: *mut ordered_events, new: *mut ordered_event) {
    let mut last = (*oe).last;
    let timestamp = (*new).timestamp;
    let mut p: *mut list_head;

    (*oe).nr_events = (*oe).nr_events.wrapping_add(1);
    (*oe).last = new;

    pr_oe_time2(
        timestamp,
        b"queue_event nr_events %u\n\0".as_ptr() as *const c_char,
        (*oe).nr_events,
    );

    if last.is_null() {
        list_add(&mut (*new).list, &mut (*oe).events);
        (*oe).max_timestamp = timestamp;
        return;
    }

    /*
     * last event might point to some random place in the list as it's
     * the last queued event. We expect that the new event is close to
     * this.
     */
    if (*last).timestamp <= timestamp {
        while (*last).timestamp <= timestamp {
            p = (*last).list.next;
            if p == &mut (*oe).events {
                list_add_tail(&mut (*new).list, &mut (*oe).events);
                (*oe).max_timestamp = timestamp;
                return;
            }
            last = container_of_ordered_event(p);
        }
        list_add_tail(&mut (*new).list, &mut (*last).list);
    } else {
        while (*last).timestamp > timestamp {
            p = (*last).list.prev;
            if p == &mut (*oe).events {
                list_add(&mut (*new).list, &mut (*oe).events);
                return;
            }
            last = container_of_ordered_event(p);
        }
        list_add(&mut (*new).list, &mut (*last).list);
    }
}

unsafe fn __dup_event(oe: *mut ordered_events, event: *mut perf_event) -> *mut perf_event {
    let mut new_event: *mut perf_event = ptr::null_mut();

    if (*oe).cur_alloc_size < (*oe).max_alloc_size {
        new_event = memdup(event as *const c_void, (*event).header.size as size_t) as *mut perf_event;
        if !new_event.is_null() {
            (*oe).cur_alloc_size += (*event).header.size as u64;
        }
    }

    new_event
}

unsafe fn dup_event(oe: *mut ordered_events, event: *mut perf_event) -> *mut perf_event {
    if (*oe).copy_on_queue {
        __dup_event(oe, event)
    } else {
        event
    }
}

unsafe fn __free_dup_event(oe: *mut ordered_events, event: *mut perf_event) {
    if !event.is_null() {
        (*oe).cur_alloc_size -= (*event).header.size as u64;
        free(event as *mut c_void);
    }
}

unsafe fn free_dup_event(oe: *mut ordered_events, event: *mut perf_event) {
    if (*oe).copy_on_queue {
        __free_dup_event(oe, event);
    }
}

const MAX_SAMPLE_BUFFER: usize = 64 * 1024 / size_of::<ordered_event>();

unsafe fn alloc_event(oe: *mut ordered_events, event: *mut perf_event) -> *mut ordered_event {
    let cache = &mut (*oe).cache as *mut list_head;
    let mut new: *mut ordered_event = ptr::null_mut();
    let new_event: *mut perf_event;
    let size: size_t;

    new_event = dup_event(oe, event);
    if new_event.is_null() {
        return ptr::null_mut();
    }

    /*
     * We maintain the following scheme of buffers for ordered
     * event allocation:
     *
     *   to_free list -> buffer1 (64K)
     *                   buffer2 (64K)
     *                   ...
     *
     * Each buffer keeps an array of ordered events objects:
     *    buffer -> event[0]
     *              event[1]
     *              ...
     *
     * Each allocated ordered event is linked to one of
     * following lists:
     *   - time ordered list 'events'
     *   - list of currently removed events 'cache'
     *
     * Allocation of the ordered event uses the following order
     * to get the memory:
     *   - use recently removed object from 'cache' list
     *   - use available object in current allocation buffer
     *   - allocate new buffer if the current buffer is full
     *
     * Removal of ordered event object moves it from events to
     * the cache list.
     */
    size = size_of::<ordered_events_buffer>() + MAX_SAMPLE_BUFFER * size_of::<ordered_event>();

    if list_empty(cache) == 0 {
        new = container_of_ordered_event((*cache).next);
        list_del_init(&mut (*new).list);
    } else if !(*oe).buffer.is_null() {
        new = ordered_events_buffer_event((*oe).buffer, (*oe).buffer_idx);
        (*oe).buffer_idx = (*oe).buffer_idx.wrapping_add(1);
        if (*oe).buffer_idx == MAX_SAMPLE_BUFFER as c_uint {
            (*oe).buffer = ptr::null_mut();
        }
    } else if ((*oe).cur_alloc_size + size as u64) < (*oe).max_alloc_size {
        (*oe).buffer = malloc(size) as *mut ordered_events_buffer;
        if (*oe).buffer.is_null() {
            free_dup_event(oe, new_event);
            return ptr::null_mut();
        }

        eprintf(
            1,
            debug_ordered_events,
            b"alloc size %lluB (+%zu), max %lluB\n\0".as_ptr() as *const c_char,
            (*oe).cur_alloc_size,
            size,
            (*oe).max_alloc_size,
        );

        (*oe).cur_alloc_size += size as u64;
        list_add(&mut (*(*oe).buffer).list, &mut (*oe).to_free);

        (*oe).buffer_idx = 1;
        new = ordered_events_buffer_event((*oe).buffer, 0);
    } else {
        eprintf(
            1,
            debug_ordered_events,
            b"allocation limit reached %lluB\n\0".as_ptr() as *const c_char,
            (*oe).max_alloc_size,
        );
        return ptr::null_mut();
    }

    (*new).event = new_event;
    new
}

unsafe fn ordered_events__new_event(
    oe: *mut ordered_events,
    timestamp: u64,
    event: *mut perf_event,
) -> *mut ordered_event {
    let new = alloc_event(oe, event);
    if !new.is_null() {
        (*new).timestamp = timestamp;
        queue_event(oe, new);
    }

    new
}

#[no_mangle]
pub unsafe extern "C" fn ordered_events__delete(
    oe: *mut ordered_events,
    event: *mut ordered_event,
) {
    list_move(&mut (*event).list, &mut (*oe).cache);
    (*oe).nr_events = (*oe).nr_events.wrapping_sub(1);
    free_dup_event(oe, (*event).event);
    (*event).event = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn ordered_events__queue(
    oe: *mut ordered_events,
    event: *mut perf_event,
    timestamp: u64,
    file_offset: u64,
    file_path: *const c_char,
) -> c_int {
    let mut oevent: *mut ordered_event;

    if timestamp == 0 || timestamp == !0u64 {
        return -ETIME;
    }

    if timestamp < (*oe).last_flush {
        pr_oe_time(timestamp, b"out of order event\n\0".as_ptr() as *const c_char);
        pr_oe_time(
            (*oe).last_flush,
            b"last flush, last_flush_type %d\n\0".as_ptr() as *const c_char,
            (*oe).last_flush_type,
        );

        (*oe).nr_unordered_events = (*oe).nr_unordered_events.wrapping_add(1);
    }

    oevent = ordered_events__new_event(oe, timestamp, event);
    if oevent.is_null() {
        ordered_events__flush(oe, OE_FLUSH__HALF);
        oevent = ordered_events__new_event(oe, timestamp, event);
    }

    if oevent.is_null() {
        return -ENOMEM;
    }

    (*oevent).file_offset = file_offset;
    (*oevent).file_path = file_path;
    0
}

unsafe fn do_flush(oe: *mut ordered_events, show_progress: bool) -> c_int {
    let head = &mut (*oe).events as *mut list_head;
    let mut tmp: *mut ordered_event;
    let mut iter: *mut ordered_event;
    let limit = (*oe).next_flush;
    let last_ts = if !(*oe).last.is_null() {
        (*(*oe).last).timestamp
    } else {
        0
    };
    let mut prog: ui_progress = core::mem::zeroed();
    let mut ret: c_int;

    if limit == 0 {
        return 0;
    }

    if show_progress {
        ui_progress__init(
            &mut prog,
            (*oe).nr_events,
            b"Processing time ordered events...\0".as_ptr() as *const c_char,
        );
    }

    iter = container_of_ordered_event((*head).next);
    while &mut (*iter).list as *mut list_head != head {
        tmp = container_of_ordered_event((*iter).list.next);
        if session_done() {
            return 0;
        }

        if (*iter).timestamp > limit {
            break;
        }
        ret = ((*oe).deliver.expect("ordered_events deliver callback"))(oe, iter);
        if ret < 0 {
            return ret;
        }

        ordered_events__delete(oe, iter);
        (*oe).last_flush = (*iter).timestamp;

        if show_progress {
            ui_progress__update(&mut prog, 1);
        }

        iter = tmp;
    }

    if list_empty(head) != 0 {
        (*oe).last = ptr::null_mut();
    } else if last_ts <= limit {
        (*oe).last = container_of_ordered_event((*head).prev);
    }

    if show_progress {
        ui_progress__finish();
    }

    0
}

unsafe fn __ordered_events__flush(
    oe: *mut ordered_events,
    how: oe_flush,
    timestamp: u64,
) -> c_int {
    static STR: [&[u8]; 6] = [
        b"NONE\0",
        b"FINAL\0",
        b"ROUND\0",
        b"HALF \0",
        b"TOP  \0",
        b"TIME \0",
    ];
    let err: c_int;
    let mut show_progress = false;

    if (*oe).nr_events == 0 {
        return 0;
    }

    match how {
        OE_FLUSH__FINAL => {
            show_progress = true;
            (*oe).next_flush = ULLONG_MAX;
        }
        OE_FLUSH__TOP => {
            (*oe).next_flush = ULLONG_MAX;
        }
        OE_FLUSH__HALF => {
            let first: *mut ordered_event;
            let last: *mut ordered_event;
            let head = &mut (*oe).events as *mut list_head;

            first = container_of_ordered_event((*head).next);
            last = (*oe).last;

            /* Warn if we are called before any event got allocated. */
            if WARN_ONCE(
                last.is_null() || list_empty(head) != 0,
                b"empty queue\0".as_ptr() as *const c_char,
            ) {
                return 0;
            }

            (*oe).next_flush = (*first).timestamp;
            (*oe).next_flush += ((*last).timestamp - (*first).timestamp) / 2;
        }
        OE_FLUSH__TIME => {
            (*oe).next_flush = timestamp;
            show_progress = false;
        }
        OE_FLUSH__ROUND | OE_FLUSH__NONE => {}
        _ => {}
    }

    pr_oe_time(
        (*oe).next_flush,
        b"next_flush - ordered_events__flush PRE  %s, nr_events %u\n\0".as_ptr()
            as *const c_char,
        STR[how as usize].as_ptr() as *const c_char,
        (*oe).nr_events,
    );
    pr_oe_time(
        (*oe).max_timestamp,
        b"max_timestamp\n\0".as_ptr() as *const c_char,
    );

    err = do_flush(oe, show_progress);

    if err == 0 {
        if how == OE_FLUSH__ROUND {
            (*oe).next_flush = (*oe).max_timestamp;
        }

        (*oe).last_flush_type = how;
    }

    pr_oe_time(
        (*oe).next_flush,
        b"next_flush - ordered_events__flush POST %s, nr_events %u\n\0".as_ptr()
            as *const c_char,
        STR[how as usize].as_ptr() as *const c_char,
        (*oe).nr_events,
    );
    pr_oe_time(
        (*oe).last_flush,
        b"last_flush\n\0".as_ptr() as *const c_char,
    );

    err
}

#[no_mangle]
pub unsafe extern "C" fn ordered_events__flush(oe: *mut ordered_events, how: oe_flush) -> c_int {
    __ordered_events__flush(oe, how, 0)
}

#[no_mangle]
pub unsafe extern "C" fn ordered_events__flush_time(
    oe: *mut ordered_events,
    timestamp: u64,
) -> c_int {
    __ordered_events__flush(oe, OE_FLUSH__TIME, timestamp)
}

#[no_mangle]
pub unsafe extern "C" fn ordered_events__first_time(oe: *mut ordered_events) -> u64 {
    let event: *mut ordered_event;

    if list_empty(&mut (*oe).events) != 0 {
        return 0;
    }

    event = container_of_ordered_event((*oe).events.next);
    (*event).timestamp
}

#[no_mangle]
pub unsafe extern "C" fn ordered_events__init(
    oe: *mut ordered_events,
    deliver: ordered_events__deliver_t,
    data: *mut c_void,
) {
    INIT_LIST_HEAD(&mut (*oe).events);
    INIT_LIST_HEAD(&mut (*oe).cache);
    INIT_LIST_HEAD(&mut (*oe).to_free);
    (*oe).max_alloc_size = -1i64 as u64;
    (*oe).cur_alloc_size = 0;
    (*oe).deliver = deliver;
    (*oe).data = data;
}

unsafe fn ordered_events_buffer__free(
    buffer: *mut ordered_events_buffer,
    max: c_uint,
    oe: *mut ordered_events,
) {
    if (*oe).copy_on_queue {
        let mut i: c_uint = 0;

        while i < max {
            __free_dup_event(oe, (*ordered_events_buffer_event(buffer, i)).event);
            i += 1;
        }
    }

    free(buffer as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn ordered_events__free(oe: *mut ordered_events) {
    let mut buffer: *mut ordered_events_buffer;
    let mut tmp: *mut ordered_events_buffer;

    if list_empty(&mut (*oe).to_free) != 0 {
        return;
    }

    /*
     * Current buffer might not have all the events allocated
     * yet, we need to free only allocated ones ...
     */
    if !(*oe).buffer.is_null() {
        list_del_init(&mut (*(*oe).buffer).list);
        ordered_events_buffer__free((*oe).buffer, (*oe).buffer_idx, oe);
    }

    /* ... and continue with the rest */
    buffer = container_of_ordered_events_buffer((*oe).to_free.next);
    while &mut (*buffer).list as *mut list_head != &mut (*oe).to_free {
        tmp = container_of_ordered_events_buffer((*buffer).list.next);
        list_del_init(&mut (*buffer).list);
        ordered_events_buffer__free(buffer, MAX_SAMPLE_BUFFER as c_uint, oe);
        buffer = tmp;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ordered_events__reinit(oe: *mut ordered_events) {
    let old_deliver = (*oe).deliver;

    ordered_events__free(oe);
    memset(
        oe as *mut c_void,
        0,
        size_of::<ordered_events>() as size_t,
    );
    ordered_events__init(oe, old_deliver, (*oe).data);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
