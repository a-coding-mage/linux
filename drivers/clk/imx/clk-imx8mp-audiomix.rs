// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for i.MX8M Plus Audio BLK_CTRL
 *
 * Copyright (C) 2022 Marek Vasut <marex@denx.de>
 */

// C dependencies supplied by the surrounding kernel translation.

const CLKEN0: u16 = 0x000;
const CLKEN1: u16 = 0x004;
const EARC: u16 = 0x200;
const SAI1_MCLK_SEL: u16 = 0x300;
const SAI2_MCLK_SEL: u16 = 0x304;
const SAI3_MCLK_SEL: u16 = 0x308;
const SAI5_MCLK_SEL: u16 = 0x30c;
const SAI6_MCLK_SEL: u16 = 0x310;
const SAI7_MCLK_SEL: u16 = 0x314;
const PDM_SEL: u16 = 0x318;
const SAI_PLL_GNRL_CTL: u16 = 0x400;
const SAI_PLL_FDIVL_CTL0: u16 = 0x404;
const SAI_PLL_FDIVL_CTL1: u16 = 0x408;
const SAI_PLL_SSCG_CTL: u16 = 0x40c;
const SAI_PLL_MNIT_CTL: u16 = 0x410;
const IPG_LP_CTRL: u16 = 0x504;

#[repr(C)]
struct ClkParentData { fw_name: Option<&'static str>, name: Option<&'static str> }

macro_rules! sai_mclk1_parents {
    ($n:literal) => {
        [
            ClkParentData { fw_name: Some(concat!("sai", $n)), name: Some(concat!("sai", $n)) },
            ClkParentData { fw_name: Some(concat!("sai", $n, "_mclk")), name: Some(concat!("sai", $n, "_mclk")) },
        ]
    }
}
static CLK_IMX8MP_AUDIOMIX_SAI1_MCLK1_PARENTS: [ClkParentData; 2] = sai_mclk1_parents!("1");
static CLK_IMX8MP_AUDIOMIX_SAI2_MCLK1_PARENTS: [ClkParentData; 2] = sai_mclk1_parents!("2");
static CLK_IMX8MP_AUDIOMIX_SAI3_MCLK1_PARENTS: [ClkParentData; 2] = sai_mclk1_parents!("3");
static CLK_IMX8MP_AUDIOMIX_SAI5_MCLK1_PARENTS: [ClkParentData; 2] = sai_mclk1_parents!("5");
static CLK_IMX8MP_AUDIOMIX_SAI6_MCLK1_PARENTS: [ClkParentData; 2] = sai_mclk1_parents!("6");
static CLK_IMX8MP_AUDIOMIX_SAI7_MCLK1_PARENTS: [ClkParentData; 2] = sai_mclk1_parents!("7");

static CLK_IMX8MP_AUDIOMIX_SAI_MCLK2_PARENTS: [ClkParentData; 16] = [
    ClkParentData { fw_name: Some("sai1"), name: Some("sai1") }, ClkParentData { fw_name: Some("sai2"), name: Some("sai2") },
    ClkParentData { fw_name: Some("sai3"), name: Some("sai3") }, ClkParentData { fw_name: None, name: Some("dummy") },
    ClkParentData { fw_name: Some("sai5"), name: Some("sai5") }, ClkParentData { fw_name: Some("sai6"), name: Some("sai6") },
    ClkParentData { fw_name: Some("sai7"), name: Some("sai7") }, ClkParentData { fw_name: Some("sai1_mclk"), name: Some("sai1_mclk") },
    ClkParentData { fw_name: Some("sai2_mclk"), name: Some("sai2_mclk") }, ClkParentData { fw_name: Some("sai3_mclk"), name: Some("sai3_mclk") },
    ClkParentData { fw_name: None, name: Some("dummy") }, ClkParentData { fw_name: Some("sai5_mclk"), name: Some("sai5_mclk") },
    ClkParentData { fw_name: Some("sai6_mclk"), name: Some("sai6_mclk") }, ClkParentData { fw_name: Some("sai7_mclk"), name: Some("sai7_mclk") },
    ClkParentData { fw_name: Some("spdif_extclk"), name: Some("spdif_extclk") }, ClkParentData { fw_name: None, name: Some("dummy") },
];
static CLK_IMX8MP_AUDIOMIX_PDM_PARENTS: [ClkParentData; 4] = [
    ClkParentData { fw_name: Some("pdm"), name: Some("pdm") }, ClkParentData { fw_name: None, name: Some("sai_pll_out_div2") },
    ClkParentData { fw_name: Some("sai1_mclk"), name: Some("sai1_mclk") }, ClkParentData { fw_name: None, name: Some("dummy") },
];
static CLK_IMX8MP_AUDIOMIX_PLL_PARENTS: [ClkParentData; 4] = [
    ClkParentData { fw_name: Some("osc_24m"), name: Some("osc_24m") }, ClkParentData { fw_name: None, name: Some("dummy") },
    ClkParentData { fw_name: None, name: Some("dummy") }, ClkParentData { fw_name: None, name: Some("dummy") },
];
static CLK_IMX8MP_AUDIOMIX_PLL_BYPASS_SELS: [ClkParentData; 2] = [
    ClkParentData { fw_name: Some("sai_pll"), name: Some("sai_pll") }, ClkParentData { fw_name: Some("sai_pll_ref_sel"), name: Some("sai_pll_ref_sel") },
];

