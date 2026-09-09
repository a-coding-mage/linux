/*
 * Copyright 2012-17 Advanced Micro Devices, Inc.
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

// C header dependencies are supplied by the surrounding translation unit.

unsafe fn hubp201_program_surface_config(
    hubp: *mut hubp,
    format: surface_pixel_format,
    tiling_info: *mut dc_tiling_info,
    plane_size: *mut plane_size,
    rotation: dc_rotation_angle,
    dcc: *mut dc_plane_dcc_param,
    horizontal_mirror: bool,
    compat_level: u32,
) {
    hubp1_dcc_control(hubp, (*dcc).enable, (*dcc).independent_64b_blks);
    hubp1_program_tiling(hubp, tiling_info, format);
    hubp1_program_size(hubp, format, plane_size, dcc);
    hubp1_program_pixel_format(hubp, format);
}

unsafe fn hubp201_program_deadline(
    hubp: *mut hubp,
    dlg_attr: *mut _vcs_dpi_display_dlg_regs_st,
    ttu_attr: *mut _vcs_dpi_display_ttu_regs_st,
) {
    hubp1_program_deadline(hubp, dlg_attr, ttu_attr);
}

unsafe fn hubp201_program_requestor(
    hubp: *mut hubp,
    rq_regs: *mut _vcs_dpi_display_rq_regs_st,
) {
    let hubp201: *mut dcn201_hubp = TO_DCN201_HUBP(hubp);

    REG_UPDATE!(hubp201, HUBPRET_CONTROL,
        DET_BUF_PLANE1_BASE_ADDRESS, (*rq_regs).plane1_base_address);

    REG_SET_4!(hubp201, DCN_EXPANSION_MODE, 0,
        DRQ_EXPANSION_MODE, (*rq_regs).drq_expansion_mode,
        PRQ_EXPANSION_MODE, (*rq_regs).prq_expansion_mode,
        MRQ_EXPANSION_MODE, (*rq_regs).mrq_expansion_mode,
        CRQ_EXPANSION_MODE, (*rq_regs).crq_expansion_mode);

    /* no need to program PTE */
    REG_SET_5!(hubp201, DCHUBP_REQ_SIZE_CONFIG, 0,
        CHUNK_SIZE, (*rq_regs).rq_regs_l.chunk_size,
        MIN_CHUNK_SIZE, (*rq_regs).rq_regs_l.min_chunk_size,
        META_CHUNK_SIZE, (*rq_regs).rq_regs_l.meta_chunk_size,
        MIN_META_CHUNK_SIZE, (*rq_regs).rq_regs_l.min_meta_chunk_size,
        SWATH_HEIGHT, (*rq_regs).rq_regs_l.swath_height);

    REG_SET_5!(hubp201, DCHUBP_REQ_SIZE_CONFIG_C, 0,
        CHUNK_SIZE_C, (*rq_regs).rq_regs_c.chunk_size,
        MIN_CHUNK_SIZE_C, (*rq_regs).rq_regs_c.min_chunk_size,
        META_CHUNK_SIZE_C, (*rq_regs).rq_regs_c.meta_chunk_size,
        MIN_META_CHUNK_SIZE_C, (*rq_regs).rq_regs_c.min_meta_chunk_size,
        SWATH_HEIGHT_C, (*rq_regs).rq_regs_c.swath_height);
}

unsafe fn hubp201_setup(
    hubp: *mut hubp,
    dlg_attr: *mut _vcs_dpi_display_dlg_regs_st,
    ttu_attr: *mut _vcs_dpi_display_ttu_regs_st,
    rq_regs: *mut _vcs_dpi_display_rq_regs_st,
    pipe_dest: *mut _vcs_dpi_display_pipe_dest_params_st,
) {
    /*
     * otg is locked when this func is called. Register are double buffered.
     * disable the requestors is not needed
     */
    hubp2_vready_at_or_After_vsync(hubp, pipe_dest);
    hubp201_program_requestor(hubp, rq_regs);
    hubp201_program_deadline(hubp, dlg_attr, ttu_attr);
}

static mut dcn201_hubp_funcs: hubp_funcs = hubp_funcs {
    hubp_enable_tripleBuffer: Some(hubp2_enable_triplebuffer),
    hubp_is_triplebuffer_enabled: Some(hubp2_is_triplebuffer_enabled),
    hubp_program_surface_flip_and_addr: Some(hubp1_program_surface_flip_and_addr),
    hubp_program_surface_config: Some(hubp201_program_surface_config),
    hubp_is_flip_pending: Some(hubp1_is_flip_pending),
    hubp_setup: Some(hubp201_setup),
    hubp_setup_interdependent: Some(hubp2_setup_interdependent),
    set_cursor_attributes: Some(hubp2_cursor_set_attributes),
    set_cursor_position: Some(hubp1_cursor_set_position),
    set_blank: Some(hubp1_set_blank),
    dcc_control: Some(hubp1_dcc_control),
    hubp_reset: Some(hubp_reset),
    mem_program_viewport: Some(min_set_viewport),
    hubp_clk_cntl: Some(hubp1_clk_cntl),
    hubp_vtg_sel: Some(hubp1_vtg_sel),
    dmdata_set_attributes: Some(hubp2_dmdata_set_attributes),
    dmdata_load: Some(hubp2_dmdata_load),
    dmdata_status_done: Some(hubp2_dmdata_status_done),
    hubp_read_state: Some(hubp2_read_state),
    hubp_clear_underflow: Some(hubp1_clear_underflow),
    hubp_set_flip_control_surface_gsl: Some(hubp2_set_flip_control_surface_gsl),
    hubp_init: Some(hubp1_init),
    hubp_clear_tiling: Some(hubp1_clear_tiling),
};

pub unsafe fn dcn201_hubp_construct(
    hubp201: *mut dcn201_hubp,
    ctx: *mut dc_context,
    inst: u32,
    hubp_regs: *const dcn201_hubp_registers,
    hubp_shift: *const dcn201_hubp_shift,
    hubp_mask: *const dcn201_hubp_mask,
) -> bool {
    (*hubp201).base.funcs = &mut dcn201_hubp_funcs;
    (*hubp201).base.ctx = ctx;
    (*hubp201).hubp_regs = hubp_regs;
    (*hubp201).hubp_shift = hubp_shift;
    (*hubp201).hubp_mask = hubp_mask;
    (*hubp201).base.inst = inst;
    (*hubp201).base.opp_id = OPP_ID_INVALID;
    (*hubp201).base.mpcc_id = 0xf;

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
