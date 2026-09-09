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

// C dependencies: dm_services.h, dc.h, core_types.h, dce120_hwseq.h,
// dce/dce_hwseq.h, dce100/dce100_hwseq.h, dce110/dce110_hwseq.h,
// dce/dce_12_0_offset.h, dce/dce_12_0_sh_mask.h, soc15_hw_ip.h,
// vega10_ip_offset.h, reg_helper.h

// #define CTX hws->ctx
// #define REG(reg) hws->regs->reg
// #define FN(reg_name, field_name) hws->shifts->field_name, hws->masks->field_name

#[repr(C)]
pub struct dce120_hw_seq_reg_offsets {
    pub crtc: u32,
}

/* The original file keeps the following register-offset implementation under #if 0.
static const struct dce120_hw_seq_reg_offsets reg_offsets[] = { ... };
#define HW_REG_CRTC(reg, id) (reg + reg_offsets[id].crtc)
#define CNTL_ID(controller_id) controller_id
*/

fn dce120_enable_display_power_gating(
    dc: *mut dc,
    controller_id: u8,
    dcb: *mut dc_bios,
    power_gating: pipe_gating_control,
) -> bool {
    let _ = dc;
    let _ = controller_id;
    let _ = dcb;
    let _ = power_gating;
    /* disable for bringup */
    /* The original implementation is disabled with #if 0. */
    false
}

fn dce120_update_dchub(hws: *mut dce_hwseq, dh_data: *mut dchub_init_data) {
    /* TODO: port code from dal2 */
    unsafe {
        match (*dh_data).fb_mode {
            FRAME_BUFFER_MODE_ZFB_ONLY => {
                /* For ZFB case need to put DCHUB FB BASE and TOP upside down to indicate ZFB mode */
                REG_UPDATE_2!(hws, DCHUB_FB_LOCATION, FB_TOP, 0, FB_BASE, 0x0FFFF);
                REG_UPDATE!(hws, DCHUB_AGP_BASE, AGP_BASE,
                    ((*dh_data).zfb_phys_addr_base >> 22) as u32);
                REG_UPDATE!(hws, DCHUB_AGP_BOT, AGP_BOT,
                    ((*dh_data).zfb_mc_base_addr >> 22) as u32);
                REG_UPDATE!(hws, DCHUB_AGP_TOP, AGP_TOP,
                    (((*dh_data).zfb_mc_base_addr + (*dh_data).zfb_size_in_byte - 1) >> 22) as u32);
            }
            FRAME_BUFFER_MODE_MIXED_ZFB_AND_LOCAL => {
                /* Should not touch FB LOCATION (done by VBIOS on AsicInit table) */
                REG_UPDATE!(hws, DCHUB_AGP_BASE, AGP_BASE,
                    ((*dh_data).zfb_phys_addr_base >> 22) as u32);
                REG_UPDATE!(hws, DCHUB_AGP_BOT, AGP_BOT,
                    ((*dh_data).zfb_mc_base_addr >> 22) as u32);
                REG_UPDATE!(hws, DCHUB_AGP_TOP, AGP_TOP,
                    (((*dh_data).zfb_mc_base_addr + (*dh_data).zfb_size_in_byte - 1) >> 22) as u32);
            }
            FRAME_BUFFER_MODE_LOCAL_ONLY => {
                /* Should not touch FB LOCATION (done by VBIOS on AsicInit table) */
                REG_UPDATE!(hws, DCHUB_AGP_BASE, AGP_BASE, 0);
                REG_UPDATE!(hws, DCHUB_AGP_BOT, AGP_BOT, 0x03FFFF);
                REG_UPDATE!(hws, DCHUB_AGP_TOP, AGP_TOP, 0);
            }
            _ => {}
        }

        (*dh_data).dchub_initialzied = true;
        (*dh_data).dchub_info_valid = false;
    }
}

/**
 * dce121_xgmi_enabled() - Check if xGMI is enabled
 * @hws: DCE hardware sequencer object
 *
 * Return true if xGMI is enabled. False otherwise.
 */
pub unsafe fn dce121_xgmi_enabled(hws: *mut dce_hwseq) -> bool {
    let mut pf_max_region: u32 = 0;
    REG_GET!(hws, MC_VM_XGMI_LFB_CNTL, PF_MAX_REGION, &mut pf_max_region);
    /* PF_MAX_REGION == 0 means xgmi is disabled */
    pf_max_region != 0
}

pub unsafe fn dce120_hw_sequencer_construct(dc: *mut dc) {
    /* All registers used by dce11.2 match those in dce11 in offset and
     * structure
     */
    dce110_hw_sequencer_construct(dc);
    (*(*dc).hwseq).funcs.enable_display_power_gating =
        Some(dce120_enable_display_power_gating);
    (*dc).hwss.update_dchub = Some(dce120_update_dchub);
    (*dc).hwss.clear_surface_dcc_and_tiling = Some(dce100_reset_surface_dcc_and_tiling);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
