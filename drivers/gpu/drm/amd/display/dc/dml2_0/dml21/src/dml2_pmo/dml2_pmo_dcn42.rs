// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

/* External declarations are supplied by the translated dependency units. */

/*
 * DCN42 PMO Policy Implementation
 * This implementation provides VBlank-only strategies for 1, 2, 3, and 4 display
 * configurations, ensuring p-state watermark support in the blank period only.
 */

static DCN42_STRATEGY_LIST_1_DISPLAY: [dml2_pmo_pstate_strategy; 1] = [
    dml2_pmo_pstate_strategy {
        per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_na, dml2_pstate_method_na, dml2_pstate_method_na],
        allow_state_increase: true,
    },
];
static DCN42_STRATEGY_LIST_1_DISPLAY_SIZE: i32 = DCN42_STRATEGY_LIST_1_DISPLAY.len() as i32;

static DCN42_STRATEGY_LIST_2_DISPLAY: [dml2_pmo_pstate_strategy; 1] = [
    dml2_pmo_pstate_strategy {
        per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_na, dml2_pstate_method_na],
        allow_state_increase: true,
    },
];
static DCN42_STRATEGY_LIST_2_DISPLAY_SIZE: i32 = DCN42_STRATEGY_LIST_2_DISPLAY.len() as i32;

static DCN42_STRATEGY_LIST_3_DISPLAY: [dml2_pmo_pstate_strategy; 1] = [
    dml2_pmo_pstate_strategy {
        per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_na],
        allow_state_increase: true,
    },
];
static DCN42_STRATEGY_LIST_3_DISPLAY_SIZE: i32 = DCN42_STRATEGY_LIST_3_DISPLAY.len() as i32;

static DCN42_STRATEGY_LIST_4_DISPLAY: [dml2_pmo_pstate_strategy; 1] = [
    dml2_pmo_pstate_strategy {
        per_stream_pstate_method: [dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vactive, dml2_pstate_method_vactive],
        allow_state_increase: true,
    },
];
static DCN42_STRATEGY_LIST_4_DISPLAY_SIZE: i32 = DCN42_STRATEGY_LIST_4_DISPLAY.len() as i32;

unsafe fn is_bit_set_in_bitfield(bit_field: u32, bit_offset: u32) -> bool {
    (bit_field & (0x1u32 << bit_offset)) != 0
}

unsafe fn dcn42_set_bit_in_bitfield(bit_field: *mut u32, bit_offset: u32) {
    *bit_field |= 0x1u32 << bit_offset;
}

unsafe fn setup_planes_for_vactive_by_mask(
    display_config: *mut display_configuation_with_meta,
    pmo: *mut dml2_pmo_instance,
    plane_mask: i32,
) {
    let mut plane_index: u32 = 0;
    while plane_index < (*display_config).display_config.num_planes {
        if is_bit_set_in_bitfield(plane_mask as u32, plane_index) {
            let plane = &mut (*display_config).display_config.plane_descriptors[plane_index as usize];
            plane.overrides.reserved_vblank_time_ns = math_max2(
                (*pmo).soc_bb.power_management_parameters.dram_clk_change_blackout_us * 1000.0,
                plane.overrides.reserved_vblank_time_ns,
            ) as i64;
            (*display_config).stage3.pstate_switch_modes[plane_index as usize] = dml2_pstate_method_vactive;
        }
        plane_index += 1;
    }
}

unsafe fn reset_display_configuration(display_config: *mut display_configuation_with_meta) {
    let mut stream_index: u32 = 0;
    while stream_index < (*display_config).display_config.num_streams {
        (*display_config).stage3.stream_svp_meta[stream_index as usize].valid = false;
        stream_index += 1;
    }
    let mut plane_index: u32 = 0;
    while plane_index < (*display_config).display_config.num_planes {
        let plane = &mut (*display_config).display_config.plane_descriptors[plane_index as usize];
        plane.overrides.legacy_svp_config = dml2_svp_mode_override_auto;
        plane.overrides.reserved_vblank_time_ns = 0;
        plane.overrides.uclk_pstate_change_strategy = dml2_uclk_pstate_change_strategy_auto;
        (*display_config).stage3.pstate_switch_modes[plane_index as usize] = dml2_pstate_method_na;
        plane_index += 1;
    }
}

unsafe fn setup_display_config(display_config: *mut display_configuation_with_meta, pmo: *mut dml2_pmo_instance, strategy_index: i32) -> bool {
    let scratch = &mut (*pmo).scratch;
    let mut success = true;
    reset_display_configuration(display_config);
    let mut stream_index: u32 = 0;
    while stream_index < (*display_config).display_config.num_streams {
        let method = scratch.pmo_dcn4.pstate_strategy_candidates[strategy_index as usize].per_stream_pstate_method[stream_index as usize];
        if method == dml2_pstate_method_na {
            success = false;
            break;
        } else if method == dml2_pstate_method_vactive {
            setup_planes_for_vactive_by_mask(display_config, pmo, scratch.pmo_dcn4.stream_plane_mask[stream_index as usize]);
        }
        stream_index += 1;
    }
    success
}

