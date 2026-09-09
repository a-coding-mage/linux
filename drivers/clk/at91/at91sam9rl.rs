// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to the Linux clock-provider, syscon, slab,
// device-tree clock bindings, and pmc headers are supplied externally.

static mut sam9rl_mck_lock: spinlock_t = spinlock_t::new();

static sam9rl_mck_characteristics: clk_master_characteristics = clk_master_characteristics {
    output: clk_range { min: 0, max: 94000000 },
    divisors: [1, 2, 4, 0],
};

static mut sam9rl_plla_out: [u8; 2] = [0, 2];

static sam9rl_plla_outputs: [clk_range; 2] = [
    clk_range { min: 80000000, max: 200000000 },
    clk_range { min: 190000000, max: 240000000 },
];

static sam9rl_plla_characteristics: clk_pll_characteristics = clk_pll_characteristics {
    input: clk_range { min: 1000000, max: 32000000 },
    num_output: sam9rl_plla_outputs.len(),
    output: sam9rl_plla_outputs.as_ptr(),
    out: unsafe { sam9rl_plla_out.as_ptr() },
};

#[repr(C)]
struct At91sam9rlSystemck {
    n: *const core::ffi::c_char,
    p: *const core::ffi::c_char,
    id: u8,
}

static at91sam9rl_systemck: [At91sam9rlSystemck; 2] = [
    At91sam9rlSystemck { n: b"pck0\0".as_ptr() as _, p: b"prog0\0".as_ptr() as _, id: 8 },
    At91sam9rlSystemck { n: b"pck1\0".as_ptr() as _, p: b"prog1\0".as_ptr() as _, id: 9 },
];

#[repr(C)]
struct At91sam9rlPeriphck {
    n: *const core::ffi::c_char,
    id: u8,
}

static at91sam9rl_periphck: [At91sam9rlPeriphck; 22] = [
    At91sam9rlPeriphck { n: b"pioA_clk\0".as_ptr() as _, id: 2 },
    At91sam9rlPeriphck { n: b"pioB_clk\0".as_ptr() as _, id: 3 },
    At91sam9rlPeriphck { n: b"pioC_clk\0".as_ptr() as _, id: 4 },
    At91sam9rlPeriphck { n: b"pioD_clk\0".as_ptr() as _, id: 5 },
    At91sam9rlPeriphck { n: b"usart0_clk\0".as_ptr() as _, id: 6 },
    At91sam9rlPeriphck { n: b"usart1_clk\0".as_ptr() as _, id: 7 },
    At91sam9rlPeriphck { n: b"usart2_clk\0".as_ptr() as _, id: 8 },
    At91sam9rlPeriphck { n: b"usart3_clk\0".as_ptr() as _, id: 9 },
    At91sam9rlPeriphck { n: b"mci0_clk\0".as_ptr() as _, id: 10 },
    At91sam9rlPeriphck { n: b"twi0_clk\0".as_ptr() as _, id: 11 },
    At91sam9rlPeriphck { n: b"twi1_clk\0".as_ptr() as _, id: 12 },
    At91sam9rlPeriphck { n: b"spi0_clk\0".as_ptr() as _, id: 13 },
    At91sam9rlPeriphck { n: b"ssc0_clk\0".as_ptr() as _, id: 14 },
    At91sam9rlPeriphck { n: b"ssc1_clk\0".as_ptr() as _, id: 15 },
    At91sam9rlPeriphck { n: b"tc0_clk\0".as_ptr() as _, id: 16 },
    At91sam9rlPeriphck { n: b"tc1_clk\0".as_ptr() as _, id: 17 },
    At91sam9rlPeriphck { n: b"tc2_clk\0".as_ptr() as _, id: 18 },
    At91sam9rlPeriphck { n: b"pwm_clk\0".as_ptr() as _, id: 19 },
    At91sam9rlPeriphck { n: b"adc_clk\0".as_ptr() as _, id: 20 },
    At91sam9rlPeriphck { n: b"dma0_clk\0".as_ptr() as _, id: 21 },
    At91sam9rlPeriphck { n: b"udphs_clk\0".as_ptr() as _, id: 22 },
    At91sam9rlPeriphck { n: b"lcd_clk\0".as_ptr() as _, id: 23 },
];

