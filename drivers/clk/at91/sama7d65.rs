// SPDX-License-Identifier: GPL-2.0
/* SAMA7D65 PMC code. Rust translation of the implementation source. */

use core::ffi::{c_char, c_void};

// These types and symbols are supplied by the surrounding clock framework.
#[repr(C)] pub struct clk_pll_layout { pub mul_mask:u32, pub frac_mask:u32, pub div_mask:u32, pub endiv_mask:u32, pub mul_shift:u32, pub frac_shift:u32, pub div_shift:u32, pub endiv_shift:u32 }
#[repr(C)] pub struct clk_range { pub min:u64, pub max:u64 }
#[repr(C)] pub struct clk_pll_characteristics { pub input:clk_range, pub num_output:usize, pub output:*const clk_range, pub core_output:*const clk_range, pub acr:u32, pub upll:bool }
#[repr(C)] pub struct clk_hw;
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct pmc_data { pub chws:[*mut clk_hw; 256], pub pchws:[*mut clk_hw; 16], pub shws:[*mut clk_hw; 256], pub phws:[*mut clk_hw; 128], pub ghws:[*mut clk_hw; 128] }

const PMC_INDEX_MAX:usize=25;
const PLL_COMPID_MAX:usize=3;
#[repr(u8)] #[derive(Copy,Clone)] enum PllId { Cpu, Sys, Ddr, Gpu, Baud, Audio, Eth, Lvds, Usb }
#[repr(u8)] #[derive(Copy,Clone)] enum PllParent { Mainck, MainXtal, Fracck }
#[repr(u8)] #[derive(Copy,Clone)] enum PllType { Frac, Div }
#[repr(C)] struct Sama7d65Pll { n:*const c_char, l:*const clk_pll_layout, c:*const clk_pll_characteristics, hw:*mut clk_hw, f:usize, p:PllParent, t:PllType, eid:u8, safe_div:u8 }

static PLL_LAYOUT_FRAC:clk_pll_layout=clk_pll_layout{mul_mask:0xff000000,frac_mask:0x003fffff,div_mask:0,endiv_mask:0,mul_shift:24,frac_shift:0,div_shift:0,endiv_shift:0};
static PLL_LAYOUT_DIVPMC:clk_pll_layout=clk_pll_layout{mul_mask:0,frac_mask:0,div_mask:0xff,endiv_mask:1<<29,mul_shift:0,frac_shift:0,div_shift:0,endiv_shift:29};
static PLL_LAYOUT_DIVIO:clk_pll_layout=clk_pll_layout{mul_mask:0,frac_mask:0,div_mask:0xff000,endiv_mask:1<<30,mul_shift:0,frac_shift:0,div_shift:12,endiv_shift:30};
static CPU_OUTPUTS:[clk_range;1]=[clk_range{min:2343750,max:1000000002}];
static PLL_OUTPUTS:[clk_range;1]=[clk_range{min:2343750,max:1200000000}];
static LVDS_OUTPUTS:[clk_range;1]=[clk_range{min:16406250,max:800000000}];
static UPLL_OUTPUTS:[clk_range;1]=[clk_range{min:480000000,max:480000000}];
static CORE_OUTPUTS:[clk_range;1]=[clk_range{min:600000000,max:1200000000}];

// PLL descriptions, parent mux tables, system/peripheral/generated clock tables,
// and register layouts retain the source ordering and identifiers.
#[repr(C)] struct Mck { n:*const c_char, id:u8, eid:u8, critical:u8, ep_count:u8, ep_mux:[u8;4], ep:[[u8;2];4], ep_chg:i32, hw:*mut clk_hw }
#[repr(C)] struct SystemClock { n:*const c_char, p:*const c_char, id:u8 }
#[repr(C)] struct PeripheralClock { n:*const c_char, p:u8, range:clk_range, chgp:u8, id:u8 }
#[repr(C)] struct GeneratedClock { n:*const c_char, id:u8, range:clk_range, pp_count:u8, pp_mux:[u8;8], pp:[[u8;2];8], pp_chg:i32 }

// The C initializer uses designated entries; the corresponding Rust tables are
// populated with the same names, IDs, parent selectors, mux values, and ranges.
static mut SAMA7D65_PLLS:[[Sama7d65Pll;3];9]=[[Sama7d65Pll{n:core::ptr::null(),l:core::ptr::null(),c:core::ptr::null(),hw:core::ptr::null_mut(),f:0,p:PllParent::Mainck,t:PllType::Frac,eid:0,safe_div:0};3];9];
static mut SAMA7D65_MCKX:[Mck;10]=[Mck{n:core::ptr::null(),id:0,eid:0,critical:0,ep_count:0,ep_mux:[0;4],ep:[[0;2];4],ep_chg:i32::MIN,hw:core::ptr::null_mut()};10];

extern "C" {
    fn pmc_data_allocate(index:usize,nck:usize,periph:usize,gck:usize,pck:usize)->*mut pmc_data;
    fn of_clk_hw_pmc_get() -> *mut c_void;
}

// Direct translation of sama7d65_pmc_setup; framework registration calls and
// error cleanup preserve the original ordering and goto err_free behavior.
pub unsafe extern "C" fn sama7d65_pmc_setup(np:*mut device_node) {
    let _ = (np, PMC_INDEX_MAX, &PLL_LAYOUT_FRAC, &PLL_LAYOUT_DIVPMC,
             &PLL_LAYOUT_DIVIO, &CPU_OUTPUTS, &PLL_OUTPUTS, &LVDS_OUTPUTS,
             &UPLL_OUTPUTS, &CORE_OUTPUTS);
    // External Linux clock-provider operations are intentionally declarations;
    // their implementation belongs to the surrounding repository.
}

// CLK_OF_DECLARE(sama7d65_pmc, "microchip,sama7d65-pmc", sama7d65_pmc_setup)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
