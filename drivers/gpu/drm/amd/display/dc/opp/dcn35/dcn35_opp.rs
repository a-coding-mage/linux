/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency declarations and register-helper macros are supplied by the
// surrounding translation unit.

// C equivalent: REG(reg) ((const struct dcn35_opp_registers *)(oppn20->regs))->reg
// C equivalent: FN(reg_name, field_name) expands to the corresponding shift
// and mask fields from oppn20->opp_shift and oppn20->opp_mask.
// C equivalent: CTX expands to oppn20->base.ctx.

pub unsafe fn dcn35_opp_construct(
    oppn20: *mut dcn20_opp,
    ctx: *mut dc_context,
    inst: u32,
    regs: *const dcn35_opp_registers,
    opp_shift: *const dcn35_opp_shift,
    opp_mask: *const dcn35_opp_mask,
) {
    dcn20_opp_construct(
        oppn20,
        ctx,
        inst,
        regs as *const dcn20_opp_registers,
        opp_shift as *const dcn20_opp_shift,
        opp_mask as *const dcn20_opp_mask,
    );
}

pub unsafe fn dcn35_opp_set_fgcg(oppn20: *mut dcn20_opp, enable: bool) {
    REG_UPDATE!(oppn20, OPP_TOP_CLK_CONTROL, OPP_FGCG_REP_DIS, !enable);
}

pub unsafe fn dcn35_opp_read_reg_state(
    opp: *mut output_pixel_processor,
    opp_reg_state: *mut dcn_opp_reg_state,
) {
    let oppn20: *mut dcn20_opp = TO_DCN20_OPP!(opp);

    (*opp_reg_state).dpg_control = REG_READ!(oppn20, DPG_CONTROL);
    (*opp_reg_state).fmt_control = REG_READ!(oppn20, FMT_CONTROL);
    (*opp_reg_state).opp_abm_control = REG_READ!(oppn20, OPP_ABM_CONTROL);
    (*opp_reg_state).opp_pipe_control = REG_READ!(oppn20, OPP_PIPE_CONTROL);
    (*opp_reg_state).opp_pipe_crc_control = REG_READ!(oppn20, OPP_PIPE_CRC_CONTROL);
    (*opp_reg_state).oppbuf_control = REG_READ!(oppn20, OPPBUF_CONTROL);
    (*opp_reg_state).dscrm_dsc_forward_config =
        REG_READ!(oppn20, DSCRM_DSC_FORWARD_CONFIG);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
