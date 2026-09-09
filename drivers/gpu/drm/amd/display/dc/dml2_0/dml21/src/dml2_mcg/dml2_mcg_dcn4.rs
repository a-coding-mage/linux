// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by dml2_mcg_dcn4.h and dml_top_soc_parameter_types.h.

pub unsafe fn mcg_dcn4_build_min_clock_table(
    in_out: *mut dml2_mcg_build_min_clock_table_params_in_out,
) -> bool {
    build_min_clock_table((*in_out).soc_bb, (*in_out).min_clk_table)
}

unsafe fn uclk_to_dram_bw_kbps(
    uclk_khz: u64,
    dram_config: *const dml2_dram_params,
) -> u64 {
    uclk_khz
        .wrapping_mul((*dram_config).channel_count as u64)
        .wrapping_mul((*dram_config).channel_width_bytes as u64)
        .wrapping_mul((*dram_config).transactions_per_clock as u64)
}

unsafe fn round_up_to_quantized_values(
    value: u64,
    quantized_values: *const u64,
    num_quantized_values: i32,
) -> u64 {
    if quantized_values.is_null() {
        return 0;
    }

    let mut i = 0;
    while i < num_quantized_values {
        if *quantized_values.add(i as usize) > value {
            return *quantized_values.add(i as usize);
        }
        i += 1;
    }
    0
}

