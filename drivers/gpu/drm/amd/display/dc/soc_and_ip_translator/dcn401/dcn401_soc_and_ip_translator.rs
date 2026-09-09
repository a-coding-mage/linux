// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit.

unsafe fn get_default_soc_bb(soc_bb: *mut dml2_soc_bb) {
    core::ptr::copy_nonoverlapping(
        &dml2_socbb_dcn401 as *const dml2_soc_bb,
        soc_bb,
        1,
    );
    core::ptr::copy_nonoverlapping(
        &dml_dcn4_variant_a_soc_qos_params as *const dml2_soc_qos_parameters,
        &mut (*soc_bb).qos_parameters,
        1,
    );
}

unsafe fn dcn401_convert_dc_clock_table_to_soc_bb_clock_table(
    dml_clk_table: *mut dml2_soc_state_table,
    dc_bw_params: *const clk_bw_params,
    use_clock_dc_limits: bool,
) {
    if dc_bw_params.is_null() {
        return;
    }

    let dc_clk_table = &(*dc_bw_params).clk_table;
    let limit = core::cmp::min(DML_MAX_CLK_TABLE_SIZE, MAX_NUM_DPM_LVL);

    macro_rules! convert_clock {
        ($count:ident, $clock:ident, $limit_field:ident, $entry_field:ident) => {
            if dc_clk_table.num_entries_per_clk.$count != 0 {
                (*dml_clk_table).$clock.num_clk_values = dc_clk_table.num_entries_per_clk.$count as u8;
                for i in 0..limit {
                    if i < (*dml_clk_table).$clock.num_clk_values as usize {
                        if use_clock_dc_limits
                            && (*dc_bw_params).dc_mode_limit.$limit_field != 0
                            && dc_clk_table.entries[i].$entry_field > (*dc_bw_params).dc_mode_limit.$limit_field
                        {
                            if i == 0 || dc_clk_table.entries[i - 1].$entry_field < (*dc_bw_params).dc_mode_limit.$limit_field {
                                (*dml_clk_table).$clock.clk_values_khz[i] = (*dc_bw_params).dc_mode_limit.$limit_field * 1000;
                                (*dml_clk_table).$clock.num_clk_values = (i + 1) as u8;
                            } else {
                                (*dml_clk_table).$clock.clk_values_khz[i] = 0;
                                (*dml_clk_table).$clock.num_clk_values = i as u8;
                            }
                        } else {
                            (*dml_clk_table).$clock.clk_values_khz[i] = dc_clk_table.entries[i].$entry_field * 1000;
                        }
                    } else {
                        (*dml_clk_table).$clock.clk_values_khz[i] = 0;
                    }
                }
            }
        };
    }

    convert_clock!(num_dcfclk_levels, dcfclk, dcfclk_mhz, dcfclk_mhz);
    convert_clock!(num_fclk_levels, fclk, fclk_mhz, fclk_mhz);
    convert_clock!(num_memclk_levels, uclk, memclk_mhz, memclk_mhz);
    convert_clock!(num_dispclk_levels, dispclk, dispclk_mhz, dispclk_mhz);
    convert_clock!(num_dppclk_levels, dppclk, dppclk_mhz, dppclk_mhz);
    convert_clock!(num_dtbclk_levels, dtbclk, dtbclk_mhz, dtbclk_mhz);
    convert_clock!(num_socclk_levels, socclk, socclk_mhz, socclk_mhz);

    (*dml_clk_table).dram_config.channel_count = (*dc_bw_params).num_channels;
    (*dml_clk_table).dram_config.channel_width_bytes = (*dc_bw_params).dram_channel_width_bytes;
}

pub unsafe fn dcn401_update_soc_bb_with_values_from_clk_mgr(
    soc_bb: *mut dml2_soc_bb, dc: *const dc, config: *const dml2_configuration_options,
) {
    (*soc_bb).dprefclk_mhz = (*dc).clk_mgr.dprefclk_khz / 1000;
    (*soc_bb).dispclk_dppclk_vco_speed_mhz = (*dc).clk_mgr.dentist_vco_freq_khz as f64 / 1000.0;
    (*soc_bb).mall_allocated_for_dcn_mbytes = (*dc).caps.mall_size_total / (1024 * 1024);
    if (*dc).clk_mgr.funcs.is_smu_present && ((*dc).clk_mgr.funcs.is_smu_present)((*dc).clk_mgr) {
        dcn401_convert_dc_clock_table_to_soc_bb_clock_table(&mut (*soc_bb).clk_table, (*dc).clk_mgr.bw_params, (*config).use_clock_dc_limits);
    }
}

