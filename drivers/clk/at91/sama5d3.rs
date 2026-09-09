// SPDX-License-Identifier: GPL-2.0
// Translated from sama5d3.c. Kernel and PMC declarations are supplied by
// external dependencies.

use core::ffi::{c_char, c_void};

static mut MCK_LOCK: c_void = unsafe { core::mem::zeroed() };

static MCK_CHARACTERISTICS: clk_master_characteristics = clk_master_characteristics {
    output: clk_range { min: 0, max: 166000000 },
    divisors: [1, 2, 4, 3],
};

static mut PLLA_OUT: [u8; 1] = [0];
static mut PLLA_ICPLL: [u16; 1] = [0];

static PLLA_OUTPUTS: [clk_range; 1] = [clk_range { min: 400000000, max: 1000000000 }];

static PLLA_CHARACTERISTICS: clk_pll_characteristics = clk_pll_characteristics {
    input: clk_range { min: 8000000, max: 50000000 },
    num_output: 1,
    output: PLLA_OUTPUTS.as_ptr(),
    icpll: unsafe { PLLA_ICPLL.as_ptr() },
    out: unsafe { PLLA_OUT.as_ptr() },
};

static SAMA5D3_PCR_LAYOUT: clk_pcr_layout = clk_pcr_layout {
    offset: 0x10c,
    cmd: 1 << 12,
    pid_mask: (1 << 7) - 1,
    div_mask: ((1 << 18) - 1) & !((1 << 16) - 1),
};

#[repr(C)]
struct SystemClock { n: *const c_char, p: *const c_char, flags: usize, id: u8 }

static SAMA5D3_SYSTEMCK: [SystemClock; 8] = [
    SystemClock { n: c"ddrck".as_ptr(), p: c"masterck_div".as_ptr(), id: 2, flags: CLK_IS_CRITICAL },
    SystemClock { n: c"lcdck".as_ptr(), p: c"masterck_div".as_ptr(), id: 3, flags: 0 },
    SystemClock { n: c"smdck".as_ptr(), p: c"smdclk".as_ptr(), id: 4, flags: 0 },
    SystemClock { n: c"uhpck".as_ptr(), p: c"usbck".as_ptr(), id: 6, flags: 0 },
    SystemClock { n: c"udpck".as_ptr(), p: c"usbck".as_ptr(), id: 7, flags: 0 },
    SystemClock { n: c"pck0".as_ptr(), p: c"prog0".as_ptr(), id: 8, flags: 0 },
    SystemClock { n: c"pck1".as_ptr(), p: c"prog1".as_ptr(), id: 9, flags: 0 },
    SystemClock { n: c"pck2".as_ptr(), p: c"prog2".as_ptr(), id: 10, flags: 0 },
];

#[repr(C)]
struct PeripheralClock { n: *const c_char, id: u8, r: clk_range, flags: usize }

const R0_83: clk_range = clk_range { min: 0, max: 83000000 };
const R0_41: clk_range = clk_range { min: 0, max: 41500000 };
const R0_166: clk_range = clk_range { min: 0, max: 166000000 };
const R_NONE: clk_range = clk_range { min: 0, max: 0 };

macro_rules! pc { ($n:literal, $id:expr, $r:expr) => { PeripheralClock { n: c"$n".as_ptr(), id: $id, r: $r, flags: 0 } }; }
static SAMA5D3_PERIPHCK: [PeripheralClock; 48] = [
    pc!("dbgu_clk",2,R_NONE), pc!("hsmc_clk",5,R_NONE), pc!("pioA_clk",6,R_NONE), pc!("pioB_clk",7,R_NONE), pc!("pioC_clk",8,R_NONE), pc!("pioD_clk",9,R_NONE), pc!("pioE_clk",10,R_NONE),
    pc!("usart0_clk",12,R0_83), pc!("usart1_clk",13,R0_83), pc!("usart2_clk",14,R0_83), pc!("usart3_clk",15,R0_83), pc!("uart0_clk",16,R0_83), pc!("uart1_clk",17,R0_83), pc!("twi0_clk",18,R0_41), pc!("twi1_clk",19,R0_41), pc!("twi2_clk",20,R0_41),
    pc!("mci0_clk",21,R_NONE), pc!("mci1_clk",22,R_NONE), pc!("mci2_clk",23,R_NONE), pc!("spi0_clk",24,R0_166), pc!("spi1_clk",25,R0_166), pc!("tcb0_clk",26,R0_166), pc!("tcb1_clk",27,R0_166), pc!("pwm_clk",28,R_NONE), pc!("adc_clk",29,R0_83), pc!("dma0_clk",30,R_NONE), pc!("dma1_clk",31,R_NONE), pc!("uhphs_clk",32,R_NONE), pc!("udphs_clk",33,R_NONE), pc!("macb0_clk",34,R_NONE), pc!("macb1_clk",35,R_NONE), pc!("lcdc_clk",36,R_NONE), pc!("isi_clk",37,R_NONE), pc!("ssc0_clk",38,R0_83), pc!("ssc1_clk",39,R0_83), pc!("can0_clk",40,R0_83), pc!("can1_clk",41,R0_83), pc!("sha_clk",42,R_NONE), pc!("aes_clk",43,R_NONE), pc!("tdes_clk",44,R_NONE), pc!("trng_clk",45,R_NONE), pc!("fuse_clk",48,R_NONE),
    PeripheralClock { n: c"mpddr_clk".as_ptr(), id: 49, r: R_NONE, flags: CLK_IS_CRITICAL },
];

