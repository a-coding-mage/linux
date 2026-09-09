// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.
//
// Dependencies supplied by the corresponding DML2 headers and libraries are
// intentionally left as external Rust items.

const DFS_DIVIDER_RANGE_SCALE_FACTOR: f64 = 4.0;
const CLOCK_UNIT_GRANULARITY: f64 = 0.001;
const DPPREFCLK_DIVIDER: f64 = 255.0;

unsafe fn cga_dcn6_add_overhead_percent(clk: f64, overhead_percent: f64) -> f64 {
    clk * (1.0 + overhead_percent / 100.0)
}

unsafe fn cga_dcn6_calculate_refclk_mhz(
    adjuster: *const dml2_clock_granularity_adjuster,
    clks_mhz: *const f64,
    count: u32,
) -> f64 {
    let mut max_clk_mhz = 0.0;
    for i in 0..count {
        max_clk_mhz = math_max2(*clks_mhz.add(i as usize), max_clk_mhz);
    }
    let mut refclk_mhz = math_floor2(max_clk_mhz, CLOCK_UNIT_GRANULARITY);
    refclk_mhz = cga_dcn6_add_overhead_percent(refclk_mhz, (*adjuster).dcn_downspread_percent);
    math_floor2(refclk_mhz, CLOCK_UNIT_GRANULARITY)
}

unsafe fn cga_dcn6_calculate_actual_dispclk_mhz(
    adjuster: *const dml2_clock_granularity_adjuster,
    mut dispclk_mhz: f64,
) -> f64 {
    dispclk_mhz = math_floor2(dispclk_mhz, CLOCK_UNIT_GRANULARITY);
    let dispclk_with_downspread_mhz =
        cga_dcn6_add_overhead_percent(dispclk_mhz, (*adjuster).dcn_downspread_percent);
    let dispclk_with_ramp_margin_mhz = cga_dcn6_add_overhead_percent(
        dispclk_with_downspread_mhz,
        (*adjuster).dispclk_ramp_margin_percent,
    );
    if dispclk_with_downspread_mhz <= (*adjuster).max_dispclk_mhz
        && dispclk_with_ramp_margin_mhz > (*adjuster).max_dispclk_mhz
    {
        // when dispclk with ramp margin is slightly over max, clamp the ramp margin to the max dispclk
        math_floor2((*adjuster).max_dispclk_mhz, CLOCK_UNIT_GRANULARITY)
    } else if dispclk_with_downspread_mhz > (*adjuster).max_dispclk_mhz {
        math_floor2(dispclk_with_downspread_mhz, CLOCK_UNIT_GRANULARITY)
    } else {
        math_floor2(dispclk_with_ramp_margin_mhz, CLOCK_UNIT_GRANULARITY)
    }
}

unsafe fn cga_dcn6_adjust_to_dfs_clock_value_mhz(
    adjuster: *const dml2_clock_granularity_adjuster,
    clk_mhz: f64,
) -> f64 {
    dml_assert_msg((*adjuster).dispclk_dppclk_vco_speed_mhz > 1.0, "invalid dispclk_dppclk_vco_speed_mhz value!\n");
    if clk_mhz == 0.0 {
        // There are cases when a clock is not needed
        return 0.0;
    }
    let vco_speed_scaled_mhz =
        math_floor2((*adjuster).dispclk_dppclk_vco_speed_mhz, 0.001) * DFS_DIVIDER_RANGE_SCALE_FACTOR;
    let vco_divider = math_floor(vco_speed_scaled_mhz / clk_mhz);
    let adjusted_clock_mhz = vco_speed_scaled_mhz / vco_divider;
    math_floor2(adjusted_clock_mhz, CLOCK_UNIT_GRANULARITY)
}

unsafe fn dga_dcn6_calculate_adjusted_dppclk_mhz(
    adjuster: *const dml2_clock_granularity_adjuster,
    dpprefclk_mhz: f64,
    mut dppclk_mhz: f64,
) -> f64 {
    let granularity_mhz = dpprefclk_mhz / DPPREFCLK_DIVIDER;
    dppclk_mhz = math_floor2(dppclk_mhz, CLOCK_UNIT_GRANULARITY);
    dppclk_mhz = cga_dcn6_add_overhead_percent(dppclk_mhz, (*adjuster).dcn_downspread_percent);
    dppclk_mhz = math_ceil2(dppclk_mhz, granularity_mhz);
    math_floor2(dppclk_mhz, CLOCK_UNIT_GRANULARITY)
}

