// SPDX-License-Identifier: GPL-2.0
/*
 * StarFive JH7110 System-Top-Group Clock Driver
 *
 * Copyright (C) 2022 Emil Renner Berthing <kernel@esmil.dk>
 * Copyright (C) 2022 StarFive Technology Co., Ltd.
 */

// External kernel and device-tree dependencies are supplied by other files.

/* external clocks */
const JH7110_STGCLK_OSC: u32 = JH7110_STGCLK_END + 0;
const JH7110_STGCLK_HIFI4_CORE: u32 = JH7110_STGCLK_END + 1;
const JH7110_STGCLK_STG_AXIAHB: u32 = JH7110_STGCLK_END + 2;
const JH7110_STGCLK_USB_125M: u32 = JH7110_STGCLK_END + 3;
const JH7110_STGCLK_CPU_BUS: u32 = JH7110_STGCLK_END + 4;
const JH7110_STGCLK_HIFI4_AXI: u32 = JH7110_STGCLK_END + 5;
const JH7110_STGCLK_NOCSTG_BUS: u32 = JH7110_STGCLK_END + 6;
const JH7110_STGCLK_APB_BUS: u32 = JH7110_STGCLK_END + 7;
const JH7110_STGCLK_EXT_END: u32 = JH7110_STGCLK_END + 8;

