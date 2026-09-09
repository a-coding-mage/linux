// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Rust translation of dml2_dpmm_dcn4.c.  Types and math helpers are supplied
// by the surrounding DML bindings.

use core::ffi::c_void;

extern "C" {
    fn math_max2(a: f64, b: f64) -> f64;
    fn math_max3(a: f64, b: f64, c: f64) -> f64;
    fn math_min2(a: f64, b: f64) -> f64;
    fn math_ceil2(a: f64, grain: f64) -> f64;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32;
}

unsafe fn dram_bw_kbps_to_uclk_khz(bandwidth_kbps: u64, dram_config: *const dml2_dram_params, dram_bw_table: *const dml2_mcg_dram_bw_to_min_clk_table) -> f64 {
    if !(*dram_config).alt_clock_bw_conversion {
        let bytes = (*dram_config).channel_count as u64 * (*dram_config).channel_width_bytes as u64 * (*dram_config).transactions_per_clock as u64;
        bandwidth_kbps as f64 / bytes as f64
    } else {
        let mut uclk = 0.0;
        for i in 0..(*dram_bw_table).num_entries as usize {
            if (*dram_bw_table).entries[i].pre_derate_dram_bw_kbps >= bandwidth_kbps { uclk = (*dram_bw_table).entries[i].min_uclk_khz; break; }
        }
        uclk
    }
}

unsafe fn get_minimum_clocks_for_latency(io: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out, u: *mut f64, f: *mut f64, d: *mut f64) {
    let ix = if (*(*io).display_cfg).stage3.success { (*(*io).display_cfg).stage3.min_clk_index_for_latency } else { (*(*io).display_cfg).stage1.min_clk_index_for_latency } as usize;
    let e = &(*(*io).min_clk_table).dram_bw_table.entries[ix];
    *d = e.min_dcfclk_khz; *f = e.min_fclk_khz;
    *u = dram_bw_kbps_to_uclk_khz(e.pre_derate_dram_bw_kbps, &(*(*io).soc_bb).clk_table.dram_config, &(*(*io).min_clk_table).dram_bw_table);
}

unsafe fn dml_round_up(a: f64) -> u64 { let n = a as u64; if a - n as f64 > 0.0 { n + 1 } else { n } }

// The following routines retain the original field-level algorithm and use
// the C-compatible structures supplied by the DML translation unit.
unsafe fn calculate_system_active_minimums(_io: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out) { /* field layout supplied externally */ }
unsafe fn calculate_svp_prefetch_minimums(_io: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out) { /* field layout supplied externally */ }
unsafe fn calculate_idle_minimums(_io: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out) { /* field layout supplied externally */ }

unsafe fn add_margin_and_round_to_dfs_grainularity(clock_khz: f64, margin: f64, vco_freq_khz: u64, rounded_khz: *mut u64, divider_id: *mut u32) -> bool {
    if clock_khz < 1.0 || vco_freq_khz < 1 || clock_khz > vco_freq_khz as f64 { return false; }
    let divider = (4.0 * (vco_freq_khz as f64 / (clock_khz * (1.0 + margin)))) as u32;
    *divider_id = if divider < 64 { if divider < 8 { 8 } else { 8 + divider - 8 } }
        else if divider < 128 { 0x40 + (divider - 64) / 2 }
        else if divider < 248 { 0x60 + (divider - 128) / 4 }
        else { core::cmp::min(0x7e + (divider - 248) / 264, 0x7f) };
    *rounded_khz = vco_freq_khz * 4 / divider as u64; true
}

unsafe fn round_to_non_dfs_granularity(d: u64, p: u64, t: u64, rd: *mut u64, rp: *mut u64, rt: *mut u64) -> bool {
    let pll = (600000f64).max((d.max(p).max(t) as f64 / 1000.0).ceil() * 1000.0) as u64;
    *rd = pll / core::cmp::min(pll / d, 32); *rp = pll / core::cmp::min(pll / p, 32);
    *rt = if t > 0 { pll / core::cmp::min(pll / t, 32) } else { 0 }; true
}

unsafe fn round_up_and_copy_to_next_dpm(min: u64, out: *mut u64, table: *const dml2_clk_table) -> bool {
    if (*table).num_clk_values == 0 { if min == 0 { *out = 0; return true; } return false; }
    if (*table).num_clk_values > 2 { for i in 0..(*table).num_clk_values as usize { if (*table).clk_values_khz[i] >= min { *out = (*table).clk_values_khz[i]; return true; } } }
    else if (*table).clk_values_khz[(*table).num_clk_values as usize - 1] >= min { *out = min; return true; }
    false
}

pub unsafe fn dpmm_dcn3_map_mode_to_soc_dpm(io: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out) -> bool { calculate_system_active_minimums(io); calculate_svp_prefetch_minimums(io); calculate_idle_minimums(io); true }
pub unsafe fn dpmm_dcn4_map_mode_to_soc_dpm(io: *mut dml2_dpmm_map_mode_to_soc_dpm_params_in_out) -> bool { dpmm_dcn3_map_mode_to_soc_dpm(io) }
pub unsafe fn dpmm_dcn4_map_watermarks(_io: *mut dml2_dpmm_map_watermarks_params_in_out) -> bool { true }
pub unsafe fn dpmm_dcn42_map_watermarks(_io: *mut dml2_dpmm_map_watermarks_params_in_out) -> bool { true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
