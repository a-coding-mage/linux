// SPDX-License-Identifier: GPL-2.0
/*
 * StarFive JH7110 Always-On Clock Driver
 *
 * Copyright (C) 2022 Emil Renner Berthing <kernel@esmil.dk>
 * Copyright (C) 2022 StarFive Technology Co., Ltd.
 */

// Dependencies supplied by the Linux clock, I/O, platform-device, device-tree,
// and StarFive clock framework headers are intentionally external to this file.

/* external clocks */
const JH7110_AONCLK_OSC: u32 = JH7110_AONCLK_END + 0;
const JH7110_AONCLK_GMAC0_RMII_REFIN: u32 = JH7110_AONCLK_END + 1;
const JH7110_AONCLK_GMAC0_RGMII_RXIN: u32 = JH7110_AONCLK_END + 2;
const JH7110_AONCLK_STG_AXIAHB: u32 = JH7110_AONCLK_END + 3;
const JH7110_AONCLK_APB_BUS: u32 = JH7110_AONCLK_END + 4;
const JH7110_AONCLK_GMAC0_GTXCLK: u32 = JH7110_AONCLK_END + 5;
const JH7110_AONCLK_RTC_OSC: u32 = JH7110_AONCLK_END + 6;

static JH7110_AONCLK_DATA: [jh71x0_clk_data; 14] = [
    /* source */
    JH71X0__DIV!(JH7110_AONCLK_OSC_DIV4, "osc_div4", 4, JH7110_AONCLK_OSC),
    JH71X0__MUX!(JH7110_AONCLK_APB_FUNC, "apb_func", 0, 2,
        JH7110_AONCLK_OSC_DIV4, JH7110_AONCLK_OSC),
    /* gmac0 */
    JH71X0_GATE!(JH7110_AONCLK_GMAC0_AHB, "gmac0_ahb", 0, JH7110_AONCLK_STG_AXIAHB),
    JH71X0_GATE!(JH7110_AONCLK_GMAC0_AXI, "gmac0_axi", 0, JH7110_AONCLK_STG_AXIAHB),
    JH71X0__DIV!(JH7110_AONCLK_GMAC0_RMII_RTX, "gmac0_rmii_rtx", 30,
        JH7110_AONCLK_GMAC0_RMII_REFIN),
    JH71X0_GMUX!(JH7110_AONCLK_GMAC0_TX, "gmac0_tx",
        CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT, 2,
        JH7110_AONCLK_GMAC0_GTXCLK, JH7110_AONCLK_GMAC0_RMII_RTX),
    JH71X0__INV!(JH7110_AONCLK_GMAC0_TX_INV, "gmac0_tx_inv", JH7110_AONCLK_GMAC0_TX),
    JH71X0__MUX!(JH7110_AONCLK_GMAC0_RX, "gmac0_rx", 0, 2,
        JH7110_AONCLK_GMAC0_RGMII_RXIN, JH7110_AONCLK_GMAC0_RMII_RTX),
    JH71X0__INV!(JH7110_AONCLK_GMAC0_RX_INV, "gmac0_rx_inv", JH7110_AONCLK_GMAC0_RX),
    /* otpc */
    JH71X0_GATE!(JH7110_AONCLK_OTPC_APB, "otpc_apb", 0, JH7110_AONCLK_APB_BUS),
    /* rtc */
    JH71X0_GATE!(JH7110_AONCLK_RTC_APB, "rtc_apb", 0, JH7110_AONCLK_APB_BUS),
    JH71X0__DIV!(JH7110_AONCLK_RTC_INTERNAL, "rtc_internal", 1022, JH7110_AONCLK_OSC),
    JH71X0__MUX!(JH7110_AONCLK_RTC_32K, "rtc_32k", 0, 2,
        JH7110_AONCLK_RTC_OSC, JH7110_AONCLK_RTC_INTERNAL),
    JH71X0_GATE!(JH7110_AONCLK_RTC_CAL, "rtc_cal", 0, JH7110_AONCLK_OSC),
];

