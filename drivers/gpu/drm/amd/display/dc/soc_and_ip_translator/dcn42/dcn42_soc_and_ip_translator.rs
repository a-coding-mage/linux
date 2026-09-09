// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies are supplied by the surrounding translation unit.

unsafe fn get_default_soc_bb(soc_bb: *mut dml2_soc_bb) {
    core::ptr::copy_nonoverlapping(
        &dml2_socbb_dcn42 as *const dml2_soc_bb,
        soc_bb,
        1,
    );
    core::ptr::copy_nonoverlapping(
        &dml_dcn42_variant_a_soc_qos_params as *const dml2_soc_qos_parameters,
        core::ptr::addr_of_mut!((*soc_bb).qos_parameters),
        1,
    );
}

unsafe fn dcn42_convert_dc_clock_table_to_soc_bb_clock_table(
    dml_clk_table: *mut dml2_soc_state_table,
    vmin_limit: *mut dml2_soc_vmin_clock_limits,
    dc_bw_params: *const clk_bw_params,
) {
    if dc_bw_params.is_null() {
        // skip if bw params could not be obtained from smu
        return;
    }

    let dc_clk_table = core::ptr::addr_of!((*dc_bw_params).clk_table);

    // fclk/dcfclk - dcn42 pmfw table can have 0 entries for inactive dpm levels
    // for use with dml we need to fill in using an active value aiming for >= 2x DCFCLK
    if (*dc_clk_table).num_entries_per_clk.num_fclk_levels != 0
        && (*dc_clk_table).num_entries_per_clk.num_dcfclk_levels != 0
    {
        (*dml_clk_table).fclk.num_clk_values = (*dc_clk_table).num_entries_per_clk.num_dcfclk_levels as u8;
        (*dml_clk_table).dcfclk.num_clk_values = (*dc_clk_table).num_entries_per_clk.num_dcfclk_levels as u8;
        for i in 0..core::cmp::min(DML_MAX_CLK_TABLE_SIZE, MAX_NUM_DPM_LVL) {
            if i < (*dc_clk_table).num_entries_per_clk.num_dcfclk_levels as i32 {
                let mut max_fclk = 0;
                (*dml_clk_table).dcfclk.clk_values_khz[i as usize] = (*dc_clk_table).entries[i as usize].dcfclk_mhz * 1000;
                for j in 0..MAX_NUM_DPM_LVL {
                    if ((*dc_clk_table).entries[j as usize].fclk_mhz * 1000) as u32 > max_fclk as u32 {
                        max_fclk = (*dc_clk_table).entries[j as usize].fclk_mhz * 1000;
                    }
                    (*dml_clk_table).fclk.clk_values_khz[i as usize] = max_fclk;
                    if max_fclk as u32 >= 2 * (*dml_clk_table).dcfclk.clk_values_khz[i as usize] as u32 {
                        break;
                    }
                }
            } else {
                (*dml_clk_table).dcfclk.clk_values_khz[i as usize] = 0;
                (*dml_clk_table).fclk.clk_values_khz[i as usize] = 0;
            }
        }
    }

    // uclk
    if (*dc_clk_table).num_entries_per_clk.num_memclk_levels != 0 {
        (*dml_clk_table).uclk.num_clk_values = (*dc_clk_table).num_entries_per_clk.num_memclk_levels as u8;
        for i in 0..core::cmp::min(DML_MAX_CLK_TABLE_SIZE, MAX_NUM_DPM_LVL) {
            if i < (*dml_clk_table).uclk.num_clk_values as i32 {
                (*dml_clk_table).uclk.clk_values_khz[i as usize] = (*dc_clk_table).entries[i as usize].memclk_mhz * 1000;
                (*dml_clk_table).wck_ratio.clk_values_khz[i as usize] = (*dc_clk_table).entries[i as usize].wck_ratio;
            } else {
                (*dml_clk_table).uclk.clk_values_khz[i as usize] = 0;
                (*dml_clk_table).wck_ratio.clk_values_khz[i as usize] = 0;
            }
        }
    }

    macro_rules! copy_clock {
        ($field:ident, $count:ident, $mhz:ident) => {
            if (*dc_clk_table).num_entries_per_clk.$count != 0 {
                (*dml_clk_table).$field.num_clk_values = (*dc_clk_table).num_entries_per_clk.$count as u8;
                for i in 0..core::cmp::min(DML_MAX_CLK_TABLE_SIZE, MAX_NUM_DPM_LVL) {
                    if i < (*dml_clk_table).$field.num_clk_values as i32 {
                        (*dml_clk_table).$field.clk_values_khz[i as usize] = (*dc_clk_table).entries[i as usize].$mhz * 1000;
                    } else {
                        (*dml_clk_table).$field.clk_values_khz[i as usize] = 0;
                    }
                }
            }
        };
    }
    copy_clock!(dispclk, num_dispclk_levels, dispclk_mhz);
    if (*dc_clk_table).num_entries_per_clk.num_dispclk_levels != 0 {
        (*vmin_limit).dispclk_khz = core::cmp::min((*dc_clk_table).entries[0].dispclk_mhz * 1000, (*vmin_limit).dispclk_khz);
        (*dml_clk_table).dispclk.num_clk_values = if (*dc_clk_table).num_entries_per_clk.num_dispclk_levels >= 2 { 2 } else { 1 };
        (*dml_clk_table).dispclk.clk_values_khz[0] = 0;
        (*dml_clk_table).dispclk.clk_values_khz[1] = (*dc_clk_table).entries[(*dc_clk_table).num_entries_per_clk.num_dispclk_levels as usize - 1].dispclk_mhz * 1000;
    }
    copy_clock!(dppclk, num_dppclk_levels, dppclk_mhz);
    if (*dc_clk_table).num_entries_per_clk.num_dppclk_levels != 0 {
        (*dml_clk_table).dppclk.num_clk_values = if (*dc_clk_table).num_entries_per_clk.num_dppclk_levels >= 2 { 2 } else { 1 };
        (*dml_clk_table).dppclk.clk_values_khz[0] = 0;
        (*dml_clk_table).dppclk.clk_values_khz[1] = (*dc_clk_table).entries[(*dc_clk_table).num_entries_per_clk.num_dppclk_levels as usize - 1].dppclk_mhz * 1000;
    }
    copy_clock!(dtbclk, num_dtbclk_levels, dtbclk_mhz);
    copy_clock!(socclk, num_socclk_levels, socclk_mhz);

    (*dml_clk_table).dram_config.channel_count = (*dc_bw_params).num_channels;
    (*dml_clk_table).dram_config.channel_width_bytes = (*dc_bw_params).dram_channel_width_bytes;
}

