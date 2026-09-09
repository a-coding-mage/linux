// SPDX-License-Identifier: MIT
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

// C headers and symbols supplied by the surrounding kernel/driver are external dependencies.

static PIPE_CRC_SOURCES: [&[u8]; 6] = [b"none\0", b"crtc\0", b"crtc dither\0", b"dprx\0", b"dprx dither\0", b"auto\0"];

unsafe fn dm_parse_crc_source(source: *const i8) -> amdgpu_dm_pipe_crc_source {
    if source.is_null() || strcmp(source, b"none\0".as_ptr() as *const i8) == 0 { return AMDGPU_DM_PIPE_CRC_SOURCE_NONE; }
    if strcmp(source, b"auto\0".as_ptr() as *const i8) == 0 || strcmp(source, b"crtc\0".as_ptr() as *const i8) == 0 { return AMDGPU_DM_PIPE_CRC_SOURCE_CRTC; }
    if strcmp(source, b"dprx\0".as_ptr() as *const i8) == 0 { return AMDGPU_DM_PIPE_CRC_SOURCE_DPRX; }
    if strcmp(source, b"crtc dither\0".as_ptr() as *const i8) == 0 { return AMDGPU_DM_PIPE_CRC_SOURCE_CRTC_DITHER; }
    if strcmp(source, b"dprx dither\0".as_ptr() as *const i8) == 0 { return AMDGPU_DM_PIPE_CRC_SOURCE_DPRX_DITHER; }
    AMDGPU_DM_PIPE_CRC_SOURCE_INVALID
}

fn dm_is_crc_source_crtc(src: amdgpu_dm_pipe_crc_source) -> bool {
    src == AMDGPU_DM_PIPE_CRC_SOURCE_CRTC || src == AMDGPU_DM_PIPE_CRC_SOURCE_CRTC_DITHER
}
fn dm_is_crc_source_dprx(src: amdgpu_dm_pipe_crc_source) -> bool {
    src == AMDGPU_DM_PIPE_CRC_SOURCE_DPRX || src == AMDGPU_DM_PIPE_CRC_SOURCE_DPRX_DITHER
}
fn dm_need_crc_dither(src: amdgpu_dm_pipe_crc_source) -> bool {
    src == AMDGPU_DM_PIPE_CRC_SOURCE_CRTC_DITHER || src == AMDGPU_DM_PIPE_CRC_SOURCE_DPRX_DITHER || src == AMDGPU_DM_PIPE_CRC_SOURCE_NONE
}
fn dm_need_dp_aux(source: amdgpu_dm_pipe_crc_source, cur_crc_src: amdgpu_dm_pipe_crc_source) -> bool {
    dm_is_crc_source_dprx(source) || (source == AMDGPU_DM_PIPE_CRC_SOURCE_NONE && dm_is_crc_source_dprx(cur_crc_src))
}
fn dm_crc_source_should_start_dprx(source: amdgpu_dm_pipe_crc_source, cur_crc_src: amdgpu_dm_pipe_crc_source) -> bool {
    !amdgpu_dm_is_valid_crc_source(cur_crc_src) && amdgpu_dm_is_valid_crc_source(source) && dm_is_crc_source_dprx(source)
}
fn dm_crc_source_should_stop_dprx(source: amdgpu_dm_pipe_crc_source, cur_crc_src: amdgpu_dm_pipe_crc_source) -> bool {
    amdgpu_dm_is_valid_crc_source(cur_crc_src) && !amdgpu_dm_is_valid_crc_source(source) && dm_is_crc_source_dprx(cur_crc_src)
}

unsafe fn amdgpu_dm_crtc_get_crc_sources(_crtc: *mut drm_crtc, count: *mut usize) -> *const *const i8 {
    *count = PIPE_CRC_SOURCES.len();
    PIPE_CRC_SOURCES.as_ptr() as *const *const i8
}