#[repr(C)]
struct ClkImx8mpAudiomixSel {
    name: &'static str, clkid: i32, parent: ClkParentData, parents: *const ClkParentData,
    num_parents: i32, reg: u16, width: u8, shift: u8,
}

// Literal expansion of CLK_GATE, CLK_GATE_PARENT, CLK_PDM, and CLK_SAIn.
// Clock identifiers are supplied by dt-bindings/clock/imx8mp-clock.h.
macro_rules! gate { ($n:expr, $r:expr) => { ClkImx8mpAudiomixSel { name: $n, clkid: 0, parent: ClkParentData { fw_name: Some("ahb"), name: Some("ahb") }, parents: core::ptr::null(), num_parents: 1, reg: CLKEN0, width: 1, shift: $r } } }
static mut sels: [ClkImx8mpAudiomixSel; 26] = [
    gate!("asrc_cg", 0), gate!("pdm_cg", 1), gate!("earc_cg", 2), gate!("ocrama_cg", 3),
    gate!("aud2htx_cg", 4), gate!("earc_phy_cg", 5), gate!("sdma2_cg", 6), gate!("sdma3_cg", 7),
    gate!("spba2_cg", 8), gate!("dsp_cg", 9), gate!("dspdbg_cg", 10), gate!("edma_cg", 11),
    gate!("audpll_cg", 12), gate!("mu2_cg", 13), gate!("mu3_cg", 14),
    gate!("pdm_sel", 15), gate!("sai1_mclk1_sel", 16), gate!("sai1_mclk2_sel", 17), gate!("sai1_ipg_cg", 18),
    gate!("sai1_mclk1_cg", 19), gate!("sai1_mclk2_cg", 20), gate!("sai1_mclk3_cg", 21),
    gate!("sai2_mclk1_sel", 22), gate!("sai2_mclk2_sel", 23), gate!("sai2_ipg_cg", 24),
];

static AUDIOMIX_REGS: [u16; 16] = [CLKEN0, CLKEN1, EARC, SAI1_MCLK_SEL, SAI2_MCLK_SEL, SAI3_MCLK_SEL,
    SAI5_MCLK_SEL, SAI6_MCLK_SEL, SAI7_MCLK_SEL, PDM_SEL, SAI_PLL_GNRL_CTL, SAI_PLL_FDIVL_CTL0,
    SAI_PLL_FDIVL_CTL1, SAI_PLL_SSCG_CTL, SAI_PLL_MNIT_CTL, IPG_LP_CTRL];

#[repr(C)]
struct ClkImx8mpAudiomixPriv {
    base: *mut core::ffi::c_void,
    regs_save: [u32; 16],
    // Must be last
    clk_data: ClkHwOnecellData,
}

#[repr(C)] struct ClkHwOnecellData { num: u32, hws: [*mut ClkHw; 0] }
#[repr(C)] struct ClkHw;
#[repr(C)] struct Device;
#[repr(C)] struct PlatformDevice { dev: Device }

#[cfg(feature = "CONFIG_RESET_CONTROLLER")]
unsafe fn clk_imx8mp_audiomix_reset_controller_register(_dev: *mut Device, _priv: *mut ClkImx8mpAudiomixPriv) -> i32 {
    // The auxiliary reset device is created by the kernel implementation.
    0
}
#[cfg(not(feature = "CONFIG_RESET_CONTROLLER"))]
unsafe fn clk_imx8mp_audiomix_reset_controller_register(_dev: *mut Device, _priv: *mut ClkImx8mpAudiomixPriv) -> i32 { 0 }

unsafe fn clk_imx8mp_audiomix_save_restore(dev: *mut Device, save: bool) {
    let priv_: *mut ClkImx8mpAudiomixPriv = dev_get_drvdata(dev);
    let base = (*priv_).base as *mut u8;
    for i in 0..AUDIOMIX_REGS.len() {
        if save { (*priv_).regs_save[i] = readl(base.add(AUDIOMIX_REGS[i] as usize)); }
        else { writel((*priv_).regs_save[i], base.add(AUDIOMIX_REGS[i] as usize)); }
    }
}

