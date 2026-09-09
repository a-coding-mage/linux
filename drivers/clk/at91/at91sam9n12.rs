// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux clock, device-tree, regmap, and PMC code.

static mut AT91SAM9N12_MCK_LOCK: SpinLock = DEFINE_SPINLOCK!();

static MCK_CHARACTERISTICS: ClkMasterCharacteristics = ClkMasterCharacteristics {
    output: ClkRange { min: 0, max: 133333333 },
    divisors: [1, 2, 4, 3],
    have_div3_pres: 1,
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

static PLLB_OUT: [u8; 1] = [0];
static PLLB_OUTPUTS: [ClkRange; 1] = [ClkRange { min: 30000000, max: 100000000 }];
static PLLB_CHARACTERISTICS: ClkPllCharacteristics = ClkPllCharacteristics {
    input: ClkRange { min: 2000000, max: 32000000 },
    num_output: PLLB_OUTPUTS.len(),
    output: PLLB_OUTPUTS.as_ptr(),
    out: PLLB_OUT.as_ptr(),
};

static AT91SAM9N12_SYSTEMCK: [SystemClock; 6] = [
    // ddrck feeds DDR controller and is enabled by bootloader thus we need
    // to keep it enabled in case there is no Linux consumer for it.
    SystemClock { n: "ddrck", p: "masterck_div", id: 2, flags: CLK_IS_CRITICAL },
    SystemClock { n: "lcdck", p: "masterck_div", id: 3, flags: 0 },
    SystemClock { n: "uhpck", p: "usbck", id: 6, flags: 0 },
    SystemClock { n: "udpck", p: "usbck", id: 7, flags: 0 },
    SystemClock { n: "pck0", p: "prog0", id: 8, flags: 0 },
    SystemClock { n: "pck1", p: "prog1", id: 9, flags: 0 },
];

static AT91SAM9N12_PCR_LAYOUT: ClkPcrLayout = ClkPcrLayout {
    offset: 0x10c,
    cmd: 1 << 12,
    pid_mask: (1 << 6) - 1,
    div_mask: ((1 << 2) - 1) << 16,
};

struct Pck { n: &'static str, id: u8 }

static AT91SAM9N12_PERIPHCK: [Pck; 25] = [
    Pck { n: "pioAB_clk", id: 2 }, Pck { n: "pioCD_clk", id: 3 },
    Pck { n: "fuse_clk", id: 4 }, Pck { n: "usart0_clk", id: 5 },
    Pck { n: "usart1_clk", id: 6 }, Pck { n: "usart2_clk", id: 7 },
    Pck { n: "usart3_clk", id: 8 }, Pck { n: "twi0_clk", id: 9 },
    Pck { n: "twi1_clk", id: 10 }, Pck { n: "mci0_clk", id: 12 },
    Pck { n: "spi0_clk", id: 13 }, Pck { n: "spi1_clk", id: 14 },
    Pck { n: "uart0_clk", id: 15 }, Pck { n: "uart1_clk", id: 16 },
    Pck { n: "tcb_clk", id: 17 }, Pck { n: "pwm_clk", id: 18 },
    Pck { n: "adc_clk", id: 19 }, Pck { n: "dma0_clk", id: 20 },
    Pck { n: "uhphs_clk", id: 22 }, Pck { n: "udphs_clk", id: 23 },
    Pck { n: "lcdc_clk", id: 25 }, Pck { n: "sha_clk", id: 27 },
    Pck { n: "ssc0_clk", id: 28 }, Pck { n: "aes_clk", id: 29 },
    Pck { n: "trng_clk", id: 30 },
];

unsafe fn at91sam9n12_pmc_setup(np: *mut DeviceNode) {
    let mut range = clk_range!(0, 0);
    let mut parent_names: [*const core::ffi::c_char; 6] = [core::ptr::null(); 6];
    let mut i: i32;
    let mut bypass: bool;
    let mut at91sam9n12_pmc: *mut PmcData;
    let mut regmap: *mut Regmap;
    let mut hw: *mut ClkHw;

    i = of_property_match_string(np, "clock-names", "slow_clk");
    if i < 0 { return; }
    let slck_name = of_clk_get_parent_name(np, i);
    i = of_property_match_string(np, "clock-names", "main_xtal");
    if i < 0 { return; }
    let mainxtal_name = of_clk_get_parent_name(np, i);
    regmap = device_node_to_regmap(np);
    if IS_ERR(regmap) { return; }
    at91sam9n12_pmc = pmc_data_allocate(PMC_PLLBCK + 1, nck(AT91SAM9N12_SYSTEMCK.len()), 31, 0, 2);
    if at91sam9n12_pmc.is_null() { return; }

    hw = at91_clk_register_main_rc_osc(regmap, "main_rc_osc", 12000000, 50000000);
    if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
    bypass = of_property_read_bool(np, "atmel,osc-bypass");
    hw = at91_clk_register_main_osc(regmap, "main_osc", mainxtal_name, core::ptr::null(), bypass);
    if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
    parent_names[0] = cstr!("main_rc_osc"); parent_names[1] = cstr!("main_osc");
    hw = at91_clk_register_sam9x5_main(regmap, "mainck", parent_names.as_ptr(), core::ptr::null(), 2);
    if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
    (*at91sam9n12_pmc).chws[PMC_MAIN] = hw;
    hw = at91_clk_register_pll(regmap, "pllack", "mainck", 0, &at91rm9200_pll_layout, &PLLA_CHARACTERISTICS);
    if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
    hw = at91_clk_register_plldiv(regmap, "plladivck", "pllack");
    if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
    (*at91sam9n12_pmc).chws[PMC_PLLACK] = hw;
    hw = at91_clk_register_pll(regmap, "pllbck", "mainck", 1, &at91rm9200_pll_layout, &PLLB_CHARACTERISTICS);
    if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
    (*at91sam9n12_pmc).chws[PMC_PLLBCK] = hw;
    parent_names[0] = slck_name; parent_names[1] = cstr!("mainck"); parent_names[2] = cstr!("plladivck"); parent_names[3] = cstr!("pllbck");
    hw = at91_clk_register_master_pres(regmap, "masterck_pres", 4, parent_names.as_ptr(), core::ptr::null(), &at91sam9x5_master_layout, &MCK_CHARACTERISTICS, &mut AT91SAM9N12_MCK_LOCK);
    if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
    hw = at91_clk_register_master_div(regmap, "masterck_div", "masterck_pres", core::ptr::null(), &at91sam9x5_master_layout, &MCK_CHARACTERISTICS, &mut AT91SAM9N12_MCK_LOCK, CLK_SET_RATE_GATE, 0);
    if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
    (*at91sam9n12_pmc).chws[PMC_MCK] = hw;
    hw = at91sam9n12_clk_register_usb(regmap, "usbck", "pllbck");
    if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
    parent_names[4] = cstr!("masterck_div");
    for i in 0..2 {
        let name = format!("prog{}", i);
        hw = at91_clk_register_programmable(regmap, &name, parent_names.as_ptr(), core::ptr::null(), 5, i, &at91sam9x5_programmable_layout, core::ptr::null());
        if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
        (*at91sam9n12_pmc).pchws[i as usize] = hw;
    }
    for entry in AT91SAM9N12_SYSTEMCK.iter() {
        hw = at91_clk_register_system(regmap, entry.n, entry.p, core::ptr::null(), entry.id, entry.flags);
        if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
        (*at91sam9n12_pmc).shws[entry.id as usize] = hw;
    }
    for entry in AT91SAM9N12_PERIPHCK.iter() {
        hw = at91_clk_register_sam9x5_peripheral(regmap, &pmc_pcr_lock, &AT91SAM9N12_PCR_LAYOUT, entry.n, "masterck_div", core::ptr::null(), entry.id, &range, i32::MIN, 0);
        if IS_ERR(hw) { goto_err_free!(at91sam9n12_pmc); }
        (*at91sam9n12_pmc).phws[entry.id as usize] = hw;
    }
    of_clk_add_hw_provider(np, of_clk_hw_pmc_get, at91sam9n12_pmc);
    return;
}

// The TCB is used as the clocksource so its clock is needed early. This means
// this can't be a platform driver.
CLK_OF_DECLARE!(at91sam9n12_pmc, "atmel,at91sam9n12-pmc", at91sam9n12_pmc_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