#[cfg(feature = "CONFIG_DRM_AMD_SECURE_DISPLAY")]
unsafe fn update_phy_id_mapping(adev: *mut amdgpu_device) {
    let ddev = adev_to_drm(adev); let dm = &mut (*adev).dm;
    let mut sort_connector: [*mut amdgpu_dm_connector; AMDGPU_DM_MAX_CRTC] = [core::ptr::null_mut(); AMDGPU_DM_MAX_CRTC];
    let mut idx: u8 = 0; let mut connector_cnt: u8 = 0;
    dm.secure_display_ctx.phy_mapping_updated = false;
    mutex_lock(&mut (*ddev).mode_config.mutex);
    let mut iter = core::mem::zeroed(); let mut connector = core::ptr::null_mut();
    drm_connector_list_iter_begin(ddev, &mut iter);
    drm_for_each_connector_iter!(connector, iter, {
        if (*connector).status != connector_status_connected { continue; }
        if idx >= AMDGPU_DM_MAX_CRTC as u8 { drm_warn!(ddev, "connected connectors exceed max crtc\n"); mutex_unlock(&mut (*ddev).mode_config.mutex); return; }
        sort_connector[idx as usize] = to_amdgpu_dm_connector(connector); idx += 1; connector_cnt += 1;
    });
    drm_connector_list_iter_end(&mut iter);
    let mut n = connector_cnt;
    while n > 1 { let mut j = 0; while j + 1 < n { if (*(*sort_connector[j as usize])).dc_link.link_enc_hw_inst > (*(*sort_connector[(j+1) as usize])).dc_link.link_enc_hw_inst { sort_connector.swap(j as usize, (j+1) as usize); } j += 1; } n -= 1; }
    idx = 0;
    while idx < connector_cnt { if (*sort_connector[idx as usize]).mst_root { let root = (*sort_connector[idx as usize]).mst_root; let mut cnt: u8 = 1; while idx + cnt < connector_cnt && (*sort_connector[(idx+cnt) as usize]).mst_root == root { cnt += 1; } let mut i = cnt; while i > 1 { let mut j = idx; while j + i - 2 >= j { let a = sort_connector[j as usize]; let b = sort_connector[(j+1) as usize]; if (*a).mst_output_port->parent->lct > (*b).mst_output_port->parent->lct { sort_connector.swap(j as usize, (j+1) as usize); } j += 1; if j >= idx+i-2 { break; } } i -= 1; } idx += cnt; } else { idx += 1; } }
    memset(dm.secure_display_ctx.phy_id_mapping.as_mut_ptr() as *mut _, 0, core::mem::size_of_val(&dm.secure_display_ctx.phy_id_mapping));
    for i in 0..connector_cnt as usize { let c = sort_connector[i]; let m = &mut dm.secure_display_ctx.phy_id_mapping[i]; m.assigned = true; m.is_mst = false; m.enc_hw_inst = (*c).dc_link.link_enc_hw_inst; if !(*c).mst_root.is_null() { m.is_mst = true; m.lct = (*c).mst_output_port->parent->lct; m.port_num = (*c).mst_output_port->port_num; memcpy(m.rad.as_mut_ptr() as *mut _, (*c).mst_output_port->parent->rad.as_ptr() as *const _, core::mem::size_of_val(&(*c).mst_output_port->parent->rad)); } }
    mutex_unlock(&mut (*ddev).mode_config.mutex); dm.secure_display_ctx.phy_id_mapping_cnt = connector_cnt; dm.secure_display_ctx.phy_mapping_updated = true;
}

unsafe fn get_phy_id(dm: *mut amdgpu_display_manager, aconnector: *mut amdgpu_dm_connector, phy_id: *mut u8) -> bool {
    if !(*dm).secure_display_ctx.phy_mapping_updated { drm_warn!("Should update the phy id table before get it's value"); return false; }
    for idx in 0..(*dm).secure_display_ctx.phy_id_mapping_cnt as usize { let m = &(*dm).secure_display_ctx.phy_id_mapping[idx]; if !m.assigned { drm_error!("phy_id_mapping should be assigned"); return false; } if (*aconnector).dc_link.link_enc_hw_inst == m.enc_hw_inst && (!m.is_mst || ((*aconnector).mst_output_port->parent->lct == m.lct && (*aconnector).mst_output_port->port_num == m.port_num)) { *phy_id = idx as u8; drm_debug_driver!("Associated secure display PHY ID as %d", idx); return true; } }
    drm_warn!("Can't find associated phy ID"); false
}

