// SPDX-License-Identifier: MIT
//
// Copyright 2024-2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit are intentionally external.

static BASE_PSTATE_STRATEGY_LIST_1_DISPLAY: [dml2_pmo_pstate_strategy; 3] = [
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_na, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vblank, dml2_pstate_method_na, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_fw_drr, dml2_pstate_method_na, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
];
static BASE_PSTATE_STRATEGY_LIST_1_DISPLAY_SIZE: i32 = 3;

static BASE_PSTATE_STRATEGY_LIST_2_DISPLAY: [dml2_pmo_pstate_strategy; 5] = [
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vblank, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vblank, dml2_pstate_method_vblank, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_fw_drr, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
];
static BASE_PSTATE_STRATEGY_LIST_2_DISPLAY_SIZE: i32 = 5;

static BASE_PSTATE_STRATEGY_LIST_3_DISPLAY: [dml2_pmo_pstate_strategy; 4] = [
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_na], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vblank, dml2_pstate_method_na], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vblank, dml2_pstate_method_vblank, dml2_pstate_method_vblank, dml2_pstate_method_na], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr, dml2_pstate_method_na], allow_state_increase: true },
];
static BASE_PSTATE_STRATEGY_LIST_3_DISPLAY_SIZE: i32 = 4;

static BASE_PSTATE_STRATEGY_LIST_4_DISPLAY: [dml2_pmo_pstate_strategy; 4] = [
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vactive], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vblank], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vblank, dml2_pstate_method_vblank, dml2_pstate_method_vblank, dml2_pstate_method_vblank], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr], allow_state_increase: true },
];
static BASE_PSTATE_STRATEGY_LIST_4_DISPLAY_SIZE: i32 = 4;

unsafe fn dml2_pmo_dcn5_assign_pstate_strategies(pmo: *mut dml2_pmo_instance) {
    let mut i = 0;
    while i <= PMO_DCN4_MAX_DISPLAYS {
        match i {
            1 => { DML_ASSERT!(BASE_PSTATE_STRATEGY_LIST_1_DISPLAY_SIZE <= PMO_DCN4_MAX_BASE_STRATEGIES); pmo_dcn4_fams2_expand_base_pstate_strategies(BASE_PSTATE_STRATEGY_LIST_1_DISPLAY.as_ptr(), BASE_PSTATE_STRATEGY_LIST_1_DISPLAY_SIZE, i, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_1_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[(i - 1) as usize]); }
            2 => { DML_ASSERT!(BASE_PSTATE_STRATEGY_LIST_2_DISPLAY_SIZE <= PMO_DCN4_MAX_BASE_STRATEGIES); pmo_dcn4_fams2_expand_base_pstate_strategies(BASE_PSTATE_STRATEGY_LIST_2_DISPLAY.as_ptr(), BASE_PSTATE_STRATEGY_LIST_2_DISPLAY_SIZE, i, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_2_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[(i - 1) as usize]); }
            3 => { DML_ASSERT!(BASE_PSTATE_STRATEGY_LIST_3_DISPLAY_SIZE <= PMO_DCN4_MAX_BASE_STRATEGIES); pmo_dcn4_fams2_expand_base_pstate_strategies(BASE_PSTATE_STRATEGY_LIST_3_DISPLAY.as_ptr(), BASE_PSTATE_STRATEGY_LIST_3_DISPLAY_SIZE, i, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_3_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[(i - 1) as usize]); }
            4 => { DML_ASSERT!(BASE_PSTATE_STRATEGY_LIST_4_DISPLAY_SIZE <= PMO_DCN4_MAX_BASE_STRATEGIES); pmo_dcn4_fams2_expand_base_pstate_strategies(BASE_PSTATE_STRATEGY_LIST_4_DISPLAY.as_ptr(), BASE_PSTATE_STRATEGY_LIST_4_DISPLAY_SIZE, i, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_4_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[(i - 1) as usize]); }
            _ => {}
        }
        i += 1;
    }
}

