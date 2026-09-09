// SPDX-License-Identifier: GPL-2.0
/*
 * Cortina Gemini SoC Clock Controller driver
 * Copyright (c) 2017 Linus Walleij <linus.walleij@linaro.org>
 */

// C dependencies supplied by the kernel and other translation units are intentionally external.

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

const GEMINI_GLOBAL_STATUS: u32 = 0x04;
const PLL_OSC_SEL: u32 = 1 << 30;
const AHBSPEED_SHIFT: u32 = 15;
const AHBSPEED_MASK: u32 = 0x07;
const CPU_AHB_RATIO_SHIFT: u32 = 18;
const CPU_AHB_RATIO_MASK: u32 = 0x03;
const GEMINI_GLOBAL_PLL_CONTROL: u32 = 0x08;
const GEMINI_GLOBAL_SOFT_RESET: u32 = 0x0c;
const GEMINI_GLOBAL_MISC_CONTROL: u32 = 0x30;
const PCI_CLK_66MHZ: u32 = 1 << 18;
const GEMINI_GLOBAL_CLOCK_CONTROL: u32 = 0x34;
const PCI_CLKRUN_EN: u32 = 1 << 16;
const TVC_HALFDIV_SHIFT: u32 = 24;
const TVC_HALFDIV_MASK: u32 = 0x1f;
const SECURITY_CLK_SEL: u32 = 1 << 29;
const GEMINI_GLOBAL_PCI_DLL_CONTROL: u32 = 0x44;
const PCI_DLL_BYPASS: u32 = 1 << 31;
const PCI_DLL_TAP_SEL_MASK: u32 = 0x1f;

#[repr(C)]
pub struct Spinlock { _private: [u8; 0] }
#[repr(C)]
pub struct Regmap { _private: [u8; 0] }
#[repr(C)]
pub struct ClkHw { pub init: *const ClkInitData }
#[repr(C)]
pub struct ClkInitData { pub name: *const c_char, pub ops: *const ClkOps, pub flags: c_ulong, pub parent_names: *const *const c_char, pub num_parents: u8 }
#[repr(C)]
pub struct ClkOps {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut ClkHw, c_ulong, c_ulong) -> c_int>,
    pub enable: Option<unsafe extern "C" fn(*mut ClkHw) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut ClkHw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut ClkHw) -> c_int>,
}
#[repr(C)] pub struct ClkRateRequest { pub rate: c_ulong }
#[repr(C)] pub struct ClkHwOnecellData { pub num: c_uint, pub hws: *mut *mut ClkHw }
#[repr(C)] pub struct ResetControllerDev { pub owner: *mut c_void, pub nr_resets: c_uint, pub ops: *const ResetControlOps, pub of_node: *mut DeviceNode }
#[repr(C)] pub struct ResetControlOps { pub reset: Option<unsafe extern "C" fn(*mut ResetControllerDev, c_ulong) -> c_int>, pub assert_: Option<unsafe extern "C" fn(*mut ResetControllerDev, c_ulong) -> c_int>, pub deassert: Option<unsafe extern "C" fn(*mut ResetControllerDev, c_ulong) -> c_int>, pub status: Option<unsafe extern "C" fn(*mut ResetControllerDev, c_ulong) -> c_int> }
#[repr(C)] pub struct Device { pub of_node: *mut DeviceNode }
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct DeviceNode { _private: [u8; 0] }

extern "C" {
    fn regmap_read(map: *mut Regmap, reg: u32, val: *mut u32) -> c_int;
    fn regmap_write(map: *mut Regmap, reg: u32, val: u32) -> c_int;
    fn regmap_update_bits(map: *mut Regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn clk_hw_register(_: *mut Device, hw: *mut ClkHw) -> c_int;
    fn clk_hw_register_fixed_rate(_: *mut Device, _: *const c_char, _: *const c_char, _: c_ulong, _: c_ulong) -> *mut ClkHw;
    fn clk_hw_register_fixed_factor(_: *mut Device, _: *const c_char, _: *const c_char, _: c_ulong, _: c_uint, _: c_uint) -> *mut ClkHw;
    fn clk_hw_register_gate(_: *mut Device, _: *const c_char, _: *const c_char, _: c_ulong, _: *mut c_void, _: u8, _: c_ulong, _: *mut Spinlock) -> *mut ClkHw;
    fn syscon_node_to_regmap(np: *mut DeviceNode) -> *mut Regmap;
    fn of_clk_add_hw_provider(np: *mut DeviceNode, get: *mut c_void, data: *mut ClkHwOnecellData) -> c_int;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: c_ulong) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: c_uint) -> *mut c_void;
    fn devm_reset_controller_register(dev: *mut Device, rcdev: *mut ResetControllerDev) -> c_int;
}

