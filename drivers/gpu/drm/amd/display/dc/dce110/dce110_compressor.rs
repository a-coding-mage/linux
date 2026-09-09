/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding translated driver.

#[repr(C)]
struct Dce110CompressorRegOffsets {
    dcp_offset: u32,
    dmif_offset: u32,
}

static REG_OFFSETS: [Dce110CompressorRegOffsets; 3] = [
    Dce110CompressorRegOffsets { dcp_offset: mmDCP0_GRPH_CONTROL - mmDCP0_GRPH_CONTROL, dmif_offset: mmDMIF_PG0_DPG_PIPE_DPM_CONTROL - mmDMIF_PG0_DPG_PIPE_DPM_CONTROL },
    Dce110CompressorRegOffsets { dcp_offset: mmDCP1_GRPH_CONTROL - mmDCP0_GRPH_CONTROL, dmif_offset: mmDMIF_PG1_DPG_PIPE_DPM_CONTROL - mmDMIF_PG0_DPG_PIPE_DPM_CONTROL },
    Dce110CompressorRegOffsets { dcp_offset: mmDCP2_GRPH_CONTROL - mmDCP0_GRPH_CONTROL, dmif_offset: mmDMIF_PG2_DPG_PIPE_DPM_CONTROL - mmDMIF_PG0_DPG_PIPE_DPM_CONTROL },
];

#[inline]
fn align_to_chunks_number_per_line(pixels: u32) -> u32 { 256 * ((pixels + 255) / 256) }

unsafe fn reset_lb_on_vblank(compressor: *mut compressor, crtc_inst: u32) {
    let mut value: u32;
    let mut frame_count: u32;
    let status_pos: u32;
    let mut retry: u32 = 0;
    let cp110 = TO_DCE110_COMPRESSOR(compressor);
    (*cp110).offsets = REG_OFFSETS[crtc_inst as usize];
    status_pos = dm_read_reg((*compressor).ctx, DCP_REG!(mmCRTC_STATUS_POSITION, (*cp110).offsets));
    if status_pos != dm_read_reg((*compressor).ctx, DCP_REG!(mmCRTC_STATUS_POSITION, (*cp110).offsets)) {
        value = dm_read_reg((*compressor).ctx, DCP_REG!(mmLB_SYNC_RESET_SEL, (*cp110).offsets));
        set_reg_field_value!(value, 3, LB_SYNC_RESET_SEL, LB_SYNC_RESET_SEL);
        set_reg_field_value!(value, 1, LB_SYNC_RESET_SEL, LB_SYNC_RESET_SEL2);
        dm_write_reg((*compressor).ctx, DCP_REG!(mmLB_SYNC_RESET_SEL, (*cp110).offsets), value);
        frame_count = dm_read_reg((*compressor).ctx, DCP_REG!(mmCRTC_STATUS_FRAME_COUNT, (*cp110).offsets));
        retry = 10000;
        while retry > 0 {
            if frame_count != dm_read_reg((*compressor).ctx, DCP_REG!(mmCRTC_STATUS_FRAME_COUNT, (*cp110).offsets)) { break; }
            udelay(10);
            retry -= 1;
        }
        if retry == 0 { dm_error!("Frame count did not increase for 100ms.\n"); }
        value = dm_read_reg((*compressor).ctx, DCP_REG!(mmLB_SYNC_RESET_SEL, (*cp110).offsets));
        set_reg_field_value!(value, 2, LB_SYNC_RESET_SEL, LB_SYNC_RESET_SEL);
        set_reg_field_value!(value, 0, LB_SYNC_RESET_SEL, LB_SYNC_RESET_SEL2);
        dm_write_reg((*compressor).ctx, DCP_REG!(mmLB_SYNC_RESET_SEL, (*cp110).offsets), value);
    }
}

unsafe fn wait_for_fbc_state_changed(cp110: *mut dce110_compressor, enabled: bool) {
    let mut counter: u32 = 0;
    let addr = mmFBC_STATUS;
    let mut value: u32;
    while counter < 1000 {
        value = dm_read_reg((*cp110).base.ctx, addr);
        if get_reg_field_value!(value, FBC_STATUS, FBC_ENABLE_STATUS) == enabled { break; }
        udelay(100); counter += 1;
    }
    if counter == 1000 { DC_LOG_WARNING!((*cp110).base.ctx, "%s: wait counter exceeded, changes to HW not applied", __func__); }
    else { DC_LOG_SYNC!((*cp110).base.ctx, "FBC status changed to %d", enabled); }
}

