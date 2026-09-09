/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation unit. */

use core::ffi::{c_ulong, c_void};

#[repr(C)]
pub struct trace_buffer;
#[repr(C)]
pub struct ring_buffer_iter;
#[repr(C)]
pub struct lock_class_key;
#[repr(C)]
pub struct file;
#[repr(C)]
pub struct poll_table;
#[repr(C)]
pub struct hlist_node;
#[repr(C)]
pub struct vm_area_struct;
#[repr(C)]
pub struct trace_seq;
#[repr(C)]
pub struct buffer_data_read_page;

pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type gfp_t = core::ffi::c_uint;
pub type __poll_t = core::ffi::c_uint;

#[repr(C)]
pub struct ring_buffer_event {
    /* C bitfields: type_len:5, time_delta:27. */
    pub type_len: u32,
    pub time_delta: u32,
    pub array: [u32; 0],
}

#[repr(C)]
pub enum ring_buffer_type {
    RINGBUF_TYPE_DATA_TYPE_LEN_MAX = 28,
    RINGBUF_TYPE_PADDING,
    RINGBUF_TYPE_TIME_EXTEND,
    RINGBUF_TYPE_TIME_STAMP,
}

extern "C" {
    pub fn ring_buffer_event_length(event: *mut ring_buffer_event) -> u32;
    pub fn ring_buffer_event_data(event: *mut ring_buffer_event) -> *mut c_void;
    pub fn ring_buffer_event_time_stamp(buffer: *mut trace_buffer, event: *mut ring_buffer_event) -> u64;
    pub fn ring_buffer_discard_commit(buffer: *mut trace_buffer, event: *mut ring_buffer_event);
    pub fn __ring_buffer_alloc(size: c_ulong, flags: u32, key: *mut lock_class_key) -> *mut trace_buffer;
    pub fn __ring_buffer_alloc_range(size: c_ulong, flags: u32, order: i32, start: c_ulong,
                                     range_size: c_ulong, scratch_size: c_ulong,
                                     key: *mut lock_class_key) -> *mut trace_buffer;
    pub fn ring_buffer_meta_scratch(buffer: *mut trace_buffer, size: *mut u32) -> *mut c_void;
    pub fn ring_buffer_wait(buffer: *mut trace_buffer, cpu: i32, full: i32,
                            cond: ring_buffer_cond_fn, data: *mut c_void) -> i32;
    pub fn ring_buffer_poll_wait(buffer: *mut trace_buffer, cpu: i32, filp: *mut file,
                                 poll_table: *mut poll_table, full: i32) -> __poll_t;
    pub fn ring_buffer_wake_waiters(buffer: *mut trace_buffer, cpu: i32);
    pub fn ring_buffer_free(buffer: *mut trace_buffer);
    pub fn ring_buffer_resize(buffer: *mut trace_buffer, size: c_ulong, cpu: i32) -> i32;
    pub fn ring_buffer_change_overwrite(buffer: *mut trace_buffer, val: i32);
    pub fn ring_buffer_lock_reserve(buffer: *mut trace_buffer, length: c_ulong) -> *mut ring_buffer_event;
    pub fn ring_buffer_unlock_commit(buffer: *mut trace_buffer) -> i32;
    pub fn ring_buffer_write(buffer: *mut trace_buffer, length: c_ulong, data: *mut c_void) -> i32;
    pub fn ring_buffer_nest_start(buffer: *mut trace_buffer);
    pub fn ring_buffer_nest_end(buffer: *mut trace_buffer);
    pub fn ring_buffer_peek(buffer: *mut trace_buffer, cpu: i32, ts: *mut u64, lost_events: *mut c_ulong) -> *mut ring_buffer_event;
    pub fn ring_buffer_consume(buffer: *mut trace_buffer, cpu: i32, ts: *mut u64, lost_events: *mut c_ulong) -> *mut ring_buffer_event;
    pub fn ring_buffer_read_start(buffer: *mut trace_buffer, cpu: i32, flags: gfp_t) -> *mut ring_buffer_iter;
    pub fn ring_buffer_read_finish(iter: *mut ring_buffer_iter);
    pub fn ring_buffer_iter_peek(iter: *mut ring_buffer_iter, ts: *mut u64) -> *mut ring_buffer_event;
    pub fn ring_buffer_iter_advance(iter: *mut ring_buffer_iter);
    pub fn ring_buffer_iter_reset(iter: *mut ring_buffer_iter);
    pub fn ring_buffer_iter_empty(iter: *mut ring_buffer_iter) -> i32;
    pub fn ring_buffer_iter_dropped(iter: *mut ring_buffer_iter) -> bool;
    pub fn ring_buffer_size(buffer: *mut trace_buffer, cpu: i32) -> c_ulong;
    pub fn ring_buffer_max_event_size(buffer: *mut trace_buffer) -> c_ulong;
    pub fn ring_buffer_reset_cpu(buffer: *mut trace_buffer, cpu: i32);
    pub fn ring_buffer_reset_online_cpus(buffer: *mut trace_buffer);
    pub fn ring_buffer_reset(buffer: *mut trace_buffer);
    pub fn ring_buffer_empty(buffer: *mut trace_buffer) -> bool;
    pub fn ring_buffer_empty_cpu(buffer: *mut trace_buffer, cpu: i32) -> bool;
    pub fn ring_buffer_record_disable(buffer: *mut trace_buffer);
    pub fn ring_buffer_record_enable(buffer: *mut trace_buffer);
    pub fn ring_buffer_record_off(buffer: *mut trace_buffer);
    pub fn ring_buffer_record_on(buffer: *mut trace_buffer);
    pub fn ring_buffer_record_is_on(buffer: *mut trace_buffer) -> bool;
    pub fn ring_buffer_record_is_set_on(buffer: *mut trace_buffer) -> bool;
    pub fn ring_buffer_record_is_on_cpu(buffer: *mut trace_buffer, cpu: i32) -> bool;
    pub fn ring_buffer_record_disable_cpu(buffer: *mut trace_buffer, cpu: i32);
    pub fn ring_buffer_record_enable_cpu(buffer: *mut trace_buffer, cpu: i32);
    pub fn ring_buffer_oldest_event_ts(buffer: *mut trace_buffer, cpu: i32) -> u64;
    pub fn ring_buffer_bytes_cpu(buffer: *mut trace_buffer, cpu: i32) -> c_ulong;
    pub fn ring_buffer_entries(buffer: *mut trace_buffer) -> c_ulong;
    pub fn ring_buffer_overruns(buffer: *mut trace_buffer) -> c_ulong;
    pub fn ring_buffer_entries_cpu(buffer: *mut trace_buffer, cpu: i32) -> c_ulong;
    pub fn ring_buffer_overrun_cpu(buffer: *mut trace_buffer, cpu: i32) -> c_ulong;
    pub fn ring_buffer_commit_overrun_cpu(buffer: *mut trace_buffer, cpu: i32) -> c_ulong;
    pub fn ring_buffer_dropped_events_cpu(buffer: *mut trace_buffer, cpu: i32) -> c_ulong;
    pub fn ring_buffer_read_events_cpu(buffer: *mut trace_buffer, cpu: i32) -> c_ulong;
    pub fn ring_buffer_time_stamp(buffer: *mut trace_buffer) -> u64;
    pub fn ring_buffer_normalize_time_stamp(buffer: *mut trace_buffer, cpu: i32, ts: *mut u64);
    pub fn ring_buffer_set_clock(buffer: *mut trace_buffer, clock: Option<unsafe extern "C" fn() -> u64>);
    pub fn ring_buffer_set_time_stamp_abs(buffer: *mut trace_buffer, abs: bool);
    pub fn ring_buffer_time_stamp_abs(buffer: *mut trace_buffer) -> bool;
    pub fn ring_buffer_nr_dirty_pages(buffer: *mut trace_buffer, cpu: i32) -> usize;
    pub fn ring_buffer_alloc_read_page(buffer: *mut trace_buffer, cpu: i32) -> *mut buffer_data_read_page;
    pub fn ring_buffer_free_read_page(buffer: *mut trace_buffer, cpu: i32, page: *mut buffer_data_read_page);
    pub fn ring_buffer_read_page(buffer: *mut trace_buffer, data_page: *mut buffer_data_read_page,
                                 len: usize, cpu: i32, full: i32) -> i32;
    pub fn ring_buffer_read_page_data(page: *mut buffer_data_read_page) -> *mut c_void;
    pub fn ring_buffer_print_entry_header(s: *mut trace_seq) -> i32;
    pub fn ring_buffer_print_page_header(buffer: *mut trace_buffer, s: *mut trace_seq) -> i32;
    pub fn ring_buffer_subbuf_order_get(buffer: *mut trace_buffer) -> i32;
    pub fn ring_buffer_subbuf_order_set(buffer: *mut trace_buffer, order: i32) -> i32;
    pub fn ring_buffer_subbuf_size_get(buffer: *mut trace_buffer) -> i32;
    pub fn ring_buffer_map(buffer: *mut trace_buffer, cpu: i32, vma: *mut vm_area_struct) -> i32;
    pub fn ring_buffer_map_dup(buffer: *mut trace_buffer, cpu: i32);
    pub fn ring_buffer_unmap(buffer: *mut trace_buffer, cpu: i32) -> i32;
    pub fn ring_buffer_map_get_reader(buffer: *mut trace_buffer, cpu: i32) -> i32;
}

