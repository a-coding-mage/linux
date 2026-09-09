// SPDX-License-Identifier: GPL-2.0
/*
 * StarFive JH7110 Video-Output Clock Driver
 *
 * Copyright (C) 2022-2023 StarFive Technology Co., Ltd.
 */

// Dependencies supplied by the Linux clock, platform, runtime-PM, reset,
// device-tree bindings, and StarFive clock-driver interfaces.

const JH7110_VOUTCLK_VOUT_SRC: u32 = JH7110_VOUTCLK_END + 0;
const JH7110_VOUTCLK_VOUT_TOP_AHB: u32 = JH7110_VOUTCLK_END + 1;
const JH7110_VOUTCLK_VOUT_TOP_AXI: u32 = JH7110_VOUTCLK_END + 2;
const JH7110_VOUTCLK_VOUT_TOP_HDMITX0_MCLK: u32 = JH7110_VOUTCLK_END + 3;
const JH7110_VOUTCLK_I2STX0_BCLK: u32 = JH7110_VOUTCLK_END + 4;
const JH7110_VOUTCLK_HDMITX0_PIXELCLK: u32 = JH7110_VOUTCLK_END + 5;
const JH7110_VOUTCLK_EXT_END: u32 = JH7110_VOUTCLK_END + 6;

static mut jh7110_vout_top_clks: [clk_bulk_data; 2] = [
    clk_bulk_data { id: "vout_src" },
    clk_bulk_data { id: "vout_top_ahb" },
];

static jh7110_voutclk_data: [jh71x0_clk_data; 17] = [
    JH71X0__DIV!(JH7110_VOUTCLK_APB, "apb", 8, JH7110_VOUTCLK_VOUT_TOP_AHB),
    JH71X0__DIV!(JH7110_VOUTCLK_DC8200_PIX, "dc8200_pix", 63, JH7110_VOUTCLK_VOUT_SRC),
    JH71X0__DIV!(JH7110_VOUTCLK_DSI_SYS, "dsi_sys", 31, JH7110_VOUTCLK_VOUT_SRC),
    JH71X0__DIV!(JH7110_VOUTCLK_TX_ESC, "tx_esc", 31, JH7110_VOUTCLK_VOUT_TOP_AHB),
    JH71X0_GATE!(JH7110_VOUTCLK_DC8200_AXI, "dc8200_axi", 0, JH7110_VOUTCLK_VOUT_TOP_AXI),
    JH71X0_GATE!(JH7110_VOUTCLK_DC8200_CORE, "dc8200_core", 0, JH7110_VOUTCLK_VOUT_TOP_AXI),
    JH71X0_GATE!(JH7110_VOUTCLK_DC8200_AHB, "dc8200_ahb", 0, JH7110_VOUTCLK_VOUT_TOP_AHB),
    JH71X0_GMUX!(JH7110_VOUTCLK_DC8200_PIX0, "dc8200_pix0", 0, 2, JH7110_VOUTCLK_DC8200_PIX, JH7110_VOUTCLK_HDMITX0_PIXELCLK),
    JH71X0_GMUX!(JH7110_VOUTCLK_DC8200_PIX1, "dc8200_pix1", 0, 2, JH7110_VOUTCLK_DC8200_PIX, JH7110_VOUTCLK_HDMITX0_PIXELCLK),
    JH71X0_GMUX!(JH7110_VOUTCLK_DOM_VOUT_TOP_LCD, "dom_vout_top_lcd", 0, 2, JH7110_VOUTCLK_DC8200_PIX0, JH7110_VOUTCLK_DC8200_PIX1),
    JH71X0_GATE!(JH7110_VOUTCLK_DSITX_APB, "dsiTx_apb", 0, JH7110_VOUTCLK_DSI_SYS),
    JH71X0_GATE!(JH7110_VOUTCLK_DSITX_SYS, "dsiTx_sys", 0, JH7110_VOUTCLK_DSI_SYS),
    JH71X0_GMUX!(JH7110_VOUTCLK_DSITX_DPI, "dsiTx_dpi", 0, 2, JH7110_VOUTCLK_DC8200_PIX, JH7110_VOUTCLK_HDMITX0_PIXELCLK),
    JH71X0_GATE!(JH7110_VOUTCLK_DSITX_TXESC, "dsiTx_txesc", 0, JH7110_VOUTCLK_TX_ESC),
    JH71X0_GATE!(JH7110_VOUTCLK_MIPITX_DPHY_TXESC, "mipitx_dphy_txesc", 0, JH7110_VOUTCLK_TX_ESC),
    JH71X0_GATE!(JH7110_VOUTCLK_HDMI_TX_MCLK, "hdmi_tx_mclk", 0, JH7110_VOUTCLK_VOUT_TOP_HDMITX0_MCLK),
    JH71X0_GATE!(JH7110_VOUTCLK_HDMI_TX_BCLK, "hdmi_tx_bclk", 0, JH7110_VOUTCLK_I2STX0_BCLK),
    JH71X0_GATE!(JH7110_VOUTCLK_HDMI_TX_SYS, "hdmi_tx_sys", 0, JH7110_VOUTCLK_APB),
];