unsafe fn sama5d3_pmc_setup(np: *mut device_node) {
    let mut i: i32;
    let mut parent_names: [*const c_char; 5] = [core::ptr::null(); 5];
    let mut regmap: *mut regmap;
    let mut hw: *mut clk_hw;
    let mut sama5d3_pmc: *mut pmc_data;
    let slck_name: *const c_char;
    let mainxtal_name: *const c_char;
    let bypass: bool;

    i = of_property_match_string(np, c"clock-names".as_ptr(), c"slow_clk".as_ptr());
    if i < 0 { return; }
    slck_name = of_clk_get_parent_name(np, i);
    i = of_property_match_string(np, c"clock-names".as_ptr(), c"main_xtal".as_ptr());
    if i < 0 { return; }
    mainxtal_name = of_clk_get_parent_name(np, i);
    regmap = device_node_to_regmap(np);
    if IS_ERR(regmap) { return; }
    sama5d3_pmc = pmc_data_allocate(PMC_PLLACK + 1, SAMA5D3_SYSTEMCK.len(), SAMA5D3_PERIPHCK.len(), 0, 3);
    if sama5d3_pmc.is_null() { return; }

    hw = at91_clk_register_main_rc_osc(regmap, c"main_rc_osc".as_ptr(), 12000000, 50000000);
    if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; }
    bypass = of_property_read_bool(np, c"atmel,osc-bypass".as_ptr());
    hw = at91_clk_register_main_osc(regmap, c"main_osc".as_ptr(), mainxtal_name, core::ptr::null(), bypass);
    if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; }
    parent_names[0] = c"main_rc_osc".as_ptr(); parent_names[1] = c"main_osc".as_ptr();
    hw = at91_clk_register_sam9x5_main(regmap, c"mainck".as_ptr(), parent_names.as_ptr(), core::ptr::null(), 2);
    if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; }
    hw = at91_clk_register_pll(regmap, c"pllack".as_ptr(), c"mainck".as_ptr(), 0, &sama5d3_pll_layout, &PLLA_CHARACTERISTICS);
    if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; }
    hw = at91_clk_register_plldiv(regmap, c"plladivck".as_ptr(), c"pllack".as_ptr());
    if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; }
    (*sama5d3_pmc).chws[PMC_PLLACK] = hw;
    hw = at91_clk_register_utmi(regmap, core::ptr::null(), c"utmick".as_ptr(), c"mainck".as_ptr(), core::ptr::null());
    if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; }
    (*sama5d3_pmc).chws[PMC_UTMI] = hw;
    parent_names = [slck_name, c"mainck".as_ptr(), c"plladivck".as_ptr(), c"utmick".as_ptr(), core::ptr::null()];
    hw = at91_clk_register_master_pres(regmap, c"masterck_pres".as_ptr(), 4, parent_names.as_ptr(), core::ptr::null(), &at91sam9x5_master_layout, &MCK_CHARACTERISTICS, &MCK_LOCK);
    if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; }
    hw = at91_clk_register_master_div(regmap, c"masterck_div".as_ptr(), c"masterck_pres".as_ptr(), core::ptr::null(), &at91sam9x5_master_layout, &MCK_CHARACTERISTICS, &MCK_LOCK, CLK_SET_RATE_GATE, 0);
    if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; }
    (*sama5d3_pmc).chws[PMC_MCK] = hw;
    parent_names[0] = c"plladivck".as_ptr(); parent_names[1] = c"utmick".as_ptr();
    hw = at91sam9x5_clk_register_usb(regmap, c"usbck".as_ptr(), parent_names.as_ptr(), 2);
    if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; }
    hw = at91sam9x5_clk_register_smd(regmap, c"smdclk".as_ptr(), parent_names.as_ptr(), 2);
    if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; }
    parent_names = [slck_name, c"mainck".as_ptr(), c"plladivck".as_ptr(), c"utmick".as_ptr(), c"masterck_div".as_ptr()];
    for i in 0..3 { let mut name = [0i8; 6]; snprintf(name.as_mut_ptr(), name.len(), c"prog%d".as_ptr(), i); hw = at91_clk_register_programmable(regmap, name.as_ptr(), parent_names.as_ptr(), core::ptr::null(), 5, i, &at91sam9x5_programmable_layout, core::ptr::null()); if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; } (*sama5d3_pmc).pchws[i as usize] = hw; }
    for item in SAMA5D3_SYSTEMCK.iter() { hw = at91_clk_register_system(regmap, item.n, item.p, core::ptr::null(), item.id, item.flags); if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; } (*sama5d3_pmc).shws[item.id as usize] = hw; }
    for item in SAMA5D3_PERIPHCK.iter() { hw = at91_clk_register_sam9x5_peripheral(regmap, &pmc_pcr_lock, &SAMA5D3_PCR_LAYOUT, item.n, c"masterck_div".as_ptr(), core::ptr::null(), item.id, &item.r, INT_MIN, item.flags); if IS_ERR(hw) { goto_err_free(sama5d3_pmc); return; } (*sama5d3_pmc).phws[item.id as usize] = hw; }
    of_clk_add_hw_provider(np, of_clk_hw_pmc_get, sama5d3_pmc);
}

// The TCB is used as the clocksource so its clock is needed early; this is
// therefore declared directly rather than as a platform driver.
// Equivalent of CLK_OF_DECLARE(sama5d3_pmc, "atmel,sama5d3-pmc", sama5d3_pmc_setup).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