pub unsafe fn dce110_compressor_power_up_fbc(compressor: *mut compressor) {
    let mut value: u32;
    let mut addr = mmFBC_CNTL;
    value = dm_read_reg((*compressor).ctx, addr);
    set_reg_field_value!(value, 0, FBC_CNTL, FBC_GRPH_COMP_EN);
    set_reg_field_value!(value, 1, FBC_CNTL, FBC_EN);
    set_reg_field_value!(value, 2, FBC_CNTL, FBC_COHERENCY_MODE);
    if (*compressor).options.bits.CLK_GATING_DISABLED == 1 { set_reg_field_value!(value, 0, FBC_CNTL, FBC_COMP_CLK_GATE_EN); }
    dm_write_reg((*compressor).ctx, addr, value);
    addr = mmFBC_COMP_MODE; value = dm_read_reg((*compressor).ctx, addr);
    set_reg_field_value!(value, 1, FBC_COMP_MODE, FBC_RLE_EN); set_reg_field_value!(value, 1, FBC_COMP_MODE, FBC_DPCM4_RGB_EN); set_reg_field_value!(value, 1, FBC_COMP_MODE, FBC_IND_EN); dm_write_reg((*compressor).ctx, addr, value);
    addr = mmFBC_COMP_CNTL; value = dm_read_reg((*compressor).ctx, addr); set_reg_field_value!(value, 1, FBC_COMP_CNTL, FBC_DEPTH_RGB08_EN); dm_write_reg((*compressor).ctx, addr, value);
    set_reg_field_value!(value, 0xF, FBC_COMP_CNTL, FBC_MIN_COMPRESSION); dm_write_reg((*compressor).ctx, addr, value); (*compressor).min_compress_ratio = FBC_COMPRESS_RATIO_1TO1;
    dm_write_reg((*compressor).ctx, mmFBC_IND_LUT0, 0); dm_write_reg((*compressor).ctx, mmFBC_IND_LUT1, 0xFFFFFF);
}

pub unsafe fn dce110_compressor_enable_fbc(compressor: *mut compressor, params: *mut compr_addr_and_pitch_params) {
    let cp110 = TO_DCE110_COMPRESSOR(compressor);
    if (*compressor).options.bits.FBC_SUPPORT && !dce110_compressor_is_fbc_enabled_in_hw(compressor, core::ptr::null_mut()) {
        let addr = mmFBC_CNTL; let mut value = dm_read_reg((*compressor).ctx, addr);
        set_reg_field_value!(value, 1, FBC_CNTL, FBC_GRPH_COMP_EN); set_reg_field_value!(value, (*params).inst, FBC_CNTL, FBC_SRC_SEL); dm_write_reg((*compressor).ctx, addr, value);
        (*compressor).is_enabled = true; (*compressor).attached_inst = (*params).inst + CONTROLLER_ID_D0;
        set_reg_field_value!(value, 0, FBC_CNTL, FBC_GRPH_COMP_EN); dm_write_reg((*compressor).ctx, addr, value);
        let mut misc_value = dm_read_reg((*compressor).ctx, mmFBC_MISC);
        set_reg_field_value!(misc_value, 1, FBC_MISC, FBC_INVALIDATE_ON_ERROR); set_reg_field_value!(misc_value, 1, FBC_MISC, FBC_DECOMPRESS_ERROR_CLEAR); set_reg_field_value!(misc_value, 0x14, FBC_MISC, FBC_SLOW_REQ_INTERVAL); dm_write_reg((*compressor).ctx, mmFBC_MISC, misc_value);
        set_reg_field_value!(value, 1, FBC_CNTL, FBC_GRPH_COMP_EN); dm_write_reg((*compressor).ctx, addr, value); wait_for_fbc_state_changed(cp110, true);
    }
}

pub unsafe fn dce110_compressor_disable_fbc(compressor: *mut compressor) {
    let cp110 = TO_DCE110_COMPRESSOR(compressor); let mut crtc_inst = 0;
    if (*compressor).options.bits.FBC_SUPPORT {
        if dce110_compressor_is_fbc_enabled_in_hw(compressor, &mut crtc_inst) { let mut reg_data = dm_read_reg((*compressor).ctx, mmFBC_CNTL); set_reg_field_value!(reg_data, 0, FBC_CNTL, FBC_GRPH_COMP_EN); dm_write_reg((*compressor).ctx, mmFBC_CNTL, reg_data); (*compressor).attached_inst = 0; (*compressor).is_enabled = false; wait_for_fbc_state_changed(cp110, false); }
        if crtc_inst > CONTROLLER_ID_UNDEFINED && crtc_inst < CONTROLLER_ID_D3 { reset_lb_on_vblank(compressor, crtc_inst - CONTROLLER_ID_D0); }
    }
}

pub unsafe fn dce110_compressor_is_fbc_enabled_in_hw(compressor: *mut compressor, inst: *mut u32) -> bool {
    let mut value = dm_read_reg((*compressor).ctx, mmFBC_STATUS);
    if get_reg_field_value!(value, FBC_STATUS, FBC_ENABLE_STATUS) { if !inst.is_null() { *inst = (*compressor).attached_inst; } return true; }
    value = dm_read_reg((*compressor).ctx, mmFBC_MISC);
    if get_reg_field_value!(value, FBC_MISC, FBC_STOP_ON_HFLIP_EVENT) { value = dm_read_reg((*compressor).ctx, mmFBC_CNTL); if get_reg_field_value!(value, FBC_CNTL, FBC_GRPH_COMP_EN) { if !inst.is_null() { *inst = (*compressor).attached_inst; } return true; } }
    false
}

