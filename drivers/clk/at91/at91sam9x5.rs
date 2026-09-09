// SPDX-License-Identifier: GPL-2.0
// Translated from at91sam9x5.c. Kernel dependencies are supplied externally.

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Corresponds to DEFINE_SPINLOCK(mck_lock).
static mut mck_lock: spinlock_t = spinlock_t { _private: [] };

#[repr(C)]
pub struct spinlock_t { pub _private: [u8; 0] }

#[repr(C)]
pub struct clk_master_characteristics { pub output: clk_range, pub divisors: [u32; 4], pub have_div3_pres: bool }
#[repr(C)]
pub struct clk_range { pub min: u64, pub max: u64 }
#[repr(C)]
pub struct clk_pll_characteristics { pub input: clk_range, pub num_output: usize, pub output: *const clk_range, pub icpll: *const u16, pub out: *const u8 }
#[repr(C)]
pub struct clk_pcr_layout { pub offset: u32, pub cmd: u32, pub pid_mask: u32, pub div_mask: u32 }
#[repr(C)]
pub struct pck { pub n: *const c_char, pub id: u8 }
#[repr(C)]
pub struct clk_hw;
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct pmc_data { pub chws: [*mut clk_hw; 4], pub pchws: [*mut clk_hw; 2], pub shws: [*mut clk_hw; 32], pub phws: [*mut clk_hw; 32] }

extern "C" {
    static at91rm9200_pll_layout: c_void;
    static at91sam9x5_master_layout: c_void;
    static at91sam9x5_programmable_layout: c_void;
    static mut pmc_pcr_lock: spinlock_t;
    fn of_property_match_string(np: *mut device_node, prop: *const c_char, string: *const c_char) -> c_int;
    fn of_clk_get_parent_name(np: *mut device_node, index: c_int) -> *const c_char;
    fn device_node_to_regmap(np: *mut device_node) -> *mut regmap;
    fn pmc_data_allocate(a: c_int, b: usize, c: c_int, d: c_int, e: c_int) -> *mut pmc_data;
    fn at91_clk_register_main_rc_osc(r: *mut regmap, n: *const c_char, f: u32, t: u32) -> *mut clk_hw;
    fn at91_clk_register_main_osc(r: *mut regmap, n: *const c_char, p: *const c_char, x: *const c_char, bypass: bool) -> *mut clk_hw;
    fn at91_clk_register_sam9x5_main(r: *mut regmap, n: *const c_char, p: *mut *const c_char, x: *const c_char, count: c_int) -> *mut clk_hw;
    fn at91_clk_register_pll(r: *mut regmap, n: *const c_char, p: *const c_char, id: c_int, layout: *const c_void, chars: *const clk_pll_characteristics) -> *mut clk_hw;
    fn at91_clk_register_plldiv(r: *mut regmap, n: *const c_char, p: *const c_char) -> *mut clk_hw;
    fn at91_clk_register_utmi(r: *mut regmap, a: *const c_char, n: *const c_char, p: *const c_char, x: *const c_char) -> *mut clk_hw;
    fn at91_clk_register_master_pres(r: *mut regmap, n: *const c_char, count: c_int, p: *mut *const c_char, x: *const c_char, layout: *const c_void, chars: *const clk_master_characteristics, lock: *mut spinlock_t) -> *mut clk_hw;
    fn at91_clk_register_master_div(r: *mut regmap, n: *const c_char, p: *const c_char, x: *const c_char, layout: *const c_void, chars: *const clk_master_characteristics, lock: *mut spinlock_t, flags: c_ulong, id: c_int) -> *mut clk_hw;
    fn at91sam9x5_clk_register_usb(r: *mut regmap, n: *const c_char, p: *mut *const c_char, count: c_int) -> *mut clk_hw;
    fn at91sam9x5_clk_register_smd(r: *mut regmap, n: *const c_char, p: *mut *const c_char, count: c_int) -> *mut clk_hw;
    fn at91_clk_register_programmable(r: *mut regmap, n: *const c_char, p: *mut *const c_char, x: *const c_char, count: c_int, id: c_int, layout: *const c_void, lock: *const c_void) -> *mut clk_hw;
    fn at91_clk_register_system(r: *mut regmap, n: *const c_char, p: *const c_char, x: *const c_char, id: u8, flags: c_ulong) -> *mut clk_hw;
    fn at91_clk_register_sam9x5_peripheral(r: *mut regmap, lock: *mut spinlock_t, layout: *const clk_pcr_layout, n: *const c_char, p: *const c_char, x: *const c_char, id: u8, range: *mut clk_range, min: c_int, max: c_int) -> *mut clk_hw;
    fn of_clk_add_hw_provider(np: *mut device_node, get: *const c_void, data: *mut pmc_data);
    fn kfree(p: *mut pmc_data);
}

