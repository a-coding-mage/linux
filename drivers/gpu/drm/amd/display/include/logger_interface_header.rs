/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// The C header includes "logger_types.h"; its declarations are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct dc;
#[repr(C)]
pub struct dc_context;
#[repr(C)]
pub struct dc_link;
#[repr(C)]
pub struct dc_surface_update;
#[repr(C)]
pub struct resource_context;
#[repr(C)]
pub struct dc_state;

extern "C" {
    pub fn update_surface_trace(
        dc: *mut dc,
        updates: *const dc_surface_update,
        surface_count: ::core::ffi::c_int,
    );

    pub fn post_surface_trace(dc: *mut dc);

    pub fn context_clock_trace(dc: *mut dc, context: *mut dc_state);
}

/* Any function which is empty or have incomplete implementation should be
 * marked by this macro.
 * Note that the message will be printed exactly once for every function
 * it is used in order to avoid repeating of the same message. */
#[macro_export]
macro_rules! DAL_LOGGER_NOT_IMPL {
    ($fmt:expr $(, $arg:expr)*) => {{
        static mut PRINT_NOT_IMPL: bool = true;
        unsafe {
            if PRINT_NOT_IMPL == true {
                PRINT_NOT_IMPL = false;
                DRM_WARN!(concat!("DAL_NOT_IMPL: ", $fmt) $(, $arg)*);
            }
        }
    }};
}

/* Convenience macros to save on typing. */
#[macro_export]
macro_rules! DC_ERROR {
    ($($arg:tt)*) => {{
        let _ = dc_ctx;
        DC_LOG_ERROR!($($arg)*);
    }};
}

#[macro_export]
macro_rules! DC_SYNC_INFO {
    ($($arg:tt)*) => {{
        let _ = dc_ctx;
        DC_LOG_SYNC!($($arg)*);
    }};
}

#[macro_export]
macro_rules! CONN_DATA_DETECT {
    ($link:expr, $hex_data:expr, $hex_len:expr $(, $arg:tt)*) => {{
        let _ = ($link, $hex_data, $hex_len);
        DC_LOG_EVENT_DETECTION!($($arg)*);
    }};
}

#[macro_export]
macro_rules! CONN_DATA_LINK_LOSS {
    ($link:expr, $hex_data:expr, $hex_len:expr $(, $arg:tt)*) => {{
        let _ = ($link, $hex_data, $hex_len);
        DC_LOG_EVENT_LINK_LOSS!($($arg)*);
    }};
}

#[macro_export]
macro_rules! CONN_MSG_LT {
    ($link:expr $(, $arg:tt)*) => {{
        let _ = $link;
        DC_LOG_EVENT_LINK_TRAINING!($($arg)*);
    }};
}

#[macro_export]
macro_rules! CONN_MSG_MODE {
    ($link:expr $(, $arg:tt)*) => {{
        let _ = $link;
        DC_LOG_EVENT_MODE_SET!($($arg)*);
    }};
}

/* Display Test Next logging. */
#[macro_export]
macro_rules! DTN_INFO_BEGIN {
    () => { dm_dtn_log_begin(dc_ctx, log_ctx) };
}

#[macro_export]
macro_rules! DTN_INFO {
    ($msg:expr $(, $arg:expr)*) => { dm_dtn_log_append_v(dc_ctx, log_ctx, $msg $(, $arg)*) };
}

#[macro_export]
macro_rules! DTN_INFO_END {
    () => { dm_dtn_log_end(dc_ctx, log_ctx) };
}

#[macro_export]
macro_rules! PERFORMANCE_TRACE_START {
    () => { let mut perf_trc_start_stmp: u64 = dm_get_timestamp(dc.ctx); };
}

#[macro_export]
macro_rules! PERFORMANCE_TRACE_END {
    () => {{
        let perf_trc_end_stmp: u64 = dm_get_timestamp(dc.ctx);
        if dc.debug.performance_trace {
            DC_LOG_PERF_TRACE!("%s duration: %lld ticks\n", "__func__", perf_trc_end_stmp.wrapping_sub(perf_trc_start_stmp));
        }
    }};
}

#[macro_export]
macro_rules! DISPLAY_STATS_BEGIN {
    ($entry:expr) => { let _ = $entry; };
}

#[macro_export]
macro_rules! DISPLAY_STATS {
    ($msg:expr $(, $arg:expr)*) => { DC_LOG_PERF_TRACE!($msg $(, $arg)*) };
}

#[macro_export]
macro_rules! DISPLAY_STATS_END {
    ($entry:expr) => { let _ = $entry; };
}

#[macro_export]
macro_rules! LOG_GAMMA_WRITE {
    ($msg:expr $(, $arg:expr)*) => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
