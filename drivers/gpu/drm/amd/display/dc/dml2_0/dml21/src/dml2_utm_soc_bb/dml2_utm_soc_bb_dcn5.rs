// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit:
// dml2_utm_soc_bb_dcn5.h, bounding_boxes/dcn5_soc_bb.h, dml2_debug.h

unsafe fn dcn5_sop_table_get_highest_index(table: *const dml2_sop_table) -> u32 {
    (*(*table).model).sop_count - 1
}

unsafe fn dcn5_sop_table_get_sop_constraint_at_index(
    table: *const dml2_sop_table,
    index: u32,
    constraint: *mut dml2_sop_constraint,
) {
    let model = (*table).model;
    let dchub = (*model).dchub_v1;

    DML_ASSERT_MSG(index < (*model).sop_count, "unsupported sop index\n");
    (*constraint).dcn5.clocks.fclk_khz = (*model).sops[index as usize].fclk_khz;
    (*constraint).dcn5.clocks.uclk_khz = (*model).sops[index as usize].uclk_khz;
    (*constraint).dcn5.clocks.dcfclk_khz = (*dchub).dcfclks_khz[index as usize];
    (*constraint).dcn5.clocks.socclk_khz = (*dchub).socclks_khz[index as usize];
    (*constraint).dcn5.latency.dcn5.urgent_ramp = (*dchub).latencies[index as usize].urgent_ramp_ps as f64 / 1000000.0;
    (*constraint).dcn5.latency.dcn5.t_trip = (*dchub).latencies[index as usize].t_trip_ps as f64 / 1000000.0;
    (*constraint).dcn5.latency.dcn5.meta_trip_to_mem = (*dchub).latencies[index as usize].meta_trip_to_mem_ps as f64 / 1000000.0;
    (*constraint).dcn5.latency.dcn5.max_req_latency_urg = (*dchub).latencies[index as usize].max_req_latency_urg_ps as f64 / 1000000.0;
    (*constraint).dcn5.latency.dcn5.avg_req_latency_urg = (*dchub).latencies[index as usize].avg_req_latency_urg_ps as f64 / 1000000.0;
    (*constraint).dcn5.latency.dcn5.max_req_latency_non_urg = (*dchub).latencies[index as usize].max_req_latency_non_urg_ps as f64 / 1000000.0;
    (*constraint).dcn5.latency.dcn5.avg_req_latency_non_urg = (*dchub).latencies[index as usize].avg_req_latency_non_urg_ps as f64 / 1000000.0;
    (*constraint).dcn5.latency.dcn5.df_response_time_us = (*dchub).latencies[index as usize].df_response_time_ps as f64 / 1000000.0;
    (*constraint).dcn5.min_available_urgent_bandwidth_KBps = (*dchub).bandwidths[index as usize].urgent_bandwidth_KBps;
    (*constraint).dcn5.min_sop_index = index;
}

unsafe fn dcn5_sop_table_is_bandwidth_supported_at_index(
    table: *const dml2_sop_table,
    bw: *const dml2_memory_path_bandwidth,
    index: u32,
) -> bool {
    let mut qos_bandwidth = utm_qos_model_dchub_memory_path_bandwidth_v1 {
        nominal_bandwidth_KBps: (*bw).dcn5.non_urgent_bandwidth_kbps as u32,
        urgent_bandwidth_KBps: (*bw).dcn5.urgent_bandwidth_kbps as u32,
    };
    if !dchub_v1_is_qos_bandwidth_supported_by_sop((*table).model, &mut qos_bandwidth, index as u8) {
        false
    } else {
        true
    }
}

unsafe fn dcn5_sop_table_get_max_sop(table: *const dml2_sop_table, sop: *mut dml2_soc_operating_point) {
    let model = (*table).model;
    let dchub = (*model).dchub_v1;
    DML_ASSERT_MSG((*model).sop_count > 0, "utm_qos_model must contain at least 1 sop\n");
    if (*model).sop_count > 0 {
        let i = (*model).sop_count as usize - 1;
        (*sop).fclk_khz = (*model).sops[i].fclk_khz;
        (*sop).uclk_khz = (*model).sops[i].uclk_khz;
        (*sop).dcfclk_khz = (*dchub).dcfclks_khz[i];
        (*sop).socclk_khz = (*dchub).socclks_khz[i];
    }
}

unsafe fn dcn5_sop_table_get_min_sop(table: *const dml2_sop_table, sop: *mut dml2_soc_operating_point) {
    let model = (*table).model;
    let dchub = (*model).dchub_v1;
    (*sop).fclk_khz = (*model).sops[0].fclk_khz;
    (*sop).uclk_khz = (*model).sops[0].uclk_khz;
    (*sop).dcfclk_khz = (*dchub).dcfclks_khz[0];
    (*sop).socclk_khz = (*dchub).socclks_khz[0];
}

pub unsafe fn dml2_utm_soc_bb_dcn5_build_sop_table(table: *mut dml2_sop_table, utm_soc_bb: *const dml2_utm_soc_bb) {
    (*table).get_highest_sop_index = Some(dcn5_sop_table_get_highest_index);
    (*table).get_sop_constraint_at_index = Some(dcn5_sop_table_get_sop_constraint_at_index);
    (*table).is_bw_supported_at_index = Some(dcn5_sop_table_is_bandwidth_supported_at_index);
    (*table).get_max_sop = Some(dcn5_sop_table_get_max_sop);
    (*table).get_min_sop = Some(dcn5_sop_table_get_min_sop);
    (*table).model = &(*utm_soc_bb).qos_model;
    DML_ASSERT_MSG((*table).model.sop_count > 0, "qos_model must contain at least 1 sop\n");
}

