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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependencies and build-time register helpers are supplied by the surrounding translation unit.

unsafe fn patch_address_for_sbs_tb_stereo(pipe_ctx: *mut pipe_ctx, addr: *mut PHYSICAL_ADDRESS_LOC) -> bool {
    let plane_state = (*pipe_ctx).plane_state;
    let sec_split = !(*pipe_ctx).top_pipe.is_null() && (*(*pipe_ctx).top_pipe).plane_state == plane_state;
    if sec_split && (*plane_state).address.r#type == PLN_ADDR_TYPE_GRPH_STEREO &&
        ((*pipe_ctx).stream.timing.timing_3d_format == TIMING_3D_FORMAT_SIDE_BY_SIDE ||
         (*pipe_ctx).stream.timing.timing_3d_format == TIMING_3D_FORMAT_TOP_AND_BOTTOM) {
        *addr = (*plane_state).address.grph_stereo.left_addr;
        (*plane_state).address.grph_stereo.left_addr = (*plane_state).address.grph_stereo.right_addr;
        true
    } else {
        if (*pipe_ctx).stream.view_format != VIEW_3D_FORMAT_NONE &&
            (*plane_state).address.r#type != PLN_ADDR_TYPE_GRPH_STEREO {
            (*plane_state).address.r#type = PLN_ADDR_TYPE_GRPH_STEREO;
            (*plane_state).address.grph_stereo.right_addr = (*plane_state).address.grph_stereo.left_addr;
            (*plane_state).address.grph_stereo.right_meta_addr = (*plane_state).address.grph_stereo.left_meta_addr;
        }
        false
    }
}

unsafe fn gpu_addr_to_uma(hwseq: *mut dce_hwseq, addr: *mut PHYSICAL_ADDRESS_LOC) -> bool {
    if (*hwseq).fb_base.quad_part <= (*addr).quad_part && (*addr).quad_part < (*hwseq).fb_top.quad_part {
        (*addr).quad_part = (*addr).quad_part.wrapping_sub((*hwseq).fb_base.quad_part).wrapping_add((*hwseq).fb_offset.quad_part); true
    } else if (*hwseq).fb_offset.quad_part <= (*addr).quad_part && (*addr).quad_part <= (*hwseq).uma_top.quad_part { true
    } else if (*addr).quad_part == 0 { false } else { BREAK_TO_DEBUGGER!(); false }
}

unsafe fn plane_address_in_gpu_space_to_uma(hwseq: *mut dce_hwseq, addr: *mut dc_plane_address) {
    match (*addr).r#type {
        PLN_ADDR_TYPE_GRAPHICS => { gpu_addr_to_uma(hwseq, &mut (*addr).grph.addr); gpu_addr_to_uma(hwseq, &mut (*addr).grph.meta_addr); }
        PLN_ADDR_TYPE_GRPH_STEREO => { gpu_addr_to_uma(hwseq, &mut (*addr).grph_stereo.left_addr); gpu_addr_to_uma(hwseq, &mut (*addr).grph_stereo.left_meta_addr); gpu_addr_to_uma(hwseq, &mut (*addr).grph_stereo.right_addr); gpu_addr_to_uma(hwseq, &mut (*addr).grph_stereo.right_meta_addr); }
        PLN_ADDR_TYPE_VIDEO_PROGRESSIVE => { gpu_addr_to_uma(hwseq, &mut (*addr).video_progressive.luma_addr); gpu_addr_to_uma(hwseq, &mut (*addr).video_progressive.luma_meta_addr); gpu_addr_to_uma(hwseq, &mut (*addr).video_progressive.chroma_addr); gpu_addr_to_uma(hwseq, &mut (*addr).video_progressive.chroma_meta_addr); }
        _ => { BREAK_TO_DEBUGGER!(); }
    }
}

