// SPDX-License-Identifier: GPL-2.0
/*
 * StarFive JH7110 Image-Signal-Process Clock Driver
 *
 * Copyright (C) 2022-2023 StarFive Technology Co., Ltd.
 */

// Dependencies supplied by the Linux clock, platform, PM, reset, and
// StarFive clock-driver interfaces are intentionally external to this file.

/* external clocks */
const JH7110_ISPCLK_ISP_TOP_CORE: u32 = JH7110_ISPCLK_END + 0;
const JH7110_ISPCLK_ISP_TOP_AXI: u32 = JH7110_ISPCLK_END + 1;
const JH7110_ISPCLK_NOC_BUS_ISP_AXI: u32 = JH7110_ISPCLK_END + 2;
const JH7110_ISPCLK_DVP_CLK: u32 = JH7110_ISPCLK_END + 3;
const JH7110_ISPCLK_EXT_END: u32 = JH7110_ISPCLK_END + 4;

static mut jh7110_isp_top_clks: [clk_bulk_data; 2] = [
    clk_bulk_data { id: "isp_top_core" },
    clk_bulk_data { id: "isp_top_axi" },
];

static jh7110_ispclk_data: [jh71x0_clk_data; JH7110_ISPCLK_END as usize] = [
    JH71X0__DIV(JH7110_ISPCLK_DOM4_APB_FUNC, "dom4_apb_func", 15,
               JH7110_ISPCLK_ISP_TOP_AXI),
    JH71X0__DIV(JH7110_ISPCLK_MIPI_RX0_PXL, "mipi_rx0_pxl", 8,
               JH7110_ISPCLK_ISP_TOP_CORE),
    JH71X0__INV(JH7110_ISPCLK_DVP_INV, "dvp_inv", JH7110_ISPCLK_DVP_CLK),
    JH71X0__DIV(JH7110_ISPCLK_M31DPHY_CFG_IN, "m31dphy_cfg_in", 16,
               JH7110_ISPCLK_ISP_TOP_CORE),
    JH71X0__DIV(JH7110_ISPCLK_M31DPHY_REF_IN, "m31dphy_ref_in", 16,
               JH7110_ISPCLK_ISP_TOP_CORE),
    JH71X0__DIV(JH7110_ISPCLK_M31DPHY_TX_ESC_LAN0, "m31dphy_tx_esc_lan0", 60,
               JH7110_ISPCLK_ISP_TOP_CORE),
    JH71X0_GATE(JH7110_ISPCLK_VIN_APB, "vin_apb", 0,
               JH7110_ISPCLK_DOM4_APB_FUNC),
    JH71X0__DIV(JH7110_ISPCLK_VIN_SYS, "vin_sys", 8, JH7110_ISPCLK_ISP_TOP_CORE),
    JH7110_GATE(JH7110_ISPCLK_VIN_PIXEL_IF0, "vin_pixel_if0", 0,
               JH7110_ISPCLK_MIPI_RX0_PXL),
    JH7110_GATE(JH7110_ISPCLK_VIN_PIXEL_IF1, "vin_pixel_if1", 0,
               JH7110_ISPCLK_MIPI_RX0_PXL),
    JH7110_GATE(JH7110_ISPCLK_VIN_PIXEL_IF2, "vin_pixel_if2", 0,
               JH7110_ISPCLK_MIPI_RX0_PXL),
    JH7110_GATE(JH7110_ISPCLK_VIN_PIXEL_IF3, "vin_pixel_if3", 0,
               JH7110_ISPCLK_MIPI_RX0_PXL),
    JH71X0__MUX(JH7110_ISPCLK_VIN_P_AXI_WR, "vin_p_axi_wr", 0, 2,
               JH7110_ISPCLK_MIPI_RX0_PXL, JH7110_ISPCLK_DVP_INV),
    JH71X0_GMUX(JH7110_ISPCLK_ISPV2_TOP_WRAPPER_C, "ispv2_top_wrapper_c", 0, 2,
               JH7110_ISPCLK_MIPI_RX0_PXL, JH7110_ISPCLK_DVP_INV),
];

#[inline]
unsafe fn jh7110_isp_top_rst_init(priv_: *mut jh71x0_clk_priv) -> i32 {
    let mut top_rsts: *mut reset_control;

    // The resets should be shared and other ISP modules will use its.
    top_rsts = devm_reset_control_array_get_shared((*priv_).dev);
    if IS_ERR(top_rsts) {
        return dev_err_probe((*priv_).dev, PTR_ERR(top_rsts),
                             "failed to get top resets\n");
    }

    reset_control_deassert(top_rsts)
}

#[cfg(CONFIG_PM)]
unsafe fn jh7110_ispcrg_suspend(dev: *mut device) -> i32 {
    let top: *mut jh7110_top_sysclk = dev_get_drvdata(dev);
    clk_bulk_disable_unprepare((*top).top_clks_num, (*top).top_clks);
    0
}

#[cfg(CONFIG_PM)]
unsafe fn jh7110_ispcrg_resume(dev: *mut device) -> i32 {
    let top: *mut jh7110_top_sysclk = dev_get_drvdata(dev);
    clk_bulk_prepare_enable((*top).top_clks_num, (*top).top_clks)
}

