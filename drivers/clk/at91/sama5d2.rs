// SPDX-License-Identifier: GPL-2.0
// External kernel types, constants, macros, and functions are supplied by dependencies.

static mut MCK_LOCK: spinlock_t = spinlock_t::new();

static MCK_CHARACTERISTICS: clk_master_characteristics = clk_master_characteristics {
    output: clk_range { min: 124000000, max: 166000000 },
    divisors: [1, 2, 4, 3],
};

static mut PLLA_OUT: [u8; 1] = [0];
static mut PLLA_ICPLL: [u16; 1] = [0];
static PLLA_OUTPUTS: [clk_range; 1] = [clk_range { min: 600000000, max: 1200000000 }];
static PLLA_CHARACTERISTICS: clk_pll_characteristics = clk_pll_characteristics {
    input: clk_range { min: 12000000, max: 24000000 },
    num_output: 1,
    output: unsafe { &PLLA_OUTPUTS },
    icpll: unsafe { &PLLA_ICPLL },
    out: unsafe { &PLLA_OUT },
};

static SAMA5D2_PCR_LAYOUT: clk_pcr_layout = clk_pcr_layout {
    offset: 0x10c, cmd: BIT(12), gckcss_mask: GENMASK(10, 8), pid_mask: GENMASK(6, 0),
};

#[repr(C)]
struct Sama5d2Systemck { n: *mut c_char, p: *mut c_char, flags: c_ulong, id: u8 }
static SAMA5D2_SYSTEMCK: [Sama5d2Systemck; 8] = [
    Sama5d2Systemck { n: cstr!("ddrck"), p: cstr!("masterck_div"), id: 2, flags: CLK_IS_CRITICAL },
    Sama5d2Systemck { n: cstr!("lcdck"), p: cstr!("masterck_div"), id: 3, flags: 0 },
    Sama5d2Systemck { n: cstr!("uhpck"), p: cstr!("usbck"), id: 6, flags: 0 },
    Sama5d2Systemck { n: cstr!("udpck"), p: cstr!("usbck"), id: 7, flags: 0 },
    Sama5d2Systemck { n: cstr!("pck0"), p: cstr!("prog0"), id: 8, flags: 0 },
    Sama5d2Systemck { n: cstr!("pck1"), p: cstr!("prog1"), id: 9, flags: 0 },
    Sama5d2Systemck { n: cstr!("pck2"), p: cstr!("prog2"), id: 10, flags: 0 },
    Sama5d2Systemck { n: cstr!("iscck"), p: cstr!("masterck_div"), id: 18, flags: 0 },
];

#[repr(C)] struct Sama5d2Periph32ck { n: *mut c_char, id: u8, r: clk_range }
static SAMA5D2_PERIPH32CK: [Sama5d2Periph32ck; 34] = [
    Sama5d2Periph32ck { n: cstr!("macb0_clk"), id: 5, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("tdes_clk"), id: 11, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("matrix1_clk"), id: 14, r: clk_range { min: 0, max: 0 } },
    Sama5d2Periph32ck { n: cstr!("hsmc_clk"), id: 17, r: clk_range { min: 0, max: 0 } },
    Sama5d2Periph32ck { n: cstr!("pioA_clk"), id: 18, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("flx0_clk"), id: 19, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("flx1_clk"), id: 20, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("flx2_clk"), id: 21, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("flx3_clk"), id: 22, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("flx4_clk"), id: 23, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("uart0_clk"), id: 24, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("uart1_clk"), id: 25, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("uart2_clk"), id: 26, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("uart3_clk"), id: 27, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("uart4_clk"), id: 28, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("twi0_clk"), id: 29, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("twi1_clk"), id: 30, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("spi0_clk"), id: 33, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("spi1_clk"), id: 34, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("tcb0_clk"), id: 35, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("tcb1_clk"), id: 36, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("pwm_clk"), id: 38, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("adc_clk"), id: 40, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("uhphs_clk"), id: 41, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("udphs_clk"), id: 42, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("ssc0_clk"), id: 43, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("ssc1_clk"), id: 44, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("trng_clk"), id: 47, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("pdmic_clk"), id: 48, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("securam_clk"), id: 51, r: clk_range { min: 0, max: 0 } },
    Sama5d2Periph32ck { n: cstr!("i2s0_clk"), id: 54, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("i2s1_clk"), id: 55, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("can0_clk"), id: 56, r: clk_range { min: 0, max: 83000000 } },
    Sama5d2Periph32ck { n: cstr!("can1_clk"), id: 57, r: clk_range { min: 0, max: 83000000 } },
];

