// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023 Daniel Golle <daniel@makrotopia.org>
 */

// Linux clock-provider, device-tree, platform-device, MediaTek clock, gate,
// and generated clock-binding dependencies are supplied by other files.

/* Register to control USXGMII XFI PLL analog */
const XFI_PLL_ANA_GLB8: usize = 0x108;
const RG_XFI_PLL_ANA_SWWA: u32 = 0x0228_3248;

static XFIPLL_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x8,
    clr_ofs: 0x8,
    sta_ofs: 0x8,
};

static XFIPLL_DIVS: [MtkFixedFactor; 1] = [
    MtkFixedFactor {
        id: CLK_XFIPLL_PLL,
        name: "xfipll_pll",
        parent_name: "top_xtal",
        mult: 125,
        div: 32,
    },
];

static XFIPLL_CLKS: [MtkGate; 1] = [
    MtkGate {
        id: CLK_XFIPLL_PLL_EN,
        name: "xfipll_pll_en",
        parent_name: "xfipll_pll",
        regs: &XFIPLL_CG_REGS,
        shift: 31,
        ops: &MTK_CLK_GATE_OPS_NO_SETCLR_INV,
    },
];

static XFIPLL_DESC: MtkClkDesc = MtkClkDesc {
    clks: XFIPLL_CLKS.as_ptr(),
    num_clks: XFIPLL_CLKS.len(),
    factor_clks: XFIPLL_DIVS.as_ptr(),
    num_factor_clks: XFIPLL_DIVS.len(),
};

unsafe fn clk_mt7988_xfipll_probe(pdev: *mut PlatformDevice) -> i32 {
    let node = (*pdev).dev.of_node;
    let base = of_iomap(node, 0);

    if base.is_null() {
        return -ENOMEM;
    }

    /* Apply software workaround for USXGMII PLL TCL issue */
    writel(RG_XFI_PLL_ANA_SWWA, base.add(XFI_PLL_ANA_GLB8));
    iounmap(base);

    mtk_clk_simple_probe(pdev)
}

static OF_MATCH_CLK_MT7988_XFIPLL: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: "mediatek,mt7988-xfi-pll",
        data: &XFIPLL_DESC,
    },
    OfDeviceId::SENTINEL,
];

static mut CLK_MT7988_XFIPLL_DRV: PlatformDriver = PlatformDriver {
    driver: DeviceDriver {
        name: "clk-mt7988-xfipll",
        of_match_table: &OF_MATCH_CLK_MT7988_XFIPLL,
    },
    probe: Some(clk_mt7988_xfipll_probe),
    remove: Some(mtk_clk_simple_remove),
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt7988_xfipll);
// module_platform_driver(clk_mt7988_xfipll_drv);

// MODULE_DESCRIPTION("MediaTek MT7988 XFI PLL clock driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