pub unsafe fn dce110_compressor_program_compressed_surface_address_and_pitch(compressor: *mut compressor, params: *mut compr_addr_and_pitch_params) {
    let cp110 = TO_DCE110_COMPRESSOR(compressor); let mut value = 0; let mut fbc_pitch; let compressed_surf_address_low_part = (*compressor).compr_surface_address.addr.low_part;
    (*cp110).offsets = REG_OFFSETS[(*params).inst as usize];
    dm_write_reg((*compressor).ctx, DCP_REG!(mmGRPH_COMPRESS_SURFACE_ADDRESS_HIGH, (*cp110).offsets), 0); dm_write_reg((*compressor).ctx, DCP_REG!(mmGRPH_COMPRESS_SURFACE_ADDRESS, (*cp110).offsets), 0);
    dm_write_reg((*compressor).ctx, DCP_REG!(mmGRPH_COMPRESS_SURFACE_ADDRESS_HIGH, (*cp110).offsets), (*compressor).compr_surface_address.addr.high_part); dm_write_reg((*compressor).ctx, DCP_REG!(mmGRPH_COMPRESS_SURFACE_ADDRESS, (*cp110).offsets), compressed_surf_address_low_part);
    fbc_pitch = align_to_chunks_number_per_line((*params).source_view_width);
    if (*compressor).min_compress_ratio == FBC_COMPRESS_RATIO_1TO1 { fbc_pitch /= 8; } else { DC_LOG_WARNING!((*cp110).base.ctx, "%s: Unexpected DCE11 compression ratio", __func__); }
    dm_write_reg((*compressor).ctx, DCP_REG!(mmGRPH_COMPRESS_PITCH, (*cp110).offsets), 0); set_reg_field_value!(value, fbc_pitch, GRPH_COMPRESS_PITCH, GRPH_COMPRESS_PITCH); dm_write_reg((*compressor).ctx, DCP_REG!(mmGRPH_COMPRESS_PITCH, (*cp110).offsets), value);
}

pub unsafe fn dce110_compressor_set_fbc_invalidation_triggers(compressor: *mut compressor, fbc_trigger: u32) {
    let addr = mmFBC_CLIENT_REGION_MASK; let mut value = dm_read_reg((*compressor).ctx, addr); set_reg_field_value!(value, 0, FBC_CLIENT_REGION_MASK, FBC_MEMORY_REGION_MASK); dm_write_reg((*compressor).ctx, addr, value);
    let addr = mmFBC_IDLE_FORCE_CLEAR_MASK; let mut value = dm_read_reg((*compressor).ctx, addr); set_reg_field_value!(value, fbc_trigger, FBC_IDLE_FORCE_CLEAR_MASK, FBC_IDLE_FORCE_CLEAR_MASK); dm_write_reg((*compressor).ctx, addr, value);
}

pub unsafe fn dce110_compressor_create(ctx: *mut dc_context) -> *mut compressor { let cp110 = kzalloc_obj::<dce110_compressor>(); if cp110.is_null() { return core::ptr::null_mut(); } dce110_compressor_construct(cp110, ctx); &mut (*cp110).base }
pub unsafe fn dce110_compressor_destroy(compressor: *mut *mut compressor) { kfree(TO_DCE110_COMPRESSOR(*compressor)); *compressor = core::ptr::null_mut(); }

static DCE110_COMPRESSOR_FUNCS: compressor_funcs = compressor_funcs {
    power_up_fbc: Some(dce110_compressor_power_up_fbc), enable_fbc: Some(dce110_compressor_enable_fbc), disable_fbc: Some(dce110_compressor_disable_fbc), set_fbc_invalidation_triggers: Some(dce110_compressor_set_fbc_invalidation_triggers), surface_address_and_pitch: Some(dce110_compressor_program_compressed_surface_address_and_pitch), is_fbc_enabled_in_hw: Some(dce110_compressor_is_fbc_enabled_in_hw),
};

pub unsafe fn dce110_compressor_construct(compressor: *mut dce110_compressor, ctx: *mut dc_context) {
    (*compressor).base.options.raw = 0; (*compressor).base.options.bits.FBC_SUPPORT = true; (*compressor).base.lpt_channels_num = 1; (*compressor).base.options.bits.DUMMY_BACKEND = false; (*compressor).base.options.bits.CLK_GATING_DISABLED = false; (*compressor).base.ctx = ctx;
    (*compressor).base.embedded_panel_h_size = 0; (*compressor).base.embedded_panel_v_size = 0; (*compressor).base.memory_bus_width = (*ctx).asic_id.vram_width; (*compressor).base.allocated_size = 0; (*compressor).base.preferred_requested_size = 0; (*compressor).base.min_compress_ratio = FBC_COMPRESS_RATIO_INVALID; (*compressor).base.banks_num = 0; (*compressor).base.raw_size = 0; (*compressor).base.channel_interleave_size = 0; (*compressor).base.dram_channels_num = 0; (*compressor).base.lpt_channels_num = 0; (*compressor).base.attached_inst = CONTROLLER_ID_UNDEFINED; (*compressor).base.is_enabled = false; (*compressor).base.funcs = &DCE110_COMPRESSOR_FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
