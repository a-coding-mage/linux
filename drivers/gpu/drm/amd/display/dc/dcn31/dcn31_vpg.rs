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

// C dependencies supplied by the surrounding translation unit:
// dc_bios_types.h, dcn30/dcn30_vpg.h, dcn31_vpg.h, reg_helper.h, dc/dc.h

static dcn31_vpg_funcs: vpg_funcs = vpg_funcs {
    update_generic_info_packet: vpg3_update_generic_info_packet,
    vpg_poweron: vpg31_poweron,
    vpg_powerdown: vpg31_powerdown,
};

pub unsafe fn vpg31_powerdown(vpg: *mut vpg) {
    let vpg31: *mut dcn31_vpg = DCN31_VPG_FROM_VPG(vpg);

    if (*(*vpg).ctx).dc.debug.enable_mem_low_power.bits.vpg == false {
        return;
    }

    REG_UPDATE_2!(
        vpg31,
        VPG_MEM_PWR,
        VPG_GSP_MEM_LIGHT_SLEEP_DIS,
        0,
        VPG_GSP_LIGHT_SLEEP_FORCE,
        1
    );
}

pub unsafe fn vpg31_poweron(vpg: *mut vpg) {
    let vpg31: *mut dcn31_vpg = DCN31_VPG_FROM_VPG(vpg);

    let mut vpg_gsp_mem_pwr_state: u32 = 0;

    REG_GET!(
        vpg31,
        VPG_MEM_PWR,
        VPG_GSP_MEM_PWR_STATE,
        &mut vpg_gsp_mem_pwr_state
    );

    if (*(*vpg).ctx).dc.debug.enable_mem_low_power.bits.vpg == false
        && vpg_gsp_mem_pwr_state == 0
    {
        return;
    }

    REG_UPDATE_2!(
        vpg31,
        VPG_MEM_PWR,
        VPG_GSP_MEM_LIGHT_SLEEP_DIS,
        1,
        VPG_GSP_LIGHT_SLEEP_FORCE,
        0
    );
}

pub unsafe fn vpg31_construct(
    vpg31: *mut dcn31_vpg,
    ctx: *mut dc_context,
    inst: u32,
    vpg_regs: *const dcn31_vpg_registers,
    vpg_shift: *const dcn31_vpg_shift,
    vpg_mask: *const dcn31_vpg_mask,
) {
    (*vpg31).base.ctx = ctx;

    (*vpg31).base.inst = inst;
    (*vpg31).base.funcs = &dcn31_vpg_funcs;

    (*vpg31).regs = vpg_regs;
    (*vpg31).vpg_shift = vpg_shift;
    (*vpg31).vpg_mask = vpg_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
