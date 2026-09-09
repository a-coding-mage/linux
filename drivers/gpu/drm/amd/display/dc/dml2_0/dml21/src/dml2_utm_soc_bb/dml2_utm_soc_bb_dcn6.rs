// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

const SDP_DERATE_PERCENT_NOMINAL: f64 = 76.0;
const SDP_DERATE_PERCENT_URGENT: f64 = 100.0;

unsafe fn dcn6_sop_table_get_highest_index(table: *const dml2_sop_table) -> c_uint {
    (*(*table).model).sop_count - 1
}

unsafe fn dcn6_sop_table_get_sop_constraint_at_index(
    table: *const dml2_sop_table,
    index: c_uint,
    constraint: *mut dml2_sop_constraint,
) {
    let model = (*table).model;
    let dchub = (*model).dchub_v2;
    DML_ASSERT_MSG(index < (*model).sop_count, "unsupported sop index\n");
    (*constraint).dcn5.clocks.fclk_khz = (*model).sops[index as usize].fclk_khz;
    (*constraint).dcn5.clocks.uclk_khz = (*model).sops[index as usize].uclk_khz;
    (*constraint).dcn5.clocks.dcfclk_khz = (*table).sop_optimal_dcfclks_khz[index as usize];
    (*constraint).dcn5.latency.dcn5.urgent_ramp = (*dchub).latencies[index as usize].urgent_ramp_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.t_trip = (*dchub).latencies[index as usize].t_trip_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.meta_trip_to_mem = (*dchub).latencies[index as usize].meta_trip_to_mem_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.max_req_latency_urg = (*dchub).latencies[index as usize].max_req_latency_urg_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.avg_req_latency_urg = (*dchub).latencies[index as usize].avg_req_latency_urg_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.max_req_latency_non_urg = (*dchub).latencies[index as usize].max_req_latency_non_urg_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.avg_req_latency_non_urg = (*dchub).latencies[index as usize].avg_req_latency_non_urg_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.df_response_time_us = (*dchub).latencies[index as usize].df_response_time_ps / 1000000.0;
    (*constraint).dcn5.min_available_urgent_bandwidth_KBps = (*table).sop_min_available_urgent_bandwidths_KBps[index as usize];
    (*constraint).dcn5.min_sop_index = index;
}

unsafe fn dcn6_sop_table_is_bandwidth_supported_at_index(table: *const dml2_sop_table, bw: *const dml2_memory_path_bandwidth, index: c_uint) -> bool {
    let model = (*table).model;
    let mut qos_bandwidth = utm_qos_model_dchub_memory_path_bandwidth_v2 { nominal_bandwidth_KBps: (*bw).dcn5.non_urgent_bandwidth_kbps as u32, urgent_bandwidth_KBps: (*bw).dcn5.urgent_bandwidth_kbps as u32 };
    let highest_sop_index = (*model).sop_count - 1;
    let mut result = true;
    if !dchub_v2_is_qos_bandwidth_supported_by_sop(model, &mut qos_bandwidth, index as u8, (*(*model).dchub_v2).max_nominal_utm_budget_percent, (*(*model).dchub_v2).max_urgent_utm_budget_percent) { result = false; }
    if !dchub_v2_is_qos_bandwidth_supported_by_sop(model, &mut qos_bandwidth, highest_sop_index as u8, (*(*model).dchub_v2).min_nominal_utm_budget_percent, (*(*model).dchub_v2).min_urgent_utm_budget_percent) { result = false; }
    result
}

unsafe fn dcn6_sop_table_get_max_sop(table: *const dml2_sop_table, sop: *mut dml2_soc_operating_point) {
    let model = (*table).model;
    DML_ASSERT_MSG((*model).sop_count > 0, "utm_qos_model must contain at least 1 sop\n");
    if (*model).sop_count > 0 {
        let i = (*model).sop_count - 1;
        (*sop).fclk_khz = (*model).sops[i as usize].fclk_khz;
        (*sop).uclk_khz = (*model).sops[i as usize].uclk_khz;
        (*sop).dcfclk_khz = (*table).sop_optimal_dcfclks_khz[i as usize];
    }
}

unsafe fn dcn6_sop_table_get_min_sop(table: *const dml2_sop_table, sop: *mut dml2_soc_operating_point) {
    let model = (*table).model;
    (*sop).fclk_khz = (*model).sops[0].fclk_khz;
    (*sop).uclk_khz = (*model).sops[0].uclk_khz;
    (*sop).dcfclk_khz = (*table).sop_optimal_dcfclks_khz[0];
}