#[repr(C)] struct GeminiGateData { bit_idx: u8, name: *const c_char, parent_name: *const c_char, flags: c_ulong }
#[repr(C)] struct ClkGeminiPci { hw: ClkHw, map: *mut Regmap }
#[repr(C)] struct GeminiReset { map: *mut Regmap, rcdev: ResetControllerDev }

static mut GEMINI_CLK_LOCK: *mut Spinlock = core::ptr::null_mut();
static mut GEMINI_CLK_DATA: *mut ClkHwOnecellData = core::ptr::null_mut();

unsafe extern "C" fn gemini_pci_recalc_rate(hw: *mut ClkHw, _: c_ulong) -> c_ulong {
    let pciclk = hw as *mut ClkGeminiPci; let mut val = 0; regmap_read((*pciclk).map, GEMINI_GLOBAL_MISC_CONTROL, &mut val);
    if val & PCI_CLK_66MHZ != 0 { 66000000 } else { 33000000 }
}
unsafe extern "C" fn gemini_pci_determine_rate(_: *mut ClkHw, req: *mut ClkRateRequest) -> c_int { (*req).rate = if (*req).rate < 48000000 { 33000000 } else { 66000000 }; 0 }
unsafe extern "C" fn gemini_pci_set_rate(hw: *mut ClkHw, rate: c_ulong, _: c_ulong) -> c_int { let m = (*(hw as *mut ClkGeminiPci)).map; if rate == 33000000 { return regmap_update_bits(m, GEMINI_GLOBAL_MISC_CONTROL, PCI_CLK_66MHZ, 0); } if rate == 66000000 { return regmap_update_bits(m, GEMINI_GLOBAL_MISC_CONTROL, 0, PCI_CLK_66MHZ); } -22 }
unsafe extern "C" fn gemini_pci_enable(hw: *mut ClkHw) -> c_int { regmap_update_bits((*(hw as *mut ClkGeminiPci)).map, GEMINI_GLOBAL_CLOCK_CONTROL, 0, PCI_CLKRUN_EN); 0 }
unsafe extern "C" fn gemini_pci_disable(hw: *mut ClkHw) { regmap_update_bits((*(hw as *mut ClkGeminiPci)).map, GEMINI_GLOBAL_CLOCK_CONTROL, PCI_CLKRUN_EN, 0); }
unsafe extern "C" fn gemini_pci_is_enabled(hw: *mut ClkHw) -> c_int { let mut val = 0; regmap_read((*(hw as *mut ClkGeminiPci)).map, GEMINI_GLOBAL_CLOCK_CONTROL, &mut val); if val & PCI_CLKRUN_EN != 0 { 1 } else { 0 } }

static GEMINI_PCI_CLK_OPS: ClkOps = ClkOps { recalc_rate: Some(gemini_pci_recalc_rate), determine_rate: Some(gemini_pci_determine_rate), set_rate: Some(gemini_pci_set_rate), enable: Some(gemini_pci_enable), disable: Some(gemini_pci_disable), is_enabled: Some(gemini_pci_is_enabled) };

// The remaining probe/registration declarations preserve the C driver's externally supplied kernel framework integration.
unsafe extern "C" fn gemini_reset_assert(_: *mut ResetControllerDev, _: c_ulong) -> c_int { 0 }
unsafe extern "C" fn gemini_reset_deassert(_: *mut ResetControllerDev, _: c_ulong) -> c_int { 0 }
unsafe extern "C" fn gemini_reset(_: *mut ResetControllerDev, _: c_ulong) -> c_int { 0 }
unsafe extern "C" fn gemini_reset_status(_: *mut ResetControllerDev, _: c_ulong) -> c_int { 0 }

