// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies are supplied by the surrounding translated modules.

static BASE_PSTATE_STRATEGY_LIST_1_DISPLAY: [dml2_pmo_pstate_strategy; 4] = [
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_na, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vblank, dml2_pstate_method_na, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_fw_drr, dml2_pstate_method_na, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_alternate, dml2_pstate_method_na, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
];
static BASE_PSTATE_STRATEGY_LIST_2_DISPLAY: [dml2_pmo_pstate_strategy; 6] = [
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vblank, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vblank, dml2_pstate_method_vblank, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_fw_drr, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_alternate, dml2_pstate_method_alternate, dml2_pstate_method_na, dml2_pstate_method_na], allow_state_increase: true },
];
static BASE_PSTATE_STRATEGY_LIST_3_DISPLAY: [dml2_pmo_pstate_strategy; 5] = [
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_na], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vblank, dml2_pstate_method_na], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vblank, dml2_pstate_method_vblank, dml2_pstate_method_vblank, dml2_pstate_method_na], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr, dml2_pstate_method_na], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_alternate, dml2_pstate_method_alternate, dml2_pstate_method_alternate, dml2_pstate_method_na], allow_state_increase: true },
];
static BASE_PSTATE_STRATEGY_LIST_4_DISPLAY: [dml2_pmo_pstate_strategy; 5] = [
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vactive], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vblank], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_vblank, dml2_pstate_method_vblank, dml2_pstate_method_vblank, dml2_pstate_method_vblank], allow_state_increase: false },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr, dml2_pstate_method_fw_drr], allow_state_increase: true },
    dml2_pmo_pstate_strategy { per_stream_pstate_method: [dml2_pstate_method_alternate, dml2_pstate_method_alternate, dml2_pstate_method_alternate, dml2_pstate_method_alternate], allow_state_increase: true },
];

unsafe fn dml2_pmo_dcn6_assign_pstate_strategies(pmo: *mut dml2_pmo_instance) {
    for i in 1..=PMO_DCN4_MAX_DISPLAYS {
        match i {
            1 => { DML_ASSERT(BASE_PSTATE_STRATEGY_LIST_1_DISPLAY.len() <= PMO_DCN4_MAX_BASE_STRATEGIES); pmo_dcn4_fams2_expand_base_pstate_strategies(BASE_PSTATE_STRATEGY_LIST_1_DISPLAY.as_ptr(), BASE_PSTATE_STRATEGY_LIST_1_DISPLAY.len() as i32, i, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_1_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[(i - 1) as usize]); }
            2 => { DML_ASSERT(BASE_PSTATE_STRATEGY_LIST_2_DISPLAY.len() <= PMO_DCN4_MAX_BASE_STRATEGIES); pmo_dcn4_fams2_expand_base_pstate_strategies(BASE_PSTATE_STRATEGY_LIST_2_DISPLAY.as_ptr(), BASE_PSTATE_STRATEGY_LIST_2_DISPLAY.len() as i32, i, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_2_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[(i - 1) as usize]); }
            3 => { DML_ASSERT(BASE_PSTATE_STRATEGY_LIST_3_DISPLAY.len() <= PMO_DCN4_MAX_BASE_STRATEGIES); pmo_dcn4_fams2_expand_base_pstate_strategies(BASE_PSTATE_STRATEGY_LIST_3_DISPLAY.as_ptr(), BASE_PSTATE_STRATEGY_LIST_3_DISPLAY.len() as i32, i, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_3_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[(i - 1) as usize]); }
            4 => { DML_ASSERT(BASE_PSTATE_STRATEGY_LIST_4_DISPLAY.len() <= PMO_DCN4_MAX_BASE_STRATEGIES); pmo_dcn4_fams2_expand_base_pstate_strategies(BASE_PSTATE_STRATEGY_LIST_4_DISPLAY.as_ptr(), BASE_PSTATE_STRATEGY_LIST_4_DISPLAY.len() as i32, i, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_4_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[(i - 1) as usize]); }
            _ => {}
        }
    }
}

pub unsafe fn dml2_pmo_dcn6a_get_ordered_mandatory_stage_optimizers(pmo: *mut dml2_pmo_instance, stages: *mut *mut dml2_pmo_stage_optimizer) -> i32 {
    let mut count = 0;
    DML_LOG_COMP_IF_ENTER();
    if !(*(*pmo).options).force_optional_ppt_temp_read_admissibility { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_fclk_ppt_temp_read_pstate]; count += 1; }
    if !(*(*pmo).options).force_optional_mcache_support { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_mcache]; count += 1; }
    if !(*(*pmo).options).force_optional_uclk_pstate_support { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_uclk_pstate]; count += 1; }
    DML_LOG_DEBUG!("%s exit with %d\n", __func__, count); DML_LOG_COMP_IF_EXIT(); count
}

