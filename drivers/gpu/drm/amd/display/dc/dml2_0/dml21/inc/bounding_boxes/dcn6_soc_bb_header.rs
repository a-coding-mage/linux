// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit:
// dml2_external_lib_deps.h, utm_qos_model_dchub_v2.h,
// utm_qos_model_dchub_v3.h, and dml_top_soc_parameter_types.h.

pub unsafe fn dcn6_test_initialize_soc_bb(soc_bb: *mut dml2_soc_bb) {
    core::ptr::write_bytes(soc_bb, 0, 1);
}

pub unsafe fn dcn6b_test_initialize_soc_bb(soc_bb: *mut dml2_soc_bb) {
    dcn6_test_initialize_soc_bb(soc_bb);
}

pub unsafe fn dcn6_test_initialize_ip_caps(ip_caps: *mut dml2_ip_capabilities) {
    core::ptr::write_bytes(ip_caps, 0, 1);
}

pub unsafe fn dcn6_initialize_utm_qos_model_with_fixed_allocation(
    qos_model: *mut utm_qos_model,
    dchub: *mut utm_qos_model_dchub_v2,
) {
    core::ptr::write_bytes(qos_model, 0, 1);
    core::ptr::write_bytes(dchub, 0, 1);
    (*qos_model).dchub_v2 = dchub;
}

pub unsafe fn dcn6_test_initialize_utm_qos_model(
    _qos_model: *mut utm_qos_model,
    _dchub: *mut utm_qos_model_dchub_v2,
) {
}

pub unsafe fn dcn6b_test_initialize_utm_qos_model(
    qos_model: *mut utm_qos_model,
    dchub: *mut utm_qos_model_dchub_v2,
) {
    dcn6_initialize_utm_qos_model_with_fixed_allocation(qos_model, dchub);
}

pub unsafe fn dcn6_n_minus_1_initialize_utm_qos_model(
    qos_model: *mut utm_qos_model,
    dchub: *mut utm_qos_model_dchub_v2,
) {
    core::ptr::write_bytes(qos_model, 0, 1);
    core::ptr::write_bytes(dchub, 0, 1);
    (*qos_model).dchub_v2 = dchub;
    (*qos_model).sops[0].fclk_khz = 300000;
    (*qos_model).sops[0].uclk_khz = 97000;
    (*qos_model).sops[1].fclk_khz = 503684;
    (*qos_model).sops[1].uclk_khz = 435000;
    (*qos_model).sops[2].fclk_khz = 1007368;
    (*qos_model).sops[2].uclk_khz = 521000;
    (*qos_model).sops[3].fclk_khz = 1206526;
    (*qos_model).sops[3].uclk_khz = 731000;
    (*qos_model).sops[4].fclk_khz = 1250000;
    (*qos_model).sops[4].uclk_khz = 822000;
    (*qos_model).sops[5].fclk_khz = 1250000;
    (*qos_model).sops[5].uclk_khz = 962000;
    (*qos_model).sops[6].fclk_khz = 1250000;
    (*qos_model).sops[6].uclk_khz = 1069000;
    (*qos_model).sops[7].fclk_khz = 1250000;
    (*qos_model).sops[7].uclk_khz = 1187000;
    (*qos_model).socbb.fabric_datapath_to_dcn_data_return_bytes = 64;
    (*qos_model).socbb.dram_channel_width_bytes = 2;
    (*qos_model).socbb.dram_channel_count = 16;
    (*qos_model).socbb.dram_transactions_per_clock = 16;
    (*qos_model).socbb.fabric_derate_percent_nominal = 57;
    (*qos_model).socbb.fabric_derate_percent_urgent = 75;
    (*qos_model).socbb.dram_derate_percent_nominal = 17;
    (*qos_model).socbb.dram_derate_percent_urgent = 22;
    (*qos_model).sop_count = 8;

    let latencies = &mut (*dchub).latencies;
    let values: [[u32; 8]; 8] = [
        [9865636, 12650172, 11227505, 2531615, 1772436, 12650171, 2548107, 1000000],
        [3411495, 3467378, 2620137, 1369447, 834368, 3468646, 970920, 595611],
        [2388169, 2594969, 2171306, 875775, 526390, 2595139, 630268, 297805],
        [1896062, 1934965, 1581238, 768344, 445581, 1935135, 506977, 248647],
        [1769285, 1769285, 1416798, 742910, 425284, 1758238, 475065, 240000],
        [1652902, 1652902, 1230505, 734108, 416888, 1571945, 451191, 240000],
        [1582791, 1582791, 1122960, 727450, 410145, 1464400, 436076, 240000],
        [1520802, 1520802, 1022428, 722083, 404088, 1363868, 422992, 240000],
    ];
    for i in 0..8 {
        latencies[i].urgent_ramp_ps = values[i][0];
        latencies[i].t_trip_ps = values[i][1];
        latencies[i].meta_trip_to_mem_ps = values[i][2];
        latencies[i].max_req_latency_urg_ps = values[i][3];
        latencies[i].avg_req_latency_urg_ps = values[i][4];
        latencies[i].max_req_latency_non_urg_ps = values[i][5];
        latencies[i].avg_req_latency_non_urg_ps = values[i][6];
        latencies[i].df_response_time_ps = values[i][7];
    }

    // TODO: currently both utm budget percent and derate percent are both included in derate percent params. Need
    // to separate them. So we can use the actual utm budget percent values below.
    (*dchub).max_nominal_utm_budget_percent = 100;
    (*dchub).min_nominal_utm_budget_percent = 100;
    (*dchub).max_urgent_utm_budget_percent = 100;
    (*dchub).min_urgent_utm_budget_percent = 100;
}

