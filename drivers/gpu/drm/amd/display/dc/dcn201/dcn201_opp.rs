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

// Declarations supplied by dm_services.h, dcn201_opp.h, and reg_helper.h.

extern "C" {
    fn opp1_set_dyn_expansion();
    fn opp1_program_fmt();
    fn opp1_program_bit_depth_reduction();
    fn opp1_program_stereo();
    fn opp1_pipe_clock_control();
    fn opp2_set_disp_pattern_generator();
    fn opp2_program_dpg_dimensions();
    fn opp2_dpg_is_blanked();
    fn opp2_dpg_is_pending();
    fn opp2_dpg_set_blank_color();
    fn opp1_destroy();
    fn opp2_program_left_edge_extra_pixel();
    fn opp2_get_left_edge_extra_pixel_count();
}

static mut dcn201_opp_funcs: opp_funcs = opp_funcs {
    opp_set_dyn_expansion: opp1_set_dyn_expansion,
    opp_program_fmt: opp1_program_fmt,
    opp_program_bit_depth_reduction: opp1_program_bit_depth_reduction,
    opp_program_stereo: opp1_program_stereo,
    opp_pipe_clock_control: opp1_pipe_clock_control,
    opp_set_disp_pattern_generator: opp2_set_disp_pattern_generator,
    opp_program_dpg_dimensions: opp2_program_dpg_dimensions,
    dpg_is_blanked: opp2_dpg_is_blanked,
    dpg_is_pending: opp2_dpg_is_pending,
    opp_dpg_set_blank_color: opp2_dpg_set_blank_color,
    opp_destroy: opp1_destroy,
    opp_program_left_edge_extra_pixel: opp2_program_left_edge_extra_pixel,
    opp_get_left_edge_extra_pixel_count: opp2_get_left_edge_extra_pixel_count,
};

pub unsafe fn dcn201_opp_construct(
    oppn201: *mut dcn201_opp,
    ctx: *mut dc_context,
    inst: u32,
    regs: *const dcn201_opp_registers,
    opp_shift: *const dcn201_opp_shift,
    opp_mask: *const dcn201_opp_mask,
) {
    (*oppn201).base.ctx = ctx;
    (*oppn201).base.inst = inst;
    (*oppn201).base.funcs = &raw const dcn201_opp_funcs;

    (*oppn201).regs = regs;
    (*oppn201).opp_shift = opp_shift;
    (*oppn201).opp_mask = opp_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
