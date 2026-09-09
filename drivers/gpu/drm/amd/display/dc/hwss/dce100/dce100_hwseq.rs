/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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

#[repr(C)]
struct dce100_hw_seq_reg_offsets {
    blnd: u32,
    crtc: u32,
}

static REG_OFFSETS: [dce100_hw_seq_reg_offsets; 6] = [
    dce100_hw_seq_reg_offsets { blnd: 0, crtc: mmCRTC0_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
    dce100_hw_seq_reg_offsets { blnd: 0, crtc: mmCRTC1_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
    dce100_hw_seq_reg_offsets { blnd: 0, crtc: mmCRTC2_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
    dce100_hw_seq_reg_offsets { blnd: 0, crtc: mmCRTC3_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
    dce100_hw_seq_reg_offsets { blnd: 0, crtc: mmCRTC4_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
    dce100_hw_seq_reg_offsets { blnd: 0, crtc: mmCRTC5_CRTC_GSL_CONTROL - mmCRTC_GSL_CONTROL },
];

#[inline]
unsafe fn hw_reg_crtc(reg: u32, id: usize) -> u32 {
    reg + REG_OFFSETS[id].crtc
}

/* Private definitions */
/* PIPE_CONTROL */

pub unsafe fn dce100_enable_display_power_gating(
    dc: *mut dc,
    controller_id: u8,
    dcb: *mut dc_bios,
    power_gating: pipe_gating_control,
) -> bool {
    let mut bp_result = BP_RESULT_OK;
    let cntl;
    let ctx = (*dc).ctx;

    if power_gating == PIPE_GATING_CONTROL_INIT {
        cntl = ASIC_PIPE_INIT;
    } else if power_gating == PIPE_GATING_CONTROL_ENABLE {
        cntl = ASIC_PIPE_ENABLE;
    } else {
        cntl = ASIC_PIPE_DISABLE;
    }

    if !(power_gating == PIPE_GATING_CONTROL_INIT && controller_id != 0) {
        bp_result = ((*(*dcb).funcs).enable_disp_power_gating)(
            dcb,
            controller_id + 1,
            cntl,
        );

        /* Revert MASTER_UPDATE_MODE to 0 because bios sets it 2
         * by default when command table is called
         */
        dm_write_reg(ctx, hw_reg_crtc(mmMASTER_UPDATE_MODE, controller_id as usize), 0);
    }

    if bp_result == BP_RESULT_OK {
        true
    } else {
        false
    }
}

pub unsafe fn dce100_prepare_bandwidth(dc: *mut dc, context: *mut dc_state) {
    dce110_set_safe_displaymarks(&mut (*context).res_ctx, (*dc).res_pool);
    ((*(*(*dc).clk_mgr).funcs).update_clocks)(
        (*dc).clk_mgr,
        context,
        false,
    );
}

pub unsafe fn dce100_optimize_bandwidth(dc: *mut dc, context: *mut dc_state) {
    dce110_set_safe_displaymarks(&mut (*context).res_ctx, (*dc).res_pool);
    ((*(*(*dc).clk_mgr).funcs).update_clocks)(
        (*dc).clk_mgr,
        context,
        true,
    );
}

pub unsafe fn dce100_hw_sequencer_construct(dc: *mut dc) {
    dce110_hw_sequencer_construct(dc);

    (*(*dc).hwseq).funcs.enable_display_power_gating = Some(dce100_enable_display_power_gating);
    (*(*dc).hwss).prepare_bandwidth = Some(dce100_prepare_bandwidth);
    (*(*dc).hwss).optimize_bandwidth = Some(dce100_optimize_bandwidth);
    (*(*dc).hwss).clear_surface_dcc_and_tiling = Some(dce100_reset_surface_dcc_and_tiling);
}

/**
 * dce100_reset_surface_dcc_and_tiling - Set DCC and tiling in DCE to their disable mode.
 *
 * @pipe_ctx: Pointer to the pipe context structure.
 * @plane_state: Surface state
 * @clear_tiling: If true set tiling to Linear, otherwise does not change tiling
 *
 * This function is responsible for call the HUBP block to disable DCC and set
 * tiling to the linear mode.
 */
pub unsafe fn dce100_reset_surface_dcc_and_tiling(
    pipe_ctx: *mut pipe_ctx,
    plane_state: *mut dc_plane_state,
    clear_tiling: bool,
) {
    let mi = (*pipe_ctx).plane_res.mi;

    if mi.is_null() {
        return;
    }

    /* if framebuffer is tiled, disable tiling */
    if clear_tiling && (*mi).funcs.mem_input_clear_tiling.is_some() {
        ((*(*mi).funcs).mem_input_clear_tiling)(mi);
    }

    /* force page flip to see the new content of the framebuffer */
    ((*(*mi).funcs).mem_input_program_surface_flip_and_addr)(
        mi,
        &mut (*plane_state).address,
        true,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
