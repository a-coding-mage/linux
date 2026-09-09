// SPDX-License-Identifier: MIT
/*
 * Copyright (C) 2021 Advanced Micro Devices, Inc.
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

// Dependencies supplied by dcn303/dcn303_hwseq.h, dcn30/dcn30_init.h,
// dc.h, and dcn303_init.h.

extern "C" {
    pub fn dcn30_hw_sequencer_construct(dc: *mut dc);
}

extern "C" {
    pub fn dcn303_dpp_pg_control();
    pub fn dcn303_hubp_pg_control();
    pub fn dcn303_dsc_pg_control();
    pub fn dcn303_enable_power_gating_plane();
}

pub unsafe fn dcn303_hw_sequencer_construct(dc: *mut dc) {
    dcn30_hw_sequencer_construct(dc);

    (*(*dc).hwseq).funcs.dpp_pg_control = Some(dcn303_dpp_pg_control);
    (*(*dc).hwseq).funcs.hubp_pg_control = Some(dcn303_hubp_pg_control);
    (*(*dc).hwseq).funcs.dsc_pg_control = Some(dcn303_dsc_pg_control);
    (*(*dc).hwseq).funcs.enable_power_gating_plane = Some(dcn303_enable_power_gating_plane);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