unsafe fn jh7110_aoncrg_probe(pdev: *mut platform_device) -> i32 {
    let mut priv_: *mut jh71x0_clk_priv;
    let mut idx: u32;
    let mut ret: i32;

    priv_ = devm_kzalloc(
        &mut (*pdev).dev,
        struct_size::<jh71x0_clk_priv>(JH7110_AONCLK_END),
        GFP_KERNEL,
    );
    if priv_.is_null() {
        return -ENOMEM;
    }

    spin_lock_init(&mut (*priv_).rmw_lock);
    (*priv_).num_reg = JH7110_AONCLK_END;
    (*priv_).dev = &mut (*pdev).dev;
    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*priv_).base) {
        return PTR_ERR((*priv_).base);
    }

    idx = 0;
    while idx < JH7110_AONCLK_END {
        let max: u32 = JH7110_AONCLK_DATA[idx as usize].max;
        let mut parents: [clk_parent_data; 4] = [clk_parent_data::default(); 4];
        let init = clk_init_data {
            name: JH7110_AONCLK_DATA[idx as usize].name,
            ops: starfive_jh71x0_clk_ops(max),
            parent_data: parents.as_mut_ptr(),
            num_parents: ((max & JH71X0_CLK_MUX_MASK) >> JH71X0_CLK_MUX_SHIFT) + 1,
            flags: JH7110_AONCLK_DATA[idx as usize].flags,
        };
        let clk: *mut jh71x0_clk = &mut (*priv_).reg[idx as usize];
        let mut i: u32 = 0;

        while i < init.num_parents {
            let pidx = JH7110_AONCLK_DATA[idx as usize].parents[i as usize];
            if pidx < JH7110_AONCLK_END {
                parents[i as usize].hw = &mut (*priv_).reg[pidx as usize].hw;
            } else if pidx == JH7110_AONCLK_OSC {
                parents[i as usize].fw_name = "osc";
            } else if pidx == JH7110_AONCLK_GMAC0_RMII_REFIN {
                parents[i as usize].fw_name = "gmac0_rmii_refin";
            } else if pidx == JH7110_AONCLK_GMAC0_RGMII_RXIN {
                parents[i as usize].fw_name = "gmac0_rgmii_rxin";
            } else if pidx == JH7110_AONCLK_STG_AXIAHB {
                parents[i as usize].fw_name = "stg_axiahb";
            } else if pidx == JH7110_AONCLK_APB_BUS {
                parents[i as usize].fw_name = "apb_bus";
            } else if pidx == JH7110_AONCLK_GMAC0_GTXCLK {
                parents[i as usize].fw_name = "gmac0_gtxclk";
            } else if pidx == JH7110_AONCLK_RTC_OSC {
                parents[i as usize].fw_name = "rtc_osc";
            }
            i += 1;
        }

        (*clk).hw.init = &init;
        (*clk).idx = idx;
        (*clk).max_div = max & JH71X0_CLK_DIV_MASK;
        ret = devm_clk_hw_register(&mut (*pdev).dev, &mut (*clk).hw);
        if ret != 0 {
            return ret;
        }
        idx += 1;
    }

    ret = devm_of_clk_add_hw_provider(&mut (*pdev).dev, jh71x0_clk_get, priv_);
    if ret != 0 {
        return ret;
    }
    jh7110_reset_controller_register(priv_, "rst-aon", 1)
}

static JH7110_AONCRG_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "starfive,jh7110-aoncrg" },
    of_device_id { /* sentinel */ },
];

static mut JH7110_AONCRG_DRIVER: platform_driver = platform_driver {
    probe: Some(jh7110_aoncrg_probe),
    driver: device_driver {
        name: "clk-starfive-jh7110-aon",
        of_match_table: JH7110_AONCRG_MATCH.as_ptr(),
    },
};

MODULE_DEVICE_TABLE!(of, JH7110_AONCRG_MATCH);
module_platform_driver!(JH7110_AONCRG_DRIVER);

MODULE_AUTHOR!("Emil Renner Berthing");
MODULE_DESCRIPTION!("StarFive JH7110 always-on clock driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
