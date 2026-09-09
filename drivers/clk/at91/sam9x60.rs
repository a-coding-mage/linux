// SPDX-License-Identifier: GPL-2.0
// Translated from sam9x60.c. Kernel headers and symbols are supplied externally.

static pmc_pll_lock: spinlock_t = DEFINE_SPINLOCK!();
static mck_lock: spinlock_t = DEFINE_SPINLOCK!();

static mck_characteristics: clk_master_characteristics = clk_master_characteristics {
    output: clk_range { min: 140000000, max: 200000000 },
    divisors: [1, 2, 4, 3],
    have_div3_pres: 1,
};

static sam9x60_master_layout: clk_master_layout = clk_master_layout {
    mask: 0x373, pres_shift: 4, offset: 0x28,
};

static plla_outputs: [clk_range; 1] = [clk_range { min: 2343750, max: 1200000000 }];
static core_outputs: [clk_range; 1] = [clk_range { min: 600000000, max: 1200000000 }];

static plla_characteristics: clk_pll_characteristics = clk_pll_characteristics {
    input: clk_range { min: 12000000, max: 48000000 },
    num_output: ARRAY_SIZE!(plla_outputs), output: plla_outputs.as_ptr(),
    core_output: core_outputs.as_ptr(), acr: 0x00020010,
};

static upll_outputs: [clk_range; 1] = [clk_range { min: 300000000, max: 500000000 }];
static upll_characteristics: clk_pll_characteristics = clk_pll_characteristics {
    input: clk_range { min: 12000000, max: 48000000 },
    num_output: ARRAY_SIZE!(upll_outputs), output: upll_outputs.as_ptr(),
    core_output: core_outputs.as_ptr(), upll: true, acr: 0x12023010,
};

static pll_frac_layout: clk_pll_layout = clk_pll_layout {
    mul_mask: GENMASK!(31, 24), frac_mask: GENMASK!(21, 0), mul_shift: 24, frac_shift: 0,
};
static pll_div_layout: clk_pll_layout = clk_pll_layout {
    div_mask: GENMASK!(7, 0), endiv_mask: BIT!(29), div_shift: 0, endiv_shift: 29,
};
static sam9x60_programmable_layout: clk_programmable_layout = clk_programmable_layout {
    pres_mask: 0xff, pres_shift: 8, css_mask: 0x1f, have_slck_mck: 0, is_pres_direct: 1,
};
static sam9x60_pcr_layout: clk_pcr_layout = clk_pcr_layout {
    offset: 0x88, cmd: BIT!(31), gckcss_mask: GENMASK!(12, 8), pid_mask: GENMASK!(6, 0),
};

static sam9x60_systemck: [clk_system; 5] = [
    clk_system { n: "ddrck", p: "masterck_div", id: 2, flags: CLK_IS_CRITICAL },
    clk_system { n: "uhpck", p: "usbck", id: 6, flags: 0 },
    clk_system { n: "pck0", p: "prog0", id: 8, flags: 0 },
    clk_system { n: "pck1", p: "prog1", id: 9, flags: 0 },
    clk_system { n: "qspick", p: "masterck_div", id: 19, flags: 0 },
];

