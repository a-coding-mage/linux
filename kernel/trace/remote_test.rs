// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025 - Google LLC
 * Author: Vincent Donnefort <vdonnefort@google.com>
 */

// Kernel headers and generated remote-event declarations are supplied by the
// surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_ulong, c_void};

const REMOTE_TEST_EVENT_ID: u16 = 0; // Supplied by trace/define_remote_events.h.

#[repr(C)]
pub struct simple_rb_per_cpu {
    pub bpages: *mut simple_buffer_page,
}
#[repr(C)] pub struct simple_buffer_page;
#[repr(C)] pub struct ring_buffer_desc { pub cpu: c_int, pub nr_page_va: usize }
#[repr(C)] pub struct trace_buffer_desc;
#[repr(C)] pub struct remote_event_format_selftest { pub hdr: remote_event_header, pub id: c_ulong }
#[repr(C)] pub struct remote_event_header { pub id: u16 }
#[repr(C)] pub struct file;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct mutex;

extern "C" {
    static mut remote_test_buffer_desc: *mut trace_buffer_desc;
    static mut simple_rbs: *mut simple_rb_per_cpu;
    static mut remote_event_selftest: remote_event_format_selftest;
    static cpu_possible_mask: *const c_void;

    fn kmalloc(size: usize, flags: c_ulong) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn simple_ring_buffer_init(cpu: *mut simple_rb_per_cpu, pages: *mut simple_buffer_page, desc: *mut ring_buffer_desc) -> c_int;
    fn simple_ring_buffer_unload(cpu: *mut simple_rb_per_cpu);
    fn simple_ring_buffer_enable_tracing(cpu: *mut simple_rb_per_cpu, enable: bool) -> c_int;
    fn simple_ring_buffer_swap_reader_page(cpu: *mut simple_rb_per_cpu) -> c_int;
    fn simple_ring_buffer_reset(cpu: *mut simple_rb_per_cpu) -> c_int;
    fn simple_ring_buffer_reserve(cpu: *mut simple_rb_per_cpu, size: usize, clock: u64) -> *mut remote_event_format_selftest;
    fn simple_ring_buffer_commit(cpu: *mut simple_rb_per_cpu);
    fn trace_buffer_desc_size(size: usize, cpus: usize) -> usize;
    fn trace_remote_alloc_buffer(desc: *mut trace_buffer_desc, desc_size: usize, size: usize, mask: *const c_void) -> c_int;
    fn trace_remote_free_buffer(desc: *mut trace_buffer_desc);
    fn trace_clock_global() -> u64;
    fn kstrtoul_from_user(ubuf: *const c_char, cnt: usize, base: c_uint, val: *mut c_ulong) -> c_int;
    fn tracefs_create_file(name: *const c_char, mode: u32, d: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
}

type c_uint = u32;

// The trace_remote lock already serializes accesses from the trace_remote_callbacks.
// However write_event can still race with load/unload.
static mut SIMPLE_RBS_LOCK: mutex = mutex;

#[repr(C)] pub struct file_operations { pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, usize, *mut i64) -> isize> }

unsafe extern "C" fn remote_test_load_simple_rb(cpu: c_int, rb_desc: *mut ring_buffer_desc) -> c_int {
    let cpu_buffer = kmalloc(core::mem::size_of::<simple_rb_per_cpu>(), 0) as *mut simple_rb_per_cpu;
    if cpu_buffer.is_null() { return -12; }
    let bpages = kmalloc((*rb_desc).nr_page_va * core::mem::size_of::<simple_buffer_page>(), 0) as *mut simple_buffer_page;
    if bpages.is_null() { kfree(cpu_buffer.cast()); return -12; }
    let ret = simple_ring_buffer_init(cpu_buffer, bpages, rb_desc);
    if ret != 0 { kfree(bpages.cast()); kfree(cpu_buffer.cast()); return ret; }
    // scoped_guard(mutex, &simple_rbs_lock)
    let _ = cpu;
    simple_rbs = cpu_buffer;
    0
}

unsafe extern "C" fn remote_test_unload_simple_rb(_cpu: c_int) {
    let cpu_buffer = simple_rbs;
    if cpu_buffer.is_null() { return; }
    let bpages = (*cpu_buffer).bpages;
    simple_ring_buffer_unload(cpu_buffer);
    kfree(bpages.cast());
    kfree(cpu_buffer.cast());
    simple_rbs = core::ptr::null_mut();
}

unsafe extern "C" fn remote_test_load(size: usize, _unused: *mut c_void) -> *mut trace_buffer_desc {
    let desc_size = trace_buffer_desc_size(size, 0);
    if desc_size == usize::MAX { return (-7isize) as *mut trace_buffer_desc; }
    let desc = kmalloc(desc_size, 0) as *mut trace_buffer_desc;
    if desc.is_null() { return (-12isize) as *mut trace_buffer_desc; }
    let ret = trace_remote_alloc_buffer(desc, desc_size, size, cpu_possible_mask);
    if ret != 0 { kfree(desc.cast()); return ret as isize as *mut trace_buffer_desc; }
    remote_test_buffer_desc = desc;
    desc
}

unsafe extern "C" fn remote_test_unload(desc: *mut trace_buffer_desc, _unused: *mut c_void) {
    remote_test_unload_simple_rb(0);
    remote_test_buffer_desc = core::ptr::null_mut();
    trace_remote_free_buffer(desc);
    kfree(desc.cast());
}

unsafe extern "C" fn remote_test_enable_tracing(enable: bool, _unused: *mut c_void) -> c_int {
    if remote_test_buffer_desc.is_null() { return -19; }
    let _ = simple_ring_buffer_enable_tracing(simple_rbs, enable);
    0
}

unsafe extern "C" fn remote_test_swap_reader_page(_cpu: u32, _unused: *mut c_void) -> c_int { simple_ring_buffer_swap_reader_page(simple_rbs) }
unsafe extern "C" fn remote_test_reset(_cpu: u32, _unused: *mut c_void) -> c_int { simple_ring_buffer_reset(simple_rbs) }

unsafe extern "C" fn remote_test_enable_event(id: u16, _enable: bool, _unused: *mut c_void) -> c_int {
    if id != REMOTE_TEST_EVENT_ID { return -22; }
    // Use the struct remote_event enabled field toggled by trace_remote.
    0
}

unsafe extern "C" fn write_event_write(_filp: *mut file, ubuf: *const c_char, cnt: usize, _pos: *mut i64) -> isize {
    let mut val = 0u64;
    let ret = kstrtoul_from_user(ubuf, cnt, 10, &mut val);
    if ret != 0 { return ret as isize; }
    if !remote_event_selftest.enabled() { return -19; }
    let evt_test = simple_ring_buffer_reserve(simple_rbs, core::mem::size_of::<remote_event_format_selftest>(), trace_clock_global());
    if evt_test.is_null() { return -19; }
    (*evt_test).hdr.id = REMOTE_TEST_EVENT_ID;
    (*evt_test).id = val as c_ulong;
    simple_ring_buffer_commit(simple_rbs);
    cnt as isize
}

impl remote_event_format_selftest { unsafe fn enabled(&self) -> bool { false } }

static WRITE_EVENT_FOPS: file_operations = file_operations { write: Some(write_event_write) };

unsafe extern "C" fn remote_test_init_tracefs(d: *mut dentry, _unused: *mut c_void) -> c_int {
    let name = b"write_event\0";
    if tracefs_create_file(name.as_ptr().cast(), 0o200, d, core::ptr::null_mut(), &WRITE_EVENT_FOPS).is_null() { -12 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