pub unsafe fn dcn401_update_soc_bb_with_values_from_vbios(soc_bb: *mut dml2_soc_bb, dc: *const dc) {
    (*soc_bb).dchub_refclk_mhz = (*dc).res_pool.ref_clocks.dchub_ref_clock_inKhz / 1000;
    (*soc_bb).xtalclk_mhz = (*dc).ctx.dc_bios.fw_info.pll_info.crystal_frequency / 1000;
    if (*dc).ctx.dc_bios.bb_info.dram_clock_change_latency_100ns != 0 { (*soc_bb).power_management_parameters.dram_clk_change_blackout_us = (*dc).ctx.dc_bios.bb_info.dram_clock_change_latency_100ns as f64 / 10.0; }
    if (*dc).ctx.dc_bios.bb_info.dram_sr_enter_exit_latency_100ns != 0 { (*soc_bb).power_management_parameters.stutter_enter_plus_exit_latency_us = (*dc).ctx.dc_bios.bb_info.dram_sr_enter_exit_latency_100ns as f64 / 10.0; }
    if (*dc).ctx.dc_bios.bb_info.dram_sr_exit_latency_100ns != 0 { (*soc_bb).power_management_parameters.stutter_exit_latency_us = (*dc).ctx.dc_bios.bb_info.dram_sr_exit_latency_100ns as f64 / 10.0; }
}

pub unsafe fn dcn401_update_soc_bb_with_values_from_software_policy(soc_bb: *mut dml2_soc_bb, dc: *const dc) {
    macro_rules! override_us { ($src:ident, $dst:ident) => { if (*dc).bb_overrides.$src != 0 { (*soc_bb).power_management_parameters.$dst = (*dc).bb_overrides.$src as f64 / 1000.0; } }; }
    override_us!(sr_exit_time_ns, stutter_exit_latency_us);
    override_us!(sr_enter_plus_exit_time_ns, stutter_enter_plus_exit_latency_us);
    override_us!(dram_clock_change_latency_ns, dram_clk_change_blackout_us);
    override_us!(fclk_clock_change_latency_ns, fclk_change_blackout_us);
    override_us!(sr_exit_z8_time_ns, z8_stutter_exit_latency_us);
    override_us!(sr_enter_plus_exit_z8_time_ns, z8_stutter_enter_plus_exit_latency_us);
    for i in 0..(*dc).debug.dml21_custom_derate_num_dpms as usize {
        let v = (*dc).debug.dml21_custom_derate_at_dpm[i];
        (*soc_bb).qos_parameters.derate_table_per_dpm.system_active_derates_per_dpm.dram_derate_percent_pixel[i] = v & 0xff;
        (*soc_bb).qos_parameters.derate_table_per_dpm.system_active_derates_per_dpm.fclk_derate_percent[i] = (v >> 8) & 0xff;
        (*soc_bb).qos_parameters.derate_table_per_dpm.system_active_derates_per_dpm.dcfclk_derate_percent[i] = (v >> 16) & 0xff;
    }
}

unsafe fn apply_soc_bb_updates(soc_bb: *mut dml2_soc_bb, dc: *const dc, config: *const dml2_configuration_options) {
    dc_assert_fp_enabled();
    dcn401_update_soc_bb_with_values_from_clk_mgr(soc_bb, dc, config);
    dcn401_update_soc_bb_with_values_from_vbios(soc_bb, dc);
    dcn401_update_soc_bb_with_values_from_software_policy(soc_bb, dc);
}

pub unsafe fn dcn401_get_soc_bb(soc_bb: *mut dml2_soc_bb, dc: *const dc, config: *const dml2_configuration_options) {
    get_default_soc_bb(soc_bb);
    apply_soc_bb_updates(soc_bb, dc, config);
}

unsafe fn dcn401_get_ip_caps(ip_caps: *mut dml2_ip_capabilities) { *ip_caps = dml2_dcn401_max_ip_caps; }

static mut dcn401_translator_funcs: soc_and_ip_translator_funcs = soc_and_ip_translator_funcs {
    get_soc_bb: dcn401_get_soc_bb,
    get_ip_caps: dcn401_get_ip_caps,
};

pub unsafe fn dcn401_construct_soc_and_ip_translator(soc_and_ip_translator: *mut soc_and_ip_translator) {
    (*soc_and_ip_translator).translator_funcs = &mut dcn401_translator_funcs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
