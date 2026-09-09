// SPDX-License-Identifier: GPL-2.0
/*
 * LPASS Audio CC and Always ON CC Glitch Free Mux clock driver
 *
 * Copyright (c) 2020 Linaro Ltd.
 * Author: Srinivas Kandagatla <srinivas.kandagatla@linaro.org>
 */

// Kernel types, functions, constants, and device-tree bindings referenced by
// this file are supplied by the surrounding Linux/Rust bindings.

#[repr(C)]
pub struct lpass_gfm {
    pub dev: *mut device,
    pub base: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct clk_gfm {
    pub mux_reg: u32,
    pub mux_mask: u32,
    pub hw: clk_hw,
    pub priv_: *mut lpass_gfm,
    pub gfm_mux: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const u8,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub num_parents: u8,
    pub parent_data: *const clk_parent_data,
}

#[repr(C)]
pub struct clk_parent_data {
    pub index: u8,
    pub fw_name: *const u8,
}

#[repr(C)]
pub struct clk_ops {
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>,
    pub determine_rate: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct clk_hw_onecell_data {
    pub hws: *mut *mut clk_hw,
    pub num: u32,
}

#[repr(C)]
pub struct lpass_gfm_data {
    pub onecell_data: *mut clk_hw_onecell_data,
    pub gfm_clks: *mut *mut clk_gfm,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device { pub dev: device }

unsafe extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn of_device_get_match_data(dev: *mut device) -> *const lpass_gfm_data;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn devm_pm_runtime_enable(dev: *mut device) -> i32;
    fn devm_pm_clk_create(dev: *mut device) -> i32;
    fn of_pm_clk_add_clks(dev: *mut device) -> i32;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: unsafe extern "C" fn(), data: *mut clk_hw_onecell_data) -> i32;
    fn pm_clk_suspend() -> i32;
    fn pm_clk_resume() -> i32;
}

const BIT_0: u32 = 1;
const CLK_SET_RATE_PARENT: u32 = 1 << 0;
const CLK_OPS_PARENT_ENABLE: u32 = 1 << 1;

unsafe fn clk_gfm_from_hw(hw: *mut clk_hw) -> *mut clk_gfm {
    hw as *mut clk_gfm
}

unsafe extern "C" fn clk_gfm_get_parent(hw: *mut clk_hw) -> u8 {
    let clk = &*clk_gfm_from_hw(hw);
    (readl(clk.gfm_mux) & clk.mux_mask) as u8
}

unsafe extern "C" fn clk_gfm_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let clk = &*clk_gfm_from_hw(hw);
    let mut val = readl(clk.gfm_mux);
    if index != 0 { val |= clk.mux_mask; } else { val &= !clk.mux_mask; }
    writel(val, clk.gfm_mux);
    0
}

static clk_gfm_ops: clk_ops = clk_ops {
    get_parent: Some(clk_gfm_get_parent),
    set_parent: Some(clk_gfm_set_parent),
    determine_rate: None, // __clk_mux_determine_rate
};

macro_rules! gfm { ($reg:expr, $name:literal, $p0:literal, $p1:literal) => {
    clk_gfm { mux_reg: $reg, mux_mask: BIT_0, hw: clk_hw { init: core::ptr::null() }, priv_: core::ptr::null_mut(), gfm_mux: core::ptr::null_mut() }
}; }

static mut lpass_gfm_va_mclk: clk_gfm = gfm!(0x20000, "VA_MCLK", "LPASS_CLK_ID_TX_CORE_MCLK", "LPASS_CLK_ID_VA_CORE_MCLK");
static mut lpass_gfm_tx_npl: clk_gfm = gfm!(0x20000, "TX_NPL", "LPASS_CLK_ID_TX_CORE_NPL_MCLK", "LPASS_CLK_ID_VA_CORE_2X_MCLK");
static mut lpass_gfm_wsa_mclk: clk_gfm = gfm!(0x220d8, "WSA_MCLK", "LPASS_CLK_ID_TX_CORE_MCLK", "LPASS_CLK_ID_WSA_CORE_MCLK");
static mut lpass_gfm_wsa_npl: clk_gfm = gfm!(0x220d8, "WSA_NPL", "LPASS_CLK_ID_TX_CORE_NPL_MCLK", "LPASS_CLK_ID_WSA_CORE_NPL_MCLK");
static mut lpass_gfm_rx_mclk_mclk2: clk_gfm = gfm!(0x240d8, "RX_MCLK_MCLK2", "LPASS_CLK_ID_TX_CORE_MCLK", "LPASS_CLK_ID_RX_CORE_MCLK");
static mut lpass_gfm_rx_npl: clk_gfm = gfm!(0x240d8, "RX_NPL", "LPASS_CLK_ID_TX_CORE_NPL_MCLK", "LPASS_CLK_ID_RX_CORE_NPL_MCLK");

