/*
* Copyright 2018 Advanced Micro Devices, Inc.
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

/* Dependencies supplied by the surrounding translated sources. */

unsafe extern "C" {
    fn hubbub1_program_urgent_watermarks(
        hubbub: *mut hubbub,
        watermarks: *mut dcn_watermark_set,
        refclk_mhz: u32,
        safe_to_lower: bool,
    ) -> bool;
    fn hubbub1_program_pstate_watermarks(
        hubbub: *mut hubbub,
        watermarks: *mut dcn_watermark_set,
        refclk_mhz: u32,
        safe_to_lower: bool,
    ) -> bool;
    fn hubbub1_allow_self_refresh_control(hubbub: *mut hubbub, enable: bool);
    fn hubbub2_update_dchub();
    fn hubbub2_dcc_support_swizzle();
    fn hubbub2_dcc_support_pixel_format();
    fn hubbub2_get_dcc_compression_cap();
    fn hubbub2_wm_read_state();
    fn hubbub2_get_dchub_ref_freq();
    fn hubbub2_read_state();
}

unsafe fn hubbub201_program_watermarks(
    hubbub: *mut hubbub,
    watermarks: *mut dcn_watermark_set,
    refclk_mhz: u32,
    safe_to_lower: bool,
) -> bool {
    let hubbub1: *mut dcn20_hubbub = TO_DCN20_HUBBUB(hubbub);
    let mut wm_pending = false;

    if hubbub1_program_urgent_watermarks(hubbub, watermarks, refclk_mhz, safe_to_lower) {
        wm_pending = true;
    }

    if hubbub1_program_pstate_watermarks(hubbub, watermarks, refclk_mhz, safe_to_lower) {
        wm_pending = true;
    }

    REG_SET!(hubbub1, DCHUBBUB_ARB_SAT_LEVEL, 0,
        DCHUBBUB_ARB_SAT_LEVEL, 60 * refclk_mhz);
    REG_UPDATE!(hubbub1, DCHUBBUB_ARB_DF_REQ_OUTSTAND,
        DCHUBBUB_ARB_MIN_REQ_OUTSTAND, 68);

    unsafe {
        hubbub1_allow_self_refresh_control(
            hubbub,
            !(*(*hubbub).ctx).dc.debug.disable_stutter,
        );
    }

    wm_pending
}

static hubbub201_funcs: hubbub_funcs = hubbub_funcs {
    update_dchub: Some(hubbub2_update_dchub),
    init_dchub_sys_ctx: None,
    init_vm_ctx: None,
    dcc_support_swizzle: Some(hubbub2_dcc_support_swizzle),
    dcc_support_pixel_format: Some(hubbub2_dcc_support_pixel_format),
    get_dcc_compression_cap: Some(hubbub2_get_dcc_compression_cap),
    wm_read_state: Some(hubbub2_wm_read_state),
    get_dchub_ref_freq: Some(hubbub2_get_dchub_ref_freq),
    program_watermarks: Some(hubbub201_program_watermarks),
    hubbub_read_state: Some(hubbub2_read_state),
};

pub unsafe fn hubbub201_construct(
    hubbub: *mut dcn20_hubbub,
    ctx: *mut dc_context,
    hubbub_regs: *const dcn_hubbub_registers,
    hubbub_shift: *const dcn_hubbub_shift,
    hubbub_mask: *const dcn_hubbub_mask,
) {
    (*hubbub).base.ctx = ctx;

    (*hubbub).base.funcs = &hubbub201_funcs;

    (*hubbub).regs = hubbub_regs;
    (*hubbub).shifts = hubbub_shift;
    (*hubbub).masks = hubbub_mask;

    (*hubbub).debug_test_index_pstate = 0xB;
    (*hubbub).detile_buf_size = 164 * 1024; /* 164KB for DCN2.0 */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