pub unsafe fn dml2_pmo_dcn5_initialize(in_out: *mut dml2_pmo_initialize_in_out) -> bool {
    let pmo = (*in_out).instance;
    (*pmo).ip_caps = (*in_out).ip_caps;
    (*pmo).options = (*in_out).options;
    (*pmo).utm_soc_bb = (*in_out).utm_soc_bb;
    (*pmo).mpc_combine_limit = 2;
    (*pmo).odm_combine_limit = 4;
    (*pmo).fams_params.v2.drr.refresh_rate_limit_max = 1000;
    (*pmo).fams_params.v2.drr.refresh_rate_limit_min = 119;
    dml2_pmo_dcn5_assign_pstate_strategies(pmo);
    dml2_pmo_dcn5_stage_optimizer_mcache_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_mcache as usize]);
    dml2_pmo_dcn5_stage_optimizer_uclk_pstate_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_uclk_pstate as usize]);
    dml2_pmo_dcn5_stage_optimizer_qos_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_qos as usize]);
    dml2_pmo_dcn5_stage_optimizer_vmin_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_vmin as usize]);
    dml2_pmo_dcn5_stage_optimizer_stutter_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_stutter as usize]);
    true
}

pub unsafe fn dml2_pmo_dcn5_get_ordered_mandatory_stage_optimizers(pmo: *mut dml2_pmo_instance, stages: *mut *mut dml2_pmo_stage_optimizer) -> i32 {
    let mut count = 0;
    if !(*(*pmo).options).force_optional_mcache_support { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_mcache as usize]; count += 1; }
    if !(*(*pmo).options).force_optional_uclk_pstate_support { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_uclk_pstate as usize]; count += 1; }
    count
}

pub unsafe fn dml2_pmo_dcn5_get_ordered_optional_stages_optimizers(pmo: *mut dml2_pmo_instance, stages: *mut *mut dml2_pmo_stage_optimizer) -> i32 {
    let mut count = 0;
    if (*(*pmo).options).force_optional_mcache_support { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_mcache as usize]; count += 1; }
    if (*(*pmo).options).force_optional_uclk_pstate_support { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_uclk_pstate as usize]; count += 1; }
    *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_qos as usize]; count += 1;
    *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_vmin as usize]; count += 1;
    *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_stutter as usize]; count + 1
}

unsafe fn dml2_pmo_dcn5_assign_timing_groups(worksheet: *mut dml2_optimization_worksheet) {
    (*worksheet).timing_group_count = 0;
    memset((*worksheet).timing_group_ids.as_mut_ptr() as *mut c_void, 0xFF, size_of_val(&(*worksheet).timing_group_ids));
    let mut i = 0u32;
    while i < (*worksheet).orig_dispcfg.num_planes {
        if (*worksheet).timing_group_ids[i as usize] != 0xFFFFFFFF { i += 1; continue; }
        (*worksheet).timing_group_ids[i as usize] = (*worksheet).timing_group_count;
        (*worksheet).timing_group_count += 1;
        let plane = &(*worksheet).orig_dispcfg.plane_descriptors[i as usize];
        let cur_stream = &(*worksheet).orig_dispcfg.stream_descriptors[plane.stream_index as usize];
        let mut j = i + 1;
        while j < (*worksheet).orig_dispcfg.num_planes {
            if (*worksheet).timing_group_ids[j as usize] != 0xFFFFFFFF { j += 1; continue; }
            let plane = &(*worksheet).orig_dispcfg.plane_descriptors[j as usize];
            let other_stream = &(*worksheet).orig_dispcfg.stream_descriptors[plane.stream_index as usize];
            if core::ptr::eq(cur_stream, other_stream) || (memcmp(&cur_stream.timing as *const _ as *const c_void, &other_stream.timing as *const _ as *const c_void, size_of::<dml2_timing_cfg>()) == 0 && !cur_stream.timing.drr_config.enabled) { (*worksheet).timing_group_ids[j as usize] = (*worksheet).timing_group_ids[i as usize]; }
            j += 1;
        }
        i += 1;
    }
}