static sam9x60_periphck: [clk_periph; 48] = [
    clk_periph { n: "pioA_clk", id: 2, flags: 0 }, clk_periph { n: "pioB_clk", id: 3, flags: 0 },
    clk_periph { n: "pioC_clk", id: 4, flags: 0 }, clk_periph { n: "flex0_clk", id: 5, flags: 0 },
    clk_periph { n: "flex1_clk", id: 6, flags: 0 }, clk_periph { n: "flex2_clk", id: 7, flags: 0 },
    clk_periph { n: "flex3_clk", id: 8, flags: 0 }, clk_periph { n: "flex6_clk", id: 9, flags: 0 },
    clk_periph { n: "flex7_clk", id: 10, flags: 0 }, clk_periph { n: "flex8_clk", id: 11, flags: 0 },
    clk_periph { n: "sdmmc0_clk", id: 12, flags: 0 }, clk_periph { n: "flex4_clk", id: 13, flags: 0 },
    clk_periph { n: "flex5_clk", id: 14, flags: 0 }, clk_periph { n: "flex9_clk", id: 15, flags: 0 },
    clk_periph { n: "flex10_clk", id: 16, flags: 0 }, clk_periph { n: "tcb0_clk", id: 17, flags: 0 },
    clk_periph { n: "pwm_clk", id: 18, flags: 0 }, clk_periph { n: "adc_clk", id: 19, flags: 0 },
    clk_periph { n: "dma0_clk", id: 20, flags: 0 }, clk_periph { n: "matrix_clk", id: 21, flags: 0 },
    clk_periph { n: "uhphs_clk", id: 22, flags: 0 }, clk_periph { n: "udphs_clk", id: 23, flags: 0 },
    clk_periph { n: "macb0_clk", id: 24, flags: 0 }, clk_periph { n: "lcd_clk", id: 25, flags: 0 },
    clk_periph { n: "sdmmc1_clk", id: 26, flags: 0 }, clk_periph { n: "macb1_clk", id: 27, flags: 0 },
    clk_periph { n: "ssc_clk", id: 28, flags: 0 }, clk_periph { n: "can0_clk", id: 29, flags: 0 },
    clk_periph { n: "can1_clk", id: 30, flags: 0 }, clk_periph { n: "flex11_clk", id: 32, flags: 0 },
    clk_periph { n: "flex12_clk", id: 33, flags: 0 }, clk_periph { n: "i2s_clk", id: 34, flags: 0 },
    clk_periph { n: "qspi_clk", id: 35, flags: 0 }, clk_periph { n: "gfx2d_clk", id: 36, flags: 0 },
    clk_periph { n: "pit64b_clk", id: 37, flags: 0 }, clk_periph { n: "trng_clk", id: 38, flags: 0 },
    clk_periph { n: "aes_clk", id: 39, flags: 0 }, clk_periph { n: "tdes_clk", id: 40, flags: 0 },
    clk_periph { n: "sha_clk", id: 41, flags: 0 }, clk_periph { n: "classd_clk", id: 42, flags: 0 },
    clk_periph { n: "isi_clk", id: 43, flags: 0 }, clk_periph { n: "pioD_clk", id: 44, flags: 0 },
    clk_periph { n: "tcb1_clk", id: 45, flags: 0 }, clk_periph { n: "dbgu_clk", id: 47, flags: 0 },
    clk_periph { n: "mpddr_clk", id: 49, flags: CLK_IS_CRITICAL },
];

static sam9x60_gck: [clk_gck; 20] = [
    clk_gck { n: "flex0_gclk", id: 5, r: clk_range { min: 0, max: 0 } }, clk_gck { n: "flex1_gclk", id: 6, r: clk_range { min: 0, max: 0 } },
    clk_gck { n: "flex2_gclk", id: 7, r: clk_range { min: 0, max: 0 } }, clk_gck { n: "flex3_gclk", id: 8, r: clk_range { min: 0, max: 0 } },
    clk_gck { n: "flex6_gclk", id: 9, r: clk_range { min: 0, max: 0 } }, clk_gck { n: "flex7_gclk", id: 10, r: clk_range { min: 0, max: 0 } },
    clk_gck { n: "flex8_gclk", id: 11, r: clk_range { min: 0, max: 0 } }, clk_gck { n: "sdmmc0_gclk", id: 12, r: clk_range { min: 0, max: 105000000 } },
    clk_gck { n: "flex4_gclk", id: 13, r: clk_range { min: 0, max: 0 } }, clk_gck { n: "flex5_gclk", id: 14, r: clk_range { min: 0, max: 0 } },
    clk_gck { n: "flex9_gclk", id: 15, r: clk_range { min: 0, max: 0 } }, clk_gck { n: "flex10_gclk", id: 16, r: clk_range { min: 0, max: 0 } },
    clk_gck { n: "tcb0_gclk", id: 17, r: clk_range { min: 0, max: 0 } }, clk_gck { n: "adc_gclk", id: 19, r: clk_range { min: 0, max: 0 } },
    clk_gck { n: "lcd_gclk", id: 25, r: clk_range { min: 0, max: 140000000 } }, clk_gck { n: "sdmmc1_gclk", id: 26, r: clk_range { min: 0, max: 105000000 } },
    clk_gck { n: "flex11_gclk", id: 32, r: clk_range { min: 0, max: 0 } }, clk_gck { n: "flex12_gclk", id: 33, r: clk_range { min: 0, max: 0 } },
    clk_gck { n: "i2s_gclk", id: 34, r: clk_range { min: 0, max: 105000000 } }, clk_gck { n: "pit64b_gclk", id: 37, r: clk_range { min: 0, max: 0 } },
    clk_gck { n: "classd_gclk", id: 42, r: clk_range { min: 0, max: 100000000 } }, clk_gck { n: "tcb1_gclk", id: 45, r: clk_range { min: 0, max: 0 } },
    clk_gck { n: "dbgu_gclk", id: 47, r: clk_range { min: 0, max: 0 } },
];

