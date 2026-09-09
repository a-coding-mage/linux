// SPDX-License-Identifier: GPL-2.0
// Translated from at91rm9200.c. Kernel headers and symbols are supplied by
// the surrounding Rust translation environment.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct sck {
    pub n: *mut c_char,
    pub p: *mut c_char,
    pub id: u8,
}

#[repr(C)]
pub struct pck {
    pub n: *mut c_char,
    pub id: u8,
}

// Opaque declarations supplied by the translated kernel clock framework.
#[repr(C)] pub struct clk_master_characteristics { _private: [u8; 0] }
#[repr(C)] pub struct clk_pll_characteristics { _private: [u8; 0] }
#[repr(C)] pub struct clk_range { pub min: u32, pub max: u32 }

static mut RM9200_MCK_LOCK: *mut c_void = core::ptr::null_mut();

static mut RM9200_MCK_CHARACTERISTICS: clk_master_characteristics =
    clk_master_characteristics { _private: [] };

static mut RM9200_PLL_OUT: [u8; 2] = [0, 2];

static RM9200_PLL_OUTPUTS: [clk_range; 2] = [
    clk_range { min: 80000000, max: 160000000 },
    clk_range { min: 150000000, max: 180000000 },
];

static mut RM9200_PLL_CHARACTERISTICS: clk_pll_characteristics =
    clk_pll_characteristics { _private: [] };

static AT91RM9200_SYSTEMCK: [sck; 6] = [
    sck { n: b"udpck\0" as *const u8 as *mut c_char, p: b"usbck\0" as *const u8 as *mut c_char, id: 1 },
    sck { n: b"uhpck\0" as *const u8 as *mut c_char, p: b"usbck\0" as *const u8 as *mut c_char, id: 4 },
    sck { n: b"pck0\0" as *const u8 as *mut c_char, p: b"prog0\0" as *const u8 as *mut c_char, id: 8 },
    sck { n: b"pck1\0" as *const u8 as *mut c_char, p: b"prog1\0" as *const u8 as *mut c_char, id: 9 },
    sck { n: b"pck2\0" as *const u8 as *mut c_char, p: b"prog2\0" as *const u8 as *mut c_char, id: 10 },
    sck { n: b"pck3\0" as *const u8 as *mut c_char, p: b"prog3\0" as *const u8 as *mut c_char, id: 11 },
];

static AT91RM9200_PERIPHCK: [pck; 23] = [
    pck { n: b"pioA_clk\0" as *const u8 as *mut c_char, id: 2 },
    pck { n: b"pioB_clk\0" as *const u8 as *mut c_char, id: 3 },
    pck { n: b"pioC_clk\0" as *const u8 as *mut c_char, id: 4 },
    pck { n: b"pioD_clk\0" as *const u8 as *mut c_char, id: 5 },
    pck { n: b"usart0_clk\0" as *const u8 as *mut c_char, id: 6 },
    pck { n: b"usart1_clk\0" as *const u8 as *mut c_char, id: 7 },
    pck { n: b"usart2_clk\0" as *const u8 as *mut c_char, id: 8 },
    pck { n: b"usart3_clk\0" as *const u8 as *mut c_char, id: 9 },
    pck { n: b"mci0_clk\0" as *const u8 as *mut c_char, id: 10 },
    pck { n: b"udc_clk\0" as *const u8 as *mut c_char, id: 11 },
    pck { n: b"twi0_clk\0" as *const u8 as *mut c_char, id: 12 },
    pck { n: b"spi0_clk\0" as *const u8 as *mut c_char, id: 13 },
    pck { n: b"ssc0_clk\0" as *const u8 as *mut c_char, id: 14 },
    pck { n: b"ssc1_clk\0" as *const u8 as *mut c_char, id: 15 },
    pck { n: b"ssc2_clk\0" as *const u8 as *mut c_char, id: 16 },
    pck { n: b"tc0_clk\0" as *const u8 as *mut c_char, id: 17 },
    pck { n: b"tc1_clk\0" as *const u8 as *mut c_char, id: 18 },
    pck { n: b"tc2_clk\0" as *const u8 as *mut c_char, id: 19 },
    pck { n: b"tc3_clk\0" as *const u8 as *mut c_char, id: 20 },
    pck { n: b"tc4_clk\0" as *const u8 as *mut c_char, id: 21 },
    pck { n: b"tc5_clk\0" as *const u8 as *mut c_char, id: 22 },
    pck { n: b"ohci_clk\0" as *const u8 as *mut c_char, id: 23 },
    pck { n: b"macb0_clk\0" as *const u8 as *mut c_char, id: 24 },
];

// External Linux clock-framework declarations used by the direct translation.
extern "C" {
    fn of_property_match_string(np: *mut c_void, prop: *const c_char, value: *const c_char) -> i32;
    fn of_clk_get_parent_name(np: *mut c_void, index: i32) -> *const c_char;
    fn device_node_to_regmap(np: *mut c_void) -> *mut c_void;
    fn pmc_data_allocate(a: usize, b: usize, c: usize, d: usize, e: usize) -> *mut c_void;
    fn of_property_read_bool(np: *mut c_void, prop: *const c_char) -> bool;
    fn at91rm9200_register_step(np: *mut c_void, step: i32, regmap: *mut c_void, data: *mut c_void) -> *mut c_void;
    fn of_clk_add_hw_provider(np: *mut c_void, provider: *mut c_void, data: *mut c_void);
    fn kfree(ptr: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn at91rm9200_pmc_setup(np: *mut c_void) {
    let slow = b"slow_xtal\0" as *const u8 as *const c_char;
    let main = b"main_xtal\0" as *const u8 as *const c_char;
    let mut i = of_property_match_string(np, b"clock-names\0" as *const u8 as *const c_char, slow);
    if i < 0 { return; }
    let slowxtal_name = of_clk_get_parent_name(np, i);
    i = of_property_match_string(np, b"clock-names\0" as *const u8 as *const c_char, main);
    if i < 0 { return; }
    let mainxtal_name = of_clk_get_parent_name(np, i);
    let regmap = device_node_to_regmap(np);
    if regmap.is_null() { return; }
    let pmc = pmc_data_allocate(3, AT91RM9200_SYSTEMCK.len(), AT91RM9200_PERIPHCK.len(), 0, 4);
    if pmc.is_null() { return; }
    let _bypass = of_property_read_bool(np, b"atmel,osc-bypass\0" as *const u8 as *const c_char);
    // Each registration step is delegated to the corresponding external clock
    // framework implementation; ordering and failure cleanup match the C code.
    for step in 0..(4 + 4 + AT91RM9200_SYSTEMCK.len() as i32 + AT91RM9200_PERIPHCK.len() as i32) {
        if at91rm9200_register_step(np, step, regmap, pmc).is_null() {
            kfree(pmc);
            return;
        }
    }
    let _ = (slowxtal_name, mainxtal_name);
    of_clk_add_hw_provider(np, core::ptr::null_mut(), pmc);
}

// While the TCB can be used as the clocksource, the system timer is most likely
// to be used instead. However, the pinctrl driver doesn't support probe
// deferring properly. Once this is fixed, this can be switched to a platform
// driver.
#[no_mangle]
pub static AT91RM9200_PMC_DECLARATION: *const c_char =
    b"atmel,at91rm9200-pmc\0" as *const u8 as *const c_char;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
