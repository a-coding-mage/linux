// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.
//
// C dependencies: dml2_pmo_factory.h, dml2_pmo_dcn3.h

unsafe fn sort(list_a: *mut f64, list_a_size: i32) {
    for i in 0..(list_a_size - 1) {
        for j in i..(list_a_size - 1) {
            if *list_a.add(j as usize) > *list_a.add((j + 1) as usize) {
                let t = *list_a.add(j as usize);
                *list_a.add(j as usize) = *list_a.add((j + 1) as usize);
                *list_a.add((j + 1) as usize) = t;
            }
        }
    }
}

unsafe fn get_max_reserved_time_on_all_planes_with_stream_index(
    config: *mut display_configuation_with_meta, stream_index: u32) -> f64 {
    let mut max_reserved_time_ns: i64 = 0;
    for i in 0..(*config).display_config.num_planes {
        let p = &(*config).display_config.plane_descriptors[i as usize];
        if p.stream_index == stream_index && p.overrides.reserved_vblank_time_ns > max_reserved_time_ns {
            max_reserved_time_ns = p.overrides.reserved_vblank_time_ns;
        }
    }
    max_reserved_time_ns as f64 / 1000.0
}

unsafe fn set_reserved_time_on_all_planes_with_stream_index(
    config: *mut display_configuation_with_meta, stream_index: u32, reserved_time_us: f64) {
    for i in 0..(*config).display_config.num_planes {
        let p = &mut (*config).display_config.plane_descriptors[i as usize];
        if p.stream_index == stream_index {
            p.overrides.reserved_vblank_time_ns = (reserved_time_us * 1000.0) as i64;
        }
    }
}

unsafe fn remove_duplicates(list_a: *mut f64, list_a_size: *mut i32) {
    if *list_a_size == 0 { return; }
    let mut j = 0;
    for i in 1..*list_a_size {
        if *list_a.add(j as usize) != *list_a.add(i as usize) {
            j += 1;
            *list_a.add(j as usize) = *list_a.add(i as usize);
        }
    }
    *list_a_size = j + 1;
}

unsafe fn increase_mpc_combine_factor(mpc_combine_factor: *mut u32, limit: u32) -> bool {
    if *mpc_combine_factor < limit { *mpc_combine_factor += 1; true } else { false }
}

unsafe fn optimize_dcc_mcache_no_odm(in_out: *mut dml2_pmo_optimize_dcc_mcache_in_out, free_pipes: i32) -> bool {
    let pmo = (*in_out).instance;
    let mut result = true;
    let mut free_pipes = free_pipes;
    for i in 0..(*in_out).optimized_display_cfg.num_planes {
        if !(*in_out).dcc_mcache_supported[i as usize] {
            let plane = &mut (*in_out).optimized_display_cfg.plane_descriptors[i as usize];
            if (*in_out).cfg_support_info.stream_support_info[plane.stream_index as usize].odms_used == 1 {
                plane.overrides.mpcc_combine_factor = (*in_out).cfg_support_info.plane_support_info[i as usize].dpps_used;
                if free_pipes > 0 {
                    if !increase_mpc_combine_factor(&mut plane.overrides.mpcc_combine_factor, (*pmo).mpc_combine_limit) {
                        result = false; break;
                    }
                    free_pipes -= 1;
                } else { result = false; break; }
            } else { result = false; break; }
        }
    }
    result
}

unsafe fn iterate_to_next_candidiate(pmo: *mut dml2_pmo_instance, size: i32) -> bool {
    let c = &mut (*pmo).scratch.pmo_dcn3;
    if c.current_candidate[0] > 0 { c.current_candidate[0] -= 1; return true; }
    let mut borrow_from = 1;
    while borrow_from < size && c.current_candidate[borrow_from as usize] == 0 { borrow_from += 1; }
    if borrow_from < size {
        c.current_candidate[borrow_from as usize] -= 1;
        for i in 0..borrow_from { c.current_candidate[i as usize] = c.reserved_time_candidates_count[i as usize] - 1; }
        true
    } else { false }
}