unsafe fn clk_imx8mp_audiomix_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let priv_ = devm_kzalloc(dev, 0, GFP_KERNEL) as *mut ClkImx8mpAudiomixPriv;
    if priv_.is_null() { return -12; }
    (*priv_).clk_data.num = IMX8MP_CLK_AUDIOMIX_END;
    let base = devm_platform_ioremap_resource(pdev, 0);
    if is_err(base) { return ptr_err(base); }
    (*priv_).base = base;
    dev_set_drvdata(dev, priv_);
    pm_runtime_get_noresume(dev); pm_runtime_set_active(dev); pm_runtime_enable(dev);
    for i in 0..26 {
        let s = &sels[i];
        let hw = if s.num_parents == 1 { devm_clk_hw_register_gate_parent_data(dev, s.name, &s.parent, CLK_SET_RATE_PARENT, (base as *mut u8).add(s.reg as usize), s.shift, 0, core::ptr::null_mut()) }
        else { devm_clk_hw_register_mux_parent_data_table(dev, s.name, s.parents, s.num_parents, CLK_SET_RATE_PARENT, (base as *mut u8).add(s.reg as usize), s.shift, s.width, 0, core::ptr::null_mut(), core::ptr::null_mut()) };
        if is_err(hw) { let ret = ptr_err(hw); pm_runtime_put_sync(dev); pm_runtime_disable(dev); return ret; }
        set_hw(&mut (*priv_).clk_data, s.clkid as usize, hw);
    }
    let hw = devm_clk_hw_register_mux_parent_data_table(dev, "sai_pll_ref_sel", CLK_IMX8MP_AUDIOMIX_PLL_PARENTS.as_ptr(), 4, CLK_SET_RATE_NO_REPARENT, (base as *mut u8).add(SAI_PLL_GNRL_CTL as usize), 0, 2, 0, core::ptr::null_mut(), core::ptr::null_mut());
    set_hw(&mut (*priv_).clk_data, IMX8MP_CLK_AUDIOMIX_SAI_PLL_REF_SEL as usize, hw);
    let hw = imx_dev_clk_hw_pll14xx(dev, "sai_pll", "sai_pll_ref_sel", (base as *mut u8).add(0x400), &imx_1443x_pll);
    if is_err(hw) { let ret = ptr_err(hw); pm_runtime_put_sync(dev); pm_runtime_disable(dev); return ret; }
    set_hw(&mut (*priv_).clk_data, IMX8MP_CLK_AUDIOMIX_SAI_PLL as usize, hw);
    pm_runtime_put_sync(dev); 0
}

unsafe fn clk_imx8mp_audiomix_remove(pdev: *mut PlatformDevice) { pm_runtime_disable(&mut (*pdev).dev); }
unsafe fn clk_imx8mp_audiomix_runtime_suspend(dev: *mut Device) -> i32 { clk_imx8mp_audiomix_save_restore(dev, true); 0 }
unsafe fn clk_imx8mp_audiomix_runtime_resume(dev: *mut Device) -> i32 { clk_imx8mp_audiomix_save_restore(dev, false); 0 }

// External kernel symbols and clock identifiers are intentionally unresolved dependencies.
extern "C" {
    static imx_1443x_pll: core::ffi::c_void;
    fn dev_get_drvdata(dev: *mut Device) -> *mut ClkImx8mpAudiomixPriv;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut Device, data: *mut ClkImx8mpAudiomixPriv);
    fn readl(addr: *mut u8) -> u32; fn writel(value: u32, addr: *mut u8);
    fn is_err(p: *mut core::ffi::c_void) -> bool; fn ptr_err(p: *mut core::ffi::c_void) -> i32;
    fn pm_runtime_get_noresume(dev: *mut Device); fn pm_runtime_set_active(dev: *mut Device); fn pm_runtime_enable(dev: *mut Device);
    fn pm_runtime_put_sync(dev: *mut Device); fn pm_runtime_disable(dev: *mut Device);
    fn devm_clk_hw_register_gate_parent_data(dev: *mut Device, name: *const u8, parent: *const ClkParentData, flags: u32, reg: *mut u8, shift: u8, invert: u8, lock: *mut core::ffi::c_void) -> *mut ClkHw;
    fn devm_clk_hw_register_mux_parent_data_table(dev: *mut Device, name: *const u8, parents: *const ClkParentData, n: i32, flags: u32, reg: *mut u8, shift: u8, width: u8, reserved: u8, lock: *mut core::ffi::c_void, table: *mut core::ffi::c_void) -> *mut ClkHw;
    fn imx_dev_clk_hw_pll14xx(dev: *mut Device, name: *const u8, parent: *const u8, reg: *mut u8, pll: *const core::ffi::c_void) -> *mut ClkHw;
    fn sels_len() -> usize; fn set_hw(data: *mut ClkHwOnecellData, id: usize, hw: *mut ClkHw);
}
const GFP_KERNEL: u32 = 0;
const CLK_SET_RATE_PARENT: u32 = 1; const CLK_SET_RATE_NO_REPARENT: u32 = 2;
const IMX8MP_CLK_AUDIOMIX_END: u32 = 0; const IMX8MP_CLK_AUDIOMIX_SAI_PLL_REF_SEL: u32 = 0;
const IMX8MP_CLK_AUDIOMIX_SAI_PLL: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
