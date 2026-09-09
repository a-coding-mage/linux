/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

/* Dependencies supplied by the surrounding translation unit. */

pub unsafe fn dce_enable_fe_clock(hws: *mut dce_hwseq, fe_inst: u32, enable: bool) {
    REG_UPDATE!(hws, DCFE_CLOCK_CONTROL[fe_inst], DCFE_CLOCK_ENABLE, enable);
}

pub unsafe fn dce_pipe_control_lock(dc: *mut dc, pipe: *mut pipe_ctx, lock: bool) {
    let lock_val: u32 = if lock { 1 } else { 0 };
    let (mut dcp_grph, mut scl, mut blnd, mut update_lock_mode, mut val): (u32, u32, u32, u32, u32);
    let hws = (*dc).hwseq;

    /* Not lock pipe when blank */
    if lock && (*pipe).stream_res.tg.funcs.is_blanked.is_some()
        && ((*pipe).stream_res.tg.funcs.is_blanked.unwrap())((*pipe).stream_res.tg)) {
        return;
    }

    val = REG_GET_4!(hws, BLND_V_UPDATE_LOCK[(*pipe).stream_res.tg.inst],
        BLND_DCP_GRPH_V_UPDATE_LOCK, &mut dcp_grph,
        BLND_SCL_V_UPDATE_LOCK, &mut scl,
        BLND_BLND_V_UPDATE_LOCK, &mut blnd,
        BLND_V_UPDATE_LOCK_MODE, &mut update_lock_mode);

    dcp_grph = lock_val;
    scl = lock_val;
    blnd = lock_val;
    update_lock_mode = lock_val;

    REG_SET_2!(hws, BLND_V_UPDATE_LOCK[(*pipe).stream_res.tg.inst], val,
        BLND_DCP_GRPH_V_UPDATE_LOCK, dcp_grph,
        BLND_SCL_V_UPDATE_LOCK, scl);

    if (*hws).masks.BLND_BLND_V_UPDATE_LOCK != 0 {
        REG_SET_2!(hws, BLND_V_UPDATE_LOCK[(*pipe).stream_res.tg.inst], val,
            BLND_BLND_V_UPDATE_LOCK, blnd,
            BLND_V_UPDATE_LOCK_MODE, update_lock_mode);
    }

    if (*hws).wa.blnd_crtc_trigger && !lock {
        let value = REG_READ!(hws, CRTC_H_BLANK_START_END[(*pipe).stream_res.tg.inst]);
        REG_WRITE!(hws, CRTC_H_BLANK_START_END[(*pipe).stream_res.tg.inst], value);
    }
}

#[cfg(CONFIG_DRM_AMD_DC_SI)]
pub unsafe fn dce60_pipe_control_lock(_dc: *mut dc, _pipe: *mut pipe_ctx, _lock: bool) {
    /* DCE6 has no BLND_V_UPDATE_LOCK register */
}

pub unsafe fn dce_set_blender_mode(hws: *mut dce_hwseq, blnd_inst: u32, mode: blnd_mode) {
    let mut feedthrough: u32 = 1;
    let mut blnd_mode: u32 = 0;
    let mut multiplied_mode: u32 = 0;
    let mut alpha_mode: u32 = 2;

    match mode {
        BLND_MODE_OTHER_PIPE => { feedthrough = 0; blnd_mode = 1; alpha_mode = 0; }
        BLND_MODE_BLENDING => { feedthrough = 0; blnd_mode = 2; alpha_mode = 0; multiplied_mode = 1; }
        BLND_MODE_CURRENT_PIPE => {
            if REG!(hws, BLND_CONTROL[blnd_inst]) == REG!(hws, BLNDV_CONTROL) || blnd_inst == 0 {
                feedthrough = 0;
            }
        }
        _ => {
            if REG!(hws, BLND_CONTROL[blnd_inst]) == REG!(hws, BLNDV_CONTROL) || blnd_inst == 0 {
                feedthrough = 0;
            }
        }
    }

    REG_UPDATE!(hws, BLND_CONTROL[blnd_inst], BLND_MODE, blnd_mode);
    if (*hws).masks.BLND_ALPHA_MODE != 0 {
        REG_UPDATE_3!(hws, BLND_CONTROL[blnd_inst],
            BLND_FEEDTHROUGH_EN, feedthrough,
            BLND_ALPHA_MODE, alpha_mode,
            BLND_MULTIPLIED_MODE, multiplied_mode);
    }
}