unsafe fn increase_odm_combine_factor(odm_mode: *mut dml2_odm_mode, odms_calculated: i32) -> bool {
    let mut result = true;
    if *odm_mode == dml2_odm_mode_auto {
        *odm_mode = match odms_calculated {
            1 => dml2_odm_mode_bypass, 2 => dml2_odm_mode_combine_2to1,
            3 => dml2_odm_mode_combine_3to1, 4 => dml2_odm_mode_combine_4to1,
            _ => { result = false; dml2_odm_mode_auto }
        };
    }
    if result {
        *odm_mode = match *odm_mode {
            dml2_odm_mode_bypass => dml2_odm_mode_combine_2to1,
            dml2_odm_mode_combine_2to1 => dml2_odm_mode_combine_3to1,
            dml2_odm_mode_combine_3to1 => dml2_odm_mode_combine_4to1,
            _ => { result = false; *odm_mode }
        };
    }
    result
}

unsafe fn count_planes_with_stream_index(display_cfg: *const dml2_display_cfg, stream_index: u32) -> i32 {
    let mut count = 0;
    for i in 0..(*display_cfg).num_planes { if (*display_cfg).plane_descriptors[i as usize].stream_index == stream_index { count += 1; } }
    count
}

unsafe fn are_timings_trivially_synchronizable(display_config: *mut display_configuation_with_meta, mask: i32) -> bool {
    let mut remap = [0u32; DML2_MAX_PLANES as usize];
    let mut n = 0usize;
    for i in 0..(*display_config).display_config.num_streams { if mask & (0x1 << i) != 0 { remap[n] = i as u32; n += 1; } }
    if n <= 1 { return true; }
    let mut identical = true;
    for i in 1..n {
        if memcmp(&(*display_config).display_config.stream_descriptors[remap[i-1] as usize].timing,
                  &(*display_config).display_config.stream_descriptors[remap[i] as usize].timing,
                  core::mem::size_of::<dml2_timing_cfg>()) != 0 { identical = false; break; }
    }
    let mut contains_drr = false;
    for i in 0..n { if (*display_config).display_config.stream_descriptors[remap[i] as usize].timing.drr_config.enabled { contains_drr = true; break; } }
    !contains_drr && identical
}

pub unsafe fn pmo_dcn3_initialize(in_out: *mut dml2_pmo_initialize_in_out) -> bool {
    let pmo = (*in_out).instance;
    (*pmo).soc_bb = (*in_out).soc_bb; (*pmo).ip_caps = (*in_out).ip_caps;
    (*pmo).mpc_combine_limit = 2; (*pmo).odm_combine_limit = 4;
    (*pmo).mcg_clock_table_size = (*in_out).mcg_clock_table_size; (*pmo).options = (*in_out).options; true
}

unsafe fn is_h_timing_divisible_by(timing: *const dml2_timing_cfg, denominator: u8) -> bool {
    let h_blank_start = (*timing).h_total - (*timing).h_front_porch;
    (*timing).h_total % denominator as u32 == 0 && h_blank_start % denominator as u32 == 0 &&
        (*timing).h_blank_end % denominator as u32 == 0 && (*timing).h_sync_width % denominator as u32 == 0
}

unsafe fn is_dp_encoder(encoder_type: dml2_output_encoder_class) -> bool {
    matches!(encoder_type, dml2_dp | dml2_edp | dml2_dp2p0 | dml2_none)
}

pub unsafe fn pmo_dcn3_init_for_vmin(in_out: *mut dml2_pmo_init_for_vmin_in_out) -> bool {
    let dc = &(*in_out).base_display_config.display_config;
    let ms = &(*in_out).base_display_config.mode_support_result;
    if (*in_out).instance.options.disable_dyn_odm || ((*in_out).instance.options.disable_dyn_odm_for_multi_stream && dc.num_streams > 1) { return false; }
    for i in 0..dc.num_planes { if ms.cfg_support_info.plane_support_info[i as usize].dpps_used > 1 && ms.cfg_support_info.stream_support_info[dc.plane_descriptors[i as usize].stream_index as usize].odms_used == 1 { (*in_out).base_display_config.stage4.unoptimizable_streams[dc.plane_descriptors[i as usize].stream_index as usize] = true; } }
    for i in 0..dc.num_streams {
        let s = &dc.stream_descriptors[i as usize];
        if s.overrides.disable_dynamic_odm || ((*in_out).base_display_config.stage3.stream_svp_meta[i as usize].valid && (*in_out).instance.options.disable_dyn_odm_for_stream_with_svp) || !is_h_timing_divisible_by(&s.timing, 2) || !is_dp_encoder(s.output.output_encoder) { (*in_out).base_display_config.stage4.unoptimizable_streams[i as usize] = true; }
    }
    true
}

