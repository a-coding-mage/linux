// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/clk-provider.h, linux/mfd/syscon.h, linux/slab.h,
// dt-bindings/clock/at91.h, and pmc.h.

static mut AT91SAM9G45_MCK_LOCK: Spinlock = DEFINE_SPINLOCK!();

static MCK_CHARACTERISTICS: ClkMasterCharacteristics = ClkMasterCharacteristics {
    output: ClkRange { min: 0, max: 133333333 },
    divisors: [1, 2, 4, 3],
};

static PLLA_OUT: [u8; 8] = [0, 1, 2, 3, 0, 1, 2, 3];
static PLLA_ICPLL: [u16; 8] = [0, 0, 0, 0, 1, 1, 1, 1];

static PLLA_OUTPUTS: [ClkRange; 8] = [
    ClkRange { min: 745000000, max: 800000000 },
    ClkRange { min: 695000000, max: 750000000 },
    ClkRange { min: 645000000, max: 700000000 },
    ClkRange { min: 595000000, max: 650000000 },
    ClkRange { min: 545000000, max: 600000000 },
    ClkRange { min: 495000000, max: 555000000 },
    ClkRange { min: 445000000, max: 500000000 },
    ClkRange { min: 400000000, max: 450000000 },
];

static PLLA_CHARACTERISTICS: ClkPllCharacteristics = ClkPllCharacteristics {
    input: ClkRange { min: 2000000, max: 32000000 },
    num_output: PLLA_OUTPUTS.len(),
    output: PLLA_OUTPUTS.as_ptr(),
    icpll: PLLA_ICPLL.as_ptr(),
    out: PLLA_OUT.as_ptr(),
};

static AT91SAM9G45_SYSTEMCK: [SystemClock; 4] = [
    // ddrck feeds DDR controller and is enabled by bootloader thus we need
    // to keep it enabled in case there is no Linux consumer for it.
    SystemClock { n: "ddrck", p: "masterck_div", flags: CLK_IS_CRITICAL, id: 2 },
    SystemClock { n: "uhpck", p: "usbck", flags: 0, id: 6 },
    SystemClock { n: "pck0", p: "prog0", flags: 0, id: 8 },
    SystemClock { n: "pck1", p: "prog1", flags: 0, id: 9 },
];

struct Pck {
    n: &'static str,
    id: u8,
}

static AT91SAM9G45_PERIPHCK: [Pck; 29] = [
    Pck { n: "pioA_clk", id: 2 }, Pck { n: "pioB_clk", id: 3 },
    Pck { n: "pioC_clk", id: 4 }, Pck { n: "pioDE_clk", id: 5 },
    Pck { n: "trng_clk", id: 6 }, Pck { n: "usart0_clk", id: 7 },
    Pck { n: "usart1_clk", id: 8 }, Pck { n: "usart2_clk", id: 9 },
    Pck { n: "usart3_clk", id: 10 }, Pck { n: "mci0_clk", id: 11 },
    Pck { n: "twi0_clk", id: 12 }, Pck { n: "twi1_clk", id: 13 },
    Pck { n: "spi0_clk", id: 14 }, Pck { n: "spi1_clk", id: 15 },
    Pck { n: "ssc0_clk", id: 16 }, Pck { n: "ssc1_clk", id: 17 },
    Pck { n: "tcb0_clk", id: 18 }, Pck { n: "pwm_clk", id: 19 },
    Pck { n: "adc_clk", id: 20 }, Pck { n: "dma0_clk", id: 21 },
    Pck { n: "uhphs_clk", id: 22 }, Pck { n: "lcd_clk", id: 23 },
    Pck { n: "ac97_clk", id: 24 }, Pck { n: "macb0_clk", id: 25 },
    Pck { n: "isi_clk", id: 26 }, Pck { n: "udphs_clk", id: 27 },
    Pck { n: "aestdessha_clk", id: 28 }, Pck { n: "mci1_clk", id: 29 },
    Pck { n: "vdec_clk", id: 30 },
];