unsafe fn dcn6_sop_table_calculate_optimal_dcfclk_khz(table: *const dml2_sop_table, utm_soc_bb: *const dml2_utm_soc_bb, total: *mut utm_qos_model_dchub_memory_path_bandwidth_v2) -> u32 {
    let model = (*table).model;
    let nominal = (*total).nominal_bandwidth_KBps * ((*(*model).dchub_v2).max_nominal_utm_budget_percent / 100.0) / (SDP_DERATE_PERCENT_NOMINAL / 100.0) / (*utm_soc_bb).return_bus_width_bytes;
    let urgent = (*total).urgent_bandwidth_KBps * ((*(*model).dchub_v2).max_urgent_utm_budget_percent / 100.0) / (SDP_DERATE_PERCENT_URGENT / 100.0) / (*utm_soc_bb).return_bus_width_bytes;
    let mut result = math_ceil(math_max2(urgent, nominal)) as u32;
    if result > (*utm_soc_bb).max_dcfclk_khz { result = (*utm_soc_bb).max_dcfclk_khz; } else if result < (*utm_soc_bb).min_dcfclk_khz { result = (*utm_soc_bb).min_dcfclk_khz; }
    result
}

unsafe fn dml2_utm_soc_bb_dcn6_build_sop_table(table: *mut dml2_sop_table, utm_soc_bb: *const dml2_utm_soc_bb) {
    let model = &(*utm_soc_bb).qos_model as *const _;
    (*table).get_highest_sop_index = Some(dcn6_sop_table_get_highest_index);
    (*table).get_sop_constraint_at_index = Some(dcn6_sop_table_get_sop_constraint_at_index);
    (*table).is_bw_supported_at_index = Some(dcn6_sop_table_is_bandwidth_supported_at_index);
    (*table).get_max_sop = Some(dcn6_sop_table_get_max_sop);
    (*table).get_min_sop = Some(dcn6_sop_table_get_min_sop);
    (*table).model = model;
    let mut total = core::mem::zeroed::<utm_qos_model_dchub_memory_path_bandwidth_v2>();
    for i in 0..(*model).sop_count {
        dchub_v2_get_sop_total_available_bandwidth_KBps(model, &mut total, i as u8);
        (*table).sop_optimal_dcfclks_khz[i as usize] = dcn6_sop_table_calculate_optimal_dcfclk_khz(table, utm_soc_bb, &mut total);
        (*table).sop_min_available_urgent_bandwidths_KBps[i as usize] = math_floor(total.urgent_bandwidth_KBps * ((*(*utm_soc_bb).qos_model.dchub_v2).min_urgent_utm_budget_percent / 100.0)) as u32;
    }
    DML_ASSERT_MSG((*model).sop_count > 0, "qos_model must contain at least 1 sop\n");
}

unsafe fn dcn6_v3_sop_table_get_highest_index(table: *const dml2_sop_table) -> c_uint { (*(*table).model).dchub_v3.sop_count - 1 }

unsafe fn dcn6_v3_sop_table_get_sop_constraint_at_index(table: *const dml2_sop_table, index: c_uint, constraint: *mut dml2_sop_constraint) {
    let dchub = (*(*table).model).dchub_v3;
    let entry = &(*dchub).sops[UTM_QOS_MODEL_V3_LOAD_LEVEL_ACTIVE_ALTERNATE_PSTATE as usize][index as usize];
    DML_ASSERT_MSG(index < (*dchub).sop_count, "unsupported sop index\n");
    (*constraint).dcn5.clocks.fclk_khz = 0; (*constraint).dcn5.clocks.uclk_khz = 0;
    (*constraint).dcn5.clocks.dcfclk_khz = (*table).sop_optimal_dcfclks_khz[index as usize];
    (*constraint).dcn5.latency.dcn5.urgent_ramp = entry.urgent_ramp_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.t_trip = entry.t_trip_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.meta_trip_to_mem = entry.meta_trip_to_mem_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.max_req_latency_urg = entry.max_req_latency_urg_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.avg_req_latency_urg = entry.avg_req_latency_urg_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.max_req_latency_non_urg = entry.max_req_latency_non_urg_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.avg_req_latency_non_urg = entry.avg_req_latency_non_urg_ps / 1000000.0;
    (*constraint).dcn5.latency.dcn5.df_response_time_us = entry.df_response_time_ps / 1000000.0;
    (*constraint).dcn5.min_available_urgent_bandwidth_KBps = (*table).sop_min_available_urgent_bandwidths_KBps[index as usize];
    (*constraint).dcn5.min_sop_index = index;
}