pub unsafe fn pmo_dcn42_init_for_pstate_support(in_out: *mut dml2_pmo_init_for_pstate_support_in_out) -> bool {
    let pmo = (*in_out).instance;
    let state = &mut (*(*in_out).base_display_config).stage3;
    let s = &mut (*pmo).scratch;
    let display_config = (*in_out).base_display_config;
    state.performed = true;
    (*display_config).stage3.min_clk_index_for_latency = (*display_config).stage1.min_clk_index_for_latency;
    (*display_config).display_config.overrides.enable_subvp_implicit_pmo = true;
    core::ptr::write_bytes(s, 0, 1);
    if (*display_config).display_config.num_streams == 0 { return false; }
    s.pmo_dcn4.min_latency_index = (*display_config).stage1.min_clk_index_for_latency;
    s.pmo_dcn4.max_latency_index = (*pmo).mcg_clock_table_size;
    s.pmo_dcn4.cur_latency_index = (*display_config).stage1.min_clk_index_for_latency;
    let mut plane_index = 0u32;
    let mut build_override_strategy = true;
    let mut override_base_strategy: dml2_pmo_pstate_strategy = core::mem::zeroed();
    while plane_index < (*display_config).display_config.num_planes {
        let plane_descriptor = &(*display_config).display_config.plane_descriptors[plane_index as usize];
        dcn42_set_bit_in_bitfield(&mut s.pmo_dcn4.stream_plane_mask[plane_descriptor.stream_index as usize], plane_index);
        state.pstate_switch_modes[plane_index as usize] = dml2_pstate_method_vactive;
        build_override_strategy &= plane_descriptor.overrides.uclk_pstate_change_strategy != dml2_uclk_pstate_change_strategy_auto;
        override_base_strategy.per_stream_pstate_method[plane_descriptor.stream_index as usize] = dcn4_uclk_pstate_strategy_override_to_pstate_method(plane_descriptor.overrides.uclk_pstate_change_strategy);
        plane_index += 1;
    }
    let mut stream_index = 0u32;
    while stream_index < (*display_config).display_config.num_streams {
        if dcn4_get_vactive_pstate_margin(display_config, s.pmo_dcn4.stream_plane_mask[stream_index as usize]) >= 0 { dcn42_set_bit_in_bitfield(&mut s.pmo_dcn4.stream_vactive_capability_mask, stream_index); }
        stream_index += 1;
    }
    let (strategy_list, strategy_list_size) = if build_override_strategy {
        override_base_strategy.allow_state_increase = true;
        s.pmo_dcn4.num_expanded_override_strategies = 0;
        dcn4_insert_strategy_into_expanded_list(&override_base_strategy, (*display_config).display_config.num_streams, s.pmo_dcn4.expanded_override_strategy_list.as_mut_ptr(), &mut s.pmo_dcn4.num_expanded_override_strategies);
        dcn4_expand_variant_strategy(&override_base_strategy, (*display_config).display_config.num_streams, false, s.pmo_dcn4.expanded_override_strategy_list.as_mut_ptr(), &mut s.pmo_dcn4.num_expanded_override_strategies);
        (s.pmo_dcn4.expanded_override_strategy_list.as_ptr(), s.pmo_dcn4.num_expanded_override_strategies)
    } else {
        (dcn4_get_expanded_strategy_list(&(*pmo).init_data, (*display_config).display_config.num_streams), dcn4_get_num_expanded_strategies(&(*pmo).init_data, (*display_config).display_config.num_streams))
    };
    if strategy_list.is_null() || strategy_list_size == 0 { return false; }
    s.pmo_dcn4.num_pstate_candidates = 0;
    let mut i = 0u32;
    while i < strategy_list_size && s.pmo_dcn4.num_pstate_candidates < DML2_PMO_PSTATE_CANDIDATE_LIST_SIZE {
        dcn4_insert_into_candidate_list(&*strategy_list.add(i as usize), (*display_config).display_config.num_streams, s);
        i += 1;
    }
    if s.pmo_dcn4.num_pstate_candidates > 0 { s.pmo_dcn4.pstate_strategy_candidates[(s.pmo_dcn4.num_pstate_candidates - 1) as usize].allow_state_increase = true; s.pmo_dcn4.cur_pstate_candidate = -1; true } else { false }
}

