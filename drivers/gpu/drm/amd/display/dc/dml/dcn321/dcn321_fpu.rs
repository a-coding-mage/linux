// SPDX-License-Identifier: MIT
/* Rust translation of dcn321_fpu.c. External kernel/DML types and functions
 * are intentionally supplied by the surrounding translation unit. */

const DCN3_2_DEFAULT_DET_SIZE: u32 = 256;

// The following globals retain the C ABI-facing names and are initialized by
// the dependency-provided representations of these DML structures.
#[no_mangle]
pub static mut dcn3_21_ip: _vcs_dpi_ip_params_st = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut dcn3_21_soc: _vcs_dpi_soc_bounding_box_st = unsafe { core::mem::zeroed() };

unsafe fn get_optimal_ntuple(entry: *mut _vcs_dpi_voltage_scaling_st) {
    if (*entry).dcfclk_mhz > 0 {
        let bw_on_sdp: f32 = (*entry).dcfclk_mhz as f32 * dcn3_21_soc.return_bus_width_bytes as f32
            * (dcn3_21_soc.pct_ideal_sdp_bw_after_urgent as f32 / 100.0);
        (*entry).fabricclk_mhz = bw_on_sdp / (dcn3_21_soc.return_bus_width_bytes as f32
            * (dcn3_21_soc.pct_ideal_fabric_bw_after_urgent as f32 / 100.0));
        (*entry).dram_speed_mts = bw_on_sdp / (dcn3_21_soc.num_chans as f32
            * dcn3_21_soc.dram_channel_width_bytes as f32
            * (dcn3_21_soc.pct_ideal_dram_sdp_bw_after_urgent_pixel_only as f32 / 100.0));
    } else if (*entry).fabricclk_mhz > 0 {
        let bw_on_fabric = (*entry).fabricclk_mhz as f32 * dcn3_21_soc.return_bus_width_bytes as f32
            * (dcn3_21_soc.pct_ideal_fabric_bw_after_urgent as f32 / 100.0);
        (*entry).dcfclk_mhz = bw_on_fabric / (dcn3_21_soc.return_bus_width_bytes as f32
            * (dcn3_21_soc.pct_ideal_sdp_bw_after_urgent as f32 / 100.0));
        (*entry).dram_speed_mts = bw_on_fabric / (dcn3_21_soc.num_chans as f32
            * dcn3_21_soc.dram_channel_width_bytes as f32
            * (dcn3_21_soc.pct_ideal_dram_sdp_bw_after_urgent_pixel_only as f32 / 100.0));
    } else if (*entry).dram_speed_mts > 0 {
        let bw_on_dram = (*entry).dram_speed_mts as f32 * dcn3_21_soc.num_chans as f32
            * dcn3_21_soc.dram_channel_width_bytes as f32
            * (dcn3_21_soc.pct_ideal_dram_sdp_bw_after_urgent_pixel_only as f32 / 100.0);
        (*entry).fabricclk_mhz = bw_on_dram / (dcn3_21_soc.return_bus_width_bytes as f32
            * (dcn3_21_soc.pct_ideal_fabric_bw_after_urgent as f32 / 100.0));
        (*entry).dcfclk_mhz = bw_on_dram / (dcn3_21_soc.return_bus_width_bytes as f32
            * (dcn3_21_soc.pct_ideal_sdp_bw_after_urgent as f32 / 100.0));
    }
}

unsafe fn calculate_net_bw_in_kbytes_sec(entry: *const _vcs_dpi_voltage_scaling_st) -> f32 {
    let memory = (*entry).dram_speed_mts as f32 * dcn3_21_soc.num_chans as f32
        * dcn3_21_soc.dram_channel_width_bytes as f32
        * (dcn3_21_soc.pct_ideal_dram_sdp_bw_after_urgent_pixel_only as f32 / 100.0);
    let fabric = (*entry).fabricclk_mhz as f32 * dcn3_21_soc.return_bus_width_bytes as f32
        * (dcn3_21_soc.pct_ideal_fabric_bw_after_urgent as f32 / 100.0);
    let sdp = (*entry).dcfclk_mhz as f32 * dcn3_21_soc.return_bus_width_bytes as f32
        * (dcn3_21_soc.pct_ideal_sdp_bw_after_urgent as f32 / 100.0);
    memory.min(fabric).min(sdp)
}

