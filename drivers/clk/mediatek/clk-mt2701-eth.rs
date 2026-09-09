// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: Shunli Wang <shunli.wang@mediatek.com>
 */

// Linux clock-provider, platform-device, MediaTek clock, gate, and
// MT2701 clock-binding dependencies are supplied externally.

static ETH_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    sta_ofs: 0x0030,
};

macro_rules! GATE_ETH {
    ($id:ident, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &ETH_CG_REGS, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static ETH_CLKS: [mtk_gate; 9] = [
    GATE_DUMMY!(CLK_DUMMY, "eth_dummy"),
    GATE_ETH!(CLK_ETHSYS_HSDMA, "hsdma_clk", "ethif_sel", 5),
    GATE_ETH!(CLK_ETHSYS_ESW, "esw_clk", "ethpll_500m_ck", 6),
    GATE_ETH!(CLK_ETHSYS_GP2, "gp2_clk", "trgpll", 7),
    GATE_ETH!(CLK_ETHSYS_GP1, "gp1_clk", "ethpll_500m_ck", 8),
    GATE_ETH!(CLK_ETHSYS_PCM, "pcm_clk", "ethif_sel", 11),
    GATE_ETH!(CLK_ETHSYS_GDMA, "gdma_clk", "ethif_sel", 14),
    GATE_ETH!(CLK_ETHSYS_I2S, "i2s_clk", "ethif_sel", 17),
    GATE_ETH!(CLK_ETHSYS_CRYPTO, "crypto_clk", "ethif_sel", 29),
];

static mut RST_OFS: [u16; 1] = [0x34];

static CLK_RST_DESC: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: RST_OFS.as_ptr(),
    rst_bank_nr: RST_OFS.len(),
};

static ETH_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: ETH_CLKS.as_ptr(),
    num_clks: ETH_CLKS.len(),
    rst_desc: &CLK_RST_DESC,
};

static OF_MATCH_CLK_MT2701_ETH: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt2701-ethsys\\0".as_ptr() as *const i8,
        data: &ETH_DESC as *const _ as *const core::ffi::c_void,
    },
    of_device_id {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT2701_ETH);

static mut CLK_MT2701_ETH_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: b"clk-mt2701-eth\\0".as_ptr() as *const i8,
        of_match_table: OF_MATCH_CLK_MT2701_ETH.as_ptr(),
    },
};

module_platform_driver!(CLK_MT2701_ETH_DRV);

MODULE_DESCRIPTION!("MediaTek MT2701 Ethernet clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