unsafe fn sam9x60_pmc_setup(np: *mut device_node) {
    let mut range = CLK_RANGE!(0, 0);
    let (td_slck_name, md_slck_name, mainxtal_name): (*const c_char, *const c_char, *const c_char);
    let sam9x60_pmc: *mut pmc_data;
    let mut parent_names: [*const c_char; 6] = [core::ptr::null(); 6];
    let main_osc_hw: *mut clk_hw;
    let mut regmap: *mut regmap;
    let mut hw: *mut clk_hw;
    let mut i: c_int;

    i = of_property_match_string(np, "clock-names", "td_slck"); if i < 0 { return; }
    td_slck_name = of_clk_get_parent_name(np, i);
    i = of_property_match_string(np, "clock-names", "md_slck"); if i < 0 { return; }
    md_slck_name = of_clk_get_parent_name(np, i);
    i = of_property_match_string(np, "clock-names", "main_xtal"); if i < 0 { return; }
    mainxtal_name = of_clk_get_parent_name(np, i);
    regmap = device_node_to_regmap(np); if IS_ERR!(regmap) { return; }
    sam9x60_pmc = pmc_data_allocate!(PMC_PLLACK + 1, nck!(sam9x60_systemck), nck!(sam9x60_periphck), nck!(sam9x60_gck), 8);
    if sam9x60_pmc.is_null() { return; }

    hw = at91_clk_register_main_rc_osc(regmap, "main_rc_osc", 12000000, 50000000); if IS_ERR!(hw) { goto_err_free!(); }
    hw = at91_clk_register_main_osc(regmap, "main_osc", mainxtal_name, core::ptr::null(), 0); if IS_ERR!(hw) { goto_err_free!(); }
    main_osc_hw = hw;
    parent_names[0] = cstr!("main_rc_osc"); parent_names[1] = cstr!("main_osc");
    hw = at91_clk_register_sam9x5_main(regmap, "mainck", parent_names.as_ptr(), core::ptr::null(), 2); if IS_ERR!(hw) { goto_err_free!(); }
    (*sam9x60_pmc).chws[PMC_MAIN] = hw;
    hw = sam9x60_clk_register_frac_pll(regmap, &mut pmc_pll_lock, "pllack_fracck", "mainck", (*sam9x60_pmc).chws[PMC_MAIN], 0, &plla_characteristics, &pll_frac_layout, CLK_IS_CRITICAL | CLK_SET_RATE_GATE); if IS_ERR!(hw) { goto_err_free!(); }
    hw = sam9x60_clk_register_div_pll(regmap, &mut pmc_pll_lock, "pllack_divck", "pllack_fracck", core::ptr::null_mut(), 0, &plla_characteristics, &pll_div_layout, CLK_IS_CRITICAL | CLK_SET_RATE_GATE, 0); if IS_ERR!(hw) { goto_err_free!(); }
    (*sam9x60_pmc).chws[PMC_PLLACK] = hw;
    hw = sam9x60_clk_register_frac_pll(regmap, &mut pmc_pll_lock, "upllck_fracck", "main_osc", main_osc_hw, 1, &upll_characteristics, &pll_frac_layout, CLK_SET_RATE_GATE); if IS_ERR!(hw) { goto_err_free!(); }
    hw = sam9x60_clk_register_div_pll(regmap, &mut pmc_pll_lock, "upllck_divck", "upllck_fracck", core::ptr::null_mut(), 1, &upll_characteristics, &pll_div_layout, CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE | CLK_SET_RATE_PARENT, 0); if IS_ERR!(hw) { goto_err_free!(); }
    (*sam9x60_pmc).chws[PMC_UTMI] = hw;
    parent_names[0] = md_slck_name; parent_names[1] = cstr!("mainck"); parent_names[2] = cstr!("pllack_divck");
    hw = at91_clk_register_master_pres(regmap, "masterck_pres", 3, parent_names.as_ptr(), core::ptr::null(), &sam9x60_master_layout, &mck_characteristics, &mut mck_lock); if IS_ERR!(hw) { goto_err_free!(); }
    hw = at91_clk_register_master_div(regmap, "masterck_div", "masterck_pres", core::ptr::null(), &sam9x60_master_layout, &mck_characteristics, &mut mck_lock, CLK_SET_RATE_GATE, 0); if IS_ERR!(hw) { goto_err_free!(); }
    (*sam9x60_pmc).chws[PMC_MCK] = hw;
    parent_names[0] = cstr!("pllack_divck"); parent_names[1] = cstr!("upllck_divck"); parent_names[2] = cstr!("main_osc");
    hw = sam9x60_clk_register_usb(regmap, "usbck", parent_names.as_ptr(), 3); if IS_ERR!(hw) { goto_err_free!(); }
    parent_names[0] = md_slck_name; parent_names[1] = td_slck_name; parent_names[2] = cstr!("mainck"); parent_names[3] = cstr!("masterck_div"); parent_names[4] = cstr!("pllack_divck"); parent_names[5] = cstr!("upllck_divck");
    for i in 0..2 { let name = format!("prog{}", i); hw = at91_clk_register_programmable(regmap, name.as_ptr() as *const c_char, parent_names.as_ptr(), core::ptr::null(), 6, i, &sam9x60_programmable_layout, core::ptr::null()); if IS_ERR!(hw) { goto_err_free!(); } (*sam9x60_pmc).pchws[i as usize] = hw; }
    for i in 0..ARRAY_SIZE!(sam9x60_systemck) { hw = at91_clk_register_system(regmap, sam9x60_systemck[i].n, sam9x60_systemck[i].p, core::ptr::null(), sam9x60_systemck[i].id, sam9x60_systemck[i].flags); if IS_ERR!(hw) { goto_err_free!(); } (*sam9x60_pmc).shws[sam9x60_systemck[i].id as usize] = hw; }
    for i in 0..ARRAY_SIZE!(sam9x60_periphck) { hw = at91_clk_register_sam9x5_peripheral(regmap, &mut pmc_pcr_lock, &sam9x60_pcr_layout, sam9x60_periphck[i].n, "masterck_div", core::ptr::null(), sam9x60_periphck[i].id, &mut range, INT_MIN, sam9x60_periphck[i].flags); if IS_ERR!(hw) { goto_err_free!(); } (*sam9x60_pmc).phws[sam9x60_periphck[i].id as usize] = hw; }
    for i in 0..ARRAY_SIZE!(sam9x60_gck) { hw = at91_clk_register_generated(regmap, &mut pmc_pcr_lock, &sam9x60_pcr_layout, sam9x60_gck[i].n, parent_names.as_ptr(), core::ptr::null(), core::ptr::null(), 6, sam9x60_gck[i].id, &sam9x60_gck[i].r, INT_MIN); if IS_ERR!(hw) { goto_err_free!(); } (*sam9x60_pmc).ghws[sam9x60_gck[i].id as usize] = hw; }
    of_clk_add_hw_provider(np, of_clk_hw_pmc_get, sam9x60_pmc); return;
    goto_err_free: kfree(sam9x60_pmc);
}

// Some clks are used for a clocksource.
CLK_OF_DECLARE!(sam9x60_pmc, "microchip,sam9x60-pmc", sam9x60_pmc_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
