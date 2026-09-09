/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C trace-event header.  The Linux tracepoint machinery
// and show_gfp_flags() are supplied by other translation units.

use core::ffi::c_void;

// TRACE_SYSTEM percpu

#[repr(C)]
pub struct PercpuAllocPercpuEntry {
    pub call_site: c_ulong,
    pub reserved: bool,
    pub is_atomic: bool,
    pub size: usize,
    pub align: usize,
    pub base_addr: *mut c_void,
    pub off: i32,
    pub ptr: *mut c_void, // void __percpu *
    pub bytes_alloc: usize,
    pub gfp_flags: c_ulong,
}

#[repr(C)]
pub struct PercpuFreePercpuEntry {
    pub base_addr: *mut c_void,
    pub off: i32,
    pub ptr: *mut c_void, // void __percpu *
}

#[repr(C)]
pub struct PercpuAllocPercpuFailEntry {
    pub reserved: bool,
    pub is_atomic: bool,
    pub size: usize,
    pub align: usize,
}

#[repr(C)]
pub struct PercpuCreateChunkEntry {
    pub base_addr: *mut c_void,
}

#[repr(C)]
pub struct PercpuDestroyChunkEntry {
    pub base_addr: *mut c_void,
}

// TRACE_EVENT(percpu_alloc_percpu)
pub unsafe fn percpu_alloc_percpu_fast_assign(
    entry: *mut PercpuAllocPercpuEntry,
    call_site: c_ulong,
    reserved: bool,
    is_atomic: bool,
    size: usize,
    align: usize,
    base_addr: *mut c_void,
    off: i32,
    ptr: *mut c_void,
    bytes_alloc: usize,
    gfp_flags: gfp_t,
) {
    (*entry).call_site = call_site;
    (*entry).reserved = reserved;
    (*entry).is_atomic = is_atomic;
    (*entry).size = size;
    (*entry).align = align;
    (*entry).base_addr = base_addr;
    (*entry).off = off;
    (*entry).ptr = ptr;
    (*entry).bytes_alloc = bytes_alloc;
    (*entry).gfp_flags = gfp_flags as c_ulong;
}

// TP_printk("call_site=%pS reserved=%d is_atomic=%d size=%zu align=%zu base_addr=%p off=%d ptr=%p bytes_alloc=%zu gfp_flags=%s",
//           (void *)__entry->call_site, __entry->reserved, __entry->is_atomic,
//           __entry->size, __entry->align, __entry->base_addr, __entry->off,
//           __entry->ptr, __entry->bytes_alloc, show_gfp_flags(__entry->gfp_flags))

// TRACE_EVENT(percpu_free_percpu)
pub unsafe fn percpu_free_percpu_fast_assign(
    entry: *mut PercpuFreePercpuEntry,
    base_addr: *mut c_void,
    off: i32,
    ptr: *mut c_void,
) {
    (*entry).base_addr = base_addr;
    (*entry).off = off;
    (*entry).ptr = ptr;
}

// TP_printk("base_addr=%p off=%d ptr=%p", __entry->base_addr,
//           __entry->off, __entry->ptr)

// TRACE_EVENT(percpu_alloc_percpu_fail)
pub unsafe fn percpu_alloc_percpu_fail_fast_assign(
    entry: *mut PercpuAllocPercpuFailEntry,
    reserved: bool,
    is_atomic: bool,
    size: usize,
    align: usize,
) {
    (*entry).reserved = reserved;
    (*entry).is_atomic = is_atomic;
    (*entry).size = size;
    (*entry).align = align;
}

// TP_printk("reserved=%d is_atomic=%d size=%zu align=%zu", __entry->reserved,
//           __entry->is_atomic, __entry->size, __entry->align)

// TRACE_EVENT(percpu_create_chunk)
pub unsafe fn percpu_create_chunk_fast_assign(
    entry: *mut PercpuCreateChunkEntry,
    base_addr: *mut c_void,
) {
    (*entry).base_addr = base_addr;
}

// TP_printk("base_addr=%p", __entry->base_addr)

// TRACE_EVENT(percpu_destroy_chunk)
pub unsafe fn percpu_destroy_chunk_fast_assign(
    entry: *mut PercpuDestroyChunkEntry,
    base_addr: *mut c_void,
) {
    (*entry).base_addr = base_addr;
}

// TP_printk("base_addr=%p", __entry->base_addr)


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