unsafe fn dcn6_v3_sop_table_is_bandwidth_supported_at_index(table: *const dml2_sop_table, bw: *const dml2_memory_path_bandwidth, index: c_uint) -> bool {
    let dchub = (*(*table).model).dchub_v3; let highest = (*dchub).sop_count - 1;
    let idle = &(*dchub).sops[UTM_QOS_MODEL_V3_LOAD_LEVEL_IDLE as usize][index as usize];
    let active = &(*dchub).sops[UTM_QOS_MODEL_V3_LOAD_LEVEL_ACTIVE_ALTERNATE_PSTATE as usize][highest as usize];
    if (*bw).dcn5.non_urgent_bandwidth_kbps > idle.nominal_bandwidth_KBps || (*bw).dcn5.urgent_bandwidth_kbps > idle.urgent_bandwidth_KBps { return false; }
    if (*bw).dcn5.non_urgent_bandwidth_kbps > active.nominal_bandwidth_KBps || (*bw).dcn5.urgent_bandwidth_kbps > active.urgent_bandwidth_KBps { return false; }
    true
}

unsafe fn dcn6_v3_sop_table_get_max_sop(table: *const dml2_sop_table, sop: *mut dml2_soc_operating_point) {
    let dchub = (*(*table).model).dchub_v3;
    let utm = (table as *const u8).sub(core::mem::offset_of!(dml2_utm_soc_bb, sop_table)) as *const dml2_utm_soc_bb;
    DML_ASSERT_MSG((*dchub).sop_count > 0, "utm_qos_model must contain at least 1 sop\n");
    (*sop).fclk_khz = (*utm).max_fclk_khz; (*sop).uclk_khz = (*utm).max_uclk_khz; (*sop).dcfclk_khz = (*table).sop_optimal_dcfclks_khz[((*dchub).sop_count - 1) as usize];
}
unsafe fn dcn6_v3_sop_table_get_min_sop(table: *const dml2_sop_table, sop: *mut dml2_soc_operating_point) {
    let utm = (table as *const u8).sub(core::mem::offset_of!(dml2_utm_soc_bb, sop_table)) as *const dml2_utm_soc_bb;
    (*sop).fclk_khz = (*utm).max_fclk_khz; (*sop).uclk_khz = (*utm).max_uclk_khz; (*sop).dcfclk_khz = (*table).sop_optimal_dcfclks_khz[0];
}
unsafe fn dcn6_v3_sop_table_calculate_optimal_dcfclk_khz(utm: *const dml2_utm_soc_bb, idle: *const utm_qos_model_dchub_v3_sop_entry) -> u32 {
    let nominal = (*idle).nominal_bandwidth_KBps / ((*utm).nominal_sdp_derate_percent / 100.0) / (*utm).return_bus_width_bytes;
    let urgent = (*idle).urgent_bandwidth_KBps / ((*utm).urgent_sdp_derate_percent / 100.0) / (*utm).return_bus_width_bytes;
    let mut optimal = math_ceil(math_max2(urgent, nominal)) as u32;
    if optimal > (*utm).max_dcfclk_khz { optimal = (*utm).max_dcfclk_khz; } else if optimal < (*utm).min_dcfclk_khz { optimal = (*utm).min_dcfclk_khz; } optimal
}

unsafe fn dml2_utm_soc_bb_dcn6_v3_build_sop_table(table: *mut dml2_sop_table, utm: *const dml2_utm_soc_bb) {
    let dchub = (*utm).qos_model.dchub_v3;
    (*table).get_highest_sop_index = Some(dcn6_v3_sop_table_get_highest_index); (*table).get_sop_constraint_at_index = Some(dcn6_v3_sop_table_get_sop_constraint_at_index); (*table).is_bw_supported_at_index = Some(dcn6_v3_sop_table_is_bandwidth_supported_at_index); (*table).get_max_sop = Some(dcn6_v3_sop_table_get_max_sop); (*table).get_min_sop = Some(dcn6_v3_sop_table_get_min_sop); (*table).model = &(*utm).qos_model;
    DML_ASSERT_MSG((*dchub).sop_count > 0, "qos_model must contain at least 1 sop\n");
    for i in 0..(*dchub).sop_count { let idle = &(*dchub).sops[UTM_QOS_MODEL_V3_LOAD_LEVEL_IDLE as usize][i as usize]; let active = &(*dchub).sops[UTM_QOS_MODEL_V3_LOAD_LEVEL_ACTIVE_ALTERNATE_PSTATE as usize][i as usize]; (*table).sop_optimal_dcfclks_khz[i as usize] = dcn6_v3_sop_table_calculate_optimal_dcfclk_khz(utm, idle); (*table).sop_min_available_urgent_bandwidths_KBps[i as usize] = active.urgent_bandwidth_KBps; }
}

