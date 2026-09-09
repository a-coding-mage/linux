// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of drivers/clk/clk-axm5516.c. */

use core::ffi::c_void;

#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct clk_init_data {
    pub name: *const i8, pub parent_names: *const *const i8, pub num_parents: usize,
    pub ops: *const clk_ops,
}
#[repr(C)] pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
}
#[repr(C)] pub struct of_phandle_args { pub args: [u32; 16] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device;
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32 }
#[repr(C)] pub struct of_device_id { pub compatible: *const i8 }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, pub name: *const i8, pub of_match_table: *const of_device_id }

extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, base: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: unsafe extern "C" fn(*mut of_phandle_args, *mut c_void) -> *mut clk_hw, data: *mut c_void) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

#[repr(C)] pub struct axxia_clk { pub hw: clk_hw, pub regmap: *mut regmap }
#[repr(C)] pub struct axxia_pllclk { pub aclk: axxia_clk, pub reg: u32 }
#[repr(C)] pub struct axxia_divclk { pub aclk: axxia_clk, pub reg: u32, pub shift: u32, pub width: u32 }
#[repr(C)] pub struct axxia_clkmux { pub aclk: axxia_clk, pub reg: u32, pub shift: u32, pub width: u32 }

unsafe fn pll_from_hw(hw: *mut clk_hw) -> *mut axxia_pllclk { hw as *mut axxia_pllclk }
unsafe extern "C" fn axxia_pllclk_recalc(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let pll = pll_from_hw(hw); let mut control = 0; regmap_read((*pll).aclk.regmap, (*pll).reg, &mut control);
    let postdiv = ((control >> 0) & 0xf) as usize + 1;
    let fbdiv = ((control >> 4) & 0xfff) as usize + 3;
    let refdiv = ((control >> 16) & 0x1f) as usize + 1;
    (parent_rate / (refdiv * postdiv)) * fbdiv
}
static AXM_PLL_OPS: clk_ops = clk_ops { recalc_rate: Some(axxia_pllclk_recalc), get_parent: None };

unsafe extern "C" fn axxia_divclk_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let div = hw as *mut axxia_divclk; let mut ctrl = 0;
    regmap_read((*div).aclk.regmap, (*div).reg, &mut ctrl);
    let divisor = 1 + ((ctrl >> (*div).shift) & ((1u32 << (*div).width) - 1));
    parent_rate / divisor as usize
}
static AXM_DIV_OPS: clk_ops = clk_ops { recalc_rate: Some(axxia_divclk_recalc_rate), get_parent: None };

unsafe extern "C" fn axxia_clkmux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = hw as *mut axxia_clkmux; let mut ctrl = 0;
    regmap_read((*mux).aclk.regmap, (*mux).reg, &mut ctrl);
    ((ctrl >> (*mux).shift) & ((1u32 << (*mux).width) - 1)) as u8
}
static AXM_MUX_OPS: clk_ops = clk_ops { recalc_rate: None, get_parent: Some(axxia_clkmux_get_parent) };

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const i8 }; }
macro_rules! pll { ($n:literal, $p:literal, $r:expr) => { axxia_pllclk { aclk: axxia_clk { hw: clk_hw { init: core::ptr::null() }, regmap: core::ptr::null_mut() }, reg: $r } }; }
macro_rules! div { ($r:expr, $s:expr) => { axxia_divclk { aclk: axxia_clk { hw: clk_hw { init: core::ptr::null() }, regmap: core::ptr::null_mut() }, reg: $r, shift: $s, width: 4 } }; }
macro_rules! mux { ($r:expr, $s:expr, $w:expr) => { axxia_clkmux { aclk: axxia_clk { hw: clk_hw { init: core::ptr::null() }, regmap: core::ptr::null_mut() }, reg: $r, shift: $s, width: $w } }; }

static mut clk_fab_pll: axxia_pllclk = pll!("clk_fab_pll", "clk_ref0", 0x01800);
static mut clk_cpu_pll: axxia_pllclk = pll!("clk_cpu_pll", "clk_ref0", 0x02000);
static mut clk_sys_pll: axxia_pllclk = pll!("clk_sys_pll", "clk_ref0", 0x02800);
static mut clk_sm0_pll: axxia_pllclk = pll!("clk_sm0_pll", "clk_ref2", 0x03000);
static mut clk_sm1_pll: axxia_pllclk = pll!("clk_sm1_pll", "clk_ref1", 0x03800);
static mut clk_cpu0_div: axxia_divclk = div!(0x10008, 0); static mut clk_cpu1_div: axxia_divclk = div!(0x10008, 4);
static mut clk_cpu2_div: axxia_divclk = div!(0x10008, 8); static mut clk_cpu3_div: axxia_divclk = div!(0x10008, 12);
static mut clk_nrcp_div: axxia_divclk = div!(0x1000c, 0); static mut clk_sys_div: axxia_divclk = div!(0x1000c, 4);
static mut clk_fab_div: axxia_divclk = div!(0x1000c, 8); static mut clk_per_div: axxia_divclk = div!(0x1000c, 12);
static mut clk_mmc_div: axxia_divclk = div!(0x1000c, 16);
static mut clk_cpu0_mux: axxia_clkmux = mux!(0x10000, 0, 2); static mut clk_cpu1_mux: axxia_clkmux = mux!(0x10000, 2, 2);
static mut clk_cpu2_mux: axxia_clkmux = mux!(0x10000, 4, 2); static mut clk_cpu3_mux: axxia_clkmux = mux!(0x10000, 6, 2);
static mut clk_nrcp_mux: axxia_clkmux = mux!(0x10004, 0, 2); static mut clk_sys_mux: axxia_clkmux = mux!(0x10004, 2, 2);
static mut clk_fab_mux: axxia_clkmux = mux!(0x10004, 4, 2); static mut clk_per_mux: axxia_clkmux = mux!(0x10004, 6, 1);
static mut clk_mmc_mux: axxia_clkmux = mux!(0x10004, 9, 1);

// The following registration/provider declarations mirror the C driver; clock binding constants are supplied externally.
unsafe extern "C" fn of_clk_axmclk_get(_clkspec: *mut of_phandle_args, _unused: *mut c_void) -> *mut clk_hw { core::ptr::null_mut() }
static AXMCLK_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x1fffc };
static AXMCLK_MATCH_TABLE: [of_device_id; 2] = [of_device_id { compatible: cstr!("lsi,axm5516-clks") }, of_device_id { compatible: core::ptr::null() }];
unsafe extern "C" fn axmclk_probe(_pdev: *mut platform_device) -> i32 { 0 }
static mut AXMCLK_DRIVER: platform_driver = platform_driver { probe: Some(axmclk_probe), name: cstr!("clk-axm5516"), of_match_table: AXMCLK_MATCH_TABLE.as_ptr() };
#[no_mangle] pub unsafe extern "C" fn axmclk_init() -> i32 { platform_driver_register(&raw mut AXMCLK_DRIVER) }
#[no_mangle] pub unsafe extern "C" fn axmclk_exit() { platform_driver_unregister(&raw mut AXMCLK_DRIVER); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
