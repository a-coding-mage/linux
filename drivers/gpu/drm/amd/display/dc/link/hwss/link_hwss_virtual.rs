/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
 *
 */

// Declarations supplied by the corresponding C/Rust dependencies.
use crate::{dc_link, link_hwss, link_resource, pipe_ctx, signal_type};

pub unsafe extern "C" fn virtual_setup_stream_encoder(pipe_ctx: *mut pipe_ctx) {
    let _ = pipe_ctx;
}

pub unsafe extern "C" fn virtual_setup_stream_attribute(pipe_ctx: *mut pipe_ctx) {
    let _ = pipe_ctx;
}

pub unsafe extern "C" fn virtual_reset_stream_encoder(pipe_ctx: *mut pipe_ctx) {
    let _ = pipe_ctx;
}

unsafe extern "C" fn virtual_disable_link_output(
    link: *mut dc_link,
    link_res: *const link_resource,
    signal: signal_type,
) {
    let _ = link;
    let _ = link_res;
    let _ = signal;
}

static virtual_link_hwss: link_hwss = link_hwss {
    setup_stream_encoder: Some(virtual_setup_stream_encoder),
    reset_stream_encoder: Some(virtual_reset_stream_encoder),
    setup_stream_attribute: Some(virtual_setup_stream_attribute),
    disable_link_output: Some(virtual_disable_link_output),
};

pub unsafe extern "C" fn get_virtual_link_hwss() -> *const link_hwss {
    &virtual_link_hwss
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