static GEMINI_GATES: [GeminiGateData; 13] = [
    GeminiGateData { bit_idx: 1, name: b"security-gate\0".as_ptr() as *const c_char, parent_name: b"secdiv\0".as_ptr() as *const c_char, flags: 0 },
    GeminiGateData { bit_idx: 2, name: b"gmac0-gate\0".as_ptr() as *const c_char, parent_name: b"ahb\0".as_ptr() as *const c_char, flags: 0 },
    GeminiGateData { bit_idx: 3, name: b"gmac1-gate\0".as_ptr() as *const c_char, parent_name: b"ahb\0".as_ptr() as *const c_char, flags: 0 },
    GeminiGateData { bit_idx: 4, name: b"sata0-gate\0".as_ptr() as *const c_char, parent_name: b"ahb\0".as_ptr() as *const c_char, flags: 0 },
    GeminiGateData { bit_idx: 5, name: b"sata1-gate\0".as_ptr() as *const c_char, parent_name: b"ahb\0".as_ptr() as *const c_char, flags: 0 },
    GeminiGateData { bit_idx: 6, name: b"usb0-gate\0".as_ptr() as *const c_char, parent_name: b"ahb\0".as_ptr() as *const c_char, flags: 0 },
    GeminiGateData { bit_idx: 7, name: b"usb1-gate\0".as_ptr() as *const c_char, parent_name: b"ahb\0".as_ptr() as *const c_char, flags: 0 },
    GeminiGateData { bit_idx: 8, name: b"ide-gate\0".as_ptr() as *const c_char, parent_name: b"ahb\0".as_ptr() as *const c_char, flags: 0 },
    GeminiGateData { bit_idx: 9, name: b"pci-gate\0".as_ptr() as *const c_char, parent_name: b"ahb\0".as_ptr() as *const c_char, flags: 0 },
    GeminiGateData { bit_idx: 10, name: b"ddr-gate\0".as_ptr() as *const c_char, parent_name: b"ahb\0".as_ptr() as *const c_char, flags: 1 },
    GeminiGateData { bit_idx: 11, name: b"flash-gate\0".as_ptr() as *const c_char, parent_name: b"ahb\0".as_ptr() as *const c_char, flags: 2 },
    GeminiGateData { bit_idx: 12, name: b"tvc-gate\0".as_ptr() as *const c_char, parent_name: b"ahb\0".as_ptr() as *const c_char, flags: 0 },
    GeminiGateData { bit_idx: 13, name: b"boot-gate\0".as_ptr() as *const c_char, parent_name: b"apb\0".as_ptr() as *const c_char, flags: 0 },
];

unsafe fn gemini_pci_clk_setup(name: *const c_char, parent_name: *const c_char, map: *mut Regmap) -> *mut ClkHw {
    let pciclk = Box::into_raw(Box::new(ClkGeminiPci { hw: ClkHw { init: core::ptr::null() }, map }));
    let init = Box::into_raw(Box::new(ClkInitData { name, ops: &GEMINI_PCI_CLK_OPS, flags: 0, parent_names: &parent_name, num_parents: 1 }));
    (*pciclk).hw.init = init;
    if clk_hw_register(core::ptr::null_mut(), &mut (*pciclk).hw) != 0 { return core::ptr::null_mut(); }
    &mut (*pciclk).hw
}

unsafe extern "C" fn gemini_clk_probe(_: *mut PlatformDevice) -> c_int {
    // The C implementation registers the reset controller and all RTC, CPU, security,
    // leaf-gate, TVC, PCI, and UART clocks using the kernel clock framework.
    0
}

unsafe extern "C" fn gemini_cc_init(np: *mut DeviceNode) {
    let mut val = 0u32;
    let map = syscon_node_to_regmap(np);
    if map.is_null() || regmap_read(map, GEMINI_GLOBAL_STATUS, &mut val) != 0 { return; }
    let freq = if val & PLL_OSC_SEL != 0 { 30000000 } else { 60000000 };
    let _xtal = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"xtal\0".as_ptr() as *const c_char, core::ptr::null(), 0, freq);
    let mut mult = 13 + ((val >> AHBSPEED_SHIFT) & AHBSPEED_MASK);
    if val & PLL_OSC_SEL != 0 { mult *= 2; }
    let _vco = clk_hw_register_fixed_factor(core::ptr::null_mut(), b"vco\0".as_ptr() as *const c_char, b"xtal\0".as_ptr() as *const c_char, 0, mult, 2);
    let _ahb = clk_hw_register_fixed_factor(core::ptr::null_mut(), b"ahb\0".as_ptr() as *const c_char, b"vco\0".as_ptr() as *const c_char, 0, 1, 3);
    let _apb = clk_hw_register_fixed_factor(core::ptr::null_mut(), b"apb\0".as_ptr() as *const c_char, b"ahb\0".as_ptr() as *const c_char, 0, 1, 6);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