pub type ring_buffer_cond_fn = Option<unsafe extern "C" fn(data: *mut c_void) -> bool>;
pub const RING_BUFFER_ALL_CPUS: i32 = -1;

#[cfg(feature = "CONFIG_RING_BUFFER_ALLOW_SWAP")]
extern "C" { pub fn ring_buffer_swap_cpu(buffer_a: *mut trace_buffer, buffer_b: *mut trace_buffer, cpu: i32) -> i32; }
#[cfg(not(feature = "CONFIG_RING_BUFFER_ALLOW_SWAP"))]
pub unsafe fn ring_buffer_swap_cpu(_: *mut trace_buffer, _: *mut trace_buffer, _: i32) -> i32 { -19 }

#[repr(C)]
pub enum ring_buffer_flags { RB_FL_OVERWRITE = 1 << 0, RB_FL_TESTING = 1 << 1 }

#[repr(C)]
pub struct ring_buffer_desc {
    pub cpu: i32,
    pub nr_page_va: u32,
    pub meta_va: c_ulong,
    pub page_va: [c_ulong; 0],
}
#[repr(C)]
pub struct trace_buffer_desc { pub nr_cpus: i32, pub struct_len: usize, pub __data: [u8; 0] }

#[repr(C)]
pub struct ring_buffer_remote {
    pub desc: *mut trace_buffer_desc,
    pub swap_reader_page: Option<unsafe extern "C" fn(cpu: u32, priv_: *mut c_void) -> i32>,
    pub reset: Option<unsafe extern "C" fn(cpu: u32, priv_: *mut c_void) -> i32>,
    pub priv_: *mut c_void,
}
extern "C" {
    pub fn ring_buffer_poll_remote(buffer: *mut trace_buffer, cpu: i32) -> i32;
    pub fn __ring_buffer_alloc_remote(remote: *mut ring_buffer_remote, key: *mut lock_class_key) -> *mut trace_buffer;
}

/* ring_buffer_alloc, ring_buffer_alloc_range, ring_buffer_alloc_remote, DEFINE_GUARD,
 * CONFIG_RING_BUFFER, and for_each_ring_buffer_desc are preprocessor/build-time
 * constructs retained as declarations/comments because their dependencies are external. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
