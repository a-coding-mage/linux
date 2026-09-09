/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM: cma
// The C header uses the Linux tracepoint TRACE_EVENT infrastructure.  The
// declarations below preserve the event entry layouts and printk formats.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct CmaReleaseEntry {
    pub name: *const c_char,
    pub pfn: libc::c_ulong,
    pub page: *const c_void,
    pub count: libc::c_ulong,
}

pub const CMA_RELEASE_PRINTK: &str =
    "name=%s pfn=0x%lx page=%p count=%lu";

#[repr(C)]
pub struct CmaAllocStartEntry {
    pub name: *const c_char,
    pub request_count: libc::c_ulong,
    pub available_count: libc::c_ulong,
    pub total_count: libc::c_ulong,
    pub align: libc::c_uint,
}

pub const CMA_ALLOC_START_PRINTK: &str =
    "name=%s request_count=%lu available_count=%lu total_count=%lu align=%u";

#[repr(C)]
pub struct CmaAllocFinishEntry {
    pub name: *const c_char,
    pub pfn: libc::c_ulong,
    pub page: *const c_void,
    pub count: libc::c_ulong,
    pub align: libc::c_uint,
    pub errorno: libc::c_int,
}

pub const CMA_ALLOC_FINISH_PRINTK: &str =
    "name=%s pfn=0x%lx page=%p count=%lu align=%u errorno=%d";

#[repr(C)]
pub struct CmaAllocBusyRetryEntry {
    pub name: *const c_char,
    pub pfn: libc::c_ulong,
    pub page: *const c_void,
    pub count: libc::c_ulong,
    pub align: libc::c_uint,
}

pub const CMA_ALLOC_BUSY_RETRY_PRINTK: &str =
    "name=%s pfn=0x%lx page=%p count=%lu align=%u";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
