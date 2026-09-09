// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Translated from dcn60_soc_and_ip_translator.c. Declarations supplied by the
// corresponding C headers are intentionally left as external dependencies.

unsafe extern "C" {
    fn dc_assert_fp_enabled();
    fn dcn401_update_soc_bb_with_values_from_clk_mgr(
        soc_bb: *mut dml2_soc_bb,
        dc: *const dc,
        config: *const dml2_configuration_options,
    );
    fn dcn401_update_soc_bb_with_values_from_vbios(soc_bb: *mut dml2_soc_bb, dc: *const dc);
    fn dcn401_update_soc_bb_with_values_from_software_policy(
        soc_bb: *mut dml2_soc_bb,
        dc: *const dc,
    );
    fn dcn6_test_initialize_soc_bb(soc_bb: *mut dml2_soc_bb);
    fn dcn6_test_initialize_ip_caps(ip_caps: *mut dml2_ip_capabilities);
}

unsafe fn dcn60_update_soc_bb_with_values_from_dmub(
    soc_bb: *mut dml2_soc_bb,
    config: *const dml2_configuration_options,
) {
    let dmub_bb_params = (*config).bb_from_dmub as *const dmub_soc_bb_params;
    let mut min_alt_ch_carveout_size_mb: u32 = 0;

    if dmub_bb_params.is_null() {
        return;
    }

    macro_rules! update_us {
        ($field:ident, $target:ident) => {
            if (*dmub_bb_params).$field > 0 {
                (*soc_bb).power_management_parameters.$target =
                    (*dmub_bb_params).$field as f64 / 1000.0;
            }
        };
    }

    update_us!(dram_clk_change_blackout_ns, dram_clk_change_blackout_us);
    update_us!(dram_clk_change_read_only_ns, dram_clk_change_read_only_us);
    update_us!(dram_clk_change_write_only_ns, dram_clk_change_write_only_us);
    update_us!(fclk_change_blackout_ns, fclk_change_blackout_us);
    update_us!(g7_ppt_blackout_ns, g7_ppt_blackout_us);
    update_us!(stutter_enter_plus_exit_latency_ns, stutter_enter_plus_exit_latency_us);
    update_us!(stutter_exit_latency_ns, stutter_exit_latency_us);
    update_us!(z8_stutter_enter_plus_exit_latency_ns, z8_stutter_enter_plus_exit_latency_us);
    update_us!(z8_stutter_exit_latency_ns, z8_stutter_exit_latency_us);
    update_us!(z8_min_idle_time_ns, z8_min_idle_time);
    update_us!(type_b_dram_clk_change_blackout_ns, type_b_dram_clk_change_blackout_us);
    update_us!(type_b_ppt_blackout_ns, type_b_ppt_blackout_us);
    update_us!(g7_temperature_read_blackout_ns, g7_temperature_read_blackout_us);

    if (*dmub_bb_params).vmin_limit_dispclk_khz > 0 {
        (*soc_bb).vmin_limit.dispclk_khz = (*dmub_bb_params).vmin_limit_dispclk_khz;
    }
    if (*dmub_bb_params).vmin_limit_dcfclk_khz > 0 {
        (*soc_bb).vmin_limit.dcfclk_khz = (*dmub_bb_params).vmin_limit_dcfclk_khz;
    }

    for i in 0..2 {
        let carveout = (*config).alt_ch_cfg.region_size_bytes[i] >> 20;
        if min_alt_ch_carveout_size_mb > carveout {
            min_alt_ch_carveout_size_mb = carveout;
        }
    }
    if min_alt_ch_carveout_size_mb > 0 {
        (*soc_bb).power_management_parameters.alternate_dram_carveout_size_mb =
            min_alt_ch_carveout_size_mb;
    }
}

unsafe fn apply_soc_bb_updates(
    soc_bb: *mut dml2_soc_bb,
    dc: *const dc,
    config: *const dml2_configuration_options,
) {
    dc_assert_fp_enabled();
    dcn60_update_soc_bb_with_values_from_dmub(soc_bb, config);
    dcn401_update_soc_bb_with_values_from_clk_mgr(soc_bb, dc, config);
    dcn401_update_soc_bb_with_values_from_vbios(soc_bb, dc);
    dcn401_update_soc_bb_with_values_from_software_policy(soc_bb, dc);
}

unsafe fn dcn60_get_soc_bb(
    soc_bb: *mut dml2_soc_bb,
    dc: *const dc,
    config: *const dml2_configuration_options,
) {
    dcn6_test_initialize_soc_bb(soc_bb);
    apply_soc_bb_updates(soc_bb, dc, config);
}

unsafe fn dcn60_get_ip_caps(ip_caps: *mut dml2_ip_capabilities) {
    dcn6_test_initialize_ip_caps(ip_caps);
}

static mut dcn60_translator_funcs: soc_and_ip_translator_funcs = soc_and_ip_translator_funcs {
    get_soc_bb: Some(dcn60_get_soc_bb),
    get_ip_caps: Some(dcn60_get_ip_caps),
};

pub unsafe fn dcn60_construct_soc_and_ip_translator(
    soc_and_ip_translator: *mut soc_and_ip_translator,
) {
    (*soc_and_ip_translator).translator_funcs = &raw mut dcn60_translator_funcs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