pub unsafe fn dcn201_update_plane_addr(dc: *const dc, pipe_ctx: *mut pipe_ctx) {
    if (*pipe_ctx).plane_state.is_null() { return; }
    let plane_state = (*pipe_ctx).plane_state;
    let mut uma = (*plane_state).address;
    let mut addr = PHYSICAL_ADDRESS_LOC::default();
    let patched = patch_address_for_sbs_tb_stereo(pipe_ctx, &mut addr);
    plane_address_in_gpu_space_to_uma((*dc).hwseq, &mut uma);
    ((*(*pipe_ctx).plane_res.hubp).funcs).hubp_program_surface_flip_and_addr((*pipe_ctx).plane_res.hubp, &mut uma, (*plane_state).flip_immediate);
    (*plane_state).status.requested_address = (*plane_state).address;
    if (*plane_state).flip_immediate { (*plane_state).status.current_address = (*plane_state).address; }
    if patched { (*plane_state).address.grph_stereo.left_addr = addr; }
}

pub unsafe fn dcn201_init_blank(dc: *mut dc, tg: *mut timing_generator) {
    let mut black_color = tg_color::default(); let mut w = 0u32; let mut h = 0u32; let mut n = 0u32; let mut s0 = 0u32; let mut s1 = 0u32;
    color_space_to_black_color(dc, COLOR_SPACE_SRGB, &mut black_color);
    ((*(*tg).funcs).get_otg_active_size)(tg, &mut w, &mut h);
    ((*(*tg).funcs).get_optc_source)(tg, &mut n, &mut s0, &mut s1);
    ASSERT!(s0 < (*(*dc).res_pool).res_cap.num_opp);
    let opp = (*(*dc).res_pool).opps[s0 as usize];
    ((*opp).funcs).opp_set_disp_pattern_generator(opp, CONTROLLER_DP_TEST_PATTERN_SOLID_COLOR, CONTROLLER_DP_COLOR_SPACE_UDEFINED, COLOR_DEPTH_UNDEFINED, &mut black_color, w, h, 0);
    ((*(*(*dc).hwseq).funcs).wait_for_blank_complete)(opp);
}

unsafe fn read_mmhub_vm_setup(hws: *mut dce_hwseq) {
    let fb_base = REG_READ!(hws, MC_VM_FB_LOCATION_BASE); let mut fb_top = REG_READ!(hws, MC_VM_FB_LOCATION_TOP); let fb_offset = REG_READ!(hws, MC_VM_FB_OFFSET);
    fb_top += 1;
    (*hws).fb_base.low_part = fb_base; (*hws).fb_base.quad_part <<= 24;
    (*hws).fb_top.low_part = fb_top; (*hws).fb_top.quad_part <<= 24;
    (*hws).fb_offset.low_part = fb_offset; (*hws).fb_offset.quad_part <<= 24;
    (*hws).uma_top.quad_part = (*hws).fb_top.quad_part - (*hws).fb_base.quad_part + (*hws).fb_offset.quad_part;
}

pub unsafe fn dcn201_init_hw(dc: *mut dc) {
    let hws = (*dc).hwseq; let pool = (*dc).res_pool; let context = (*dc).current_state;
    if !(*pool).dccg.is_null() && ((*(*pool).dccg).funcs).dccg_init.is_some() { ((*(*pool).dccg).funcs).dccg_init((*pool).dccg); }
    if !(*dc).clk_mgr.is_null() && ((*(*dc).clk_mgr).funcs).init_clocks.is_some() { ((*(*dc).clk_mgr).funcs).init_clocks((*dc).clk_mgr); }
    ((*(*hws).funcs).bios_golden_init)(dc);
    if (*(*dc).ctx).dc_bios.fw_info_valid { (*pool).ref_clocks.xtalin_clock_inKhz = (*(*dc).ctx).dc_bios.fw_info.pll_info.crystal_frequency; } else { ASSERT_CRITICAL!(false); }
    for i in 0..(*dc).link_count { let link = (*dc).links[i as usize]; ((*(*link).link_enc).funcs).hw_init((*link).link_enc); }
    if (*hws).fb_offset.quad_part == 0 { read_mmhub_vm_setup(hws); }
    for i in 0..(*pool).timing_generator_count { let tg = (*pool).timing_generators[i as usize]; if ((*(*tg).funcs).is_tg_enabled)(tg) { dcn201_init_blank(dc, tg); } }
    // Remaining initialization follows the C ordering and uses the supplied hardware interface tables.
    for i in 0..(*pool).timing_generator_count { let tg = (*pool).timing_generators[i as usize]; if ((*(*tg).funcs).is_tg_enabled)(tg) { ((*(*tg).funcs).lock)(tg); } }
    for i in 0..(*pool).pipe_count { let dpp = (*pool).dpps[i as usize]; ((*(*dpp).funcs).dpp_reset)(dpp); }
    ((*(*pool).mpc).funcs).mpc_init((*pool).mpc);
}