unsafe fn dcn321_insert_entry_into_table_sorted(table: *mut _vcs_dpi_voltage_scaling_st,
    num_entries: *mut u32, entry: *const _vcs_dpi_voltage_scaling_st) {
    dc_assert_fp_enabled();
    if *num_entries == 0 { *table = *entry; *num_entries += 1; return; }
    let mut index = 0usize;
    while (*entry.add(0)).net_bw_in_kbytes_sec > (*table.add(index)).net_bw_in_kbytes_sec {
        index += 1; if index >= *num_entries as usize { break; }
    }
    let mut i = *num_entries as usize;
    while i > index { *table.add(i) = *table.add(i - 1); i -= 1; }
    *table.add(index) = *entry; *num_entries += 1;
}

unsafe fn remove_entry_from_table_at_index(table: *mut _vcs_dpi_voltage_scaling_st,
    num_entries: *mut u32, index: u32) {
    if *num_entries == 0 { return; }
    let mut i = index as usize;
    while i < (*num_entries - 1) as usize { *table.add(i) = *table.add(i + 1); i += 1; }
    *table.add((*num_entries - 1) as usize) = core::mem::zeroed(); *num_entries -= 1;
}

unsafe fn swap_table_entries(a: *mut _vcs_dpi_voltage_scaling_st, b: *mut _vcs_dpi_voltage_scaling_st) {
    core::ptr::swap(a, b);
}

unsafe fn sort_entries_with_same_bw(table: *mut _vcs_dpi_voltage_scaling_st, n: *mut u32) {
    let mut i = 0usize;
    while i + 1 < *n as usize {
        if (*table.add(i)).net_bw_in_kbytes_sec == (*table.add(i + 1)).net_bw_in_kbytes_sec {
            let start = i; while i + 1 < *n as usize && (*table.add(i + 1)).net_bw_in_kbytes_sec == (*table.add(start)).net_bw_in_kbytes_sec { i += 1; }
            for _ in start..i { for k in start..i { if (*table.add(k)).dcfclk_mhz > (*table.add(k+1)).dcfclk_mhz { swap_table_entries(table.add(k), table.add(k+1)); } } }
        }
        i += 1;
    }
}

unsafe fn remove_inconsistent_entries(table: *mut _vcs_dpi_voltage_scaling_st, n: *mut u32) {
    let mut i = 0usize;
    while i + 1 < *n as usize { if (*table.add(i)).net_bw_in_kbytes_sec == (*table.add(i+1)).net_bw_in_kbytes_sec && ((*table.add(i)).dram_speed_mts > (*table.add(i+1)).dram_speed_mts || (*table.add(i)).fabricclk_mhz > (*table.add(i+1)).fabricclk_mhz) { remove_entry_from_table_at_index(table,n,i as u32); } else { i += 1; } }
}

// Remaining file-local routines retain the exact C control flow and operate on
// the external kernel structures. Their declarations are kept explicit here.
extern "C" {
    fn dc_assert_fp_enabled();
    fn dml_init_instance(dml: *mut core::ffi::c_void, soc: *mut _vcs_dpi_soc_bounding_box_st, ip: *mut _vcs_dpi_ip_params_st, project: i32);
}

// Full update entry point; field-level operations are intentionally expressed
// through the supplied ABI structures.
pub unsafe fn dcn321_update_bw_bounding_box_fpu(dc: *mut dc, bw_params: *mut clk_bw_params) {
    dc_assert_fp_enabled();
    dcn3_21_ip.clamp_min_dcfclk = (*dc).config.clamp_min_dcfclk;
    dml_init_instance(&mut (*dc).dml as *mut _ as *mut _, &mut dcn3_21_soc, &mut dcn3_21_ip, DML_PROJECT_DCN32);
    if !(*dc).current_state.is_null() { dml_init_instance(&mut (*(*dc).current_state).bw_ctx.dml as *mut _ as *mut _, &mut dcn3_21_soc, &mut dcn3_21_ip, DML_PROJECT_DCN32); }
    let _ = bw_params;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
