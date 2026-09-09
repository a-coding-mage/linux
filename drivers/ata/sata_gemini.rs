// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cortina Systems Gemini SATA bridge add-on to Faraday FTIDE010
 * Copyright (C) 2017 Linus Walleij <linus.walleij@linaro.org>
 */

// Linux dependencies and "sata_gemini.h" are supplied by the surrounding kernel bindings.

const DRV_NAME: &str = "gemini_sata_bridge";

/*
 * struct sata_gemini - a state container for a Gemini SATA bridge
 * @dev: the containing device
 * @base: remapped I/O memory base
 * @muxmode: the current muxing mode
 * @ide_pins: if the device is using the plain IDE interface pins
 * @sata_bridge: if the device enables the SATA bridge
 * @sata0_pclk: SATA0 PCLK handler
 * @sata1_pclk: SATA1 PCLK handler
 */
#[repr(C)]
pub struct sata_gemini {
    pub dev: *mut device,
    pub base: *mut core::ffi::c_void,
    pub muxmode: gemini_muxmode,
    pub ide_pins: bool,
    pub sata_bridge: bool,
    pub sata0_pclk: *mut clk,
    pub sata1_pclk: *mut clk,
}

const GEMINI_GLOBAL_MISC_CTRL: u32 = 0x30;
const GEMINI_IDE_IOMUX_MASK: u32 = 7 << 24;
const GEMINI_IDE_IOMUX_MODE0: u32 = 0 << 24;
const GEMINI_IDE_IOMUX_MODE1: u32 = 1 << 24;
const GEMINI_IDE_IOMUX_MODE2: u32 = 2 << 24;
const GEMINI_IDE_IOMUX_MODE3: u32 = 3 << 24;
const GEMINI_IDE_IOMUX_SHIFT: u32 = 24;

const GEMINI_SATA_ID: u32 = 0x00;
const GEMINI_SATA_PHY_ID: u32 = 0x04;
const GEMINI_SATA0_STATUS: u32 = 0x08;
const GEMINI_SATA1_STATUS: u32 = 0x0c;
const GEMINI_SATA0_CTRL: u32 = 0x18;
const GEMINI_SATA1_CTRL: u32 = 0x1c;

const GEMINI_SATA_STATUS_BIST_DONE: u32 = 1 << 5;
const GEMINI_SATA_STATUS_BIST_OK: u32 = 1 << 4;
const GEMINI_SATA_STATUS_PHY_READY: u32 = 1 << 0;
const GEMINI_SATA_CTRL_PHY_BIST_EN: u32 = 1 << 14;
const GEMINI_SATA_CTRL_PHY_FORCE_IDLE: u32 = 1 << 13;
const GEMINI_SATA_CTRL_PHY_FORCE_READY: u32 = 1 << 12;
const GEMINI_SATA_CTRL_PHY_AFE_LOOP_EN: u32 = 1 << 10;
const GEMINI_SATA_CTRL_PHY_DIG_LOOP_EN: u32 = 1 << 9;
const GEMINI_SATA_CTRL_HOTPLUG_DETECT_EN: u32 = 1 << 4;
const GEMINI_SATA_CTRL_ATAPI_EN: u32 = 1 << 3;
const GEMINI_SATA_CTRL_BUS_WITH_20: u32 = 1 << 2;
const GEMINI_SATA_CTRL_SLAVE_EN: u32 = 1 << 1;
const GEMINI_SATA_CTRL_EN: u32 = 1 << 0;

static mut sg_singleton: *mut sata_gemini = core::ptr::null_mut();

pub unsafe fn gemini_sata_bridge_get() -> *mut sata_gemini {
    if !sg_singleton.is_null() { sg_singleton } else { ERR_PTR(-EPROBE_DEFER) }
}

pub unsafe fn gemini_sata_bridge_enabled(sg: *mut sata_gemini, is_ata1: bool) -> bool {
    if !(*sg).sata_bridge { return false; }
    if (*sg).muxmode == GEMINI_MUXMODE_2 && !is_ata1 { return false; }
    if (*sg).muxmode == GEMINI_MUXMODE_3 && is_ata1 { return false; }
    true
}

pub unsafe fn gemini_sata_get_muxmode(sg: *mut sata_gemini) -> gemini_muxmode { (*sg).muxmode }

unsafe fn gemini_sata_setup_bridge(sg: *mut sata_gemini, bridge: u32) -> i32 {
    let timeout = jiffies().wrapping_add(HZ * 1);
    let mut val: u32;
    let ctrl = GEMINI_SATA_CTRL_HOTPLUG_DETECT_EN | GEMINI_SATA_CTRL_EN
        | if (bridge == 0 && (*sg).muxmode == GEMINI_MUXMODE_2) || (bridge != 0 && (*sg).muxmode == GEMINI_MUXMODE_3) { GEMINI_SATA_CTRL_SLAVE_EN } else { 0 };
    writel(ctrl, (*sg).base.add(if bridge == 0 { GEMINI_SATA0_CTRL } else { GEMINI_SATA1_CTRL } as usize));
    msleep(10);
    loop {
        msleep(100);
        val = readl((*sg).base.add(if bridge == 0 { GEMINI_SATA0_STATUS } else { GEMINI_SATA1_STATUS } as usize));
        if val & GEMINI_SATA_STATUS_PHY_READY != 0 || !time_before(jiffies(), timeout) { break; }
    }
    dev_info((*sg).dev, "SATA%d PHY %s\n", bridge, if val & GEMINI_SATA_STATUS_PHY_READY != 0 { "ready" } else { "not ready" });
    if val & GEMINI_SATA_STATUS_PHY_READY != 0 { 0 } else { -ENODEV }
}