#[repr(C)] struct Sama5d2Periphck { n: *mut c_char, flags: c_ulong, id: u8 }
static SAMA5D2_PERIPHCK: [Sama5d2Periphck; 13] = [
    Sama5d2Periphck { n: cstr!("dma0_clk"), flags: 0, id: 6 }, Sama5d2Periphck { n: cstr!("dma1_clk"), flags: 0, id: 7 },
    Sama5d2Periphck { n: cstr!("aes_clk"), flags: 0, id: 9 }, Sama5d2Periphck { n: cstr!("aesb_clk"), flags: 0, id: 10 },
    Sama5d2Periphck { n: cstr!("sha_clk"), flags: 0, id: 12 }, Sama5d2Periphck { n: cstr!("mpddr_clk"), flags: CLK_IS_CRITICAL, id: 13 },
    Sama5d2Periphck { n: cstr!("matrix0_clk"), flags: 0, id: 15 }, Sama5d2Periphck { n: cstr!("sdmmc0_hclk"), flags: 0, id: 31 },
    Sama5d2Periphck { n: cstr!("sdmmc1_hclk"), flags: 0, id: 32 }, Sama5d2Periphck { n: cstr!("lcdc_clk"), flags: 0, id: 45 },
    Sama5d2Periphck { n: cstr!("isc_clk"), flags: 0, id: 46 }, Sama5d2Periphck { n: cstr!("qspi0_clk"), flags: 0, id: 52 },
];