unsafe fn dce_disable_sram_shut_down(hws: *mut dce_hwseq) {
    if REG!(hws, DC_MEM_GLOBAL_PWR_REQ_CNTL) != 0 {
        REG_UPDATE!(hws, DC_MEM_GLOBAL_PWR_REQ_CNTL, DC_MEM_GLOBAL_PWR_REQ_DIS, 1);
    }
}

unsafe fn dce_underlay_clock_enable(hws: *mut dce_hwseq) {
    /* todo: why do we need this at boot? is dce_enable_fe_clock enough? */
    if REG!(hws, DCFEV_CLOCK_CONTROL) != 0 {
        REG_UPDATE!(hws, DCFEV_CLOCK_CONTROL, DCFEV_CLOCK_ENABLE, 1);
    }
}

unsafe fn enable_hw_base_light_sleep() {
    /* TODO: implement */
}

unsafe fn disable_sw_manual_control_light_sleep() {
    /* TODO: implement */
}

pub unsafe fn dce_clock_gating_power_up(hws: *mut dce_hwseq, enable: bool) {
    if enable {
        enable_hw_base_light_sleep();
        disable_sw_manual_control_light_sleep();
    } else {
        dce_disable_sram_shut_down(hws);
        dce_underlay_clock_enable(hws);
    }
}

pub unsafe fn dce_crtc_switch_to_clk_src(hws: *mut dce_hwseq, clk_src: *mut clock_source, tg_inst: u32) {
    if (*clk_src).id == CLOCK_SOURCE_ID_DP_DTO || (*clk_src).dp_clk_src {
        REG_UPDATE!(hws, PIXEL_RATE_CNTL[tg_inst], DP_DTO0_ENABLE, 1);
    } else if (*clk_src).id >= CLOCK_SOURCE_COMBO_PHY_PLL0 {
        let rate_source = (*clk_src).id - CLOCK_SOURCE_COMBO_PHY_PLL0;
        REG_UPDATE_2!(hws, PHYPLL_PIXEL_RATE_CNTL[tg_inst],
            PHYPLL_PIXEL_RATE_SOURCE, rate_source, PIXEL_RATE_PLL_SOURCE, 0);
        REG_UPDATE!(hws, PIXEL_RATE_CNTL[tg_inst], DP_DTO0_ENABLE, 0);
    } else if (*clk_src).id <= CLOCK_SOURCE_ID_PLL2 {
        let rate_source = (*clk_src).id - CLOCK_SOURCE_ID_PLL0;
        REG_UPDATE_2!(hws, PIXEL_RATE_CNTL[tg_inst], PIXEL_RATE_SOURCE, rate_source, DP_DTO0_ENABLE, 0);
        if REG!(hws, PHYPLL_PIXEL_RATE_CNTL[tg_inst]) != 0 {
            REG_UPDATE!(hws, PHYPLL_PIXEL_RATE_CNTL[tg_inst], PIXEL_RATE_PLL_SOURCE, 1);
        }
    } else {
        DC_ERR!("Unknown clock source. clk_src id: %d, TG_inst: %d", (*clk_src).id, tg_inst);
    }
}

/* Only use LUT for 8 bit formats */
pub fn dce_use_lut(format: surface_pixel_format) -> bool {
    matches!(format, SURFACE_PIXEL_FORMAT_GRPH_ARGB8888 | SURFACE_PIXEL_FORMAT_GRPH_ABGR8888)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
