// SPDX-License-Identifier: GPL-2.0
// Translated from sama5d4.c. Kernel-provided declarations are external dependencies.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut mck_lock: c_void;
    static mut pmc_pcr_lock: c_void;
    static sama5d3_pll_layout: c_void;
    static at91sam9x5_master_layout: c_void;
    static at91sam9x5_programmable_layout: c_void;
    fn of_property_match_string(np: *mut device_node, prop: *const c_char, s: *const c_char) -> c_int;
    fn of_clk_get_parent_name(np: *mut device_node, index: c_int) -> *const c_char;
    fn device_node_to_regmap(np: *mut device_node) -> *mut regmap;
    fn pmc_data_allocate(a: c_int, b: usize, c: usize, d: c_int, e: c_int) -> *mut pmc_data;
    fn at91_clk_register_main_rc_osc(r: *mut regmap, n: *const c_char, f: c_ulong, m: c_ulong) -> *mut clk_hw;
    fn at91_clk_register_main_osc(r: *mut regmap, n: *const c_char, p: *const c_char, x: *const c_char, b: bool) -> *mut clk_hw;
    fn at91_clk_register_sam9x5_main(r: *mut regmap, n: *const c_char, p: *mut *const c_char, x: *const c_char, count: c_int) -> *mut clk_hw;
    fn at91_clk_register_pll(r: *mut regmap, n: *const c_char, p: *const c_char, id: c_int, l: *const c_void, c: *const c_void) -> *mut clk_hw;
    fn at91_clk_register_plldiv(r: *mut regmap, n: *const c_char, p: *const c_char) -> *mut clk_hw;
    fn at91_clk_register_utmi(r: *mut regmap, x: *const c_char, n: *const c_char, p: *const c_char, y: *const c_char) -> *mut clk_hw;
    fn at91_clk_register_master_pres(r: *mut regmap, n: *const c_char, count: c_int, p: *mut *const c_char, x: *const c_char, l: *const c_void, c: *const c_void, lock: *mut c_void) -> *mut clk_hw;
    fn at91_clk_register_master_div(r: *mut regmap, n: *const c_char, p: *const c_char, x: *const c_char, l: *const c_void, c: *const c_void, lock: *mut c_void, flags: c_ulong, id: c_int) -> *mut clk_hw;
    fn at91_clk_register_h32mx(r: *mut regmap, n: *const c_char, p: *const c_char) -> *mut clk_hw;
    fn at91sam9x5_clk_register_usb(r: *mut regmap, n: *const c_char, p: *mut *const c_char, count: c_int) -> *mut clk_hw;
    fn at91sam9x5_clk_register_smd(r: *mut regmap, n: *const c_char, p: *mut *const c_char, count: c_int) -> *mut clk_hw;
    fn at91_clk_register_programmable(r: *mut regmap, n: *const c_char, p: *mut *const c_char, x: *const c_char, count: c_int, id: c_int, l: *const c_void, lock: *const c_void) -> *mut clk_hw;
    fn at91_clk_register_system(r: *mut regmap, n: *const c_char, p: *const c_char, x: *const c_char, id: u8, flags: c_ulong) -> *mut clk_hw;
    fn at91_clk_register_sam9x5_peripheral(r: *mut regmap, lock: *mut c_void, layout: *const c_void, n: *const c_char, p: *const c_char, x: *const c_char, id: u8, range: *const clk_range, min: c_int, flags: c_ulong) -> *mut clk_hw;
    fn of_clk_add_hw_provider(np: *mut device_node, get: *const c_void, data: *mut pmc_data);
    fn kfree(p: *mut pmc_data);
}

#[repr(C)] struct device_node;
#[repr(C)] struct regmap;
#[repr(C)] struct clk_hw;
#[repr(C)] struct clk_range { min: c_ulong, max: c_ulong }
#[repr(C)] struct pmc_data { chws: [*mut clk_hw; 64], pchws: [*mut clk_hw; 3], shws: [*mut clk_hw; 128], phws: [*mut clk_hw; 128] }

const CLK_IS_CRITICAL: c_ulong = 1 << 6;
const CLK_SET_RATE_GATE: c_ulong = 1 << 5;
const PMC_PLLACK: usize = 0;
const PMC_UTMI: usize = 1;
const PMC_MCK: usize = 2;
const PMC_MCK2: usize = 3;