unsafe fn build_min_clk_table_fine_grained(
    soc_bb: *const dml2_soc_bb,
    min_table: *mut dml2_mcg_min_clock_table,
) -> bool {
    let dcfclk_fine_grained = (*soc_bb).clk_table.dcfclk.num_clk_values == 2;
    let fclk_fine_grained = (*soc_bb).clk_table.fclk.num_clk_values == 2;

    let min_dcfclk_khz = (*soc_bb).clk_table.dcfclk.clk_values_khz[0];
    let min_fclk_khz = (*soc_bb).clk_table.fclk.clk_values_khz[0];

    // First calculate the table for "balanced" bandwidths across UCLK/FCLK
    let mut i = 0;
    while i < (*soc_bb).clk_table.uclk.num_clk_values {
        (*min_table).dram_bw_table.entries[i].pre_derate_dram_bw_kbps =
            uclk_to_dram_bw_kbps((*soc_bb).clk_table.uclk.clk_values_khz[i], &(*soc_bb).clk_table.dram_config);

        (*min_table).dram_bw_table.entries[i].min_fclk_khz =
            ((((*min_table).dram_bw_table.entries[i].pre_derate_dram_bw_kbps as f64
                * (*soc_bb).qos_parameters.derate_table.system_active_urgent.dram_derate_percent as f64 / 100.0)
                / ((*soc_bb).qos_parameters.derate_table.system_active_urgent.fclk_derate_percent as f64 / 100.0))
                / (*soc_bb).fabric_datapath_to_dcn_data_return_bytes as f64) as u64;
        i += 1;
    }
    (*min_table).dram_bw_table.num_entries = (*soc_bb).clk_table.uclk.num_clk_values;

    // To create the minimum table, shift up all dcfclk/fclk entries by 1, then replace the lowest entry.
    i = (*min_table).dram_bw_table.num_entries - 1;
    while i > 0 {
        let prev_100 = (*min_table).dram_bw_table.entries[i - 1].min_fclk_khz;
        let cur_50 = (*min_table).dram_bw_table.entries[i].min_fclk_khz / 2;
        (*min_table).dram_bw_table.entries[i].min_fclk_khz = if prev_100 > cur_50 { prev_100 } else { cur_50 };

        if !fclk_fine_grained {
            (*min_table).dram_bw_table.entries[i].min_fclk_khz = round_up_to_quantized_values(
                (*min_table).dram_bw_table.entries[i].min_fclk_khz,
                (*soc_bb).clk_table.fclk.clk_values_khz.as_ptr(),
                (*soc_bb).clk_table.fclk.num_clk_values as i32,
            );
        }
        i -= 1;
    }
    (*min_table).dram_bw_table.entries[0].min_fclk_khz /= 2;

    i = 0;
    while i < (*min_table).dram_bw_table.num_entries {
        if (*min_table).dram_bw_table.entries[i].min_dcfclk_khz < min_dcfclk_khz {
            (*min_table).dram_bw_table.entries[i].min_dcfclk_khz = min_dcfclk_khz;
        }
        if (*min_table).dram_bw_table.entries[i].min_fclk_khz < min_fclk_khz {
            (*min_table).dram_bw_table.entries[i].min_fclk_khz = min_fclk_khz;
        }
        if (*soc_bb).max_fclk_for_uclk_dpm_khz > 0
            && (*min_table).dram_bw_table.entries[i].min_fclk_khz > (*soc_bb).max_fclk_for_uclk_dpm_khz
        {
            (*min_table).dram_bw_table.entries[i].min_fclk_khz = (*soc_bb).max_fclk_for_uclk_dpm_khz;
        }
        (*min_table).dram_bw_table.entries[i].min_dcfclk_khz =
            (*min_table).dram_bw_table.entries[i].min_fclk_khz
                * (*soc_bb).qos_parameters.derate_table.system_active_urgent.fclk_derate_percent
                / (*soc_bb).qos_parameters.derate_table.system_active_urgent.dcfclk_derate_percent;
        (*min_table).dram_bw_table.entries[i].min_dcfclk_khz =
            (*min_table).dram_bw_table.entries[i].min_dcfclk_khz
                * (*soc_bb).fabric_datapath_to_dcn_data_return_bytes / (*soc_bb).return_bus_width_bytes;
        if !dcfclk_fine_grained {
            (*min_table).dram_bw_table.entries[i].min_dcfclk_khz = round_up_to_quantized_values(
                (*min_table).dram_bw_table.entries[i].min_dcfclk_khz,
                (*soc_bb).clk_table.dcfclk.clk_values_khz.as_ptr(),
                (*soc_bb).clk_table.dcfclk.num_clk_values as i32,
            );
        }
        i += 1;
    }

    i = 0;
    while i < (*min_table).dram_bw_table.num_entries {
        if (*min_table).dram_bw_table.entries[i].min_dcfclk_khz > (*min_table).max_clocks_khz.dcfclk
            || (*min_table).dram_bw_table.entries[i].min_fclk_khz > (*min_table).max_clocks_khz.fclk
        {
            (*min_table).dram_bw_table.num_entries = i;
            break;
        }
        i += 1;
    }

    i = 0;
    while i + 1 < (*min_table).dram_bw_table.num_entries {
        if (*min_table).dram_bw_table.entries[i].min_dcfclk_khz == (*min_table).dram_bw_table.entries[i + 1].min_dcfclk_khz
            && (*min_table).dram_bw_table.entries[i].min_fclk_khz == (*min_table).dram_bw_table.entries[i + 1].min_fclk_khz
            && (*min_table).dram_bw_table.entries[i].pre_derate_dram_bw_kbps == (*min_table).dram_bw_table.entries[i + 1].pre_derate_dram_bw_kbps
        {
            // i + 1 is the same state as i, so shift everything.
            let mut j = i + 1;
            while j < (*min_table).dram_bw_table.num_entries {
                (*min_table).dram_bw_table.entries[j] = (*min_table).dram_bw_table.entries[j + 1];
                j += 1;
            }
            (*min_table).dram_bw_table.num_entries -= 1;
        }
        i += 1;
    }
    true
}

unsafe fn build_min_clk_table_coarse_grained(
    soc_bb: *const dml2_soc_bb,
    min_table: *mut dml2_mcg_min_clock_table,
) -> bool {
    let mut i = 0;
    while i < (*soc_bb).clk_table.uclk.num_clk_values {
        (*min_table).dram_bw_table.entries[i].pre_derate_dram_bw_kbps =
            uclk_to_dram_bw_kbps((*soc_bb).clk_table.uclk.clk_values_khz[i], &(*soc_bb).clk_table.dram_config);
        (*min_table).dram_bw_table.entries[i].min_dcfclk_khz = (*soc_bb).clk_table.dcfclk.clk_values_khz[i];
        (*min_table).dram_bw_table.entries[i].min_fclk_khz = (*soc_bb).clk_table.fclk.clk_values_khz[i];
        i += 1;
    }
    (*min_table).dram_bw_table.num_entries = (*soc_bb).clk_table.uclk.num_clk_values;
    true
}