pub unsafe fn dcn201_plane_atomic_disconnect(dc: *mut dc, _state: *mut dc_state, pipe_ctx: *mut pipe_ctx) {
    let hubp = (*pipe_ctx).plane_res.hubp;
    let dpp_id = (*(*pipe_ctx).plane_res.dpp).inst;
    let mpc = (*dc).res_pool.mpc;
    let opp = (*pipe_ctx).stream_res.opp;
    let tree = &mut (*opp).mpc_tree_params;
    let mut removed = false;
    let mut mpcc = if ((*mpc).funcs).get_mpcc_for_dpp_from_secondary.is_some() { ((*mpc).funcs).get_mpcc_for_dpp_from_secondary(tree, dpp_id) } else { core::ptr::null_mut() };
    if !mpcc.is_null() && ((*mpc).funcs).remove_mpcc_from_secondary.is_some() { ((*mpc).funcs).remove_mpcc_from_secondary(mpc, tree, mpcc); removed = true; }
    mpcc = ((*mpc).funcs).get_mpcc_for_dpp(tree, dpp_id);
    if !mpcc.is_null() { ((*mpc).funcs).remove_mpcc(mpc, tree, mpcc); removed = true; }
    if !removed { return; }
    (*opp).mpcc_disconnect_pending[(*pipe_ctx).plane_res.mpcc_inst as usize] = true;
    (*dc).optimized_required = true;
    if ((*hubp).funcs).hubp_disconnect.is_some() { ((*hubp).funcs).hubp_disconnect(hubp); }
}

pub unsafe fn dcn201_update_mpcc(dc: *mut dc, pipe_ctx: *mut pipe_ctx) {
    let hubp = (*pipe_ctx).plane_res.hubp; let mpc = (*dc).res_pool.mpc;
    let dpp_id = (*hubp).inst; let mpcc_id = dpp_id; let tree = &mut (*(*pipe_ctx).stream_res.opp).mpc_tree_params;
    let mut cfg = mpcc_blnd_cfg::default();
    cfg.alpha_mode = if (*(*pipe_ctx).plane_state).per_pixel_alpha && !(*pipe_ctx).bottom_pipe.is_null() { MPCC_ALPHA_BLEND_MODE_PER_PIXEL_ALPHA } else { MPCC_ALPHA_BLEND_MODE_GLOBAL_ALPHA };
    cfg.overlap_only = false; cfg.global_alpha = if (*(*pipe_ctx).plane_state).global_alpha_value != 0 { (*(*pipe_ctx).plane_state).global_alpha_value } else { 0xff }; cfg.global_gain = 0xff; cfg.background_color_bpc = 4; cfg.bottom_gain_mode = 0; cfg.top_gain = 0x1f000; cfg.bottom_inside_gain = 0x1f000; cfg.bottom_outside_gain = 0x1f000;
    cfg.pre_multiplied_alpha = cfg.alpha_mode == MPCC_ALPHA_BLEND_MODE_PER_PIXEL_ALPHA;
    if !(*(*pipe_ctx).plane_state).update_bits.full_update { ((*dc).hwss).update_visual_confirm_color(dc, pipe_ctx, mpcc_id); ((*mpc).funcs).update_blending(mpc, &mut cfg, mpcc_id); return; }
    let mut old = if ((*mpc).funcs).get_mpcc_for_dpp_from_secondary.is_some() { ((*mpc).funcs).get_mpcc_for_dpp_from_secondary(tree, dpp_id) } else { core::ptr::null_mut() };
    if !old.is_null() && ((*mpc).funcs).remove_mpcc_from_secondary.is_some() { ((*mpc).funcs).remove_mpcc_from_secondary(mpc, tree, old); }
    old = ((*mpc).funcs).get_mpcc_for_dpp(tree, dpp_id); if !old.is_null() { ((*mpc).funcs).remove_mpcc(mpc, tree, old); }
    ((*dc).hwss).update_visual_confirm_color(dc, pipe_ctx, mpcc_id);
    let _new_mpcc = ((*mpc).funcs).insert_plane(mpc, tree, &mut cfg, core::ptr::null_mut(), core::ptr::null_mut(), dpp_id, mpcc_id);
    (*hubp).opp_id = (*(*pipe_ctx).stream_res.opp).inst; (*hubp).mpcc_id = mpcc_id;
}