unsafe fn at91sam9rl_pmc_setup(np: *mut device_node) {
    let mut slck_name: *const core::ffi::c_char;
    let mut mainxtal_name: *const core::ffi::c_char;
    let mut at91sam9rl_pmc: *mut pmc_data;
    let mut parent_names: [*const core::ffi::c_char; 6] = [core::ptr::null(); 6];
    let mut regmap: *mut regmap;
    let mut hw: *mut clk_hw;
    let mut i: i32;

    i = of_property_match_string(np, b"clock-names\0".as_ptr() as _, b"slow_clk\0".as_ptr() as _);
    if i < 0 { return; }
    slck_name = of_clk_get_parent_name(np, i);

    i = of_property_match_string(np, b"clock-names\0".as_ptr() as _, b"main_xtal\0".as_ptr() as _);
    if i < 0 { return; }
    mainxtal_name = of_clk_get_parent_name(np, i);
    regmap = device_node_to_regmap(np);
    if IS_ERR(regmap) { return; }

    at91sam9rl_pmc = pmc_data_allocate(PMC_PLLACK + 1, nck(at91sam9rl_systemck.as_ptr(), at91sam9rl_systemck.len()), nck(at91sam9rl_periphck.as_ptr(), at91sam9rl_periphck.len()), 0, 2);
    if at91sam9rl_pmc.is_null() { return; }

    hw = at91_clk_register_rm9200_main(regmap, b"mainck\0".as_ptr() as _, mainxtal_name, core::ptr::null());
    if IS_ERR(hw) { kfree(at91sam9rl_pmc as *mut core::ffi::c_void); return; }
    (*at91sam9rl_pmc).chws[PMC_MAIN] = hw;
    hw = at91_clk_register_pll(regmap, b"pllack\0".as_ptr() as _, b"mainck\0".as_ptr() as _, 0, &at91rm9200_pll_layout, &sam9rl_plla_characteristics);
    if IS_ERR(hw) { kfree(at91sam9rl_pmc as *mut core::ffi::c_void); return; }
    (*at91sam9rl_pmc).chws[PMC_PLLACK] = hw;
    hw = at91_clk_register_utmi(regmap, core::ptr::null(), b"utmick\0".as_ptr() as _, b"mainck\0".as_ptr() as _, core::ptr::null());
    if IS_ERR(hw) { kfree(at91sam9rl_pmc as *mut core::ffi::c_void); return; }
    (*at91sam9rl_pmc).chws[PMC_UTMI] = hw;

    parent_names[0] = slck_name; parent_names[1] = b"mainck\0".as_ptr() as _; parent_names[2] = b"pllack\0".as_ptr() as _; parent_names[3] = b"utmick\0".as_ptr() as _;
    hw = at91_clk_register_master_pres(regmap, b"masterck_pres\0".as_ptr() as _, 4, parent_names.as_ptr(), core::ptr::null(), &at91rm9200_master_layout, &sam9rl_mck_characteristics, &raw mut sam9rl_mck_lock);
    if IS_ERR(hw) { kfree(at91sam9rl_pmc as *mut core::ffi::c_void); return; }
    hw = at91_clk_register_master_div(regmap, b"masterck_div\0".as_ptr() as _, b"masterck_pres\0".as_ptr() as _, core::ptr::null(), &at91rm9200_master_layout, &sam9rl_mck_characteristics, &raw mut sam9rl_mck_lock, CLK_SET_RATE_GATE, 0);
    if IS_ERR(hw) { kfree(at91sam9rl_pmc as *mut core::ffi::c_void); return; }
    (*at91sam9rl_pmc).chws[PMC_MCK] = hw;

    parent_names[4] = b"masterck_div\0".as_ptr() as _;
    for i in 0..2 {
        let mut name = [0i8; 6];
        snprintf(name.as_mut_ptr(), name.len(), b"prog%d\0".as_ptr() as _, i);
        hw = at91_clk_register_programmable(regmap, name.as_ptr(), parent_names.as_ptr(), core::ptr::null(), 5, i, &at91rm9200_programmable_layout, core::ptr::null());
        if IS_ERR(hw) { kfree(at91sam9rl_pmc as *mut core::ffi::c_void); return; }
        (*at91sam9rl_pmc).pchws[i as usize] = hw;
    }
    for entry in &at91sam9rl_systemck {
        hw = at91_clk_register_system(regmap, entry.n, entry.p, core::ptr::null(), entry.id, 0);
        if IS_ERR(hw) { kfree(at91sam9rl_pmc as *mut core::ffi::c_void); return; }
        (*at91sam9rl_pmc).shws[entry.id as usize] = hw;
    }
    for entry in &at91sam9rl_periphck {
        hw = at91_clk_register_peripheral(regmap, entry.n, b"masterck_div\0".as_ptr() as _, core::ptr::null(), entry.id);
        if IS_ERR(hw) { kfree(at91sam9rl_pmc as *mut core::ffi::c_void); return; }
        (*at91sam9rl_pmc).phws[entry.id as usize] = hw;
    }
    of_clk_add_hw_provider(np, of_clk_hw_pmc_get, at91sam9rl_pmc);
    return;
}

// CLK_OF_DECLARE(at91sam9rl_pmc, "atmel,at91sam9rl-pmc", at91sam9rl_pmc_setup)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