#[cfg(CONFIG_PM)]
static jh7110_ispcrg_pm_ops: dev_pm_ops = RUNTIME_PM_OPS(
    jh7110_ispcrg_suspend, jh7110_ispcrg_resume, None);

unsafe fn jh7110_ispcrg_probe(pdev: *mut platform_device) -> i32 {
    let priv_: *mut jh71x0_clk_priv;
    let top: *mut jh7110_top_sysclk;
    let mut idx: u32;
    let mut ret: i32;

    priv_ = devm_kzalloc(&mut (*pdev).dev,
                         struct_size(priv_, reg, JH7110_ISPCLK_END), GFP_KERNEL);
    if priv_.is_null() { return -ENOMEM; }

    top = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<jh7110_top_sysclk>(), GFP_KERNEL);
    if top.is_null() { return -ENOMEM; }

    spin_lock_init(&mut (*priv_).rmw_lock);
    (*priv_).num_reg = JH7110_ISPCLK_END;
    (*priv_).dev = &mut (*pdev).dev;
    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*priv_).base) { return PTR_ERR((*priv_).base); }

    (*top).top_clks = jh7110_isp_top_clks.as_mut_ptr();
    (*top).top_clks_num = 2;
    ret = devm_clk_bulk_get((*priv_).dev, (*top).top_clks_num, (*top).top_clks);
    if ret != 0 { return dev_err_probe((*priv_).dev, ret, "failed to get main clocks\n"); }
    dev_set_drvdata((*priv_).dev, top);

    pm_runtime_enable((*priv_).dev);
    ret = pm_runtime_get_sync((*priv_).dev);
    if ret < 0 { return dev_err_probe((*priv_).dev, ret, "failed to turn on power\n"); }

    ret = jh7110_isp_top_rst_init(priv_);
    if ret != 0 { goto err_exit; }

    idx = 0;
    while idx < JH7110_ISPCLK_END {
        let max: u32 = jh7110_ispclk_data[idx as usize].max;
        let mut parents: [clk_parent_data; 4] = [core::mem::zeroed(); 4];
        let mut init = clk_init_data {
            name: jh7110_ispclk_data[idx as usize].name,
            ops: starfive_jh71x0_clk_ops(max), parent_data: parents.as_mut_ptr(),
            num_parents: ((max & JH71X0_CLK_MUX_MASK) >> JH71X0_CLK_MUX_SHIFT) + 1,
            flags: jh7110_ispclk_data[idx as usize].flags,
        };
        let clk: *mut jh71x0_clk = &mut (*priv_).reg[idx as usize];
        let fw_name: [&str; 4] = ["isp_top_core", "isp_top_axi", "noc_bus_isp_axi", "dvp_clk"];
        let mut i = 0;
        while i < init.num_parents {
            let pidx = jh7110_ispclk_data[idx as usize].parents[i as usize];
            if pidx < JH7110_ISPCLK_END {
                parents[i as usize].hw = &mut (*priv_).reg[pidx as usize].hw;
            } else {
                parents[i as usize].fw_name = fw_name[(pidx - JH7110_ISPCLK_END) as usize];
            }
            i += 1;
        }
        (*clk).hw.init = &mut init;
        (*clk).idx = idx;
        (*clk).max_div = max & JH71X0_CLK_DIV_MASK;
        ret = devm_clk_hw_register(&mut (*pdev).dev, &mut (*clk).hw);
        if ret != 0 { goto err_exit; }
        idx += 1;
    }

    ret = devm_of_clk_add_hw_provider(&mut (*pdev).dev, jh71x0_clk_get, priv_);
    if ret != 0 { goto err_exit; }
    ret = jh7110_reset_controller_register(priv_, "rst-isp", 3);
    if ret != 0 { goto err_exit; }
    return 0;

err_exit:
    pm_runtime_put_sync((*priv_).dev);
    pm_runtime_disable((*priv_).dev);
    ret
}

unsafe fn jh7110_ispcrg_remove(pdev: *mut platform_device) {
    pm_runtime_put_sync(&mut (*pdev).dev);
    pm_runtime_disable(&mut (*pdev).dev);
}

static jh7110_ispcrg_match: [of_device_id; 2] = [
    of_device_id { compatible: "starfive,jh7110-ispcrg" },
    of_device_id { compatible: core::ptr::null() },
];

static mut jh7110_ispcrg_driver: platform_driver = platform_driver {
    probe: Some(jh7110_ispcrg_probe),
    remove: Some(jh7110_ispcrg_remove),
    driver: device_driver {
        name: "clk-starfive-jh7110-isp",
        of_match_table: jh7110_ispcrg_match.as_ptr(),
        pm: pm_ptr(&jh7110_ispcrg_pm_ops),
    },
};

module_platform_driver!(jh7110_ispcrg_driver);

// MODULE_DEVICE_TABLE(of, jh7110_ispcrg_match);
// MODULE_AUTHOR("Xingyu Wu <xingyu.wu@starfivetech.com>");
// MODULE_DESCRIPTION("StarFive JH7110 Image-Signal-Process clock driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
