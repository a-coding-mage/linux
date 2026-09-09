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

// Dependencies supplied by the surrounding translation unit.

unsafe fn setup_hpo_frl_stream_attribute(pipe_ctx: *mut pipe_ctx) {
    let stream_enc = (*pipe_ctx).stream_res.hpo_frl_stream_enc;
    let stream = (*pipe_ctx).stream;
    let mut odm_pipe: *mut pipe_ctx;
    let dc = (*(*stream).link).ctx.dc;
    let temp_stream = &mut (*dc).scratch.temp_stream;
    let mut odm_combine_num_segments = 1;

    core::ptr::copy_nonoverlapping(stream, temp_stream, 1);

    /* Modify patched_crtc_timing as required for padding */
    if (*pipe_ctx).dsc_padding_params.dsc_hactive_padding != 0 {
        temp_stream.timing.h_addressable = (*stream).timing.h_addressable
            + (*pipe_ctx).dsc_padding_params.dsc_hactive_padding;
        temp_stream.timing.h_total = (*stream).timing.h_total
            + (*pipe_ctx).dsc_padding_params.dsc_htotal_padding;
    }

    /* get number of ODM combine input segments */
    odm_pipe = (*pipe_ctx).next_odm_pipe;
    while !odm_pipe.is_null() {
        odm_combine_num_segments += 1;
        odm_pipe = (*odm_pipe).next_odm_pipe;
    }

    ((*(*stream_enc).funcs).hdmi_frl_set_stream_attribute)(
        stream_enc,
        &mut temp_stream.timing,
        &(*(*stream).link).frl_link_settings.borrow_params,
        odm_combine_num_segments,
    );
}

unsafe fn disable_hpo_frl_link_output(
    link: *mut dc_link,
    link_res: *const link_resource,
    signal: signal_type,
) {
    let _ = link_res;
    if dc_is_hdmi_frl_signal(signal) {
        ((*(*link).hpo_frl_link_enc).funcs.disable_link_encoder)(
            (*link).hpo_frl_link_enc,
        );
    }
    ((*(*link).link_enc).funcs.disable_output)((*link).link_enc, signal);
}

unsafe fn setup_hpo_frl_audio_output(
    pipe_ctx: *mut pipe_ctx,
    audio_output: *mut audio_output,
    audio_inst: u32,
) {
    ((*(*(*pipe_ctx).stream_res.hpo_frl_stream_enc).funcs).hdmi_audio_setup)(
        (*pipe_ctx).stream_res.hpo_frl_stream_enc,
        audio_inst,
        &(*(*pipe_ctx).stream).audio_info,
        &(*audio_output).crtc_info,
    );
}

unsafe fn enable_hpo_frl_audio_packet(pipe_ctx: *mut pipe_ctx) {
    ((*(*(*pipe_ctx).stream_res.hpo_frl_stream_enc).funcs).audio_mute_control)(
        (*pipe_ctx).stream_res.hpo_frl_stream_enc,
        false,
    );
}

unsafe fn disable_hpo_frl_audio_packet(pipe_ctx: *mut pipe_ctx) {
    ((*(*(*pipe_ctx).stream_res.hpo_frl_stream_enc).funcs).audio_mute_control)(
        (*pipe_ctx).stream_res.hpo_frl_stream_enc,
        true,
    );

    if !(*pipe_ctx).stream_res.audio.is_null() {
        ((*(*(*pipe_ctx).stream_res.hpo_frl_stream_enc).funcs).hdmi_audio_disable)(
            (*pipe_ctx).stream_res.hpo_frl_stream_enc,
        );
    }
}

static hpo_frl_link_hwss: link_hwss = link_hwss {
    setup_stream_encoder: virtual_setup_stream_encoder,
    reset_stream_encoder: virtual_reset_stream_encoder,
    setup_stream_attribute: setup_hpo_frl_stream_attribute,
    disable_link_output: disable_hpo_frl_link_output,
    setup_audio_output: setup_hpo_frl_audio_output,
    enable_audio_packet: enable_hpo_frl_audio_packet,
    disable_audio_packet: disable_hpo_frl_audio_packet,
};

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
