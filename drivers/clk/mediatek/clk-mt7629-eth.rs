// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2018 MediaTek Inc.
 * Author: Wenzhen Yu <Wenzhen Yu@mediatek.com>
 *       Ryder Lee <ryder.lee@mediatek.com>
 */

// C dependencies:
// <linux/clk-provider.h>, <linux/of.h>, <linux/platform_device.h>,
// "clk-mtk.h", "clk-gate.h", and <dt-bindings/clock/mt7629-clk.h>

macro_rules! GATE_ETH {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &eth_cg_regs, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static eth_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x30,
    clr_ofs: 0x30,
    sta_ofs: 0x30,
};

static eth_clks: [mtk_gate; 5] = [
    GATE_ETH!(CLK_ETH_FE_EN, "eth_fe_en", "eth2pll", 6),
    GATE_ETH!(CLK_ETH_GP2_EN, "eth_gp2_en", "txclk_src_pre", 7),
    GATE_ETH!(CLK_ETH_GP1_EN, "eth_gp1_en", "txclk_src_pre", 8),
    GATE_ETH!(CLK_ETH_GP0_EN, "eth_gp0_en", "txclk_src_pre", 9),
    GATE_ETH!(CLK_ETH_ESW_EN, "eth_esw_en", "eth_500m", 16),
];

static sgmii_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0xE4,
    clr_ofs: 0xE4,
    sta_ofs: 0xE4,
};

macro_rules! GATE_SGMII {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &sgmii_cg_regs, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static sgmii_clks: [[mtk_gate; 4]; 2] = [
    [
        GATE_SGMII!(CLK_SGMII_TX_EN, "sgmii_tx_en", "ssusb_tx250m", 2),
        GATE_SGMII!(CLK_SGMII_RX_EN, "sgmii_rx_en", "ssusb_eq_rx250m", 3),
        GATE_SGMII!(CLK_SGMII_CDR_REF, "sgmii_cdr_ref", "ssusb_cdr_ref", 4),
        GATE_SGMII!(CLK_SGMII_CDR_FB, "sgmii_cdr_fb", "ssusb_cdr_fb", 5),
    ],
    [
        GATE_SGMII!(CLK_SGMII_TX_EN, "sgmii_tx_en1", "ssusb_tx250m", 2),
        GATE_SGMII!(CLK_SGMII_RX_EN, "sgmii_rx_en1", "ssusb_eq_rx250m", 3),
        GATE_SGMII!(CLK_SGMII_CDR_REF, "sgmii_cdr_ref1", "ssusb_cdr_ref", 4),
        GATE_SGMII!(CLK_SGMII_CDR_FB, "sgmii_cdr_fb1", "ssusb_cdr_fb", 5),
    ],
];

static mut rst_ofs: [u16; 1] = [0x34];

static clk_rst_desc: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: rst_ofs,
    rst_bank_nr: ARRAY_SIZE!(rst_ofs),
};

unsafe fn clk_mt7629_ethsys_init(pdev: *mut platform_device) -> c_int {
    let mut clk_data: *mut clk_hw_onecell_data;
    let node: *mut device_node = (*pdev).dev.of_node;
    let mut r: c_int;

    clk_data = mtk_alloc_clk_data(CLK_ETH_NR_CLK);
    if clk_data.is_null() {
        return -ENOMEM;
    }

    mtk_clk_register_gates(&mut (*pdev).dev, node, eth_clks.as_ptr(),
                           CLK_ETH_NR_CLK, clk_data);

    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 {
        dev_err!(&(*pdev).dev,
                 "could not register clock provider: %s: %d\n",
                 (*pdev).name, r);
    }

    mtk_register_reset_controller_with_dev(&mut (*pdev).dev, &clk_rst_desc);

    r
}

unsafe fn clk_mt7629_sgmiisys_init(pdev: *mut platform_device) -> c_int {
    let mut clk_data: *mut clk_hw_onecell_data;
    let node: *mut device_node = (*pdev).dev.of_node;
    static mut id: c_int = 0;
    let mut r: c_int;

    clk_data = mtk_alloc_clk_data(CLK_SGMII_NR_CLK);
    if clk_data.is_null() {
        return -ENOMEM;
    }

    let index = id;
    id += 1;
    mtk_clk_register_gates(&mut (*pdev).dev, node, sgmii_clks[index as usize].as_ptr(),
                           CLK_SGMII_NR_CLK, clk_data);

    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 {
        dev_err!(&(*pdev).dev,
                 "could not register clock provider: %s: %d\n",
                 (*pdev).name, r);
    }

    r
}

static of_match_clk_mt7629_eth: [of_device_id; 3] = [
    of_device_id {
        compatible: "mediatek,mt7629-ethsys",
        data: clk_mt7629_ethsys_init,
    },
    of_device_id {
        compatible: "mediatek,mt7629-sgmiisys",
        data: clk_mt7629_sgmiisys_init,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt7629_eth);

unsafe fn clk_mt7629_eth_probe(pdev: *mut platform_device) -> c_int {
    let mut clk_init: Option<unsafe fn(*mut platform_device) -> c_int>;
    let mut r: c_int;

    clk_init = of_device_get_match_data(&(*pdev).dev);
    if clk_init.is_none() {
        return -EINVAL;
    }

    r = clk_init.unwrap()(pdev);
    if r != 0 {
        dev_err!(&(*pdev).dev,
                 "could not register clock provider: %s: %d\n",
                 (*pdev).name, r);
    }

    r
}

static mut clk_mt7629_eth_drv: platform_driver = platform_driver {
    probe: clk_mt7629_eth_probe,
    driver: driver {
        name: "clk-mt7629-eth",
        of_match_table: of_match_clk_mt7629_eth.as_ptr(),
    },
};

builtin_platform_driver!(clk_mt7629_eth_drv);

MODULE_DESCRIPTION!("MediaTek MT7629 Ethernet clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