unsafe fn dcn6_copy_utm_qos_model(dest: *mut utm_qos_model, dest_dchub: *mut utm_qos_model_dchub_v2, src: *const utm_qos_model) { *dest = *src; *dest_dchub = *(*src).dchub_v2; (*dest).dchub_v2 = dest_dchub; }

unsafe fn dcn6_initialize_from_soc_bb(utm: *mut dml2_utm_soc_bb, soc: *const dml2_soc_bb) {
    DML_ASSERT_MSG((*soc).clk_table.dcfclk.num_clk_values == 2, "soc_bb must provide min and max dcfclk values!\n");
    (*utm).max_dispclk_khz = (*soc).clk_table.dispclk.clk_values_khz[((*soc).clk_table.dispclk.num_clk_values - 1) as usize];
    (*utm).max_dppclk_khz = (*soc).clk_table.dppclk.clk_values_khz[((*soc).clk_table.dppclk.num_clk_values - 1) as usize];
    (*utm).max_dtbclk_khz = if (*soc).clk_table.dtbclk.num_clk_values > 0 { (*soc).clk_table.dtbclk.clk_values_khz[((*soc).clk_table.dtbclk.num_clk_values - 1) as usize] } else { 0 };
    (*utm).max_phyclk_khz = if (*soc).clk_table.phyclk.num_clk_values > 0 { (*soc).clk_table.phyclk.clk_values_khz[((*soc).clk_table.phyclk.num_clk_values - 1) as usize] } else { 0 };
    (*utm).max_dscclk_khz = if (*soc).clk_table.dscclk.num_clk_values > 0 { (*soc).clk_table.dscclk.clk_values_khz[((*soc).clk_table.dscclk.num_clk_values - 1) as usize] } else { 0 };
    (*utm).max_phyclk_d18_khz = if (*soc).clk_table.phyclk_d18.num_clk_values > 0 { (*soc).clk_table.phyclk_d18.clk_values_khz[((*soc).clk_table.phyclk_d18.num_clk_values - 1) as usize] } else { 0 };
    (*utm).max_phyclk_d32_khz = if (*soc).clk_table.phyclk_d32.num_clk_values > 0 { (*soc).clk_table.phyclk_d32.clk_values_khz[((*soc).clk_table.phyclk_d32.num_clk_values - 1) as usize] } else { 0 };
    (*utm).min_socclk_khz = (*soc).clk_table.socclk.clk_values_khz[0]; (*utm).max_dcfclk_khz = (*soc).clk_table.dcfclk.clk_values_khz[((*soc).clk_table.dcfclk.num_clk_values - 1) as usize]; (*utm).min_dcfclk_khz = (*soc).clk_table.dcfclk.clk_values_khz[0];
    (*utm).max_uclk_khz = if (*soc).clk_table.uclk.num_clk_values > 0 { (*soc).clk_table.uclk.clk_values_khz[((*soc).clk_table.uclk.num_clk_values - 1) as usize] } else { 0 }; (*utm).max_fclk_khz = if (*soc).clk_table.fclk.num_clk_values > 0 { (*soc).clk_table.fclk.clk_values_khz[((*soc).clk_table.fclk.num_clk_values - 1) as usize] } else { 0 };
    (*utm).dram_config = (*soc).clk_table.dram_config; (*utm).power_management_parameters = (*soc).power_management_parameters; (*utm).writeback_base_latency_us = (*soc).qos_parameters.writeback.base_latency_us; (*utm).vmin_limit = (*soc).vmin_limit; (*utm).dchub_refclk_mhz = (*soc).dchub_refclk_mhz; (*utm).max_outstanding_reqs = (*soc).max_outstanding_reqs; (*utm).return_bus_width_bytes = (*soc).return_bus_width_bytes; (*utm).phy_downspread_percent = (*soc).phy_downspread_percent; (*utm).dcn_downspread_percent = (*soc).dcn_downspread_percent; (*utm).nominal_sdp_derate_percent = SDP_DERATE_PERCENT_NOMINAL; (*utm).urgent_sdp_derate_percent = SDP_DERATE_PERCENT_URGENT; (*utm).dispclk_dppclk_vco_speed_mhz = (*soc).dispclk_dppclk_vco_speed_mhz; (*utm).no_dfs = (*soc).no_dfs; (*utm).mem_word_bytes = (*soc).mem_word_bytes; (*utm).num_dcc_mcaches = (*soc).num_dcc_mcaches; (*utm).mcache_size_bytes = (*soc).mcache_size_bytes; (*utm).mcache_line_size_bytes = (*soc).mcache_line_size_bytes; (*utm).lower_bound_bandwidth_dchub = (*soc).lower_bound_bandwidth_dchub; (*utm).fraction_of_urgent_bandwidth_nominal_target = (*soc).fraction_of_urgent_bandwidth_nominal_target; (*utm).fraction_of_urgent_bandwidth_flip_target = (*soc).fraction_of_urgent_bandwidth_flip_target;
}