#[repr(C)] struct systemck { n: *const c_char, p: *const c_char, flags: c_ulong, id: u8 }
#[repr(C)] struct periphck { n: *const c_char, flags: c_ulong, id: u8 }
#[repr(C)] struct periph32ck { n: *const c_char, id: u8 }

static SAMA5D4_SYSTEMCK: &[systemck] = &[
    systemck { n: b"ddrck\0".as_ptr() as _, p: b"masterck_div\0".as_ptr() as _, id: 2, flags: CLK_IS_CRITICAL },
    systemck { n: b"lcdck\0".as_ptr() as _, p: b"masterck_div\0".as_ptr() as _, id: 3, flags: 0 },
    systemck { n: b"smdck\0".as_ptr() as _, p: b"smdclk\0".as_ptr() as _, id: 4, flags: 0 },
    systemck { n: b"uhpck\0".as_ptr() as _, p: b"usbck\0".as_ptr() as _, id: 6, flags: 0 },
    systemck { n: b"udpck\0".as_ptr() as _, p: b"usbck\0".as_ptr() as _, id: 7, flags: 0 },
    systemck { n: b"pck0\0".as_ptr() as _, p: b"prog0\0".as_ptr() as _, id: 8, flags: 0 },
    systemck { n: b"pck1\0".as_ptr() as _, p: b"prog1\0".as_ptr() as _, id: 9, flags: 0 },
    systemck { n: b"pck2\0".as_ptr() as _, p: b"prog2\0".as_ptr() as _, id: 10, flags: 0 },
];

static SAMA5D4_PERIPH32CK: &[periph32ck] = &[
    periph32ck { n: b"pioD_clk\0".as_ptr() as _, id: 5 }, periph32ck { n: b"usart0_clk\0".as_ptr() as _, id: 6 }, periph32ck { n: b"usart1_clk\0".as_ptr() as _, id: 7 }, periph32ck { n: b"icm_clk\0".as_ptr() as _, id: 9 }, periph32ck { n: b"aes_clk\0".as_ptr() as _, id: 12 }, periph32ck { n: b"tdes_clk\0".as_ptr() as _, id: 14 }, periph32ck { n: b"sha_clk\0".as_ptr() as _, id: 15 }, periph32ck { n: b"matrix1_clk\0".as_ptr() as _, id: 17 }, periph32ck { n: b"hsmc_clk\0".as_ptr() as _, id: 22 }, periph32ck { n: b"pioA_clk\0".as_ptr() as _, id: 23 }, periph32ck { n: b"pioB_clk\0".as_ptr() as _, id: 24 }, periph32ck { n: b"pioC_clk\0".as_ptr() as _, id: 25 }, periph32ck { n: b"pioE_clk\0".as_ptr() as _, id: 26 }, periph32ck { n: b"uart0_clk\0".as_ptr() as _, id: 27 }, periph32ck { n: b"uart1_clk\0".as_ptr() as _, id: 28 }, periph32ck { n: b"usart2_clk\0".as_ptr() as _, id: 29 }, periph32ck { n: b"usart3_clk\0".as_ptr() as _, id: 30 }, periph32ck { n: b"usart4_clk\0".as_ptr() as _, id: 31 }, periph32ck { n: b"twi0_clk\0".as_ptr() as _, id: 32 }, periph32ck { n: b"twi1_clk\0".as_ptr() as _, id: 33 }, periph32ck { n: b"twi2_clk\0".as_ptr() as _, id: 34 }, periph32ck { n: b"mci0_clk\0".as_ptr() as _, id: 35 }, periph32ck { n: b"mci1_clk\0".as_ptr() as _, id: 36 }, periph32ck { n: b"spi0_clk\0".as_ptr() as _, id: 37 }, periph32ck { n: b"spi1_clk\0".as_ptr() as _, id: 38 }, periph32ck { n: b"spi2_clk\0".as_ptr() as _, id: 39 }, periph32ck { n: b"tcb0_clk\0".as_ptr() as _, id: 40 }, periph32ck { n: b"tcb1_clk\0".as_ptr() as _, id: 41 }, periph32ck { n: b"tcb2_clk\0".as_ptr() as _, id: 42 }, periph32ck { n: b"pwm_clk\0".as_ptr() as _, id: 43 }, periph32ck { n: b"adc_clk\0".as_ptr() as _, id: 44 }, periph32ck { n: b"dbgu_clk\0".as_ptr() as _, id: 45 }, periph32ck { n: b"uhphs_clk\0".as_ptr() as _, id: 46 }, periph32ck { n: b"udphs_clk\0".as_ptr() as _, id: 47 }, periph32ck { n: b"ssc0_clk\0".as_ptr() as _, id: 48 }, periph32ck { n: b"ssc1_clk\0".as_ptr() as _, id: 49 }, periph32ck { n: b"trng_clk\0".as_ptr() as _, id: 53 }, periph32ck { n: b"macb0_clk\0".as_ptr() as _, id: 54 }, periph32ck { n: b"macb1_clk\0".as_ptr() as _, id: 55 }, periph32ck { n: b"fuse_clk\0".as_ptr() as _, id: 57 }, periph32ck { n: b"securam_clk\0".as_ptr() as _, id: 59 }, periph32ck { n: b"smd_clk\0".as_ptr() as _, id: 61 }, periph32ck { n: b"twi3_clk\0".as_ptr() as _, id: 62 }, periph32ck { n: b"catb_clk\0".as_ptr() as _, id: 63 },
];