pub unsafe fn dcn6a_test_initialize_sop_clocks(_sop_clocks: *mut utm_soc_operating_point) -> u32 { 0 }

pub unsafe fn dcn6b_test_initialize_sop_clocks(_sop_clocks: *mut utm_soc_operating_point) -> u32 { 0 }

/// dcn6_test_initialize_v3_sop_latencies - Set latencies for one SOP entry.
pub unsafe fn dcn6_test_initialize_v3_sop_latencies(
    entry: *mut utm_qos_model_dchub_v3_sop_entry,
    urgent_ramp_ps: u32, t_trip_ps: u32, meta_trip_to_mem_ps: u32,
    max_urg_ps: u32, avg_urg_ps: u32, max_non_urg_ps: u32,
    avg_non_urg_ps: u32, df_response_time_ps: u32,
) {
    (*entry).urgent_ramp_ps = urgent_ramp_ps;
    (*entry).t_trip_ps = t_trip_ps;
    (*entry).meta_trip_to_mem_ps = meta_trip_to_mem_ps;
    (*entry).max_req_latency_urg_ps = max_urg_ps;
    (*entry).avg_req_latency_urg_ps = avg_urg_ps;
    (*entry).max_req_latency_non_urg_ps = max_non_urg_ps;
    (*entry).avg_req_latency_non_urg_ps = avg_non_urg_ps;
    (*entry).df_response_time_ps = df_response_time_ps;
}

/// dcn6_test_initialize_v3_sop_latencies_all_levels - Set identical latencies
/// across all load levels for one SOP index.
pub unsafe fn dcn6_test_initialize_v3_sop_latencies_all_levels(
    dchub: *mut utm_qos_model_dchub_v3, sop_index: usize,
    urgent_ramp_ps: u32, t_trip_ps: u32, meta_trip_to_mem_ps: u32,
    max_urg_ps: u32, avg_urg_ps: u32, max_non_urg_ps: u32,
    avg_non_urg_ps: u32, df_response_time_ps: u32,
) {
    let mut ll = 0;
    while ll < (*dchub).load_level_count {
        dcn6_test_initialize_v3_sop_latencies(
            &mut (*dchub).sops[ll][sop_index], urgent_ramp_ps, t_trip_ps,
            meta_trip_to_mem_ps, max_urg_ps, avg_urg_ps, max_non_urg_ps,
            avg_non_urg_ps, df_response_time_ps,
        );
        ll += 1;
    }
}

pub unsafe fn dcn6_test_initialize_utm_qos_model_v3(
    qos_model: *mut utm_qos_model, dchub: *mut utm_qos_model_dchub_v3,
) {
    core::ptr::write_bytes(dchub, 0, 1);
    core::ptr::write_bytes(qos_model, 0, 1);
    (*qos_model).version = utm_qos_model_version_v3;
    (*qos_model).dchub_v3 = dchub;
}

pub unsafe fn dcn6b_test_initialize_utm_qos_model_v3(
    qos_model: *mut utm_qos_model, dchub: *mut utm_qos_model_dchub_v3,
) {
    core::ptr::write_bytes(dchub, 0, 1);
    core::ptr::write_bytes(qos_model, 0, 1);
    (*qos_model).version = utm_qos_model_version_v3;
    (*qos_model).dchub_v3 = dchub;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