pub unsafe fn dml2_pmo_dcn5_initialize_worksheet(pmo: *mut dml2_pmo_instance, dispcfg: *const dml2_display_cfg, worksheet: *mut dml2_optimization_worksheet) {
    memset(worksheet as *mut c_void, 0, size_of::<dml2_optimization_worksheet>());
    (*worksheet).orig_dispcfg = dispcfg;
    let sop_table = &(*(*pmo).utm_soc_bb).sop_table;
    (*worksheet).cur.config.min_sop_index = (sop_table.get_highest_sop_index)(sop_table);
    (*worksheet).cur.unvalidated_change.raw = 0xFFFF;
    dml2_pmo_dcn5_assign_timing_groups(worksheet);
}

unsafe fn dml2_pmo_dcn5_check_total_pipe_usage(pmo: *mut dml2_pmo_instance, worksheet: *const dml2_optimization_worksheet) -> bool {
    let mut total_pipe_usage = 0u32;
    for i in 0..(*(*worksheet).orig_dispcfg).num_planes as usize {
        total_pipe_usage += if (*worksheet).cur.config.mpc_combine_overrides[i] != 0 { (*worksheet).cur.config.mpc_combine_overrides[i] } else if (*worksheet).cur.config.odm_combine_overrides[(*(*worksheet).orig_dispcfg).plane_descriptors[i].stream_index as usize] != 0 { (*worksheet).cur.config.odm_combine_overrides[(*(*worksheet).orig_dispcfg).plane_descriptors[i].stream_index as usize] } else { (*worksheet).validation_result.mode_support.cfg_support_info.plane_support_info[i].dpps_used };
    }
    total_pipe_usage <= (*(*pmo).ip_caps).pipe_count
}

unsafe fn is_h_timing_divisible_by(timing: *const dml2_timing_cfg, denominator: u32) -> bool {
    let h_blank_start = (*timing).h_total - (*timing).h_front_porch;
    (*timing).h_total % denominator == 0 && h_blank_start % denominator == 0 && (*timing).h_blank_end % denominator == 0 && (*timing).h_sync_width % denominator == 0
}

unsafe fn dml2_pmo_dcn5_check_odm_divisibility(worksheet: *const dml2_optimization_worksheet) -> bool {
    for i in 0..(*(*worksheet).orig_dispcfg).num_streams as usize {
        let override_value = (*worksheet).cur.config.odm_combine_overrides[i];
        if override_value != 0 {
            let timing = &(*(*worksheet).orig_dispcfg).stream_descriptors[i].timing;
            if !is_h_timing_divisible_by(timing, override_value) { (*(worksheet as *mut dml2_optimization_worksheet)).mcache.per_plane_status[i] = false; return false; }
            if timing.dsc.overrides.num_slices != 0 && timing.dsc.overrides.num_slices % override_value != 0 { return false; }
        }
    }
    true
}

pub unsafe fn dml2_pmo_dcn5_sanity_check(pmo: *mut dml2_pmo_instance, worksheet: *const dml2_optimization_worksheet) -> dml2_status {
    if !dml2_pmo_dcn5_check_total_pipe_usage(pmo, worksheet) { return DML2_STATUS_VALIDATE_FAIL_PMO_SANITY_TOTAL_PIPE_USAGE; }
    if !dml2_pmo_dcn5_check_odm_divisibility(worksheet) { return DML2_STATUS_VALIDATE_FAIL_PMO_SANITY_ODM_DIVISIBILITY; }
    DML2_STATUS_OK
}