const PMC_MAIN: usize = 0;
const PMC_PLLACK: usize = 1;
const PMC_UTMI: usize = 2;
const PMC_MCK: usize = 3;
const CLK_SET_RATE_GATE: c_ulong = 1;
const CLK_IS_CRITICAL: c_ulong = 1;

static MCK_CHARACTERISTICS: clk_master_characteristics = clk_master_characteristics { output: clk_range { min: 0, max: 133333333 }, divisors: [1, 2, 4, 3], have_div3_pres: true };
static PLLA_OUT: [u8; 8] = [0, 1, 2, 3, 0, 1, 2, 3];
static PLLA_ICPLL: [u16; 8] = [0, 0, 0, 0, 1, 1, 1, 1];
static PLLA_OUTPUTS: [clk_range; 8] = [
    clk_range { min: 745000000, max: 800000000 }, clk_range { min: 695000000, max: 750000000 },
    clk_range { min: 645000000, max: 700000000 }, clk_range { min: 595000000, max: 650000000 },
    clk_range { min: 545000000, max: 600000000 }, clk_range { min: 495000000, max: 555000000 },
    clk_range { min: 445000000, max: 500000000 }, clk_range { min: 400000000, max: 450000000 },
];
static PLLA_CHARACTERISTICS: clk_pll_characteristics = clk_pll_characteristics { input: clk_range { min: 2000000, max: 32000000 }, num_output: 8, output: PLLA_OUTPUTS.as_ptr(), icpll: PLLA_ICPLL.as_ptr(), out: PLLA_OUT.as_ptr() };

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
static SYSTEMCK: [( *const c_char, *const c_char, c_ulong, u8); 6] = [
    (cstr!("ddrck"), cstr!("masterck_div"), CLK_IS_CRITICAL, 2), (cstr!("smdck"), cstr!("smdclk"), 0, 4),
    (cstr!("uhpck"), cstr!("usbck"), 0, 6), (cstr!("udpck"), cstr!("usbck"), 0, 7),
    (cstr!("pck0"), cstr!("prog0"), 0, 8), (cstr!("pck1"), cstr!("prog1"), 0, 9),
];
static PCR_LAYOUT: clk_pcr_layout = clk_pcr_layout { offset: 0x10c, cmd: 1 << 12, pid_mask: (1 << 6) - 1, div_mask: 3 << 16 };

static PERIPHCK: [( *const c_char, u8); 22] = [
    (cstr!("pioAB_clk"),2),(cstr!("pioCD_clk"),3),(cstr!("smd_clk"),4),(cstr!("usart0_clk"),5),(cstr!("usart1_clk"),6),(cstr!("usart2_clk"),7),(cstr!("twi0_clk"),9),(cstr!("twi1_clk"),10),(cstr!("twi2_clk"),11),(cstr!("mci0_clk"),12),(cstr!("spi0_clk"),13),(cstr!("spi1_clk"),14),(cstr!("uart0_clk"),15),(cstr!("uart1_clk"),16),(cstr!("tcb0_clk"),17),(cstr!("pwm_clk"),18),(cstr!("adc_clk"),19),(cstr!("dma0_clk"),20),(cstr!("dma1_clk"),21),(cstr!("uhphs_clk"),22),(cstr!("udphs_clk"),23),(cstr!("mci1_clk"),26),
];

static G15: [( *const c_char, u8); 2] = [(cstr!("lcdc_clk"),25),(core::ptr::null(),0)];
static G25: [( *const c_char, u8); 3] = [(cstr!("usart3_clk"),8),(cstr!("macb0_clk"),24),(core::ptr::null(),0)];
static G35: [( *const c_char, u8); 3] = [(cstr!("macb0_clk"),24),(cstr!("lcdc_clk"),25),(core::ptr::null(),0)];
static X25: [( *const c_char, u8); 6] = [(cstr!("usart3_clk"),8),(cstr!("macb0_clk"),24),(cstr!("macb1_clk"),27),(cstr!("can0_clk"),29),(cstr!("can1_clk"),30),(core::ptr::null(),0)];
static X35: [( *const c_char, u8); 5] = [(cstr!("macb0_clk"),24),(cstr!("lcdc_clk"),25),(cstr!("can0_clk"),29),(cstr!("can1_clk"),30),(core::ptr::null(),0)];

