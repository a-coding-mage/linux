// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependencies supplied by dml2_mcg_dcn42.h and dml_top_soc_parameter_types.h.

unsafe fn uclk_to_dram_bw_kbps(
    uclk_khz: c_ulong,
    dram_config: *const dml2_dram_params,
    wck_ratio: c_ulong,
) -> c_ulonglong {
    let mut bw_kbps: c_ulonglong = 0;

    bw_kbps = (uclk_khz as c_ulonglong)
        .wrapping_mul((*dram_config).channel_count as c_ulonglong)
        .wrapping_mul((*dram_config).channel_width_bytes as c_ulonglong)
        .wrapping_mul(wck_ratio as c_ulonglong)
        .wrapping_mul(2);
    bw_kbps
}

unsafe fn build_min_clk_table_coarse_grained(
    soc_bb: *const dml2_soc_bb,
    min_table: *mut dml2_mcg_min_clock_table,
) -> bool {
    let mut i: c_int = 0;

    while i < (*soc_bb).clk_table.fclk.num_clk_values {
        if i < (*soc_bb).clk_table.uclk.num_clk_values {
            (*min_table).dram_bw_table.entries[i as usize].pre_derate_dram_bw_kbps =
                uclk_to_dram_bw_kbps(
                    (*soc_bb).clk_table.uclk.clk_values_khz[i as usize],
                    &(*soc_bb).clk_table.dram_config,
                    (*soc_bb).clk_table.wck_ratio.clk_values_khz[i as usize],
                );
            (*min_table).dram_bw_table.entries[i as usize].min_uclk_khz =
                (*soc_bb).clk_table.uclk.clk_values_khz[i as usize];
        } else if (*soc_bb).clk_table.uclk.num_clk_values > 0 {
            (*min_table).dram_bw_table.entries[i as usize].pre_derate_dram_bw_kbps = (*min_table)
                .dram_bw_table
                .entries[((*soc_bb).clk_table.uclk.num_clk_values - 1) as usize]
                .pre_derate_dram_bw_kbps;
            (*min_table).dram_bw_table.entries[i as usize].min_uclk_khz = (*soc_bb)
                .clk_table
                .uclk
                .clk_values_khz[((*soc_bb).clk_table.uclk.num_clk_values - 1) as usize];
        }

        (*min_table).dram_bw_table.entries[i as usize].min_dcfclk_khz =
            (*soc_bb).clk_table.dcfclk.clk_values_khz[i as usize];
        (*min_table).dram_bw_table.entries[i as usize].min_fclk_khz =
            (*soc_bb).clk_table.fclk.clk_values_khz[i as usize];
        i += 1;
    }
    (*min_table).dram_bw_table.num_entries = (*soc_bb).clk_table.fclk.num_clk_values;

    true
}

unsafe fn build_min_clock_table(
    soc_bb: *const dml2_soc_bb,
    min_table: *mut dml2_mcg_min_clock_table,
) -> bool {
    let result: bool;

    if soc_bb.is_null() || min_table.is_null() {
        return false;
    }

    if (*soc_bb).clk_table.uclk.num_clk_values > DML_MCG_MAX_CLK_TABLE_SIZE {
        return false;
    }

    (*min_table).fixed_clocks_khz.amclk = 0;
    (*min_table).fixed_clocks_khz.dprefclk = (*soc_bb).dprefclk_mhz * 1000;
    (*min_table).fixed_clocks_khz.pcierefclk = (*soc_bb).pcie_refclk_mhz * 1000;
    (*min_table).fixed_clocks_khz.dchubrefclk = (*soc_bb).dchub_refclk_mhz * 1000;
    (*min_table).fixed_clocks_khz.xtalclk = (*soc_bb).xtalclk_mhz * 1000;

    (*min_table).max_clocks_khz.dispclk = (*soc_bb).clk_table.dispclk.clk_values_khz
        [((*soc_bb).clk_table.dispclk.num_clk_values - 1) as usize];
    (*min_table).max_clocks_khz.dppclk = (*soc_bb).clk_table.dppclk.clk_values_khz
        [((*soc_bb).clk_table.dppclk.num_clk_values - 1) as usize];
    (*min_table).max_clocks_khz.dscclk = if (*soc_bb).clk_table.dscclk.num_clk_values > 0 {
        (*soc_bb).clk_table.dscclk.clk_values_khz
            [((*soc_bb).clk_table.dscclk.num_clk_values - 1) as usize]
    } else {
        0
    };
    (*min_table).max_clocks_khz.dtbclk = if (*soc_bb).clk_table.dtbclk.num_clk_values > 0 {
        (*soc_bb).clk_table.dtbclk.clk_values_khz
            [((*soc_bb).clk_table.dtbclk.num_clk_values - 1) as usize]
    } else {
        0
    };
    (*min_table).max_clocks_khz.phyclk = if (*soc_bb).clk_table.phyclk.num_clk_values > 0 {
        (*soc_bb).clk_table.phyclk.clk_values_khz
            [((*soc_bb).clk_table.phyclk.num_clk_values - 1) as usize]
    } else {
        0
    };

    (*min_table).max_ss_clocks_khz.dispclk =
        (((*min_table).max_clocks_khz.dispclk as f64)
            / (1.0 + (*soc_bb).dcn_downspread_percent as f64 / 100.0)) as c_uint;
    (*min_table).max_ss_clocks_khz.dppclk =
        (((*min_table).max_clocks_khz.dppclk as f64)
            / (1.0 + (*soc_bb).dcn_downspread_percent as f64 / 100.0)) as c_uint;
    (*min_table).max_ss_clocks_khz.dtbclk =
        (((*min_table).max_clocks_khz.dtbclk as f64)
            / (1.0 + (*soc_bb).dcn_downspread_percent as f64 / 100.0)) as c_uint;

    (*min_table).max_clocks_khz.dcfclk = (*soc_bb).clk_table.dcfclk.clk_values_khz
        [((*soc_bb).clk_table.dcfclk.num_clk_values - 1) as usize];
    (*min_table).max_clocks_khz.fclk = (*soc_bb).clk_table.fclk.clk_values_khz
        [((*soc_bb).clk_table.fclk.num_clk_values - 1) as usize];

    result = build_min_clk_table_coarse_grained(soc_bb, min_table);

    result
}

pub unsafe fn mcg_dcn42_build_min_clock_table(
    in_out: *mut dml2_mcg_build_min_clock_table_params_in_out,
) -> bool {
    build_min_clock_table((*in_out).soc_bb, (*in_out).min_clk_table)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
