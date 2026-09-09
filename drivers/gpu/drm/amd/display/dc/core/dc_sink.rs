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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding DC implementation.

/*******************************************************************************
 * Private functions
 ******************************************************************************/

unsafe fn dc_sink_construct(
    sink: *mut dc_sink,
    init_params: *const dc_sink_init_data,
) -> bool {
    let link = (*init_params).link;

    if link.is_null() {
        return false;
    }

    (*sink).sink_signal = (*init_params).sink_signal;
    (*sink).link = link;
    (*sink).ctx = (*link).ctx;
    (*sink).dongle_max_pix_clk = (*init_params).dongle_max_pix_clk;
    (*sink).converter_disable_audio = (*init_params).converter_disable_audio;
    (*sink).dc_container_id = core::ptr::null_mut();
    (*sink).sink_id = (*(*init_params).link).ctx.dc_sink_id_count;
    // increment dc_sink_id_count because we don't want two sinks with same ID
    // unless they are actually the same
    (*(*init_params).link).ctx.dc_sink_id_count += 1;

    true
}

/*******************************************************************************
 * Public functions
 ******************************************************************************/

pub unsafe fn dc_sink_retain(sink: *mut dc_sink) {
    kref_get(&mut (*sink).refcount);
}

// EXPORT_IF_KUNIT(dc_sink_retain);

unsafe fn dc_sink_free(kref: *mut kref) {
    let sink: *mut dc_sink = container_of!(kref, dc_sink, refcount);
    kfree((*sink).dc_container_id);
    kfree(sink);
}

pub unsafe fn dc_sink_release(sink: *mut dc_sink) {
    kref_put(&mut (*sink).refcount, dc_sink_free);
}

// EXPORT_IF_KUNIT(dc_sink_release);

pub unsafe fn dc_sink_create(init_params: *const dc_sink_init_data) -> *mut dc_sink {
    let sink: *mut dc_sink = kzalloc_obj!(*sink);

    if sink.is_null() {
        return core::ptr::null_mut();
    }

    if !dc_sink_construct(sink, init_params) {
        kfree(sink);
        return core::ptr::null_mut();
    }

    kref_init(&mut (*sink).refcount);

    return sink;
}

// EXPORT_IF_KUNIT(dc_sink_create);

/*******************************************************************************
 * Protected functions - visible only inside of DC (not visible in DM)
 ******************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