unsafe fn amdgpu_dm_crtc_verify_crc_source(crtc: *mut drm_crtc, src_name: *const i8, values_cnt: *mut usize) -> i32 { if dm_parse_crc_source(src_name) == AMDGPU_DM_PIPE_CRC_SOURCE_INVALID { drm_debug_driver!("Unknown CRC source for CRTC%d", (*crtc).index); return -EINVAL; } *values_cnt = 3; 0 }

unsafe fn amdgpu_dm_crtc_configure_crc_source(crtc: *mut drm_crtc, dm_crtc_state: *mut dm_crtc_state, source: amdgpu_dm_pipe_crc_source) -> i32 {
    let adev = drm_to_adev((*crtc).dev); let stream_state = (*dm_crtc_state).stream; if stream_state.is_null() { return -EINVAL; } let enable = amdgpu_dm_is_valid_crc_source(source); let mut ret = 0; let mut poly = CRC_POLY_MODE_16;
    mutex_lock(&mut (*adev).dm.dc_lock);
    if enable { amdgpu_dm_psr_set_event(&mut (*adev).dm, stream_state, true, psr_event_crc_window_active, true); amdgpu_dm_replay_set_event(&mut (*adev).dm, stream_state, true, replay_event_crc_window_active, true); }
    if (amdgpu_ip_version(adev, DCE_HWIP, 0) >= IP_VERSION(3,6,0)) && amdgpu_ip_version(adev, DCE_HWIP, 0) != IP_VERSION(4,0,1) { poly = to_amdgpu_crtc(crtc).dm_irq_params.crc_poly_mode; }
    if dm_is_crc_source_crtc(source) || source == AMDGPU_DM_PIPE_CRC_SOURCE_NONE { for i in 0..MAX_CRC_WINDOW_NUM { if !dc_stream_configure_crc((*stream_state).ctx->dc, stream_state, core::ptr::null_mut(), enable, enable, i, true, poly) { ret = -EINVAL; break; } } }
    if !dm_need_crc_dither(source) { dc_stream_set_dither_option(stream_state, DITHER_OPTION_TRUN8); dc_stream_set_dyn_expansion((*stream_state).ctx->dc, stream_state, DYN_EXPANSION_DISABLE); } else { dc_stream_set_dither_option(stream_state, DITHER_OPTION_DEFAULT); dc_stream_set_dyn_expansion((*stream_state).ctx->dc, stream_state, DYN_EXPANSION_AUTO); }
    if !enable { amdgpu_dm_psr_set_event(&mut (*adev).dm, stream_state, false, psr_event_crc_window_active, false); amdgpu_dm_replay_set_event(&mut (*adev).dm, stream_state, false, replay_event_crc_window_active, false); } mutex_unlock(&mut (*adev).dm.dc_lock); ret
}

unsafe fn amdgpu_dm_crtc_handle_crc_irq(crtc: *mut drm_crtc) { if crtc.is_null() || (*crtc).state.is_null() || (*crtc).dev.is_null() { return; } let state = to_dm_crtc_state((*crtc).state); if (*state).stream.is_null() { return; } let a = to_amdgpu_crtc(crtc); let src = a.dm_irq_params.crc_src; if !amdgpu_dm_is_valid_crc_source(src) { return; } if (*state).crc_skip_count < 2 { (*state).crc_skip_count += 1; return; } if dm_is_crc_source_crtc(src) { let mut crcs = [0u32; 3]; if dc_stream_get_crc((*state).stream.ctx->dc, (*state).stream, 0, &mut crcs[0], &mut crcs[1], &mut crcs[2]) { drm_crtc_add_crc_entry(crtc, true, drm_crtc_accurate_vblank_count(crtc), crcs.as_mut_ptr()); } } }

// The remaining secure-display workqueue and CRC-window entry points retain their C ABI and
// are supplied by the surrounding kernel translation; their declarations are intentionally external.
unsafe extern "C" {
    fn amdgpu_dm_crtc_set_crc_source(crtc: *mut drm_crtc, src_name: *const i8) -> i32;
    fn amdgpu_dm_crtc_handle_crc_window_irq(crtc: *mut drm_crtc);
    fn amdgpu_dm_crtc_secure_display_create_contexts(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