unsafe fn at91sam9x5_pmc_setup(np: *mut device_node, extra: &[( *const c_char, u8)], has_lcdck: bool) {
    let mut range = clk_range { min: 0, max: 0 };
    let mut parent_names = [core::ptr::null(); 6];
    let slow = of_property_match_string(np, cstr!("clock-names"), cstr!("slow_clk")); if slow < 0 { return; }
    let slck = of_clk_get_parent_name(np, slow);
    let main = of_property_match_string(np, cstr!("clock-names"), cstr!("main_xtal")); if main < 0 { return; }
    let mainxtal = of_clk_get_parent_name(np, main);
    let regmap = device_node_to_regmap(np); if regmap.is_null() { return; }
    let pmc = pmc_data_allocate(2, SYSTEMCK.len(), 31, 0, 2); if pmc.is_null() { return; }
    let mut hw = at91_clk_register_main_rc_osc(regmap, cstr!("main_rc_osc"), 12000000, 50000000); if hw.is_null() { kfree(pmc); return; }
    hw = at91_clk_register_main_osc(regmap, cstr!("main_osc"), mainxtal, core::ptr::null(), false); if hw.is_null() { kfree(pmc); return; }
    parent_names[0]=cstr!("main_rc_osc"); parent_names[1]=cstr!("main_osc");
    hw=at91_clk_register_sam9x5_main(regmap,cstr!("mainck"),parent_names.as_mut_ptr(),core::ptr::null(),2); if hw.is_null(){kfree(pmc);return;} (*pmc).chws[PMC_MAIN]=hw;
    hw=at91_clk_register_pll(regmap,cstr!("pllack"),cstr!("mainck"),0,&at91rm9200_pll_layout,&PLLA_CHARACTERISTICS); if hw.is_null(){kfree(pmc);return;}
    hw=at91_clk_register_plldiv(regmap,cstr!("plladivck"),cstr!("pllack")); if hw.is_null(){kfree(pmc);return;} (*pmc).chws[PMC_PLLACK]=hw;
    hw=at91_clk_register_utmi(regmap,core::ptr::null(),cstr!("utmick"),cstr!("mainck"),core::ptr::null()); if hw.is_null(){kfree(pmc);return;} (*pmc).chws[PMC_UTMI]=hw;
    parent_names[0]=slck; parent_names[1]=cstr!("mainck"); parent_names[2]=cstr!("plladivck"); parent_names[3]=cstr!("utmick");
    hw=at91_clk_register_master_pres(regmap,cstr!("masterck_pres"),4,parent_names.as_mut_ptr(),core::ptr::null(),&at91sam9x5_master_layout,&MCK_CHARACTERISTICS,&mut mck_lock); if hw.is_null(){kfree(pmc);return;}
    hw=at91_clk_register_master_div(regmap,cstr!("masterck_div"),cstr!("masterck_pres"),core::ptr::null(),&at91sam9x5_master_layout,&MCK_CHARACTERISTICS,&mut mck_lock,CLK_SET_RATE_GATE,0); if hw.is_null(){kfree(pmc);return;} (*pmc).chws[PMC_MCK]=hw;
    parent_names[0]=cstr!("plladivck"); parent_names[1]=cstr!("utmick");
    hw=at91sam9x5_clk_register_usb(regmap,cstr!("usbck"),parent_names.as_mut_ptr(),2); if hw.is_null(){kfree(pmc);return;}
    hw=at91sam9x5_clk_register_smd(regmap,cstr!("smdclk"),parent_names.as_mut_ptr(),2); if hw.is_null(){kfree(pmc);return;}
    of_clk_add_hw_provider(np,core::ptr::null(),pmc);
}

unsafe fn at91sam9g15_pmc_setup(np:*mut device_node){at91sam9x5_pmc_setup(np,&G15,true)}
unsafe fn at91sam9g25_pmc_setup(np:*mut device_node){at91sam9x5_pmc_setup(np,&G25,false)}
unsafe fn at91sam9g35_pmc_setup(np:*mut device_node){at91sam9x5_pmc_setup(np,&G35,true)}
unsafe fn at91sam9x25_pmc_setup(np:*mut device_node){at91sam9x5_pmc_setup(np,&X25,false)}
unsafe fn at91sam9x35_pmc_setup(np:*mut device_node){at91sam9x5_pmc_setup(np,&X35,true)}

// CLK_OF_DECLARE registrations:
// atmel,at91sam9g15-pmc -> at91sam9g15_pmc_setup
// atmel,at91sam9g25-pmc -> at91sam9g25_pmc_setup
// atmel,at91sam9g35-pmc -> at91sam9g35_pmc_setup
// atmel,at91sam9x25-pmc -> at91sam9x25_pmc_setup
// atmel,at91sam9x35-pmc -> at91sam9x35_pmc_setup

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
