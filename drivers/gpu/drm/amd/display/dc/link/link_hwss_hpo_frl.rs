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
 */

unsafe fn setup_hpo_frl_stream_attribute(pipe_ctx: *mut pipe_ctx) {
    let stream_enc = (*pipe_ctx).stream_res.hpo_frl_stream_enc;
    let stream = (*pipe_ctx).stream;
    let mut odm_pipe: *mut pipe_ctx;
    let mut odm_combine_num_segments: i32 = 1;

    /* get number of ODM combine input segments */
    odm_pipe = (*pipe_ctx).next_odm_pipe;
    while !odm_pipe.is_null() {
        odm_combine_num_segments += 1;
        odm_pipe = (*odm_pipe).next_odm_pipe;
    }

    ((*(*stream_enc).funcs).hdmi_frl_set_stream_attribute)(
        stream_enc,
        &mut (*stream).timing,
        &(*(*stream).link).frl_link_settings.borrow_params,
        odm_combine_num_segments,
    );
}

static hpo_frl_link_hwss: link_hwss = link_hwss {
    setup_stream_encoder: Some(virtual_setup_stream_encoder),
    reset_stream_encoder: Some(virtual_reset_stream_encoder),
    setup_stream_attribute: Some(setup_hpo_frl_stream_attribute),
};

unsafe extern "C" {
    fn virtual_setup_stream_encoder(pipe_ctx: *mut pipe_ctx);
    fn virtual_reset_stream_encoder(pipe_ctx: *mut pipe_ctx);
}

unsafe fn can_use_hpo_frl_link_hwss(
    link: *const dc_link,
    link_res: *const link_resource,
) -> bool {
    let _ = link;
    !(*link_res).hpo_frl_link_enc.is_null()
}

unsafe fn get_hpo_frl_link_hwss() -> *const link_hwss {
    &hpo_frl_link_hwss
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
