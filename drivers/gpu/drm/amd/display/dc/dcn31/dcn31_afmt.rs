/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding display implementation:
// dc_bios_types.h, hw_shared.h, dcn30/dcn30_afmt.h, dcn31_afmt.h,
// reg_helper.h, and dc/dc.h.

static DCN31_AFMT_FUNCS: afmt_funcs = afmt_funcs {
    setup_hdmi_audio: afmt3_setup_hdmi_audio,
    se_audio_setup: afmt3_se_audio_setup,
    audio_mute_control: afmt3_audio_mute_control,
    audio_info_immediate_update: afmt3_audio_info_immediate_update,
    setup_dp_audio: afmt3_setup_dp_audio,
    afmt_powerdown: afmt31_powerdown,
    afmt_poweron: afmt31_poweron,
};

pub unsafe fn afmt31_powerdown(afmt: *mut afmt) {
    let afmt31: *mut dcn31_afmt = DCN31_AFMT_FROM_AFMT(afmt);

    if (*(*afmt).ctx).dc.debug.enable_mem_low_power.bits.afmt == false {
        return;
    }

    REG_UPDATE_2!(afmt31, AFMT_MEM_PWR, AFMT_MEM_PWR_DIS, 0, AFMT_MEM_PWR_FORCE, 1);
}

pub unsafe fn afmt31_poweron(afmt: *mut afmt) {
    let afmt31: *mut dcn31_afmt = DCN31_AFMT_FROM_AFMT(afmt);

    if (*(*afmt).ctx).dc.debug.enable_mem_low_power.bits.afmt == false {
        return;
    }

    REG_UPDATE_2!(afmt31, AFMT_MEM_PWR, AFMT_MEM_PWR_DIS, 1, AFMT_MEM_PWR_FORCE, 0);
}

pub unsafe fn afmt31_construct(
    afmt31: *mut dcn31_afmt,
    ctx: *mut dc_context,
    inst: u32,
    afmt_regs: *const dcn31_afmt_registers,
    afmt_shift: *const dcn31_afmt_shift,
    afmt_mask: *const dcn31_afmt_mask,
) {
    (*afmt31).base.ctx = ctx;

    (*afmt31).base.inst = inst;
    (*afmt31).base.funcs = &DCN31_AFMT_FUNCS;

    (*afmt31).regs = afmt_regs;
    (*afmt31).afmt_shift = afmt_shift;
    (*afmt31).afmt_mask = afmt_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