unsafe fn at91sam9g45_pmc_setup(np: *mut DeviceNode) {
    let mut i: i32;
    let slck_name: *const c_char;
    let mainxtal_name: *const c_char;
    let at91sam9g45_pmc: *mut PmcData;
    let mut parent_names: [*const c_char; 6] = [core::ptr::null(); 6];
    let regmap: *mut Regmap;
    let mut hw: *mut ClkHw;
    let bypass: bool;

    i = of_property_match_string(np, c_str!("clock-names"), c_str!("slow_clk"));
    if i < 0 { return; }
    slck_name = of_clk_get_parent_name(np, i);

    i = of_property_match_string(np, c_str!("clock-names"), c_str!("main_xtal"));
    if i < 0 { return; }
    mainxtal_name = of_clk_get_parent_name(np, i);

    regmap = device_node_to_regmap(np);
    if is_err(regmap) { return; }

    at91sam9g45_pmc = pmc_data_allocate(PMC_PLLACK + 1, nck(&AT91SAM9G45_SYSTEMCK),
                                         nck(&AT91SAM9G45_PERIPHCK), 0, 2);
    if at91sam9g45_pmc.is_null() { return; }
    bypass = of_property_read_bool(np, c_str!("atmel,osc-bypass"));

    hw = at91_clk_register_main_osc(regmap, c_str!("main_osc"), mainxtal_name, core::ptr::null(), bypass);
    if is_err(hw) { goto_err_free(at91sam9g45_pmc); return; }
    hw = at91_clk_register_rm9200_main(regmap, c_str!("mainck"), c_str!("main_osc"), core::ptr::null());
    if is_err(hw) { goto_err_free(at91sam9g45_pmc); return; }
    (*at91sam9g45_pmc).chws[PMC_MAIN] = hw;
    hw = at91_clk_register_pll(regmap, c_str!("pllack"), c_str!("mainck"), 0,
                               &at91rm9200_pll_layout, &PLLA_CHARACTERISTICS);
    if is_err(hw) { goto_err_free(at91sam9g45_pmc); return; }
    hw = at91_clk_register_plldiv(regmap, c_str!("plladivck"), c_str!("pllack"));
    if is_err(hw) { goto_err_free(at91sam9g45_pmc); return; }
    (*at91sam9g45_pmc).chws[PMC_PLLACK] = hw;
    hw = at91_clk_register_utmi(regmap, core::ptr::null(), c_str!("utmick"), c_str!("mainck"), core::ptr::null());
    if is_err(hw) { goto_err_free(at91sam9g45_pmc); return; }
    (*at91sam9g45_pmc).chws[PMC_UTMI] = hw;
    parent_names[0] = slck_name; parent_names[1] = c_str!("mainck");
    parent_names[2] = c_str!("plladivck"); parent_names[3] = c_str!("utmick");
    hw = at91_clk_register_master_pres(regmap, c_str!("masterck_pres"), 4, parent_names.as_ptr(),
                                       core::ptr::null(), &at91rm9200_master_layout,
                                       &MCK_CHARACTERISTICS, &mut AT91SAM9G45_MCK_LOCK);
    if is_err(hw) { goto_err_free(at91sam9g45_pmc); return; }
    hw = at91_clk_register_master_div(regmap, c_str!("masterck_div"), c_str!("masterck_pres"),
                                      core::ptr::null(), &at91rm9200_master_layout,
                                      &MCK_CHARACTERISTICS, &mut AT91SAM9G45_MCK_LOCK,
                                      CLK_SET_RATE_GATE, 0);
    if is_err(hw) { goto_err_free(at91sam9g45_pmc); return; }
    (*at91sam9g45_pmc).chws[PMC_MCK] = hw;

    parent_names[0] = c_str!("plladivck"); parent_names[1] = c_str!("utmick");
    hw = at91sam9x5_clk_register_usb(regmap, c_str!("usbck"), parent_names.as_ptr(), 2);
    if is_err(hw) { goto_err_free(at91sam9g45_pmc); return; }
    parent_names[0] = slck_name; parent_names[1] = c_str!("mainck");
    parent_names[2] = c_str!("plladivck"); parent_names[3] = c_str!("utmick");
    parent_names[4] = c_str!("masterck_div");
    for i in 0..2 {
        let name = format!("prog{}", i);
        hw = at91_clk_register_programmable(regmap, name.as_ptr(), parent_names.as_ptr(),
                                            core::ptr::null(), 5, i,
                                            &at91sam9g45_programmable_layout, core::ptr::null());
        if is_err(hw) { goto_err_free(at91sam9g45_pmc); return; }
        (*at91sam9g45_pmc).pchws[i] = hw;
    }
    for ck in AT91SAM9G45_SYSTEMCK.iter() {
        hw = at91_clk_register_system(regmap, ck.n.as_ptr(), ck.p.as_ptr(), core::ptr::null(), ck.id, ck.flags);
        if is_err(hw) { goto_err_free(at91sam9g45_pmc); return; }
        (*at91sam9g45_pmc).shws[ck.id as usize] = hw;
    }
    for ck in AT91SAM9G45_PERIPHCK.iter() {
        hw = at91_clk_register_peripheral(regmap, ck.n.as_ptr(), c_str!("masterck_div"), core::ptr::null(), ck.id);
        if is_err(hw) { goto_err_free(at91sam9g45_pmc); return; }
        (*at91sam9g45_pmc).phws[ck.id as usize] = hw;
    }
    of_clk_add_hw_provider(np, of_clk_hw_pmc_get, at91sam9g45_pmc);
    return;
}

// The TCB is used as the clocksource so its clock is needed early. This means
// this can't be a platform driver.
clk_of_declare!(at91sam9g45_pmc, "atmel,at91sam9g45-pmc", at91sam9g45_pmc_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
