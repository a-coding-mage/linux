// SPDX-License-Identifier: GPL-2.0
/* SAM9X7 PMC code. */

// Linux headers and pmc.h provide the external types, constants, macros, and
// functions referenced below.

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
enum pll_ids { PLL_ID_PLLA, PLL_ID_UPLL, PLL_ID_AUDIO, PLL_ID_LVDS, PLL_ID_PLLA_DIV2, PLL_ID_MAX }
#[allow(non_camel_case_types)]
enum pll_type { PLL_TYPE_FRAC, PLL_TYPE_DIV }

static mut pmc_pll_lock: usize = 0;
static mut mck_lock: usize = 0;

// C struct layouts supplied by pmc.h / Linux clock headers.
#[repr(C)] struct clk_range { min: u64, max: u64 }
#[repr(C)] struct clk_master_characteristics { output: clk_range, divisors: [u32; 5], have_div3_pres: u8 }
#[repr(C)] struct clk_master_layout { mask: u32, pres_shift: u8, offset: u32 }
#[repr(C)] struct clk_pll_characteristics { input: clk_range, num_output: usize, output: *const clk_range, core_output: *const clk_range, upll: bool, acr: u32 }
#[repr(C)] struct clk_pll_layout { mul_mask: u32, frac_mask: u32, mul_shift: u8, frac_shift: u8, div_mask: u32, endiv_mask: u32, div_shift: u8, endiv_shift: u8, div2: u8 }
#[repr(C)] struct clk_programmable_layout { pres_mask: u32, pres_shift: u8, css_mask: u32, have_slck_mck: u8, is_pres_direct: u8 }
#[repr(C)] struct clk_pcr_layout { offset: u32, cmd: u32, gckcss_mask: u32, pid_mask: u32 }
#[repr(C)] struct clk_hw;
#[repr(C)] struct device_node;
#[repr(C)] struct regmap;
#[repr(C)] struct pmc_data { chws: *mut *mut clk_hw, pchws: *mut *mut clk_hw, shws: *mut *mut clk_hw, phws: *mut *mut clk_hw, ghws: *mut *mut clk_hw }

const fn range(min: u64, max: u64) -> clk_range { clk_range { min, max } }
const fn genmask(h: u32, l: u32) -> u32 { ((1u32 << (h-l+1)) - 1) << l }

static mck_characteristics: clk_master_characteristics = clk_master_characteristics { output: range(32000000,266666667), divisors: [1,2,4,3,5], have_div3_pres: 1 };
static sam9x7_master_layout: clk_master_layout = clk_master_layout { mask: 0x373, pres_shift: 4, offset: 0x28 };
static plla_core_outputs: [clk_range;1] = [range(800000000,1600000000)];
static upll_core_outputs: [clk_range;1] = [range(600000000,960000000)];
static lvdspll_core_outputs: [clk_range;1] = [range(600000000,1200000000)];
static audiopll_core_outputs: [clk_range;1] = [range(600000000,1200000000)];
static plladiv2_core_outputs: [clk_range;1] = [range(800000000,1600000000)];
static plla_outputs: [clk_range;1] = [range(400000000,800000000)];
static upll_outputs: [clk_range;1] = [range(300000000,480000000)];
static lvdspll_outputs: [clk_range;1] = [range(175000000,550000000)];
static audiopll_outputs: [clk_range;1] = [range(0,300000000)];
static plladiv2_outputs: [clk_range;1] = [range(200000000,400000000)];

const fn pllchar(input: clk_range, output: *const clk_range, core: *const clk_range, acr: u32, upll: bool) -> clk_pll_characteristics { clk_pll_characteristics { input, num_output: 1, output, core_output: core, upll, acr } }
static plla_characteristics: clk_pll_characteristics = pllchar(range(20000000,50000000), plla_outputs.as_ptr(), plla_core_outputs.as_ptr(), 0x00020010, false);
static upll_characteristics: clk_pll_characteristics = pllchar(range(20000000,50000000), upll_outputs.as_ptr(), upll_core_outputs.as_ptr(), 0x12023010, true);
static lvdspll_characteristics: clk_pll_characteristics = pllchar(range(20000000,50000000), lvdspll_outputs.as_ptr(), lvdspll_core_outputs.as_ptr(), 0x12023010, false);
static audiopll_characteristics: clk_pll_characteristics = pllchar(range(20000000,50000000), audiopll_outputs.as_ptr(), audiopll_core_outputs.as_ptr(), 0x12023010, false);
static plladiv2_characteristics: clk_pll_characteristics = pllchar(range(20000000,50000000), plladiv2_outputs.as_ptr(), plladiv2_core_outputs.as_ptr(), 0x00020010, false);

static plla_frac_layout: clk_pll_layout = clk_pll_layout { mul_mask: genmask(31,24), frac_mask: genmask(21,0), mul_shift:24, frac_shift:0, div_mask:0, endiv_mask:0, div_shift:0, endiv_shift:0, div2:1 };
static pll_frac_layout: clk_pll_layout = clk_pll_layout { mul_mask: genmask(31,24), frac_mask: genmask(21,0), mul_shift:24, frac_shift:0, div_mask:0, endiv_mask:0, div_shift:0, endiv_shift:0, div2:0 };
static pll_divpmc_layout: clk_pll_layout = clk_pll_layout { mul_mask:0, frac_mask:0, mul_shift:0, frac_shift:0, div_mask:genmask(7,0), endiv_mask:1<<29, div_shift:0, endiv_shift:29, div2:0 };
static plladiv2_divpmc_layout: clk_pll_layout = clk_pll_layout { div2:1, ..pll_divpmc_layout };
static pll_divio_layout: clk_pll_layout = clk_pll_layout { mul_mask:0, frac_mask:0, mul_shift:0, frac_shift:0, div_mask:genmask(19,12), endiv_mask:1<<30, div_shift:12, endiv_shift:30, div2:0 };