pub unsafe fn pmo_dcn42_fams2_optimize_for_pstate_support(in_out: *mut dml2_pmo_optimize_for_pstate_support_in_out) -> bool {
    let s = &mut (*(*in_out).instance).scratch;
    core::ptr::copy_nonoverlapping((*in_out).base_display_config, (*in_out).optimized_display_config, 1);
    let mut success = false;
    if (*in_out).last_candidate_failed && s.pmo_dcn4.pstate_strategy_candidates[s.pmo_dcn4.cur_pstate_candidate as usize].allow_state_increase && s.pmo_dcn4.cur_latency_index < s.pmo_dcn4.max_latency_index - 1 { s.pmo_dcn4.cur_latency_index += 1; success = true; }
    if !success { s.pmo_dcn4.cur_latency_index = s.pmo_dcn4.min_latency_index; s.pmo_dcn4.cur_pstate_candidate += 1; if s.pmo_dcn4.cur_pstate_candidate < s.pmo_dcn4.num_pstate_candidates { success = true; } }
    if success { (*(*in_out).optimized_display_config).stage3.min_clk_index_for_latency = s.pmo_dcn4.cur_latency_index; setup_display_config((*in_out).optimized_display_config, (*in_out).instance, s.pmo_dcn4.cur_pstate_candidate); }
    success
}

pub unsafe fn pmo_dcn42_test_for_pstate_support(in_out: *mut dml2_pmo_test_for_pstate_support_in_out) -> bool {
    if (*(*in_out).instance).scratch.pmo_dcn4.cur_pstate_candidate < 0 { return false; }
    true
}

pub unsafe fn pmo_dcn42_initialize(in_out: *mut dml2_pmo_initialize_in_out) -> bool {
    let pmo = (*in_out).instance;
    (*pmo).soc_bb = (*in_out).soc_bb;
    (*pmo).ip_caps = (*in_out).ip_caps;
    (*pmo).mpc_combine_limit = 2;
    (*pmo).odm_combine_limit = 4;
    (*pmo).mcg_clock_table_size = (*in_out).mcg_clock_table_size;
    (*pmo).fams_params.v2.subvp.refresh_rate_limit_max = 0;
    (*pmo).fams_params.v2.subvp.refresh_rate_limit_min = 0;
    (*pmo).fams_params.v2.drr.refresh_rate_limit_max = 0;
    (*pmo).fams_params.v2.drr.refresh_rate_limit_min = 0;
    (*pmo).options = (*in_out).options;
    let mut i = 0usize;
    while i < PMO_DCN4_MAX_DISPLAYS as usize {
        let (base_list, base_list_size, expanded_list, expanded_list_size) = match i + 1 {
            1 => (if !(*pmo).options.override_strategy_lists[i].is_null() && (*pmo).options.num_override_strategies_per_list[i] != 0 { (*pmo).options.override_strategy_lists[i] } else { DCN42_STRATEGY_LIST_1_DISPLAY.as_ptr() }, if !(*pmo).options.override_strategy_lists[i].is_null() && (*pmo).options.num_override_strategies_per_list[i] != 0 { (*pmo).options.num_override_strategies_per_list[i] } else { DCN42_STRATEGY_LIST_1_DISPLAY_SIZE as u32 }, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_1_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[i]),
            2 => (if !(*pmo).options.override_strategy_lists[i].is_null() && (*pmo).options.num_override_strategies_per_list[i] != 0 { (*pmo).options.override_strategy_lists[i] } else { DCN42_STRATEGY_LIST_2_DISPLAY.as_ptr() }, if !(*pmo).options.override_strategy_lists[i].is_null() && (*pmo).options.num_override_strategies_per_list[i] != 0 { (*pmo).options.num_override_strategies_per_list[i] } else { DCN42_STRATEGY_LIST_2_DISPLAY_SIZE as u32 }, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_2_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[i]),
            3 => (if !(*pmo).options.override_strategy_lists[i].is_null() && (*pmo).options.num_override_strategies_per_list[i] != 0 { (*pmo).options.override_strategy_lists[i] } else { DCN42_STRATEGY_LIST_3_DISPLAY.as_ptr() }, if !(*pmo).options.override_strategy_lists[i].is_null() && (*pmo).options.num_override_strategies_per_list[i] != 0 { (*pmo).options.num_override_strategies_per_list[i] } else { DCN42_STRATEGY_LIST_3_DISPLAY_SIZE as u32 }, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_3_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[i]),
            _ => (if !(*pmo).options.override_strategy_lists[i].is_null() && (*pmo).options.num_override_strategies_per_list[i] != 0 { (*pmo).options.override_strategy_lists[i] } else { DCN42_STRATEGY_LIST_4_DISPLAY.as_ptr() }, if !(*pmo).options.override_strategy_lists[i].is_null() && (*pmo).options.num_override_strategies_per_list[i] != 0 { (*pmo).options.num_override_strategies_per_list[i] } else { DCN42_STRATEGY_LIST_4_DISPLAY_SIZE as u32 }, (*pmo).init_data.pmo_dcn4.expanded_strategy_list_4_display.as_mut_ptr(), &mut (*pmo).init_data.pmo_dcn4.num_expanded_strategies_per_list[i]),
        };
        dml_assert(base_list_size <= PMO_DCN4_MAX_BASE_STRATEGIES);
        pmo_dcn4_fams2_expand_base_pstate_strategies(base_list, base_list_size, (i + 1) as u32, expanded_list, expanded_list_size);
        i += 1;
    }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