// The remaining declarations and setup routine retain the C control flow and call the external kernel clock APIs.
// Build-time kernel macros such as CLK_OF_DECLARE and error-pointer helpers are supplied by the integration layer.

static SAMA5D4_PERIPHCK: &[periphck] = &[
    periphck { n: b"dma0_clk\0".as_ptr() as _, id: 8, flags: 0 }, periphck { n: b"cpkcc_clk\0".as_ptr() as _, id: 10, flags: 0 }, periphck { n: b"aesb_clk\0".as_ptr() as _, id: 13, flags: 0 }, periphck { n: b"mpddr_clk\0".as_ptr() as _, id: 16, flags: CLK_IS_CRITICAL }, periphck { n: b"matrix0_clk\0".as_ptr() as _, id: 18, flags: 0 }, periphck { n: b"vdec_clk\0".as_ptr() as _, id: 19, flags: 0 }, periphck { n: b"dma1_clk\0".as_ptr() as _, id: 50, flags: 0 }, periphck { n: b"lcdc_clk\0".as_ptr() as _, id: 51, flags: 0 }, periphck { n: b"isi_clk\0".as_ptr() as _, id: 52, flags: 0 },
];

#[no_mangle]
pub unsafe extern "C" fn sama5d4_pmc_setup(np: *mut device_node) {
    let mut i = of_property_match_string(np, b"clock-names\0".as_ptr() as _, b"slow_clk\0".as_ptr() as _);
    if i < 0 { return; }
    let slck_name = of_clk_get_parent_name(np, i);
    i = of_property_match_string(np, b"clock-names\0".as_ptr() as _, b"main_xtal\0".as_ptr() as _);
    if i < 0 { return; }
    let mainxtal_name = of_clk_get_parent_name(np, i);
    let regmap = device_node_to_regmap(np);
    if regmap.is_null() { return; }
    let pmc = pmc_data_allocate(1, SAMA5D4_SYSTEMCK.len(), SAMA5D4_PERIPH32CK.len(), 0, 3);
    if pmc.is_null() { return; }
    let mut parents: [*const c_char; 5] = [core::ptr::null(); 5];
    let mut hw = at91_clk_register_main_rc_osc(regmap, b"main_rc_osc\0".as_ptr() as _, 12000000, 100000000);
    if hw.is_null() { kfree(pmc); return; }
    hw = at91_clk_register_main_osc(regmap, b"main_osc\0".as_ptr() as _, mainxtal_name, core::ptr::null(), false);
    if hw.is_null() { kfree(pmc); return; }
    parents[0] = b"main_rc_osc\0".as_ptr() as _; parents[1] = b"main_osc\0".as_ptr() as _;
    hw = at91_clk_register_sam9x5_main(regmap, b"mainck\0".as_ptr() as _, parents.as_mut_ptr(), core::ptr::null(), 2);
    if hw.is_null() { kfree(pmc); return; }
    hw = at91_clk_register_pll(regmap, b"pllack\0".as_ptr() as _, b"mainck\0".as_ptr() as _, 0, &sama5d3_pll_layout, core::ptr::null());
    if hw.is_null() { kfree(pmc); return; }
    hw = at91_clk_register_plldiv(regmap, b"plladivck\0".as_ptr() as _, b"pllack\0".as_ptr() as _);
    if hw.is_null() { kfree(pmc); return; }
    (*pmc).chws[PMC_PLLACK] = hw;
    hw = at91_clk_register_utmi(regmap, core::ptr::null(), b"utmick\0".as_ptr() as _, b"mainck\0".as_ptr() as _, core::ptr::null());
    if hw.is_null() { kfree(pmc); return; }
    (*pmc).chws[PMC_UTMI] = hw;
    parents[0] = slck_name; parents[1] = b"mainck\0".as_ptr() as _; parents[2] = b"plladivck\0".as_ptr() as _; parents[3] = b"utmick\0".as_ptr() as _;
    hw = at91_clk_register_master_pres(regmap, b"masterck_pres\0".as_ptr() as _, 4, parents.as_mut_ptr(), core::ptr::null(), &at91sam9x5_master_layout, core::ptr::null(), &mut mck_lock);
    if hw.is_null() { kfree(pmc); return; }
    hw = at91_clk_register_master_div(regmap, b"masterck_div\0".as_ptr() as _, b"masterck_pres\0".as_ptr() as _, core::ptr::null(), &at91sam9x5_master_layout, core::ptr::null(), &mut mck_lock, CLK_SET_RATE_GATE, 0);
    if hw.is_null() { kfree(pmc); return; }
    (*pmc).chws[PMC_MCK] = hw;
    hw = at91_clk_register_h32mx(regmap, b"h32mxck\0".as_ptr() as _, b"masterck_div\0".as_ptr() as _);
    if hw.is_null() { kfree(pmc); return; }
    (*pmc).chws[PMC_MCK2] = hw;
    parents[0] = b"plladivck\0".as_ptr() as _; parents[1] = b"utmick\0".as_ptr() as _;
    hw = at91sam9x5_clk_register_usb(regmap, b"usbck\0".as_ptr() as _, parents.as_mut_ptr(), 2);
    if hw.is_null() { kfree(pmc); return; }
    hw = at91sam9x5_clk_register_smd(regmap, b"smdclk\0".as_ptr() as _, parents.as_mut_ptr(), 2);
    if hw.is_null() { kfree(pmc); return; }
    parents[0] = slck_name; parents[1] = b"mainck\0".as_ptr() as _; parents[2] = b"plladivck\0".as_ptr() as _; parents[3] = b"utmick\0".as_ptr() as _; parents[4] = b"masterck_div\0".as_ptr() as _;
    for n in 0..3 { hw = at91_clk_register_programmable(regmap, [b"prog0\0".as_ptr(), b"prog1\0".as_ptr(), b"prog2\0".as_ptr()][n] as _, parents.as_mut_ptr(), core::ptr::null(), 5, n as _, &at91sam9x5_programmable_layout, core::ptr::null()); if hw.is_null() { kfree(pmc); return; } (*pmc).pchws[n] = hw; }
    for x in SAMA5D4_SYSTEMCK { hw = at91_clk_register_system(regmap, x.n, x.p, core::ptr::null(), x.id, x.flags); if hw.is_null() { kfree(pmc); return; } (*pmc).shws[x.id as usize] = hw; }
    let range = clk_range { min: 0, max: 0 };
    for x in SAMA5D4_PERIPHCK { hw = at91_clk_register_sam9x5_peripheral(regmap, &mut pmc_pcr_lock, core::ptr::null(), x.n, b"masterck_div\0".as_ptr() as _, core::ptr::null(), x.id, &range, i32::MIN, x.flags); if hw.is_null() { kfree(pmc); return; } (*pmc).phws[x.id as usize] = hw; }
    for x in SAMA5D4_PERIPH32CK { hw = at91_clk_register_sam9x5_peripheral(regmap, &mut pmc_pcr_lock, core::ptr::null(), x.n, b"h32mxck\0".as_ptr() as _, core::ptr::null(), x.id, &range, i32::MIN, 0); if hw.is_null() { kfree(pmc); return; } (*pmc).phws[x.id as usize] = hw; }
    of_clk_add_hw_provider(np, core::ptr::null(), pmc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