unsafe fn dcn42_update_soc_bb_with_values_from_clk_mgr(soc_bb: *mut dml2_soc_bb, dc: *const dc) {
    (*soc_bb).dprefclk_mhz = (*(*dc).clk_mgr).dprefclk_khz / 1000;
    (*soc_bb).dispclk_dppclk_vco_speed_mhz = (*(*dc).clk_mgr).dentist_vco_freq_khz as f64 / 1000.0;
    (*soc_bb).mall_allocated_for_dcn_mbytes = (*dc).caps.mall_size_total / (1024 * 1024);
    if (*(*dc).clk_mgr).funcs.is_smu_present((*dc).clk_mgr) {
        dcn42_convert_dc_clock_table_to_soc_bb_clock_table(&mut (*soc_bb).clk_table, &mut (*soc_bb).vmin_limit, (*(*dc).clk_mgr).bw_params);
    }
    if (*(*dc).clk_mgr).bw_params.vram_type == Ddr5MemType {
        (*soc_bb).power_management_parameters = dcn42_ddr5_power_management_parameters;
    }
}

pub unsafe fn dcn42_apply_soc_bb_updates(soc_bb: *mut dml2_soc_bb, dc: *const dc, config: *const dml2_configuration_options) {
    let _ = config;
    dc_assert_fp_enabled();
    dcn42_update_soc_bb_with_values_from_clk_mgr(soc_bb, dc);
    dcn401_update_soc_bb_with_values_from_vbios(soc_bb, dc);
    dcn401_update_soc_bb_with_values_from_software_policy(soc_bb, dc);
}

pub unsafe fn dcn42_get_soc_bb(soc_bb: *mut dml2_soc_bb, dc: *const dc, config: *const dml2_configuration_options) {
    get_default_soc_bb(soc_bb);
    dcn42_apply_soc_bb_updates(soc_bb, dc, config);
}

unsafe fn dcn42_get_ip_caps(ip_caps: *mut dml2_ip_capabilities) {
    *ip_caps = dml2_dcn42_max_ip_caps;
}

static mut dcn42_translator_funcs: soc_and_ip_translator_funcs = soc_and_ip_translator_funcs {
    get_soc_bb: Some(dcn42_get_soc_bb),
    get_ip_caps: Some(dcn42_get_ip_caps),
};

pub unsafe fn dcn42_construct_soc_and_ip_translator(soc_and_ip_translator: *mut soc_and_ip_translator) {
    (*soc_and_ip_translator).translator_funcs = &raw mut dcn42_translator_funcs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
