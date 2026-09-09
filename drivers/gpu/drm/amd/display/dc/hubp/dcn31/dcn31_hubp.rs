/*
 * Copyright 2012-20 Advanced Micro Devices, Inc.
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

// C dependencies: dm_services.h, dce_calcs.h, reg_helper.h,
// basics/conversion.h, and dcn31_hubp.h.

extern "C" {
    fn reg_update(hubp: *mut dcn20_hubp, register: u32, field: u32, value: u32);
    fn reg_get(hubp: *mut dcn20_hubp, register: u32, field: u32, value: *mut u32);
}

pub unsafe fn hubp31_set_unbounded_requesting(hubp: *mut hubp, enable: bool) {
    let hubp2: *mut dcn20_hubp = TO_DCN20_HUBP(hubp);

    reg_update(hubp2, DCHUBP_CNTL, HUBP_UNBOUNDED_REQ_MODE, enable as u32);
    reg_update(hubp2, CURSOR_CONTROL, CURSOR_REQ_MODE, 1);
}

pub unsafe fn hubp31_soft_reset(hubp: *mut hubp, reset: bool) {
    let hubp2: *mut dcn20_hubp = TO_DCN20_HUBP(hubp);

    reg_update(hubp2, DCHUBP_CNTL, HUBP_SOFT_RESET, reset as u32);
}

unsafe fn hubp31_program_extended_blank(
    hubp: *mut hubp,
    min_dst_y_next_start_optimized: u32,
) {
    let hubp2: *mut dcn20_hubp = TO_DCN20_HUBP(hubp);

    reg_update(
        hubp2,
        BLANK_OFFSET_1,
        MIN_DST_Y_NEXT_START,
        min_dst_y_next_start_optimized,
    );
}

pub unsafe fn hubp31_program_extended_blank_value(
    hubp: *mut hubp,
    min_dst_y_next_start_optimized: u32,
) {
    hubp31_program_extended_blank(hubp, min_dst_y_next_start_optimized);
}

pub unsafe fn hubp31_get_det_config_error(hubp: *mut hubp) -> u32 {
    let mut config_error: u32 = 0;
    let hubp2: *mut dcn20_hubp = TO_DCN20_HUBP(hubp);

    reg_get(
        hubp2,
        DCHUBP_CNTL,
        HUBP_SEG_ALLOC_ERR_STATUS,
        &mut config_error,
    );

    config_error
}

static mut dcn31_hubp_funcs: hubp_funcs = hubp_funcs {
    hubp_enable_tripleBuffer: Some(hubp2_enable_triplebuffer),
    hubp_is_triplebuffer_enabled: Some(hubp2_is_triplebuffer_enabled),
    hubp_program_surface_flip_and_addr: Some(hubp3_program_surface_flip_and_addr),
    hubp_program_surface_config: Some(hubp3_program_surface_config),
    hubp_is_flip_pending: Some(hubp2_is_flip_pending),
    hubp_setup: Some(hubp3_setup),
    hubp_setup_interdependent: Some(hubp2_setup_interdependent),
    hubp_set_vm_system_aperture_settings: Some(hubp3_set_vm_system_aperture_settings),
    set_blank: Some(hubp2_set_blank),
    dcc_control: Some(hubp3_dcc_control),
    hubp_reset: Some(hubp_reset),
    mem_program_viewport: Some(min_set_viewport),
    set_cursor_attributes: Some(hubp2_cursor_set_attributes),
    set_cursor_position: Some(hubp2_cursor_set_position),
    hubp_clk_cntl: Some(hubp2_clk_cntl),
    hubp_vtg_sel: Some(hubp2_vtg_sel),
    dmdata_set_attributes: Some(hubp3_dmdata_set_attributes),
    dmdata_load: Some(hubp2_dmdata_load),
    dmdata_status_done: Some(hubp2_dmdata_status_done),
    hubp_read_state: Some(hubp3_read_state),
    hubp_clear_underflow: Some(hubp2_clear_underflow),
    hubp_set_flip_control_surface_gsl: Some(hubp2_set_flip_control_surface_gsl),
    hubp_init: Some(hubp3_init),
    set_unbounded_requesting: Some(hubp31_set_unbounded_requesting),
    hubp_soft_reset: Some(hubp31_soft_reset),
    hubp_set_flip_int: Some(hubp1_set_flip_int),
    hubp_in_blank: Some(hubp1_in_blank),
    program_extended_blank: Some(hubp31_program_extended_blank),
    hubp_clear_tiling: Some(hubp3_clear_tiling),
    hubp_read_reg_state: Some(hubp3_read_reg_state),
};

pub unsafe fn hubp31_construct(
    hubp2: *mut dcn20_hubp,
    ctx: *mut dc_context,
    inst: u32,
    hubp_regs: *const dcn_hubp2_registers,
    hubp_shift: *const dcn_hubp2_shift,
    hubp_mask: *const dcn_hubp2_mask,
) -> bool {
    (*hubp2).base.funcs = &mut dcn31_hubp_funcs;
    (*hubp2).base.ctx = ctx;
    (*hubp2).hubp_regs = hubp_regs;
    (*hubp2).hubp_shift = hubp_shift;
    (*hubp2).hubp_mask = hubp_mask;
    (*hubp2).base.inst = inst;
    (*hubp2).base.opp_id = OPP_ID_INVALID;
    (*hubp2).base.mpcc_id = 0xf;

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