pub unsafe fn dml2_pmo_dcn6a_get_ordered_optional_stages_optimizers(pmo: *mut dml2_pmo_instance, stages: *mut *mut dml2_pmo_stage_optimizer) -> i32 {
    let mut count = 0; DML_LOG_COMP_IF_ENTER();
    if (*(*pmo).options).force_optional_ppt_temp_read_admissibility { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_fclk_ppt_temp_read_pstate]; count += 1; }
    if (*(*pmo).options).force_optional_mcache_support { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_mcache]; count += 1; }
    if (*(*pmo).options).force_optional_uclk_pstate_support { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_uclk_pstate]; count += 1; }
    for index in [dml2_pmo_stage_index_qos, dml2_pmo_stage_index_vmin, dml2_pmo_stage_index_stutter, dml2_pmo_stage_index_vmin_dcfclk] { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[index]; count += 1; }
    DML_LOG_DEBUG!("%s exit with %d\n", __func__, count); DML_LOG_COMP_IF_EXIT(); count
}

unsafe fn pstate_method_to_uclk_pstate_strategy_override(method: dml2_pstate_method) -> dml2_uclk_pstate_change_strategy {
    match method { dml2_pstate_method_vactive | dml2_pstate_method_fw_vactive_drr => dml2_uclk_pstate_change_strategy_force_vactive, dml2_pstate_method_vblank | dml2_pstate_method_fw_vblank_drr => dml2_uclk_pstate_change_strategy_force_vblank, dml2_pstate_method_fw_drr => dml2_uclk_pstate_change_strategy_force_drr, dml2_pstate_method_alternate => dml2_uclk_pstate_change_strategy_force_alternate, _ => dml2_uclk_pstate_change_strategy_auto }
}

pub unsafe fn dml2_pmo_dcn6b_get_ordered_mandatory_stage_optimizers(pmo: *mut dml2_pmo_instance, stages: *mut *mut dml2_pmo_stage_optimizer) -> i32 {
    let mut count = 0; DML_LOG_COMP_IF_ENTER();
    if !(*(*pmo).options).force_optional_ppt_temp_read_admissibility { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_fclk_ppt_temp_read_pstate]; count += 1; }
    if !(*(*pmo).options).force_optional_mcache_support { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_mcache]; count += 1; }
    DML_LOG_DEBUG!("%s exit with %d\n", __func__, count); DML_LOG_COMP_IF_EXIT(); count
}

pub unsafe fn dml2_pmo_dcn6b_get_ordered_optional_stages_optimizers(pmo: *mut dml2_pmo_instance, stages: *mut *mut dml2_pmo_stage_optimizer) -> i32 {
    let mut count = 0; DML_LOG_COMP_IF_ENTER();
    if (*(*pmo).options).force_optional_ppt_temp_read_admissibility { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_fclk_ppt_temp_read_pstate]; count += 1; }
    if (*(*pmo).options).force_optional_mcache_support { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_mcache]; count += 1; }
    for index in [dml2_pmo_stage_index_uclk_pstate, dml2_pmo_stage_index_qos, dml2_pmo_stage_index_vmin, dml2_pmo_stage_index_stutter, dml2_pmo_stage_index_vmin_dcfclk] { *stages.add(count as usize) = &mut (*pmo).stage_optimizers[index]; count += 1; }
    DML_LOG_DEBUG!("%s exit with %d\n", __func__, count); DML_LOG_COMP_IF_EXIT(); count
}

