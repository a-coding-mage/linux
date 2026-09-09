// SPDX-License-Identifier: GPL-2.0-only
/*
 * Register read and write tracepoints
 *
 * Copyright (c) 2021-2022 Qualcomm Innovation Center, Inc. All rights reserved.
 */

// CREATE_TRACE_POINTS
// The following declarations are provided by <trace/events/rwmmio.h>.
use core::ffi::c_void;

#[cfg(feature = "CONFIG_TRACE_MMIO_ACCESS")]
extern "C" {
    fn trace_rwmmio_write(
        caller_addr: usize,
        caller_addr0: usize,
        val: u64,
        width: u8,
        addr: *mut c_void,
    );
    fn trace_rwmmio_post_write(
        caller_addr: usize,
        caller_addr0: usize,
        val: u64,
        width: u8,
        addr: *mut c_void,
    );
    fn trace_rwmmio_read(caller_addr: usize, caller_addr0: usize, width: u8, addr: *const c_void);
    fn trace_rwmmio_post_read(
        caller_addr: usize,
        caller_addr0: usize,
        val: u64,
        width: u8,
        addr: *const c_void,
    );
}

// EXPORT_SYMBOL_GPL(log_write_mmio);
// EXPORT_TRACEPOINT_SYMBOL_GPL(rwmmio_write);
#[cfg(feature = "CONFIG_TRACE_MMIO_ACCESS")]
#[no_mangle]
pub unsafe extern "C" fn log_write_mmio(
    val: u64,
    width: u8,
    addr: *mut c_void,
    caller_addr: usize,
    caller_addr0: usize,
) {
    trace_rwmmio_write(caller_addr, caller_addr0, val, width, addr);
}

// EXPORT_SYMBOL_GPL(log_post_write_mmio);
// EXPORT_TRACEPOINT_SYMBOL_GPL(rwmmio_post_write);
#[cfg(feature = "CONFIG_TRACE_MMIO_ACCESS")]
#[no_mangle]
pub unsafe extern "C" fn log_post_write_mmio(
    val: u64,
    width: u8,
    addr: *mut c_void,
    caller_addr: usize,
    caller_addr0: usize,
) {
    trace_rwmmio_post_write(caller_addr, caller_addr0, val, width, addr);
}

// EXPORT_SYMBOL_GPL(log_read_mmio);
// EXPORT_TRACEPOINT_SYMBOL_GPL(rwmmio_read);
#[cfg(feature = "CONFIG_TRACE_MMIO_ACCESS")]
#[no_mangle]
pub unsafe extern "C" fn log_read_mmio(
    width: u8,
    addr: *const c_void,
    caller_addr: usize,
    caller_addr0: usize,
) {
    trace_rwmmio_read(caller_addr, caller_addr0, width, addr);
}

// EXPORT_SYMBOL_GPL(log_post_read_mmio);
// EXPORT_TRACEPOINT_SYMBOL_GPL(rwmmio_post_read);
#[cfg(feature = "CONFIG_TRACE_MMIO_ACCESS")]
#[no_mangle]
pub unsafe extern "C" fn log_post_read_mmio(
    val: u64,
    width: u8,
    addr: *const c_void,
    caller_addr: usize,
    caller_addr0: usize,
) {
    trace_rwmmio_post_read(caller_addr, caller_addr0, val, width, addr);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