#[repr(C)] struct Sama5d2Gck { n: *mut c_char, id: u8, r: clk_range, chg_pid: c_int }
static SAMA5D2_GCK: [Sama5d2Gck; 21] = [
    Sama5d2Gck { n: cstr!("flx0_gclk"), id: 19, r: clk_range { min: 0, max: 27666666 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("flx1_gclk"), id: 20, r: clk_range { min: 0, max: 27666666 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("flx2_gclk"), id: 21, r: clk_range { min: 0, max: 27666666 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("flx3_gclk"), id: 22, r: clk_range { min: 0, max: 27666666 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("flx4_gclk"), id: 23, r: clk_range { min: 0, max: 27666666 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("uart0_gclk"), id: 24, r: clk_range { min: 0, max: 27666666 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("uart1_gclk"), id: 25, r: clk_range { min: 0, max: 27666666 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("uart2_gclk"), id: 26, r: clk_range { min: 0, max: 27666666 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("uart3_gclk"), id: 27, r: clk_range { min: 0, max: 27666666 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("uart4_gclk"), id: 28, r: clk_range { min: 0, max: 27666666 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("sdmmc0_gclk"), id: 31, r: clk_range { min: 0, max: 0 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("sdmmc1_gclk"), id: 32, r: clk_range { min: 0, max: 0 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("tcb0_gclk"), id: 35, r: clk_range { min: 0, max: 83000000 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("tcb1_gclk"), id: 36, r: clk_range { min: 0, max: 83000000 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("pwm_gclk"), id: 38, r: clk_range { min: 0, max: 83000000 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("isc_gclk"), id: 46, r: clk_range { min: 0, max: 0 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("pdmic_gclk"), id: 48, r: clk_range { min: 0, max: 0 }, chg_pid: INT_MIN },
    Sama5d2Gck { n: cstr!("i2s0_gclk"), id: 54, r: clk_range { min: 0, max: 0 }, chg_pid: 5 },
    Sama5d2Gck { n: cstr!("i2s1_gclk"), id: 55, r: clk_range { min: 0, max: 0 }, chg_pid: 5 },
    Sama5d2Gck { n: cstr!("can0_gclk"), id: 56, r: clk_range { min: 0, max: 80000000 }, chg_pid: INT_MIN },
];

static SAMA5D2_PROGRAMMABLE_LAYOUT: clk_programmable_layout = clk_programmable_layout {
    pres_mask: 0xff, pres_shift: 4, css_mask: 0x7, have_slck_mck: 0, is_pres_direct: 1,
};

unsafe fn sama5d2_pmc_setup(np: *mut device_node) {
    let mut range = CLK_RANGE(0, 0);
    let mut i: c_int;
    let slck_name: *const c_char;
    let mainxtal_name: *const c_char;
    let sama5d2_pmc: *mut pmc_data;
    let mut parent_names: [*const c_char; 6] = [core::ptr::null(); 6];
    let regmap: *mut regmap;
    let mut regmap_sfr: *mut regmap;
    let mut hw: *mut clk_hw;
    let bypass: bool;

    i = of_property_match_string(np, cstr!("clock-names"), cstr!("slow_clk"));
    if i < 0 { return; }
    slck_name = of_clk_get_parent_name(np, i);
    i = of_property_match_string(np, cstr!("clock-names"), cstr!("main_xtal"));
    if i < 0 { return; }
    mainxtal_name = of_clk_get_parent_name(np, i);
    regmap = device_node_to_regmap(np);
    if IS_ERR(regmap) { return; }
    sama5d2_pmc = pmc_data_allocate(PMC_AUDIOPINCK + 1, ARRAY_SIZE(&SAMA5D2_SYSTEMCK), ARRAY_SIZE(&SAMA5D2_PERIPH32CK), ARRAY_SIZE(&SAMA5D2_GCK), 3);
    if sama5d2_pmc.is_null() { return; }
    hw = at91_clk_register_main_rc_osc(regmap, cstr!("main_rc_osc"), 12000000, 100000000); if IS_ERR(hw) { goto_err!(); }
    bypass = of_property_read_bool(np, cstr!("atmel,osc-bypass"));
    hw = at91_clk_register_main_osc(regmap, cstr!("main_osc"), mainxtal_name, core::ptr::null(), bypass); if IS_ERR(hw) { goto_err!(); }
    parent_names[0] = cstr!("main_rc_osc"); parent_names[1] = cstr!("main_osc");
    hw = at91_clk_register_sam9x5_main(regmap, cstr!("mainck"), parent_names.as_ptr(), core::ptr::null(), 2); if IS_ERR(hw) { goto_err!(); }
    (*sama5d2_pmc).chws[PMC_MAIN] = hw;
    hw = at91_clk_register_pll(regmap, cstr!("pllack"), cstr!("mainck"), 0, &sama5d3_pll_layout, &PLLA_CHARACTERISTICS); if IS_ERR(hw) { goto_err!(); }
    hw = at91_clk_register_plldiv(regmap, cstr!("plladivck"), cstr!("pllack")); if IS_ERR(hw) { goto_err!(); }
    (*sama5d2_pmc).chws[PMC_PLLACK] = hw;
    hw = at91_clk_register_audio_pll_frac(regmap, cstr!("audiopll_fracck"), cstr!("mainck")); if IS_ERR(hw) { goto_err!(); }
    hw = at91_clk_register_audio_pll_pad(regmap, cstr!("audiopll_padck"), cstr!("audiopll_fracck")); if IS_ERR(hw) { goto_err!(); }
    (*sama5d2_pmc).chws[PMC_AUDIOPINCK] = hw;
    hw = at91_clk_register_audio_pll_pmc(regmap, cstr!("audiopll_pmcck"), cstr!("audiopll_fracck")); if IS_ERR(hw) { goto_err!(); }
    (*sama5d2_pmc).chws[PMC_AUDIOPLLCK] = hw;
    regmap_sfr = syscon_regmap_lookup_by_compatible(cstr!("atmel,sama5d2-sfr")); if IS_ERR(regmap_sfr) { regmap_sfr = core::ptr::null_mut(); }
    hw = at91_clk_register_utmi(regmap, regmap_sfr, cstr!("utmick"), cstr!("mainck"), core::ptr::null()); if IS_ERR(hw) { goto_err!(); }
    (*sama5d2_pmc).chws[PMC_UTMI] = hw;
    parent_names[0] = slck_name; parent_names[1] = cstr!("mainck"); parent_names[2] = cstr!("plladivck"); parent_names[3] = cstr!("utmick");
    hw = at91_clk_register_master_pres(regmap, cstr!("masterck_pres"), 4, parent_names.as_ptr(), core::ptr::null(), &at91sam9x5_master_layout, &MCK_CHARACTERISTICS, &mut MCK_LOCK); if IS_ERR(hw) { goto_err!(); }
    hw = at91_clk_register_master_div(regmap, cstr!("masterck_div"), cstr!("masterck_pres"), core::ptr::null(), &at91sam9x5_master_layout, &MCK_CHARACTERISTICS, &mut MCK_LOCK, CLK_SET_RATE_GATE, 0); if IS_ERR(hw) { goto_err!(); }
    (*sama5d2_pmc).chws[PMC_MCK] = hw;
    hw = at91_clk_register_h32mx(regmap, cstr!("h32mxck"), cstr!("masterck_div")); if IS_ERR(hw) { goto_err!(); }
    (*sama5d2_pmc).chws[PMC_MCK2] = hw;
    parent_names[0] = cstr!("plladivck"); parent_names[1] = cstr!("utmick"); hw = at91sam9x5_clk_register_usb(regmap, cstr!("usbck"), parent_names.as_ptr(), 2); if IS_ERR(hw) { goto_err!(); }
    of_clk_add_hw_provider(np, of_clk_hw_pmc_get, sama5d2_pmc); return;
goto_err!();
}

// CLK_OF_DECLARE(sama5d2_pmc, "atmel,sama5d2-pmc", sama5d2_pmc_setup)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
