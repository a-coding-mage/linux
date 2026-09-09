/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// Dependency declarations supplied by hw_sequencer_private.h are external to
// this translation unit.

extern "C" {
    pub fn dcn201_set_dmdata_attributes(pipe_ctx: *mut pipe_ctx);
    pub fn dcn201_init_hw(dc: *mut dc);
    pub fn dcn201_unblank_stream(
        pipe_ctx: *mut pipe_ctx,
        link_settings: *mut dc_link_settings,
    );
    pub fn dcn201_update_plane_addr(dc: *const dc, pipe_ctx: *mut pipe_ctx);
    pub fn dcn201_plane_atomic_disconnect(
        dc: *mut dc,
        state: *mut dc_state,
        pipe_ctx: *mut pipe_ctx,
    );
    pub fn dcn201_update_mpcc(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
    pub fn dcn201_set_cursor_attribute(pipe_ctx: *mut pipe_ctx);
    pub fn dcn201_pipe_control_lock(
        dc: *mut dc,
        pipe: *mut pipe_ctx,
        lock: bool,
    );
    pub fn dcn201_init_blank(dc: *mut dc, tg: *mut timing_generator);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