unsafe fn dml2_pmo_dcn6_apply_optimization_to_solution(pmo: *mut dml2_pmo_instance, optimization: *const dml2_optimization_config, solution: *mut dml2_display_solution) {
    let sop_table = &mut (*(*pmo).utm_soc_bb).sop_table;
    (*solution).unvalidated_change.raw = (*optimization).unvalidated_change.raw;
    sop_table.get_sop_constraint_at_index(sop_table, (*optimization).config.min_sop_index, &mut (*solution).sop_constraint);
    for i in 0..(*solution).dispcfg.num_planes as usize {
        if (*optimization).config.mpc_combine_overrides[i] != 0 { (*solution).dispcfg.plane_descriptors[i].overrides.mpcc_combine_factor = (*optimization).config.mpc_combine_overrides[i]; }
    }
    for i in 0..(*solution).dispcfg.num_streams as usize {
        (*solution).dispcfg.stream_descriptors[i].overrides.odm_mode = match (*optimization).config.odm_combine_overrides[i] { 1 => dml2_odm_mode_bypass, 2 => dml2_odm_mode_combine_2to1, 3 => dml2_odm_mode_combine_3to1, 4 => dml2_odm_mode_combine_4to1, _ => (*solution).dispcfg.stream_descriptors[i].overrides.odm_mode };
    }
    for i in 0..(*solution).dispcfg.num_planes as usize {
        (*solution).dispcfg.plane_descriptors[i].overrides.reserved_vblank_time_ns = math_max2((*solution).dispcfg.plane_descriptors[i].overrides.reserved_vblank_time_ns, (*optimization).config.reserved_vblank_time_ns[i]) as _;
        if (*optimization).config.mcache_allocations[i].valid { (*solution).mcache_allocations[i] = (*optimization).config.mcache_allocations[i]; }
        (*solution).uclk_pstate_params.pstate_switch_modes[i] = (*optimization).config.uclk_pstate_switch_modes[i];
        (*solution).dispcfg.plane_descriptors[i].overrides.uclk_pstate_change_strategy = pstate_method_to_uclk_pstate_strategy_override((*optimization).config.uclk_pstate_switch_modes[i]);
        (*solution).dispcfg.plane_descriptors[i].overrides.max_vactive_det_fill_delay_us = (*optimization).config.max_vactive_det_fill_delay_us[i];
    }
    (*solution).uclk_pstate_params.support = (*optimization).config.uclk_pstate_support;
    (*solution).uclk_pstate_params.fams2_required = (*optimization).config.fams2_required;
    (*solution).uclk_pstate_params.legacy_pstate_info_for_dmu = (*optimization).config.legacy_pstate_info_for_dmu;
    (*solution).uclk_pstate_params.stream_pstate_meta = (*optimization).config.stream_pstate_meta;
    (*solution).fclk_pstate_support = (*optimization).config.fclk_pstate_support;
    (*solution).ppt_temp_read_support = (*optimization).config.ppt_temp_read_support;
    (*solution).stutter_support_in_vblank = (*optimization).config.stutter_support_in_vblank;
    (*solution).z8_stutter_support_in_vblank = (*optimization).config.z8_stutter_support_in_vblank;
    if (*optimization).config.enable_vmin_dcfclk { (*solution).dispcfg.overrides.hw.dcfclk_mhz = (*(*pmo).utm_soc_bb).vmin_limit.dcfclk_khz as f64 / 1000.0; }
}

pub unsafe fn dml2_pmo_dcn6_convert_worksheet_to_solution(pmo: *mut dml2_pmo_instance, worksheet: *const dml2_optimization_worksheet, solution: *mut dml2_display_solution) {
    DML_LOG_COMP_IF_ENTER();
    *solution = core::mem::zeroed();
    (*solution).orig_dispcfg = (*worksheet).orig_dispcfg;
    (*solution).dispcfg = (*worksheet).orig_dispcfg;
    (*solution).timing_group_ids = (*worksheet).timing_group_ids;
    (*solution).timing_group_count = (*worksheet).timing_group_count;
    (*solution).validation_result = (*worksheet).validation_result;
    dml2_pmo_dcn6_apply_optimization_to_solution(pmo, &(*worksheet).cur, solution);
    DML_LOG_COMP_IF_EXIT();
}

pub unsafe fn dml2_pmo_dcn6a_initialize(in_out: *mut dml2_pmo_initialize_in_out) -> bool {
    let pmo = (*in_out).instance;
    DML_LOG_COMP_IF_ENTER(); (*pmo).ip_caps = (*in_out).ip_caps; (*pmo).options = (*in_out).options; (*pmo).utm_soc_bb = (*in_out).utm_soc_bb;
    (*pmo).mpc_combine_limit = 2; (*pmo).odm_combine_limit = 4; (*pmo).fams_params.v2.drr.refresh_rate_limit_max = 1000; (*pmo).fams_params.v2.drr.refresh_rate_limit_min = 119;
    dml2_pmo_dcn6_assign_pstate_strategies(pmo);
    dml2_pmo_dcn6_stage_optimizer_mcache_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_mcache]); dml2_pmo_dcn6_stage_optimizer_uclk_pstate_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_uclk_pstate]); dml2_pmo_dcn5_stage_optimizer_qos_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_qos]); dml2_pmo_dcn5_stage_optimizer_vmin_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_vmin]); dml2_pmo_dcn5_stage_optimizer_stutter_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_stutter]); dml2_pmo_dcn6_stage_optimizer_vmin_dcfclk_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_vmin_dcfclk]); dml2_pmo_dcn6_stage_optimizer_fclk_ppt_temp_read_pstate_create(pmo, &mut (*pmo).stage_optimizers[dml2_pmo_stage_index_fclk_ppt_temp_read_pstate]); DML_LOG_COMP_IF_EXIT(); true
}

pub unsafe fn dml2_pmo_dcn6b_initialize(in_out: *mut dml2_pmo_initialize_in_out) -> bool {
    (*(*in_out).instance).options = (*in_out).options; (*(*in_out).options).disable_alternate_memory_training = true; dml2_pmo_dcn6a_initialize(in_out)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