pub unsafe fn pmo_dcn3_test_for_vmin(in_out: *mut dml2_pmo_test_for_vmin_in_out) -> bool {
    !((*in_out).vmin_limits.dispclk_khz > 0 && (*in_out).display_config.mode_support_result.global.dispclk_khz > (*in_out).vmin_limits.dispclk_khz)
}

unsafe fn find_highest_odm_load_stream_index(dc: *const dml2_display_cfg, ms: *const dml2_core_mode_support_result) -> i32 {
    let mut highest = -1; let mut index = -1;
    for i in 0..(*dc).num_streams { let used = (*ms).cfg_support_info.stream_support_info[i as usize].odms_used; let load = if used > 0 { (*dc).stream_descriptors[i as usize].timing.pixel_clock_khz / used } else { 0 }; if load > highest { highest = load; index = i; } }
    index
}

pub unsafe fn pmo_dcn3_optimize_for_vmin(in_out: *mut dml2_pmo_optimize_for_vmin_in_out) -> bool {
    let dc = &(*in_out).base_display_config.display_config; let ms = &(*in_out).base_display_config.mode_support_result;
    let stream_index = find_highest_odm_load_stream_index(dc, ms);
    if stream_index < 0 || (*in_out).base_display_config.stage4.unoptimizable_streams[stream_index as usize] { return false; }
    let odms = ms.cfg_support_info.stream_support_info[stream_index as usize].odms_used;
    if odms as i32 >= (*in_out).instance.odm_combine_limit { return false; }
    core::ptr::copy_nonoverlapping((*in_out).base_display_config, (*in_out).optimized_display_config, 1);
    let s = &mut (*in_out).optimized_display_config.display_config.stream_descriptors[stream_index as usize];
    let mut optimizable = false;
    while !optimizable && increase_odm_combine_factor(&mut s.overrides.odm_mode, odms as i32) {
        match s.overrides.odm_mode { dml2_odm_mode_combine_2to1 => optimizable = true,
            dml2_odm_mode_combine_3to1 => if is_h_timing_divisible_by(&dc.stream_descriptors[stream_index as usize].timing, 4) && (!ms.cfg_support_info.stream_support_info[stream_index as usize].dsc_enable || ms.cfg_support_info.stream_support_info[stream_index as usize].num_dsc_slices % 3 == 0) { optimizable = true; },
            dml2_odm_mode_combine_4to1 => if is_h_timing_divisible_by(&dc.stream_descriptors[stream_index as usize].timing, 4) && (!ms.cfg_support_info.stream_support_info[stream_index as usize].dsc_enable || ms.cfg_support_info.stream_support_info[stream_index as usize].num_dsc_slices % 4 == 0) { optimizable = true; }, _ => {} }
    }
    optimizable
}

pub unsafe fn pmo_dcn3_optimize_dcc_mcache(in_out: *mut dml2_pmo_optimize_dcc_mcache_in_out) -> bool {
    let pmo = (*in_out).instance;
    if (*in_out).display_config != (*in_out).optimized_display_cfg { core::ptr::copy_nonoverlapping((*in_out).display_config, (*in_out).optimized_display_cfg, 1); }
    let mut used = 0; for i in 0..(*in_out).optimized_display_cfg.num_planes { used += (*in_out).cfg_support_info.plane_support_info[i as usize].dpps_used; }
    let free = (*pmo).ip_caps.pipe_count - used;
    if (*in_out).optimized_display_cfg.num_streams > 1 { return optimize_dcc_mcache_no_odm(in_out, free as i32); }
    if (*in_out).optimized_display_cfg.num_streams == 1 && (*in_out).cfg_support_info.stream_support_info[0].odms_used > 1 {
        let planes = count_planes_with_stream_index((*in_out).optimized_display_cfg, 0); for i in 0..(*in_out).optimized_display_cfg.num_planes { if !(*in_out).dcc_mcache_supported[i as usize] { if free as i32 >= planes { if !increase_odm_combine_factor(&mut (*in_out).optimized_display_cfg.stream_descriptors[i as usize].overrides.odm_mode, (*in_out).cfg_support_info.plane_support_info[i as usize].dpps_used as i32) { return false; } break; } else { return false; } } } true
    } else { optimize_dcc_mcache_no_odm(in_out, free as i32) }
}

