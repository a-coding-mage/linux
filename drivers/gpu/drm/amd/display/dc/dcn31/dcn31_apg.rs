/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
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

// C dependencies are supplied by the surrounding translation unit.

macro_rules! DC_LOGGER {
    ($apg31:expr) => { unsafe { (*(*$apg31).base.ctx).logger } };
}

macro_rules! REG {
    ($apg31:expr, $reg:ident) => { unsafe { (*$apg31).regs.as_ref().unwrap().$reg } };
}

macro_rules! FN {
    ($apg31:expr, $reg_name:ident, $field_name:ident) => {
        (unsafe { (*$apg31).apg_shift.as_ref().unwrap().$field_name },
         unsafe { (*$apg31).apg_mask.as_ref().unwrap().$field_name })
    };
}

macro_rules! CTX {
    ($apg31:expr) => { unsafe { (*$apg31).base.ctx } };
}

unsafe fn apg31_enable(apg: *mut apg) {
    let apg31: *mut dcn31_apg = DCN31_APG_FROM_APG!(apg);

    /* Reset APG */
    REG_UPDATE!(apg31, APG_CONTROL, APG_RESET, 1);
    REG_WAIT!(apg31,
        APG_CONTROL,
        APG_RESET_DONE, 1,
        1, 10);
    REG_UPDATE!(apg31, APG_CONTROL, APG_RESET, 0);
    REG_WAIT!(apg31,
        APG_CONTROL,
        APG_RESET_DONE, 0,
        1, 10);

    /* Enable APG */
    REG_UPDATE!(apg31, APG_CONTROL2, APG_ENABLE, 1);
}

unsafe fn apg31_disable(apg: *mut apg) {
    let apg31: *mut dcn31_apg = DCN31_APG_FROM_APG!(apg);

    /* Disable APG */
    REG_UPDATE!(apg31, APG_CONTROL2, APG_ENABLE, 0);
}

unsafe fn apg31_se_audio_setup(
    apg: *mut apg,
    az_inst: ::core::ffi::c_uint,
    audio_info: *mut audio_info,
) {
    let _ = az_inst;
    let apg31: *mut dcn31_apg = DCN31_APG_FROM_APG!(apg);

    ASSERT!(audio_info);
    /* This should not happen.it does so we don't get BSOD*/
    if audio_info.is_null() {
        return;
    }

    /* DisplayPort only allows for one audio stream with stream ID 0 */
    REG_UPDATE!(apg31, APG_CONTROL2, APG_DP_AUDIO_STREAM_ID, 0);

    /* When running in "pair mode", pairs of audio channels have their own enable
     * this is for really old audio drivers */
    REG_UPDATE!(apg31, APG_DBG_GEN_CONTROL, APG_DBG_AUDIO_CHANNEL_ENABLE, 0xFF);

    /* Disable forced mem power off */
    REG_UPDATE!(apg31, APG_MEM_PWR, APG_MEM_PWR_FORCE, 0);
}

static mut dcn31_apg_funcs: apg_funcs = apg_funcs {
    se_audio_setup: Some(apg31_se_audio_setup),
    enable_apg: Some(apg31_enable),
    disable_apg: Some(apg31_disable),
};

unsafe fn apg31_construct(
    apg31: *mut dcn31_apg,
    ctx: *mut dc_context,
    inst: u32,
    apg_regs: *const dcn31_apg_registers,
    apg_shift: *const dcn31_apg_shift,
    apg_mask: *const dcn31_apg_mask,
) {
    (*apg31).base.ctx = ctx;

    (*apg31).base.inst = inst;
    (*apg31).base.funcs = &raw mut dcn31_apg_funcs;

    (*apg31).regs = apg_regs;
    (*apg31).apg_shift = apg_shift;
    (*apg31).apg_mask = apg_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
