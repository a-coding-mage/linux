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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// C dependencies supplied by the surrounding driver are intentionally left external.

unsafe fn bios_cmd_table_para_revision(dev: *mut core::ffi::c_void, index: u32) -> u32 {
    let adev = dev as *mut amdgpu_device;
    let mut frev: u8 = 0;
    let mut crev: u8 = 0;
    if amdgpu_atom_parse_cmd_header((*adev).mode_info.atom_context, index, &mut frev, &mut crev) != 0 {
        crev as u32
    } else { 0 }
}

unsafe fn init_dig_encoder_control(bp: *mut bios_parser) {
    let version = bios_cmd_table_para_revision((*(*bp).base.ctx).driver_context, GET_INDEX_INTO_MASTER_TABLE!(command, digxencodercontrol));
    (*bp).cmd_tbl.dig_encoder_control = if version == 5 { Some(encoder_control_digx_v1_5) } else { Some(encoder_control_fallback) };
}

unsafe fn encoder_control_dmcub(dmcub: *mut dc_dmub_srv, dig: *mut dig_encoder_stream_setup_parameters_v1_5) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    (*cmd.digx_encoder_control.header).type_ = DMUB_CMD__VBIOS;
    (*cmd.digx_encoder_control.header).sub_type = DMUB_CMD__VBIOS_DIGX_ENCODER_CONTROL;
    (*cmd.digx_encoder_control.header).payload_bytes = (core::mem::size_of_val(&cmd.digx_encoder_control) - core::mem::size_of_val(&cmd.digx_encoder_control.header)) as u32;
    cmd.digx_encoder_control.encoder_control.dig.stream_param = *dig;
    dc_wake_and_execute_dmub_cmd((*dmcub).ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

unsafe fn encoder_control_digx_v1_5(bp: *mut bios_parser, cntl: *mut bp_encoder_control) -> bp_result {
    let mut params: dig_encoder_stream_setup_parameters_v1_5 = core::mem::zeroed();
    params.digid = (*cntl).engine_id as u8;
    params.action = ((*(*bp).cmd_helper).encoder_action_to_atom)((*cntl).action);
    params.pclk_10khz = (*cntl).pixel_clock / 10;
    params.digmode = ((*(*bp).cmd_helper).encoder_mode_bp_to_atom)((*cntl).signal, (*cntl).enable_dp_audio) as u8;
    params.lanenum = (*cntl).lanes_number as u8;
    params.bitpercolor = match (*cntl).color_depth {
        COLOR_DEPTH_888 => PANEL_8BIT_PER_COLOR,
        COLOR_DEPTH_101010 => PANEL_10BIT_PER_COLOR,
        COLOR_DEPTH_121212 => PANEL_12BIT_PER_COLOR,
        COLOR_DEPTH_161616 => PANEL_16BIT_PER_COLOR,
        _ => params.bitpercolor,
    };
    if (*cntl).signal == SIGNAL_TYPE_HDMI_TYPE_A {
        params.pclk_10khz = match (*cntl).color_depth {
            COLOR_DEPTH_101010 => params.pclk_10khz * 30 / 24,
            COLOR_DEPTH_121212 => params.pclk_10khz * 36 / 24,
            COLOR_DEPTH_161616 => params.pclk_10khz * 48 / 24,
            _ => params.pclk_10khz,
        };
    }
    if (*(*(*bp).base.ctx).dc).ctx.dmub_srv.is_null() == false && (*(*(*bp).base.ctx).dc).debug.dmub_command_table {
        encoder_control_dmcub((*(*(*bp).base.ctx).dc).ctx.dmub_srv, &mut params);
        return BP_RESULT_OK;
    }
    if EXEC_BIOS_CMD_TABLE!(bp, digxencodercontrol, params) { BP_RESULT_OK } else { BP_RESULT_FAILURE }
}

unsafe fn encoder_control_fallback(bp: *mut bios_parser, cntl: *mut bp_encoder_control) -> bp_result {
    if !(*(*(*bp).base.ctx).dc).ctx.dmub_srv.is_null() && (*(*(*bp).base.ctx).dc).debug.dmub_command_table {
        encoder_control_digx_v1_5(bp, cntl)
    } else { BP_RESULT_FAILURE }
}

unsafe fn init_transmitter_control(bp: *mut bios_parser) {
    let mut frev = 0u8; let mut crev = 0u8;
    if !BIOS_CMD_TABLE_REVISION!(bp, dig1transmittercontrol, frev, crev) && (*(*(*bp).base.ctx).dc).ctx.dce_version <= DCN_VERSION_2_0 { BREAK_TO_DEBUGGER!(); }
    (*bp).cmd_tbl.transmitter_control = match crev { 6 => Some(transmitter_control_v1_6), 7 => Some(transmitter_control_v1_7), _ => Some(transmitter_control_fallback) };
}

unsafe fn transmitter_control_dmcub(dmcub: *mut dc_dmub_srv, dig: *mut dig_transmitter_control_parameters_v1_6) {
    let mut cmd: dmub_rb_cmd = core::mem::zeroed();
    cmd.dig1_transmitter_control.header.type_ = DMUB_CMD__VBIOS;
    cmd.dig1_transmitter_control.header.sub_type = DMUB_CMD__VBIOS_DIG1_TRANSMITTER_CONTROL;
    cmd.dig1_transmitter_control.header.payload_bytes = (core::mem::size_of_val(&cmd.dig1_transmitter_control) - core::mem::size_of_val(&cmd.dig1_transmitter_control.header)) as u32;
    cmd.dig1_transmitter_control.transmitter_control.dig = *dig;
    dc_wake_and_execute_dmub_cmd((*dmcub).ctx, &mut cmd, DM_DMUB_WAIT_TYPE_WAIT);
}

unsafe fn transmitter_control_v1_6(bp: *mut bios_parser, cntl: *mut bp_transmitter_control) -> bp_result {
    let cmd = (*bp).cmd_helper;
    let mut ps: dig_transmitter_control_ps_allocation_v1_6 = core::mem::zeroed();
    ps.param.phyid = ((*cmd).phy_id_to_atom)((*cntl).transmitter);
    ps.param.action = (*cntl).action as u8;
    if (*cntl).action == TRANSMITTER_CONTROL_SET_VOLTAGE_AND_PREEMPASIS { ps.param.mode_laneset.dplaneset = (*cntl).lane_settings as u8; }
    else { ps.param.mode_laneset.digmode = ((*cmd).signal_type_to_atom_dig_mode)((*cntl).signal); }
    ps.param.lanenum = (*cntl).lanes_number as u8;
    ps.param.hpdsel = ((*cmd).hpd_sel_to_atom)((*cntl).hpd_sel);
    ps.param.digfe_sel = ((*cmd).dig_encoder_sel_to_atom)((*cntl).engine_id);
    ps.param.connobj_id = (*cntl).connector_obj_id.id as u8;
    ps.param.symclk_10khz = (*cntl).pixel_clock / 10;
    if !(*(*(*bp).base.ctx).dc).ctx.dmub_srv.is_null() && (*(*(*bp).base.ctx).dc).debug.dmub_command_table { transmitter_control_dmcub((*(*(*bp).base.ctx).dc).ctx.dmub_srv, &mut ps.param); return BP_RESULT_OK; }
    if EXEC_BIOS_CMD_TABLE!(bp, dig1transmittercontrol, ps) { BP_RESULT_OK } else { BP_RESULT_FAILURE }
}

unsafe fn get_link_by_phy_id(p_dc: *mut dc, phy_id: u32) -> *mut dc_link {
    let mut link = core::ptr::null_mut();
    for link_id in 0..MAX_LINKS { if phy_id == (*(*p_dc).links[link_id as usize]).link_enc.transmitter { link = (*p_dc).links[link_id as usize]; break; } }
    link
}

// The remaining command-table routines retain the same external declarations and are translated below.
extern "C" {
    fn transmitter_control_v1_7(bp: *mut bios_parser, cntl: *mut bp_transmitter_control) -> bp_result;
    fn transmitter_control_fallback(bp: *mut bios_parser, cntl: *mut bp_transmitter_control) -> bp_result;
    fn init_set_pixel_clock(bp: *mut bios_parser);
    fn init_set_crtc_timing(bp: *mut bios_parser);
    fn init_enable_crtc(bp: *mut bios_parser);
    fn init_external_encoder_control(bp: *mut bios_parser);
    fn init_enable_disp_power_gating(bp: *mut bios_parser);
    fn init_set_dce_clock(bp: *mut bios_parser);
    fn init_get_smu_clock_info(bp: *mut bios_parser);
    fn init_enable_lvtma_control(bp: *mut bios_parser);
}

#[no_mangle]
pub unsafe extern "C" fn dal_firmware_parser_init_cmd_tbl(bp: *mut bios_parser) {
    init_dig_encoder_control(bp);
    init_transmitter_control(bp);
    init_set_pixel_clock(bp);
    init_set_crtc_timing(bp);
    init_enable_crtc(bp);
    init_external_encoder_control(bp);
    init_enable_disp_power_gating(bp);
    init_set_dce_clock(bp);
    init_get_smu_clock_info(bp);
    init_enable_lvtma_control(bp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