unsafe fn dcn5_copy_utm_qos_model(dest: *mut utm_qos_model, dest_dchub: *mut utm_qos_model_dchub_v1, src: *const utm_qos_model) {
    *dest = *src;
    *dest_dchub = *(*src).dchub_v1;
    (*dest).dchub_v1 = dest_dchub;
}

pub unsafe fn dml2_utm_soc_bb_dcn5_create(
    utm_soc_bb: *mut dml2_utm_soc_bb,
    soc_bb: *const dml2_soc_bb,
    explicit_qos_model: *const utm_qos_model,
) -> bool {
    let qos_model = &mut (*utm_soc_bb).qos_model as *mut utm_qos_model;
    if !explicit_qos_model.is_null() {
        dcn5_copy_utm_qos_model(qos_model, &mut (*utm_soc_bb).qos_model_dchub_v1, explicit_qos_model);
    } else {
        dcn5_initialize_utm_qos_model(qos_model, &mut (*utm_soc_bb).qos_model_dchub_v1);
    }

    (*utm_soc_bb).max_dispclk_khz = (*soc_bb).clk_table.dispclk.clk_values_khz[(*soc_bb).clk_table.dispclk.num_clk_values as usize - 1];
    (*utm_soc_bb).max_dppclk_khz = (*soc_bb).clk_table.dppclk.clk_values_khz[(*soc_bb).clk_table.dppclk.num_clk_values as usize - 1];
    (*utm_soc_bb).max_dtbclk_khz = if (*soc_bb).clk_table.dtbclk.num_clk_values > 0 { (*soc_bb).clk_table.dtbclk.clk_values_khz[(*soc_bb).clk_table.dtbclk.num_clk_values as usize - 1] } else { 0 };
    (*utm_soc_bb).max_phyclk_khz = if (*soc_bb).clk_table.phyclk.num_clk_values > 0 { (*soc_bb).clk_table.phyclk.clk_values_khz[(*soc_bb).clk_table.phyclk.num_clk_values as usize - 1] } else { 0 };
    (*utm_soc_bb).max_dscclk_khz = if (*soc_bb).clk_table.dscclk.num_clk_values > 0 { (*soc_bb).clk_table.dscclk.clk_values_khz[(*soc_bb).clk_table.dscclk.num_clk_values as usize - 1] } else { 0 };
    (*utm_soc_bb).max_phyclk_d18_khz = if (*soc_bb).clk_table.phyclk_d18.num_clk_values > 0 { (*soc_bb).clk_table.phyclk_d18.clk_values_khz[(*soc_bb).clk_table.phyclk_d18.num_clk_values as usize - 1] } else { 0 };
    (*utm_soc_bb).max_phyclk_d32_khz = if (*soc_bb).clk_table.phyclk_d32.num_clk_values > 0 { (*soc_bb).clk_table.phyclk_d32.clk_values_khz[(*soc_bb).clk_table.phyclk_d32.num_clk_values as usize - 1] } else { 0 };
    (*utm_soc_bb).power_management_parameters = (*soc_bb).power_management_parameters;
    (*utm_soc_bb).writeback_base_latency_us = (*soc_bb).qos_parameters.writeback.base_latency_us;
    (*utm_soc_bb).vmin_limit = (*soc_bb).vmin_limit;
    (*utm_soc_bb).dchub_refclk_mhz = (*soc_bb).dchub_refclk_mhz;
    (*utm_soc_bb).max_outstanding_reqs = (*soc_bb).max_outstanding_reqs;
    (*utm_soc_bb).return_bus_width_bytes = (*soc_bb).return_bus_width_bytes;
    (*utm_soc_bb).phy_downspread_percent = (*soc_bb).phy_downspread_percent;
    (*utm_soc_bb).dcn_downspread_percent = (*soc_bb).dcn_downspread_percent;
    (*utm_soc_bb).dispclk_dppclk_vco_speed_mhz = (*soc_bb).dispclk_dppclk_vco_speed_mhz;
    (*utm_soc_bb).no_dfs = (*soc_bb).no_dfs;
    (*utm_soc_bb).mem_word_bytes = (*soc_bb).mem_word_bytes;
    (*utm_soc_bb).num_dcc_mcaches = (*soc_bb).num_dcc_mcaches;
    (*utm_soc_bb).mcache_size_bytes = (*soc_bb).mcache_size_bytes;
    (*utm_soc_bb).mcache_line_size_bytes = (*soc_bb).mcache_line_size_bytes;
    (*utm_soc_bb).lower_bound_bandwidth_dchub = (*soc_bb).lower_bound_bandwidth_dchub;

    (*utm_soc_bb).dram_config.channel_width_bytes = (*qos_model).socbb.dram_channel_width_bytes;
    (*utm_soc_bb).dram_config.channel_count = (*qos_model).socbb.dram_channel_count;
    (*utm_soc_bb).dram_config.transactions_per_clock = (*qos_model).socbb.dram_transactions_per_clock;
    (*utm_soc_bb).max_dtbclk_khz = (*qos_model).dchub_v1.dcfclks_khz[(*qos_model).sop_count as usize - 1];
    dml2_utm_soc_bb_dcn5_build_sop_table(&mut (*utm_soc_bb).sop_table, utm_soc_bb);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