unsafe fn dcn6_initialize_from_qos_model(utm: *mut dml2_utm_soc_bb, qos: *const utm_qos_model) { (*utm).dram_config.channel_width_bytes = (*qos).socbb.dram_channel_width_bytes; (*utm).dram_config.channel_count = (*qos).socbb.dram_channel_count; (*utm).dram_config.transactions_per_clock = (*qos).socbb.dram_transactions_per_clock; }
unsafe fn dcn6a_initialize_qos_model(utm: *mut dml2_utm_soc_bb, explicit: *const utm_qos_model) { if !explicit.is_null() { dcn6_copy_utm_qos_model(&mut (*utm).qos_model, &mut (*utm).qos_model_dchub_v2, explicit); } else { dcn6_test_initialize_utm_qos_model(&mut (*utm).qos_model, &mut (*utm).qos_model_dchub_v2); } }
unsafe fn dcn6b_initialize_qos_model(utm: *mut dml2_utm_soc_bb, explicit: *const utm_qos_model) { if !explicit.is_null() { dcn6_copy_utm_qos_model(&mut (*utm).qos_model, &mut (*utm).qos_model_dchub_v2, explicit); } else { dcn6b_test_initialize_utm_qos_model(&mut (*utm).qos_model, &mut (*utm).qos_model_dchub_v2); } }

unsafe fn dml2_utm_soc_bb_dcn6a_create_legacy(utm: *mut dml2_utm_soc_bb, soc: *const dml2_soc_bb, explicit: *const utm_qos_model) -> bool { dcn6_initialize_from_soc_bb(utm, soc); dcn6a_initialize_qos_model(utm, explicit); dcn6_initialize_from_qos_model(utm, &(*utm).qos_model); dml2_utm_soc_bb_dcn6_build_sop_table(&mut (*utm).sop_table, utm); true }
unsafe fn dml2_utm_soc_bb_dcn6b_create_legacy(utm: *mut dml2_utm_soc_bb, soc: *const dml2_soc_bb, explicit: *const utm_qos_model) -> bool { dcn6_initialize_from_soc_bb(utm, soc); dcn6b_initialize_qos_model(utm, explicit); dcn6_initialize_from_qos_model(utm, &(*utm).qos_model); dml2_utm_soc_bb_dcn6_build_sop_table(&mut (*utm).sop_table, utm); true }

pub unsafe fn dml2_utm_soc_bb_dcn6a_create(utm: *mut dml2_utm_soc_bb, soc: *const dml2_soc_bb, explicit: *const utm_qos_model) -> bool { if !explicit.is_null() && (*explicit).version == utm_qos_model_version_v3 { dcn6_initialize_from_soc_bb(utm, soc); (*utm).qos_model_dchub_v3 = *(*explicit).dchub_v3; (*utm).qos_model.version = utm_qos_model_version_v3; (*utm).qos_model.dchub_v3 = &mut (*utm).qos_model_dchub_v3; dml2_utm_soc_bb_dcn6_v3_build_sop_table(&mut (*utm).sop_table, utm); } else { return dml2_utm_soc_bb_dcn6a_create_legacy(utm, soc, explicit); } true }
pub unsafe fn dml2_utm_soc_bb_dcn6b_create(utm: *mut dml2_utm_soc_bb, soc: *const dml2_soc_bb, explicit: *const utm_qos_model) -> bool { if !explicit.is_null() && (*explicit).version == utm_qos_model_version_v3 { dcn6_initialize_from_soc_bb(utm, soc); (*utm).qos_model_dchub_v3 = *(*explicit).dchub_v3; (*utm).qos_model.version = utm_qos_model_version_v3; (*utm).qos_model.dchub_v3 = &mut (*utm).qos_model_dchub_v3; dml2_utm_soc_bb_dcn6_v3_build_sop_table(&mut (*utm).sop_table, utm); } else { return dml2_utm_soc_bb_dcn6b_create_legacy(utm, soc, explicit); } true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