unsafe fn build_min_clock_table(
    soc_bb: *const dml2_soc_bb,
    min_table: *mut dml2_mcg_min_clock_table,
) -> bool {
    if soc_bb.is_null() || min_table.is_null() {
        return false;
    }
    if (*soc_bb).clk_table.dcfclk.num_clk_values < 2 || (*soc_bb).clk_table.fclk.num_clk_values < 2 {
        return false;
    }
    if (*soc_bb).clk_table.uclk.num_clk_values > DML_MCG_MAX_CLK_TABLE_SIZE {
        return false;
    }

    let dcfclk_fine_grained = (*soc_bb).clk_table.dcfclk.num_clk_values == 2;
    let fclk_fine_grained = (*soc_bb).clk_table.fclk.num_clk_values == 2;
    let clock_state_count_equal = (*soc_bb).clk_table.fclk.num_clk_values == (*soc_bb).clk_table.dcfclk.num_clk_values
        && (*soc_bb).clk_table.fclk.num_clk_values == (*soc_bb).clk_table.uclk.num_clk_values;

    (*min_table).fixed_clocks_khz.amclk = 0;
    (*min_table).fixed_clocks_khz.dprefclk = (*soc_bb).dprefclk_mhz * 1000;
    (*min_table).fixed_clocks_khz.pcierefclk = (*soc_bb).pcie_refclk_mhz * 1000;
    (*min_table).fixed_clocks_khz.dchubrefclk = (*soc_bb).dchub_refclk_mhz * 1000;
    (*min_table).fixed_clocks_khz.xtalclk = (*soc_bb).xtalclk_mhz * 1000;

    (*min_table).max_clocks_khz.dispclk = (*soc_bb).clk_table.dispclk.clk_values_khz[(*soc_bb).clk_table.dispclk.num_clk_values - 1];
    (*min_table).max_clocks_khz.dppclk = (*soc_bb).clk_table.dppclk.clk_values_khz[(*soc_bb).clk_table.dppclk.num_clk_values - 1];
    (*min_table).max_clocks_khz.dscclk = if (*soc_bb).clk_table.dscclk.num_clk_values > 0 { (*soc_bb).clk_table.dscclk.clk_values_khz[(*soc_bb).clk_table.dscclk.num_clk_values - 1] } else { 0 };
    (*min_table).max_clocks_khz.dtbclk = if (*soc_bb).clk_table.dtbclk.num_clk_values > 0 { (*soc_bb).clk_table.dtbclk.clk_values_khz[(*soc_bb).clk_table.dtbclk.num_clk_values - 1] } else { 0 };
    (*min_table).max_clocks_khz.phyclk = if (*soc_bb).clk_table.phyclk.num_clk_values > 0 { (*soc_bb).clk_table.phyclk.clk_values_khz[(*soc_bb).clk_table.phyclk.num_clk_values - 1] } else { 0 };

    let spread = 1.0 + (*soc_bb).dcn_downspread_percent as f64 / 100.0;
    (*min_table).max_ss_clocks_khz.dispclk = ((*min_table).max_clocks_khz.dispclk as f64 / spread) as u32;
    (*min_table).max_ss_clocks_khz.dppclk = ((*min_table).max_clocks_khz.dppclk as f64 / spread) as u32;
    (*min_table).max_ss_clocks_khz.dtbclk = ((*min_table).max_clocks_khz.dtbclk as f64 / spread) as u32;
    (*min_table).max_clocks_khz.dcfclk = (*soc_bb).clk_table.dcfclk.clk_values_khz[(*soc_bb).clk_table.dcfclk.num_clk_values - 1];
    (*min_table).max_clocks_khz.fclk = (*soc_bb).clk_table.fclk.clk_values_khz[(*soc_bb).clk_table.fclk.num_clk_values - 1];

    if dcfclk_fine_grained || fclk_fine_grained || !clock_state_count_equal {
        build_min_clk_table_fine_grained(soc_bb, min_table)
    } else {
        build_min_clk_table_coarse_grained(soc_bb, min_table)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
