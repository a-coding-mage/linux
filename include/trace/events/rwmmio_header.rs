/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2021-2022 Qualcomm Innovation Center, Inc. All rights reserved.
 */

// TRACE_SYSTEM is defined as `rwmmio` in the C tracepoint header.
// The Linux tracepoint declarations included by the original header are
// supplied externally and are represented here by the corresponding C-layout
// event payloads and trace-format constants.

#[repr(C)]
pub struct RwmmioRwTemplateEntry {
    pub caller: ::core::ffi::c_ulong,
    pub caller0: ::core::ffi::c_ulong,
    pub addr: ::core::ffi::c_ulong,
    pub val: u64,
    pub width: u8,
}

#[repr(C)]
pub struct RwmmioReadEntry {
    pub caller: ::core::ffi::c_ulong,
    pub caller0: ::core::ffi::c_ulong,
    pub addr: ::core::ffi::c_ulong,
    pub width: u8,
}

#[repr(C)]
pub struct RwmmioPostReadEntry {
    pub caller: ::core::ffi::c_ulong,
    pub caller0: ::core::ffi::c_ulong,
    pub addr: ::core::ffi::c_ulong,
    pub val: u64,
    pub width: u8,
}

pub const RWMMIO_RW_TEMPLATE_PRINTK: &str =
    "%pS -> %pS width=%d val=%#llx addr=%#lx";
pub const RWMMIO_READ_PRINTK: &str = "%pS -> %pS width=%d addr=%#lx";
pub const RWMMIO_POST_READ_PRINTK: &str =
    "%pS -> %pS width=%d val=%#llx addr=%#lx";

pub type RwmmioRwTemplateProto = unsafe extern "C" fn(
    caller: ::core::ffi::c_ulong,
    caller0: ::core::ffi::c_ulong,
    val: u64,
    width: u8,
    addr: *mut ::core::ffi::c_void,
);

pub type RwmmioReadProto = unsafe extern "C" fn(
    caller: ::core::ffi::c_ulong,
    caller0: ::core::ffi::c_ulong,
    width: u8,
    addr: *const ::core::ffi::c_void,
);

pub type RwmmioPostReadProto = unsafe extern "C" fn(
    caller: ::core::ffi::c_ulong,
    caller0: ::core::ffi::c_ulong,
    val: u64,
    width: u8,
    addr: *const ::core::ffi::c_void,
);

#[inline]
pub unsafe fn rwmmio_rw_template_fast_assign(
    entry: *mut RwmmioRwTemplateEntry,
    caller: ::core::ffi::c_ulong,
    caller0: ::core::ffi::c_ulong,
    val: u64,
    width: u8,
    addr: *mut ::core::ffi::c_void,
) {
    (*entry).caller = caller;
    (*entry).caller0 = caller0;
    (*entry).val = val;
    (*entry).addr = addr as ::core::ffi::c_ulong;
    (*entry).width = width;
}

#[inline]
pub unsafe fn rwmmio_read_fast_assign(
    entry: *mut RwmmioReadEntry,
    caller: ::core::ffi::c_ulong,
    caller0: ::core::ffi::c_ulong,
    width: u8,
    addr: *const ::core::ffi::c_void,
) {
    (*entry).caller = caller;
    (*entry).caller0 = caller0;
    (*entry).addr = addr as ::core::ffi::c_ulong;
    (*entry).width = width;
}

#[inline]
pub unsafe fn rwmmio_post_read_fast_assign(
    entry: *mut RwmmioPostReadEntry,
    caller: ::core::ffi::c_ulong,
    caller0: ::core::ffi::c_ulong,
    val: u64,
    width: u8,
    addr: *const ::core::ffi::c_void,
) {
    (*entry).caller = caller;
    (*entry).caller0 = caller0;
    (*entry).val = val;
    (*entry).addr = addr as ::core::ffi::c_ulong;
    (*entry).width = width;
}

// DEFINE_EVENT(rwmmio_rw_template, rwmmio_write, ...)
// DEFINE_EVENT(rwmmio_rw_template, rwmmio_post_write, ...)
// The trace-event registration and external tracepoint symbols are provided by
// the Linux tracepoint implementation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