unsafe fn jh7110_vout_top_rst_init(priv_: *mut jh71x0_clk_priv) -> c_int {
    let top_rst: *mut reset_control = devm_reset_control_get_shared((*priv_).dev, core::ptr::null());
    if IS_ERR(top_rst) { return dev_err_probe((*priv_).dev, PTR_ERR(top_rst), c"failed to get top reset\n".as_ptr()); }
    reset_control_deassert(top_rst)
}

#[cfg(CONFIG_PM)]
unsafe fn jh7110_voutcrg_suspend(dev: *mut device) -> c_int {
    let top = dev_get_drvdata(dev) as *mut jh7110_top_sysclk;
    clk_bulk_disable_unprepare((*top).top_clks_num, (*top).top_clks); 0
}

#[cfg(CONFIG_PM)]
unsafe fn jh7110_voutcrg_resume(dev: *mut device) -> c_int {
    let top = dev_get_drvdata(dev) as *mut jh7110_top_sysclk;
    clk_bulk_prepare_enable((*top).top_clks_num, (*top).top_clks)
}

#[cfg(CONFIG_PM)]
static jh7110_voutcrg_pm_ops: dev_pm_ops = dev_pm_ops { runtime_suspend: Some(jh7110_voutcrg_suspend), runtime_resume: Some(jh7110_voutcrg_resume), runtime_idle: None };

unsafe fn jh7110_voutcrg_probe(pdev: *mut platform_device) -> c_int {
    let priv_: *mut jh71x0_clk_priv = devm_kzalloc(&mut (*pdev).dev, struct_size!(priv_, reg, JH7110_VOUTCLK_END), GFP_KERNEL);
    if priv_.is_null() { return -ENOMEM; }
    let top: *mut jh7110_top_sysclk = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<jh7110_top_sysclk>(), GFP_KERNEL);
    if top.is_null() { return -ENOMEM; }
    spin_lock_init(&mut (*priv_).rmw_lock); (*priv_).num_reg = JH7110_VOUTCLK_END; (*priv_).dev = &mut (*pdev).dev;
    (*priv_).base = devm_platform_ioremap_resource(pdev, 0); if IS_ERR((*priv_).base) { return PTR_ERR((*priv_).base); }
    (*top).top_clks = jh7110_vout_top_clks.as_mut_ptr(); (*top).top_clks_num = 2;
    let mut ret = devm_clk_bulk_get((*priv_).dev, (*top).top_clks_num, (*top).top_clks); if ret != 0 { return dev_err_probe((*priv_).dev, ret, c"failed to get top clocks\n".as_ptr()); }
    dev_set_drvdata((*priv_).dev, top); pm_runtime_enable((*priv_).dev); ret = pm_runtime_resume_and_get((*priv_).dev); if ret < 0 { return dev_err_probe((*priv_).dev, ret, c"failed to turn on power\n".as_ptr()); }
    ret = jh7110_vout_top_rst_init(priv_); if ret != 0 { goto err_exit; }
    for idx in 0..JH7110_VOUTCLK_END { let max = jh7110_voutclk_data[idx as usize].max; let mut parents: [clk_parent_data; 4] = core::mem::zeroed(); let init = clk_init_data { name: jh7110_voutclk_data[idx as usize].name, ops: starfive_jh71x0_clk_ops(max), parent_data: parents.as_mut_ptr(), num_parents: ((max & JH71X0_CLK_MUX_MASK) >> JH71X0_CLK_MUX_SHIFT) + 1, flags: jh7110_voutclk_data[idx as usize].flags }; let clk = &mut (*priv_).reg[idx as usize]; clk.hw.init = &init; clk.idx = idx; clk.max_div = max & JH71X0_CLK_DIV_MASK; ret = devm_clk_hw_register(&mut (*pdev).dev, &mut clk.hw); if ret != 0 { goto err_exit; } }
    ret = devm_of_clk_add_hw_provider(&mut (*pdev).dev, jh71x0_clk_get, priv_); if ret != 0 { goto err_exit; }
    ret = jh7110_reset_controller_register(priv_, c"rst-vo\0".as_ptr(), 4); if ret != 0 { goto err_exit; } return 0;
err_exit: pm_runtime_put_sync((*priv_).dev); pm_runtime_disable((*priv_).dev); ret
}

unsafe fn jh7110_voutcrg_remove(pdev: *mut platform_device) { pm_runtime_put_sync(&mut (*pdev).dev); pm_runtime_disable(&mut (*pdev).dev); }

static jh7110_voutcrg_match: [of_device_id; 2] = [of_device_id { compatible: c"starfive,jh7110-voutcrg\0".as_ptr() }, of_device_id { compatible: core::ptr::null() }];

static mut jh7110_voutcrg_driver: platform_driver = platform_driver { probe: Some(jh7110_voutcrg_probe), remove: Some(jh7110_voutcrg_remove), driver: device_driver { name: c"clk-starfive-jh7110-vout\0".as_ptr(), of_match_table: jh7110_voutcrg_match.as_ptr(), pm: pm_ptr!(&jh7110_voutcrg_pm_ops) } };

module_platform_driver!(jh7110_voutcrg_driver);
MODULE_AUTHOR!("Xingyu Wu <xingyu.wu@starfivetech.com>");
MODULE_DESCRIPTION!("StarFive JH7110 Video-Output clock driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