unsafe fn dml2_pmo_dcn5_apply_optimization_to_solution(pmo: *mut dml2_pmo_instance, optimization: *const dml2_optimization_config, solution: *mut dml2_display_solution) {
    (*solution).unvalidated_change.raw = (*optimization).unvalidated_change.raw;
    let sop_table = &(*(*pmo).utm_soc_bb).sop_table;
    (sop_table.get_sop_constraint_at_index)(sop_table, (*optimization).config.min_sop_index, &mut (*solution).sop_constraint);
    for i in 0..(*solution).dispcfg.num_planes as usize { if (*optimization).config.mpc_combine_overrides[i] != 0 { (*solution).dispcfg.plane_descriptors[i].overrides.mpcc_combine_factor = (*optimization).config.mpc_combine_overrides[i]; } }
    for i in 0..(*solution).dispcfg.num_streams as usize { (*solution).dispcfg.stream_descriptors[i].overrides.odm_mode = match (*optimization).config.odm_combine_overrides[i] { 1 => dml2_odm_mode_bypass, 2 => dml2_odm_mode_combine_2to1, 3 => dml2_odm_mode_combine_3to1, 4 => dml2_odm_mode_combine_4to1, _ => (*solution).dispcfg.stream_descriptors[i].overrides.odm_mode }; }
    for i in 0..(*solution).dispcfg.num_planes as usize { (*solution).dispcfg.plane_descriptors[i].overrides.reserved_vblank_time_ns = math_max2((*solution).dispcfg.plane_descriptors[i].overrides.reserved_vblank_time_ns, (*optimization).config.reserved_vblank_time_ns[i]) as _; if (*optimization).config.mcache_allocations[i].valid { memcpy(&mut (*solution).mcache_allocations[i] as *mut _ as *mut c_void, &(*optimization).config.mcache_allocations[i] as *const _ as *const c_void, size_of::<dml2_mcache_surface_allocation>()); } }
    (*solution).uclk_pstate_params.support = (*optimization).config.uclk_pstate_support;
    for i in 0..(*solution).dispcfg.num_planes as usize { (*solution).uclk_pstate_params.pstate_switch_modes[i] = (*optimization).config.uclk_pstate_switch_modes[i]; memcpy((*solution).dispcfg.plane_descriptors[i].overrides.max_vactive_det_fill_delay_us.as_mut_ptr() as *mut c_void, (*optimization).config.max_vactive_det_fill_delay_us[i].as_ptr() as *const c_void, size_of_val(&(*optimization).config.max_vactive_det_fill_delay_us[i])); }
    (*solution).uclk_pstate_params.fams2_required = (*optimization).config.fams2_required;
    memcpy(&mut (*solution).uclk_pstate_params.stream_pstate_meta as *mut _ as *mut c_void, &(*optimization).config.stream_pstate_meta as *const _ as *const c_void, size_of::<dml2_pstate_meta>() * DML2_MAX_PLANES);
    (*solution).fclk_pstate_support = (*optimization).config.fclk_pstate_support;
    (*solution).stutter_support_in_vblank = (*optimization).config.stutter_support_in_vblank;
    (*solution).z8_stutter_support_in_vblank = (*optimization).config.z8_stutter_support_in_vblank;
    if (*optimization).config.enable_vmin_dcfclk { (*solution).dispcfg.overrides.hw.dcfclk_mhz = (*(*pmo).utm_soc_bb).vmin_limit.dcfclk_khz as f64 / 1000.0; }
}

pub unsafe fn dml2_pmo_dcn5_convert_worksheet_to_solution(pmo: *mut dml2_pmo_instance, worksheet: *const dml2_optimization_worksheet, solution: *mut dml2_display_solution) {
    memset(solution as *mut c_void, 0, size_of::<dml2_display_solution>());
    (*solution).orig_dispcfg = (*worksheet).orig_dispcfg;
    memcpy(&mut (*solution).dispcfg as *mut _ as *mut c_void, (*worksheet).orig_dispcfg as *const _ as *const c_void, size_of_val(&(*solution).dispcfg));
    memcpy(&mut (*solution).validation_result as *mut _ as *mut c_void, &(*worksheet).validation_result as *const _ as *const c_void, size_of::<dml2_validation_result>());
    dml2_pmo_dcn5_apply_optimization_to_solution(pmo, &(*worksheet).cur, solution);
}

pub unsafe fn dml2_pmo_dcn5_clear_pre_validation_states(_pmo: *mut dml2_pmo_instance, worksheet: *mut dml2_optimization_worksheet) {
    (*worksheet).cur.unvalidated_change.raw = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