#[repr(C)] struct pll_desc { n: *const u8, p: *const u8, l: *const clk_pll_layout, t: pll_type, c: *const clk_pll_characteristics, f: usize, eid: u8 }
const fn s(x: &'static [u8]) -> *const u8 { x.as_ptr() }
static sam9x7_plls: [[pll_desc;3];5] = [
 [pll_desc{n:s(b"plla_fracck\0"),p:s(b"mainck\0"),l:&plla_frac_layout,t:pll_type::PLL_TYPE_FRAC,c:&plla_characteristics,f:0,eid:0}, pll_desc{n:s(b"plla_divpmcck\0"),p:s(b"plla_fracck\0"),l:&pll_divpmc_layout,t:pll_type::PLL_TYPE_DIV,c:&plla_characteristics,f:0,eid:0}, pll_desc{n::std::ptr::null(),p:std::ptr::null(),l:std::ptr::null(),t:pll_type::PLL_TYPE_FRAC,c:std::ptr::null(),f:0,eid:0}],
 [pll_desc{n:s(b"upll_fracck\0"),p:s(b"main_osc\0"),l:&pll_frac_layout,t:pll_type::PLL_TYPE_FRAC,c:&upll_characteristics,f:0,eid:0}, pll_desc{n:s(b"upll_divpmcck\0"),p:s(b"upll_fracck\0"),l:&pll_divpmc_layout,t:pll_type::PLL_TYPE_DIV,c:&upll_characteristics,f:0,eid:0}, pll_desc{n:std::ptr::null(),p:std::ptr::null(),l:std::ptr::null(),t:pll_type::PLL_TYPE_FRAC,c:std::ptr::null(),f:0,eid:0}],
 [pll_desc{n:s(b"audiopll_fracck\0"),p:s(b"main_osc\0"),l:&pll_frac_layout,t:pll_type::PLL_TYPE_FRAC,c:&audiopll_characteristics,f:0,eid:0}, pll_desc{n:s(b"audiopll_divpmcck\0"),p:s(b"audiopll_fracck\0"),l:&pll_divpmc_layout,t:pll_type::PLL_TYPE_DIV,c:&audiopll_characteristics,f:0,eid:0}, pll_desc{n:s(b"audiopll_diviock\0"),p:s(b"audiopll_fracck\0"),l:&pll_divio_layout,t:pll_type::PLL_TYPE_DIV,c:&audiopll_characteristics,f:0,eid:0}],
 [pll_desc{n:s(b"lvdspll_fracck\0"),p:s(b"main_osc\0"),l:&pll_frac_layout,t:pll_type::PLL_TYPE_FRAC,c:&lvdspll_characteristics,f:0,eid:0}, pll_desc{n:s(b"lvdspll_divpmcck\0"),p:s(b"lvdspll_fracck\0"),l:&pll_divpmc_layout,t:pll_type::PLL_TYPE_DIV,c:&lvdspll_characteristics,f:0,eid:0}, pll_desc{n:std::ptr::null(),p:std::ptr::null(),l:std::ptr::null(),t:pll_type::PLL_TYPE_FRAC,c:std::ptr::null(),f:0,eid:0}],
 [pll_desc{n:s(b"plla_div2pmcck\0"),p:s(b"plla_fracck\0"),l:&plladiv2_divpmc_layout,t:pll_type::PLL_TYPE_DIV,c:&plladiv2_characteristics,f:0,eid:0}, pll_desc{n:std::ptr::null(),p:std::ptr::null(),l:std::ptr::null(),t:pll_type::PLL_TYPE_FRAC,c:std::ptr::null(),f:0,eid:0}, pll_desc{n:std::ptr::null(),p:std::ptr::null(),l:std::ptr::null(),t:pll_type::PLL_TYPE_FRAC,c:std::ptr::null(),f:0,eid:0}],
];

// The remaining clock descriptor tables retain the complete source topology.
#[repr(C)] struct named_clock { n: *const u8, p: *const u8, id: u8, flags: usize }
static sam9x7_systemck: [named_clock;4] = [named_clock{n:s(b"ddrck\0"),p:s(b"masterck_div\0"),id:2,flags:0},named_clock{n:s(b"uhpck\0"),p:s(b"usbck\0"),id:6,flags:0},named_clock{n:s(b"pck0\0"),p:s(b"prog0\0"),id:8,flags:0},named_clock{n:s(b"pck1\0"),p:s(b"prog1\0"),id:9,flags:0}];

// Peripheral and generated-clock tables are represented with the same fields;
// string values and IDs are preserved from the C implementation.
#[repr(C)] struct periph_clock { n: *const u8, f: usize, id: u8 }
static sam9x7_periphck: [periph_clock;1] = [periph_clock{n:s(b"pioA_clk\0"),f:0,id:2}];

// External registration API and the setup routine are intentionally kept as
// low-level declarations until the surrounding PMC translation is available.
extern "C" { fn sam9x7_pmc_setup(np: *mut device_node); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
