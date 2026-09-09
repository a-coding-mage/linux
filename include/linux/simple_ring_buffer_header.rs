/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux ring-buffer headers.
use core::ffi::c_void;

#[repr(C)]
pub struct simple_buffer_page {
    pub link: list_head,
    pub page: *mut buffer_data_page,
    pub entries: u64,
    pub write: u32,
    pub id: u32,
}

pub const SIMPLE_RB_UNAVAILABLE: u32 = 0;
pub const SIMPLE_RB_READY: u32 = 1;
pub const SIMPLE_RB_WRITING: u32 = 2;

#[repr(C)]
pub struct simple_rb_per_cpu {
    pub tail_page: *mut simple_buffer_page,
    pub reader_page: *mut simple_buffer_page,
    pub head_page: *mut simple_buffer_page,
    pub bpages: *mut simple_buffer_page,
    pub meta: *mut trace_buffer_meta,
    pub nr_pages: u32,
    pub status: u32,
    pub last_overrun: u64,
    pub write_stamp: u64,
    pub cbs: *mut simple_rb_cbs,
}

extern "C" {
    pub fn simple_ring_buffer_init(
        cpu_buffer: *mut simple_rb_per_cpu,
        bpages: *mut simple_buffer_page,
        desc: *const ring_buffer_desc,
    ) -> i32;

    pub fn simple_ring_buffer_unload(cpu_buffer: *mut simple_rb_per_cpu);

    pub fn simple_ring_buffer_reserve(
        cpu_buffer: *mut simple_rb_per_cpu,
        length: usize,
        timestamp: u64,
    ) -> *mut c_void;

    pub fn simple_ring_buffer_commit(cpu_buffer: *mut simple_rb_per_cpu);

    pub fn simple_ring_buffer_enable_tracing(
        cpu_buffer: *mut simple_rb_per_cpu,
        enable: bool,
    ) -> i32;

    pub fn simple_ring_buffer_reset(cpu_buffer: *mut simple_rb_per_cpu) -> i32;

    pub fn simple_ring_buffer_swap_reader_page(cpu_buffer: *mut simple_rb_per_cpu) -> i32;

    pub fn simple_ring_buffer_init_mm(
        cpu_buffer: *mut simple_rb_per_cpu,
        bpages: *mut simple_buffer_page,
        desc: *const ring_buffer_desc,
        load_page: Option<unsafe extern "C" fn(va: usize) -> *mut c_void>,
        unload_page: Option<unsafe extern "C" fn(va: *mut c_void)>,
    ) -> i32;

    pub fn simple_ring_buffer_unload_mm(
        cpu_buffer: *mut simple_rb_per_cpu,
        unload_page: Option<unsafe extern "C" fn(va: *mut c_void)>,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