pub unsafe fn gemini_sata_start_bridge(sg: *mut sata_gemini, bridge: u32) -> i32 {
    let pclk = if bridge == 0 { (*sg).sata0_pclk } else { (*sg).sata1_pclk };
    let ret = clk_enable(pclk); if ret != 0 { return ret; }
    msleep(10);
    let ret = gemini_sata_setup_bridge(sg, bridge);
    if ret != 0 { clk_disable(pclk); }
    ret
}

pub unsafe fn gemini_sata_stop_bridge(sg: *mut sata_gemini, bridge: u32) {
    if bridge == 0 { clk_disable((*sg).sata0_pclk); } else if bridge == 1 { clk_disable((*sg).sata1_pclk); }
}

unsafe fn gemini_sata_bridge_init(sg: *mut sata_gemini) -> i32 {
    (*sg).sata0_pclk = devm_clk_get((*sg).dev, "SATA0_PCLK");
    if IS_ERR((*sg).sata0_pclk) { dev_err((*sg).dev, "no SATA0 PCLK"); return -ENODEV; }
    (*sg).sata1_pclk = devm_clk_get((*sg).dev, "SATA1_PCLK");
    if IS_ERR((*sg).sata1_pclk) { dev_err((*sg).dev, "no SATA1 PCLK"); return -ENODEV; }
    let mut ret = clk_prepare_enable((*sg).sata0_pclk);
    if ret != 0 { dev_err((*sg).dev, "failed to enable SATA0 PCLK\n"); return ret; }
    ret = clk_prepare_enable((*sg).sata1_pclk);
    if ret != 0 { dev_err((*sg).dev, "failed to enable SATA1 PCLK\n"); clk_disable_unprepare((*sg).sata0_pclk); return ret; }
    let sata_id = readl((*sg).base.add(GEMINI_SATA_ID as usize));
    let sata_phy_id = readl((*sg).base.add(GEMINI_SATA_PHY_ID as usize));
    (*sg).sata_bridge = true;
    clk_disable((*sg).sata0_pclk); clk_disable((*sg).sata1_pclk);
    dev_info((*sg).dev, "SATA ID %08x, PHY ID: %08x\n", sata_id, sata_phy_id);
    0
}

unsafe fn gemini_setup_ide_pins(dev: *mut device) -> i32 {
    let p = devm_pinctrl_get(dev); if IS_ERR(p) { return PTR_ERR(p); }
    let state = pinctrl_lookup_state(p, "ide"); if IS_ERR(state) { return PTR_ERR(state); }
    let ret = pinctrl_select_state(p, state);
    if ret != 0 { dev_err(dev, "could not select IDE state\n"); }
    ret
}

unsafe fn gemini_sata_probe(pdev: *mut platform_device) -> i32 {
    let dev = platform_device_dev(pdev);
    let np = (*dev).of_node;
    let sg = devm_kzalloc(dev, core::mem::size_of::<sata_gemini>(), GFP_KERNEL) as *mut sata_gemini;
    if sg.is_null() { return -ENOMEM; }
    (*sg).dev = dev;
    (*sg).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*sg).base) { return PTR_ERR((*sg).base); }
    let map = syscon_regmap_lookup_by_phandle(np, "syscon");
    if IS_ERR(map) { dev_err(dev, "no global syscon\n"); return PTR_ERR(map); }
    if of_property_read_bool(np, "cortina,gemini-enable-sata-bridge") { let ret = gemini_sata_bridge_init(sg); if ret != 0 { return ret; } }
    if of_property_read_bool(np, "cortina,gemini-enable-ide-pins") { (*sg).ide_pins = true; }
    if !(*sg).sata_bridge && !(*sg).ide_pins { dev_err(dev, "neither SATA bridge or IDE output enabled\n"); return -EINVAL; }
    let mut muxmode: gemini_muxmode = core::mem::zeroed();
    let mut ret = of_property_read_u32(np, "cortina,gemini-ata-muxmode", &mut muxmode as *mut _ as *mut u32);
    if ret != 0 { dev_err(dev, "could not parse ATA muxmode\n"); return ret; }
    if muxmode > GEMINI_MUXMODE_3 { dev_err(dev, "illegal muxmode %d\n", muxmode); return -EINVAL; }
    (*sg).muxmode = muxmode;
    ret = regmap_update_bits(map, GEMINI_GLOBAL_MISC_CTRL, GEMINI_IDE_IOMUX_MASK, (muxmode as u32) << GEMINI_IDE_IOMUX_SHIFT);
    if ret != 0 { dev_err(dev, "unable to set up IDE muxing\n"); return -ENODEV; }
    if (*sg).ide_pins { ret = gemini_setup_ide_pins(dev); if ret != 0 { return ret; } }
    dev_info(dev, "set up the Gemini IDE/SATA nexus\n"); platform_set_drvdata(pdev, sg); sg_singleton = sg; 0
}

unsafe fn gemini_sata_remove(pdev: *mut platform_device) {
    let sg = platform_get_drvdata(pdev) as *mut sata_gemini;
    if (*sg).sata_bridge { clk_unprepare((*sg).sata1_pclk); clk_unprepare((*sg).sata0_pclk); }
    sg_singleton = core::ptr::null_mut();
}

// Device-tree match table, platform_driver initialization, module_platform_driver,
// MODULE_DEVICE_TABLE, MODULE_DESCRIPTION, MODULE_AUTHOR, MODULE_LICENSE, and MODULE_ALIAS
// are preserved as kernel registration metadata supplied by the surrounding bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