static JH7110_STGCLK_DATA: [jh71x0_clk_data; 31] = [
    JH71X0_GATE!(JH7110_STGCLK_HIFI4_CLK_CORE, "hifi4_clk_core", 0, JH7110_STGCLK_HIFI4_CORE),
    JH71X0_GATE!(JH7110_STGCLK_USB0_APB, "usb0_apb", 0, JH7110_STGCLK_APB_BUS),
    JH71X0_GATE!(JH7110_STGCLK_USB0_UTMI_APB, "usb0_utmi_apb", 0, JH7110_STGCLK_APB_BUS),
    JH71X0_GATE!(JH7110_STGCLK_USB0_AXI, "usb0_axi", 0, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GDIV!(JH7110_STGCLK_USB0_LPM, "usb0_lpm", 0, 2, JH7110_STGCLK_OSC),
    JH71X0_GDIV!(JH7110_STGCLK_USB0_STB, "usb0_stb", 0, 4, JH7110_STGCLK_OSC),
    JH71X0_GATE!(JH7110_STGCLK_USB0_APP_125, "usb0_app_125", 0, JH7110_STGCLK_USB_125M),
    JH71X0__DIV!(JH7110_STGCLK_USB0_REFCLK, "usb0_refclk", 2, JH7110_STGCLK_OSC),
    JH71X0_GATE!(JH7110_STGCLK_PCIE0_AXI_MST0, "pcie0_axi_mst0", 0, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_PCIE0_APB, "pcie0_apb", 0, JH7110_STGCLK_APB_BUS),
    JH71X0_GATE!(JH7110_STGCLK_PCIE0_TL, "pcie0_tl", 0, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_PCIE1_AXI_MST0, "pcie1_axi_mst0", 0, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_PCIE1_APB, "pcie1_apb", 0, JH7110_STGCLK_APB_BUS),
    JH71X0_GATE!(JH7110_STGCLK_PCIE1_TL, "pcie1_tl", 0, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_PCIE_SLV_MAIN, "pcie_slv_main", CLK_IS_CRITICAL, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_SEC_AHB, "sec_ahb", 0, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_SEC_MISC_AHB, "sec_misc_ahb", 0, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_GRP0_MAIN, "mtrx_grp0_main", CLK_IS_CRITICAL, JH7110_STGCLK_CPU_BUS),
    JH71X0_GATE!(JH7110_STGCLK_GRP0_BUS, "mtrx_grp0_bus", CLK_IS_CRITICAL, JH7110_STGCLK_NOCSTG_BUS),
    JH71X0_GATE!(JH7110_STGCLK_GRP0_STG, "mtrx_grp0_stg", CLK_IS_CRITICAL, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_GRP1_MAIN, "mtrx_grp1_main", CLK_IS_CRITICAL, JH7110_STGCLK_CPU_BUS),
    JH71X0_GATE!(JH7110_STGCLK_GRP1_BUS, "mtrx_grp1_bus", CLK_IS_CRITICAL, JH7110_STGCLK_NOCSTG_BUS),
    JH71X0_GATE!(JH7110_STGCLK_GRP1_STG, "mtrx_grp1_stg", CLK_IS_CRITICAL, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_GRP1_HIFI, "mtrx_grp1_hifi", CLK_IS_CRITICAL, JH7110_STGCLK_HIFI4_AXI),
    JH71X0_GDIV!(JH7110_STGCLK_E2_RTC, "e2_rtc", 0, 24, JH7110_STGCLK_OSC),
    JH71X0_GATE!(JH7110_STGCLK_E2_CORE, "e2_core", 0, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_E2_DBG, "e2_dbg", 0, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_DMA1P_AXI, "dma1p_axi", 0, JH7110_STGCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_STGCLK_DMA1P_AHB, "dma1p_ahb", 0, JH7110_STGCLK_STG_AXIAHB),
];

unsafe fn jh7110_stgcrg_probe(pdev: *mut platform_device) -> c_int {
    let mut priv_: *mut jh71x0_clk_priv;
    let mut idx: c_uint;
    let mut ret: c_int;

    priv_ = devm_kzalloc(&mut (*pdev).dev, struct_size!(priv_, reg, JH7110_STGCLK_END), GFP_KERNEL);
    if priv_.is_null() { return -ENOMEM; }

    spin_lock_init(&mut (*priv_).rmw_lock);
    (*priv_).num_reg = JH7110_STGCLK_END;
    (*priv_).dev = &mut (*pdev).dev;
    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*priv_).base) { return PTR_ERR((*priv_).base); }

    idx = 0;
    while idx < JH7110_STGCLK_END {
        let max = JH7110_STGCLK_DATA[idx as usize].max;
        let mut parents: [clk_parent_data; 4] = [clk_parent_data::default(); 4];
        let init = clk_init_data {
            name: JH7110_STGCLK_DATA[idx as usize].name,
            ops: starfive_jh71x0_clk_ops(max),
            parent_data: parents.as_mut_ptr(),
            num_parents: ((max & JH71X0_CLK_MUX_MASK) >> JH71X0_CLK_MUX_SHIFT) + 1,
            flags: JH7110_STGCLK_DATA[idx as usize].flags,
        };
        let clk = &mut (*priv_).reg[idx as usize];
        let fw_name: [&'static str; (JH7110_STGCLK_EXT_END - JH7110_STGCLK_END) as usize] = [
            "osc", "hifi4_core", "stg_axiahb", "usb_125m", "cpu_bus", "hifi4_axi", "nocstg_bus", "apb_bus",
        ];
        let mut i = 0;
        while i < init.num_parents {
            let pidx = JH7110_STGCLK_DATA[idx as usize].parents[i as usize];
            if pidx < JH7110_STGCLK_END {
                parents[i as usize].hw = &mut (*priv_).reg[pidx as usize].hw;
            } else if pidx < JH7110_STGCLK_EXT_END {
                parents[i as usize].fw_name = fw_name[(pidx - JH7110_STGCLK_END) as usize].as_ptr();
            }
            i += 1;
        }
        clk.hw.init = &init;
        clk.idx = idx;
        clk.max_div = max & JH71X0_CLK_DIV_MASK;
        ret = devm_clk_hw_register(&mut (*pdev).dev, &mut clk.hw);
        if ret != 0 { return ret; }
        idx += 1;
    }
    ret = devm_of_clk_add_hw_provider(&mut (*pdev).dev, jh71x0_clk_get, priv_);
    if ret != 0 { return ret; }
    jh7110_reset_controller_register(priv_, "rst-stg", 2)
}

static JH7110_STGCRG_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "starfive,jh7110-stgcrg" },
    of_device_id { /* sentinel */ },
];

static mut JH7110_STGCRG_DRIVER: platform_driver = platform_driver {
    probe: Some(jh7110_stgcrg_probe),
    driver: device_driver {
        name: "clk-starfive-jh7110-stg",
        of_match_table: JH7110_STGCRG_MATCH.as_ptr(),
    },
};

module_platform_driver!(JH7110_STGCRG_DRIVER);

MODULE_DEVICE_TABLE!(of, JH7110_STGCRG_MATCH);
MODULE_AUTHOR!("Xingyu Wu <xingyu.wu@starfivetech.com>");
MODULE_AUTHOR!("Emil Renner Berthing <kernel@esmil.dk>");
MODULE_DESCRIPTION!("StarFive JH7110 System-Top-Group clock driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