unsafe fn dga_dcn6_calculate_adjusted_dtbclk_mhz(
    _adjuster: *const dml2_clock_granularity_adjuster,
    dppclk_mhz: f64,
) -> f64 {
    math_floor2(dppclk_mhz, CLOCK_UNIT_GRANULARITY)
}

unsafe fn cga_dcn6_adjust_dispclk_mhz(
    adjuster: *const dml2_clock_granularity_adjuster,
    dispclk_mhz: f64,
) -> f64 {
    let adjusted = cga_dcn6_calculate_actual_dispclk_mhz(adjuster, dispclk_mhz);
    cga_dcn6_adjust_to_dfs_clock_value_mhz(adjuster, adjusted)
}

unsafe fn cga_dcn6_adjust_dppclks_mhz(
    adjuster: *const dml2_clock_granularity_adjuster,
    count: u32,
    dppclks_mhz: *const f64,
    adjusted_dppclks_mhz: *mut f64,
    adjusted_dpprefclk_mhz: *mut f64,
) {
    let dpprefclk_mhz = cga_dcn6_calculate_refclk_mhz(adjuster, dppclks_mhz, count);
    *adjusted_dpprefclk_mhz = cga_dcn6_adjust_to_dfs_clock_value_mhz(adjuster, dpprefclk_mhz);
    for i in 0..count {
        *adjusted_dppclks_mhz.add(i as usize) = dga_dcn6_calculate_adjusted_dppclk_mhz(
            adjuster, *adjusted_dpprefclk_mhz, *dppclks_mhz.add(i as usize),
        );
    }
}

unsafe fn cga_dcn6_adjust_dtbclks_mhz(
    adjuster: *const dml2_clock_granularity_adjuster,
    count: u32,
    dtbclks_mhz: *const f64,
    adjusted_dtbclks_mhz: *mut f64,
    adjusted_dtbrefclk_mhz: *mut f64,
) {
    let dtbrefclk_mhz = cga_dcn6_calculate_refclk_mhz(adjuster, dtbclks_mhz, count);
    *adjusted_dtbrefclk_mhz = cga_dcn6_adjust_to_dfs_clock_value_mhz(adjuster, dtbrefclk_mhz);
    for i in 0..count {
        *adjusted_dtbclks_mhz.add(i as usize) =
            dga_dcn6_calculate_adjusted_dtbclk_mhz(adjuster, *dtbclks_mhz.add(i as usize));
    }
}

unsafe fn cga_dcn6_adjust_dcfclk_deepsleep_mhz(
    _adjuster: *const dml2_clock_granularity_adjuster,
    dcfclk_deepsleep_mhz: f64,
) -> f64 {
    math_ceil2(dcfclk_deepsleep_mhz, CLOCK_UNIT_GRANULARITY)
}

unsafe fn cga_dcn6_initialize(in_out: *const dml2_cga_initialize_in_out) {
    (*(*in_out).adjuster).dcn_downspread_percent = (*(*in_out).soc_bb).dcn_downspread_percent;
    (*(*in_out).adjuster).dispclk_dppclk_vco_speed_mhz = (*(*in_out).soc_bb).dispclk_dppclk_vco_speed_mhz;
    (*(*in_out).adjuster).dispclk_ramp_margin_percent = (*(*in_out).ip).dispclk_ramp_margin_percent;
    (*(*in_out).adjuster).max_dispclk_mhz = (*(*in_out).soc_bb).clk_table.dispclk.clk_values_khz[
        (*(*in_out).soc_bb).clk_table.dispclk.num_clk_values - 1
    ] / 1000.0;
}

pub unsafe fn cga_dcn6_create(adjuster: *mut dml2_clock_granularity_adjuster) {
    (*adjuster).initialize = Some(cga_dcn6_initialize);
    (*adjuster).adjust_dispclk_mhz = Some(cga_dcn6_adjust_dispclk_mhz);
    (*adjuster).adjust_dppclks_mhz = Some(cga_dcn6_adjust_dppclks_mhz);
    (*adjuster).adjust_dtbclks_mhz = Some(cga_dcn6_adjust_dtbclks_mhz);
    (*adjuster).adjust_dcfclk_deepsleep_mhz = Some(cga_dcn6_adjust_dcfclk_deepsleep_mhz);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