// The following tables retain the C driver's externally supplied clock IDs.
static mut aoncc_gfm_clks: [*mut clk_gfm; 2] = [core::ptr::addr_of_mut!(lpass_gfm_va_mclk), core::ptr::addr_of_mut!(lpass_gfm_tx_npl)];
static mut audiocc_gfm_clks: [*mut clk_gfm; 4] = [core::ptr::addr_of_mut!(lpass_gfm_wsa_npl), core::ptr::addr_of_mut!(lpass_gfm_wsa_mclk), core::ptr::addr_of_mut!(lpass_gfm_rx_npl), core::ptr::addr_of_mut!(lpass_gfm_rx_mclk_mclk2)];
static mut aoncc_data: lpass_gfm_data = lpass_gfm_data { onecell_data: core::ptr::null_mut(), gfm_clks: core::ptr::addr_of_mut!(aoncc_gfm_clks) as *mut *mut clk_gfm };
static mut audiocc_data: lpass_gfm_data = lpass_gfm_data { onecell_data: core::ptr::null_mut(), gfm_clks: core::ptr::addr_of_mut!(audiocc_gfm_clks) as *mut *mut clk_gfm };

unsafe extern "C" fn lpass_gfm_clk_driver_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let data = of_device_get_match_data(dev);
    if data.is_null() { return -22; }
    let cc = devm_kzalloc(dev, core::mem::size_of::<lpass_gfm>(), 0) as *mut lpass_gfm;
    if cc.is_null() { return -12; }
    (*cc).base = devm_platform_ioremap_resource(pdev, 0);
    let err = devm_pm_runtime_enable(dev); if err != 0 { return err; }
    let err = devm_pm_clk_create(dev); if err != 0 { return err; }
    let err = of_pm_clk_add_clks(dev); if err < 0 { return err; }
    for i in 0..(*data).onecell_data.as_ref().unwrap().num {
        let gfm = *(*data).gfm_clks.add(i as usize);
        if gfm.is_null() { continue; }
        (*gfm).priv_ = cc;
        (*gfm).gfm_mux = (*cc).base.add((*gfm).mux_reg as usize);
        let err = devm_clk_hw_register(dev, &mut (*gfm).hw); if err != 0 { return err; }
    }
    devm_of_clk_add_hw_provider(dev, core::mem::transmute(0usize), (*data).onecell_data)
}

#[repr(C)]
struct of_device_id {
    compatible: *const u8,
    data: *const lpass_gfm_data,
}

static lpass_gfm_clk_match_table: [of_device_id; 3] = [
    of_device_id { compatible: b"qcom,sm8250-lpass-aoncc\0".as_ptr(), data: core::ptr::addr_of!(aoncc_data) },
    of_device_id { compatible: b"qcom,sm8250-lpass-audiocc\0".as_ptr(), data: core::ptr::addr_of!(audiocc_data) },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

#[repr(C)]
struct dev_pm_ops;
#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

static lpass_gfm_pm_ops: dev_pm_ops = unsafe { core::mem::zeroed() }; // SET_RUNTIME_PM_OPS(pm_clk_suspend, pm_clk_resume, NULL)
static mut lpass_gfm_clk_driver: platform_driver = platform_driver {
    probe: Some(lpass_gfm_clk_driver_probe),
};

// MODULE_DEVICE_TABLE(of, lpass_gfm_clk_match_table);
// module_platform_driver(lpass_gfm_clk_driver);
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("QTI SM8250 LPASS Glitch Free Mux clock driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