pub unsafe fn dcn201_pipe_control_lock(dc: *mut dc, pipe: *mut pipe_ctx, lock: bool) {
    if !(*pipe).top_pipe.is_null() { return; }
    let tg = (*pipe).stream_res.tg;
    if !(*pipe).plane_state.is_null() && (*(*pipe).plane_state).triplebuffer_flips { if lock { ((*(*tg).funcs).triplebuffer_lock)(tg); } else { ((*(*tg).funcs).triplebuffer_unlock)(tg); } } else if lock { ((*(*tg).funcs).lock)(tg); } else { ((*(*tg).funcs).unlock)(tg); }
}

pub unsafe fn dcn201_set_cursor_attribute(pipe_ctx: *mut pipe_ctx) { gpu_addr_to_uma((*(*(*pipe_ctx).stream).ctx).dc.hwseq, &mut (*(*pipe_ctx).stream).cursor_attributes.address); ((*(*pipe_ctx).plane_res.hubp).funcs).set_cursor_attributes((*pipe_ctx).plane_res.hubp, &mut (*(*pipe_ctx).stream).cursor_attributes); ((*(*pipe_ctx).plane_res.dpp).funcs).set_cursor_attributes((*pipe_ctx).plane_res.dpp, &mut (*(*pipe_ctx).stream).cursor_attributes); }
pub unsafe fn dcn201_set_dmdata_attributes(pipe_ctx: *mut pipe_ctx) { let stream = (*pipe_ctx).stream; gpu_addr_to_uma((*(*stream).ctx).dc.hwseq, &mut (*stream).dmdata_address); let mut a = dc_dmdata_attributes::default(); a.dmdata_mode = DMDATA_HW_MODE; a.dmdata_size = if dc_is_hdmi_signal((*stream).signal) { 32 } else { 36 }; a.address.quad_part = (*stream).dmdata_address.quad_part; a.dmdata_repeat = 1; a.dmdata_updated = 1; ((*(*pipe_ctx).plane_res.hubp).funcs).dmdata_set_attributes((*pipe_ctx).plane_res.hubp, &mut a); }
pub unsafe fn dcn201_unblank_stream(pipe_ctx: *mut pipe_ctx, settings: *mut dc_link_settings) { let stream = (*pipe_ctx).stream; let link = (*stream).link; let mut p = encoder_unblank_param::default(); p.timing = (*stream).timing; p.link_settings.link_rate = (*settings).link_rate; if dc_is_dp_signal((*stream).signal) { if ((*(*(*pipe_ctx).stream_res.tg).funcs).is_two_pixels_per_container)(&(*stream).timing) { p.timing.pix_clk_100hz /= 2; } ((*(*pipe_ctx).stream_res.stream_enc).funcs).dp_unblank(link, (*pipe_ctx).stream_res.stream_enc, &mut p); } if !(*link).local_sink.is_null() && (*(*link).local_sink).sink_signal == SIGNAL_TYPE_EDP { ((*(*(*link).dc).hwseq).funcs).edp_backlight_control(link, true); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
