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

// Dependencies supplied by the surrounding translation unit:
// reg_helper.h, resource.h, dwb.h, dcn10_dwb.h

/* C macros REG, CTX, FN, and TO_DCN10_DWBC are represented by the
 * corresponding surrounding Rust definitions/macros. */

unsafe fn dwb1_get_caps(dwbc: *mut dwbc, caps: *mut dwb_caps) -> bool {
    let _ = dwbc;
    if !caps.is_null() {
        (*caps).adapter_id = 0; /* we only support 1 adapter currently */
        (*caps).hw_version = DCN_VERSION_1_0;
        (*caps).num_pipes = 2;
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!((*caps).reserved).cast::<u8>(),
            0,
            core::mem::size_of_val(&(*caps).reserved),
        );
        core::ptr::write_bytes(
            core::ptr::addr_of_mut!((*caps).reserved2).cast::<u8>(),
            0,
            core::mem::size_of_val(&(*caps).reserved2),
        );
        (*caps).sw_version = dwb_ver_1_0;
        (*caps).caps.support_dwb = true;
        (*caps).caps.support_ogam = false;
        (*caps).caps.support_wbscl = true;
        (*caps).caps.support_ocsc = false;
        true
    } else {
        false
    }
}

unsafe fn dwb1_enable(dwbc: *mut dwbc, params: *mut dc_dwb_params) -> bool {
    let _ = params;
    let dwbc10 = TO_DCN10_DWBC!(dwbc);

    /* disable first. */
    ((*(*dwbc).funcs).disable.unwrap())(dwbc);

    /* disable power gating */
    REG_UPDATE_5!(dwbc10, WB_EC_CONFIG, DISPCLK_R_WB_GATE_DIS, 1,
        DISPCLK_G_WB_GATE_DIS, 1, DISPCLK_G_WBSCL_GATE_DIS, 1,
        WB_LB_LS_DIS, 1, WB_LUT_LS_DIS, 1);

    REG_UPDATE!(dwbc10, WB_ENABLE, WB_ENABLE, 1);

    true
}

unsafe fn dwb1_disable(dwbc: *mut dwbc) -> bool {
    let dwbc10 = TO_DCN10_DWBC!(dwbc);

    /* disable CNV */
    REG_UPDATE!(dwbc10, CNV_MODE, CNV_FRAME_CAPTURE_EN, 0);

    /* disable WB */
    REG_UPDATE!(dwbc10, WB_ENABLE, WB_ENABLE, 0);

    /* soft reset */
    REG_UPDATE!(dwbc10, WB_SOFT_RESET, WB_SOFT_RESET, 1);
    REG_UPDATE!(dwbc10, WB_SOFT_RESET, WB_SOFT_RESET, 0);

    /* enable power gating */
    REG_UPDATE_5!(dwbc10, WB_EC_CONFIG, DISPCLK_R_WB_GATE_DIS, 0,
        DISPCLK_G_WB_GATE_DIS, 0, DISPCLK_G_WBSCL_GATE_DIS, 0,
        WB_LB_LS_DIS, 0, WB_LUT_LS_DIS, 0);

    true
}

const dcn10_dwbc_funcs: dwbc_funcs = dwbc_funcs {
    get_caps: Some(dwb1_get_caps),
    enable: Some(dwb1_enable),
    disable: Some(dwb1_disable),
    update: None,
    set_stereo: None,
    set_new_content: None,
    set_warmup: None,
    dwb_set_scaler: None,
};

unsafe fn dcn10_dwbc_construct(
    dwbc10: *mut dcn10_dwbc,
    ctx: *mut dc_context,
    dwbc_regs: *const dcn10_dwbc_registers,
    dwbc_shift: *const dcn10_dwbc_shift,
    dwbc_mask: *const dcn10_dwbc_mask,
    inst: i32,
) {
    (*dwbc10).base.ctx = ctx;

    (*dwbc10).base.inst = inst;
    (*dwbc10).base.funcs = &dcn10_dwbc_funcs;

    (*dwbc10).dwbc_regs = dwbc_regs;
    (*dwbc10).dwbc_shift = dwbc_shift;
    (*dwbc10).dwbc_mask = dwbc_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