pub unsafe fn pmo_dcn3_init_for_pstate_support(in_out: *mut dml2_pmo_init_for_pstate_support_in_out) -> bool {
    let pmo = (*in_out).instance; let state = &mut (*in_out).base_display_config.stage3; state.performed = true; state.min_clk_index_for_latency = (*in_out).base_display_config.stage1.min_clk_index_for_latency;
    (*pmo).scratch.pmo_dcn3.min_latency_index = state.min_clk_index_for_latency; (*pmo).scratch.pmo_dcn3.max_latency_index = (*pmo).mcg_clock_table_size - 1; (*pmo).scratch.pmo_dcn3.cur_latency_index = state.min_clk_index_for_latency; (*pmo).scratch.pmo_dcn3.stream_mask = 0xF;
    for i in 0..(*in_out).base_display_config.display_config.num_planes { let p = &(*in_out).base_display_config.display_config.plane_descriptors[i as usize]; let s = &(*in_out).base_display_config.display_config.stream_descriptors[p.stream_index as usize]; let sup = &(*in_out).base_display_config.mode_support_result.cfg_support_info.plane_support_info[i as usize]; if sup.active_latency_hiding_us < pmo.soc_bb.power_management_parameters.dram_clk_change_blackout_us && s.overrides.hw.twait_budgeting.uclk_pstate == dml2_twait_budgeting_setting_if_needed || s.overrides.hw.twait_budgeting.uclk_pstate == dml2_twait_budgeting_setting_try { (*pmo).scratch.pmo_dcn3.stream_mask |= 0x1 << p.stream_index; } if sup.active_latency_hiding_us < pmo.soc_bb.power_management_parameters.fclk_change_blackout_us && s.overrides.hw.twait_budgeting.fclk_pstate == dml2_twait_budgeting_setting_if_needed || s.overrides.hw.twait_budgeting.fclk_pstate == dml2_twait_budgeting_setting_try { (*pmo).scratch.pmo_dcn3.stream_mask |= 0x1 << p.stream_index; } if p.overrides.legacy_svp_config != dml2_svp_mode_override_auto { (*pmo).scratch.pmo_dcn3.stream_mask &= !(0x1 << p.stream_index); } }
    for i in 0..(*in_out).base_display_config.display_config.num_streams { let min = get_max_reserved_time_on_all_planes_with_stream_index((*in_out).base_display_config, i); (*pmo).scratch.pmo_dcn3.reserved_time_candidates[i as usize][0] = min; (*pmo).scratch.pmo_dcn3.reserved_time_candidates_count[i as usize] = 1; (*pmo).scratch.pmo_dcn3.current_candidate[i as usize] = 0; }
    true
}

pub unsafe fn pmo_dcn3_test_for_pstate_support(in_out: *mut dml2_pmo_test_for_pstate_support_in_out) -> bool { let pmo = (*in_out).instance; for i in 0..(*in_out).base_display_config.display_config.num_planes { let p = &(*in_out).base_display_config.display_config.plane_descriptors[i as usize]; let s = p.stream_index as usize; if p.overrides.reserved_vblank_time_ns < (*pmo).scratch.pmo_dcn3.reserved_time_candidates[s][(*pmo).scratch.pmo_dcn3.current_candidate[s] as usize] * 1000.0 { return false; } } true }

pub unsafe fn pmo_dcn3_optimize_for_pstate_support(in_out: *mut dml2_pmo_optimize_for_pstate_support_in_out) -> bool {
    let pmo = (*in_out).instance; core::ptr::copy_nonoverlapping((*in_out).base_display_config, (*in_out).optimized_display_config, 1); let mut success = false;
    if (*in_out).last_candidate_failed { if (*pmo).scratch.pmo_dcn3.cur_latency_index < (*pmo).scratch.pmo_dcn3.max_latency_index { (*pmo).scratch.pmo_dcn3.cur_latency_index += 1; success = true; } else if iterate_to_next_candidiate(pmo, (*in_out).optimized_display_config.display_config.num_streams as i32) { (*pmo).scratch.pmo_dcn3.cur_latency_index = (*pmo).scratch.pmo_dcn3.min_latency_index; success = true; } } else { success = true; }
    if success { (*in_out).optimized_display_config.stage3.min_clk_index_for_latency = (*pmo).scratch.pmo_dcn3.cur_latency_index; for i in 0..(*in_out).optimized_display_config.display_config.num_streams { let c = (*pmo).scratch.pmo_dcn3.current_candidate[i as usize] as usize; set_reserved_time_on_all_planes_with_stream_index((*in_out).optimized_display_config, i, (*pmo).scratch.pmo_dcn3.reserved_time_candidates[i as usize][c]); } } success
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
