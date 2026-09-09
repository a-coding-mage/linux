// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the Linux clock, platform, device-tree, and MediaTek clock headers.

const MT8186_PLL_FMAX: usize = 3800usize * MHZ;
const MT8186_PLL_FMIN: usize = 1500usize * MHZ;
const MT8186_INTEGER_BITS: u32 = 8;

macro_rules! pll {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr,
     $flags:expr, $rst_bar_mask:expr, $pcwbits:expr, $pd_reg:expr,
     $pd_shift:expr, $tuner_reg:expr, $tuner_en_reg:expr,
     $tuner_en_bit:expr, $pcw_reg:expr) => {
        MtkPllData {
            id: $id,
            name: $name,
            reg: $reg,
            pwr_reg: $pwr_reg,
            en_mask: $en_mask,
            flags: $flags,
            rst_bar_mask: $rst_bar_mask,
            fmax: MT8186_PLL_FMAX,
            fmin: MT8186_PLL_FMIN,
            pcwbits: $pcwbits,
            pcwibits: MT8186_INTEGER_BITS,
            pd_reg: $pd_reg,
            pd_shift: $pd_shift,
            tuner_reg: $tuner_reg,
            tuner_en_reg: $tuner_en_reg,
            tuner_en_bit: $tuner_en_bit,
            pcw_reg: $pcw_reg,
            pcw_shift: 0,
            pcw_chg_reg: 0,
            en_reg: 0,
            pll_en_bit: 0,
        }
    };
}

static PLls: [MtkPllData; 14] = [
    pll!(CLK_APMIXED_ARMPLL_LL, "armpll_ll", 0x0204, 0x0210, 0, PLL_AO, 0, 22, 0x0208, 24, 0, 0, 0, 0x0208),
    pll!(CLK_APMIXED_ARMPLL_BL, "armpll_bl", 0x0214, 0x0220, 0, PLL_AO, 0, 22, 0x0218, 24, 0, 0, 0, 0x0218),
    pll!(CLK_APMIXED_CCIPLL, "ccipll", 0x0224, 0x0230, 0, PLL_AO, 0, 22, 0x0228, 24, 0, 0, 0, 0x0228),
    pll!(CLK_APMIXED_MAINPLL, "mainpll", 0x0244, 0x0250, 0xff000000, HAVE_RST_BAR, BIT(23), 22, 0x0248, 24, 0, 0, 0, 0x0248),
    pll!(CLK_APMIXED_UNIV2PLL, "univ2pll", 0x0324, 0x0330, 0xff000000, HAVE_RST_BAR, BIT(23), 22, 0x0328, 24, 0, 0, 0, 0x0328),
    pll!(CLK_APMIXED_MSDCPLL, "msdcpll", 0x038C, 0x0398, 0, 0, 0, 22, 0x0390, 24, 0, 0, 0, 0x0390),
    pll!(CLK_APMIXED_MMPLL, "mmpll", 0x0254, 0x0260, 0, 0, 0, 22, 0x0258, 24, 0, 0, 0, 0x0258),
    pll!(CLK_APMIXED_NNAPLL, "nnapll", 0x035C, 0x0368, 0, 0, 0, 22, 0x0360, 24, 0, 0, 0, 0x0360),
    pll!(CLK_APMIXED_NNA2PLL, "nna2pll", 0x036C, 0x0378, 0, 0, 0, 22, 0x0370, 24, 0, 0, 0, 0x0370),
    pll!(CLK_APMIXED_ADSPPLL, "adsppll", 0x0304, 0x0310, 0, 0, 0, 22, 0x0308, 24, 0, 0, 0, 0x0308),
    pll!(CLK_APMIXED_MFGPLL, "mfgpll", 0x0314, 0x0320, 0, 0, 0, 22, 0x0318, 24, 0, 0, 0, 0x0318),
    pll!(CLK_APMIXED_TVDPLL, "tvdpll", 0x0264, 0x0270, 0, 0, 0, 22, 0x0268, 24, 0, 0, 0, 0x0268),
    pll!(CLK_APMIXED_APLL1, "apll1", 0x0334, 0x0344, 0, 0, 0, 32, 0x0338, 24, 0x0040, 0x000C, 0, 0x033C),
    pll!(CLK_APMIXED_APLL2, "apll2", 0x0348, 0x0358, 0, 0, 0, 32, 0x034C, 24, 0x0044, 0x000C, 5, 0x0350),
];

#[repr(C)]
enum FhPllId {
    FH_ARMPLL_LL,
    FH_ARMPLL_BL,
    FH_CCIPLL,
    FH_MAINPLL,
    FH_MMPLL,
    FH_TVDPLL,
    FH_RESERVE6,
    FH_ADSPPLL,
    FH_MFGPLL,
    FH_NNAPLL,
    FH_NNA2PLL,
    FH_MSDCPLL,
    FH_RESERVE12,
    FH_NR_FH,
}

macro_rules! fh {
    ($pll_id:expr, $fh_id:expr, $offset:expr) => {
        MtkPllfhData {
            data: MtkPllfhInner {
                pll_id: $pll_id,
                fh_id: $fh_id,
                fh_ver: FHCTL_PLLFH_V2,
                fhx_offset: $offset,
                dds_mask: GENMASK(21, 0),
                slope0_value: 0x6003c97,
                slope1_value: 0x6003c97,
                sfstrx_en: BIT(2),
                frddsx_en: BIT(1),
                fhctlx_en: BIT(0),
                tgl_org: BIT(31),
                dvfs_tri: BIT(31),
                pcwchg: BIT(31),
                dt_val: 0x0,
                df_val: 0x9,
                updnlmt_shft: 16,
                msk_frddsx_dys: GENMASK(23, 20),
                msk_frddsx_dts: GENMASK(19, 16),
            },
        }
    };
}

static mut PLLFHS: [MtkPllfhData; 11] = [
    fh!(CLK_APMIXED_ARMPLL_LL, FH_ARMPLL_LL, 0x003C),
    fh!(CLK_APMIXED_ARMPLL_BL, FH_ARMPLL_BL, 0x0050),
    fh!(CLK_APMIXED_CCIPLL, FH_CCIPLL, 0x0064),
    fh!(CLK_APMIXED_MAINPLL, FH_MAINPLL, 0x0078),
    fh!(CLK_APMIXED_MMPLL, FH_MMPLL, 0x008C),
    fh!(CLK_APMIXED_TVDPLL, FH_TVDPLL, 0x00A0),
    fh!(CLK_APMIXED_ADSPPLL, FH_ADSPPLL, 0x00C8),
    fh!(CLK_APMIXED_MFGPLL, FH_MFGPLL, 0x00DC),
    fh!(CLK_APMIXED_NNAPLL, FH_NNAPLL, 0x00F0),
    fh!(CLK_APMIXED_NNA2PLL, FH_NNA2PLL, 0x0104),
    fh!(CLK_APMIXED_MSDCPLL, FH_MSDCPLL, 0x0118),
];

static OF_MATCH_CLK_MT8186_APMIXED: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "mediatek,mt8186-apmixedsys" },
    OfDeviceId { compatible: "" },
];

unsafe fn clk_mt8186_apmixed_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut clk_data: *mut ClkHwOnecellData;
    let node: *mut DeviceNode = (*pdev).dev.of_node;
    let fhctl_node: *const u8 = b"mediatek,mt8186-fhctl\0".as_ptr();
    let mut r: i32;

    clk_data = mtk_alloc_clk_data(CLK_APMIXED_NR_CLK);
    if clk_data.is_null() {
        return -ENOMEM;
    }

    fhctl_parse_dt(fhctl_node, PLLFHS.as_mut_ptr(), PLLFHS.len());

    r = mtk_clk_register_pllfhs(&mut (*pdev).dev, PLls.as_ptr(), PLls.len(), PLLFHS.as_mut_ptr(), PLLFHS.len(), clk_data);
    if r != 0 {
        goto_free_apmixed_data!();
    }

    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 {
        goto_unregister_plls!();
    }

    platform_set_drvdata(pdev, clk_data);
    return r;

    // C labels: unregister_plls and free_apmixed_data.
    unsafe {
        mtk_clk_unregister_pllfhs(PLls.as_ptr(), PLls.len(), PLLFHS.as_mut_ptr(), PLLFHS.len(), clk_data);
        mtk_free_clk_data(clk_data);
    }
    r
}

unsafe fn clk_mt8186_apmixed_remove(pdev: *mut PlatformDevice) {
    let node: *mut DeviceNode = (*pdev).dev.of_node;
    let clk_data: *mut ClkHwOnecellData = platform_get_drvdata(pdev);

    of_clk_del_provider(node);
    mtk_clk_unregister_pllfhs(PLls.as_ptr(), PLls.len(), PLLFHS.as_mut_ptr(), PLLFHS.len(), clk_data);
    mtk_free_clk_data(clk_data);
}

static mut CLK_MT8186_APMIXED_DRV: PlatformDriver = PlatformDriver {
    probe: Some(clk_mt8186_apmixed_probe),
    remove: Some(clk_mt8186_apmixed_remove),
    driver: Driver {
        name: "clk-mt8186-apmixed",
        of_match_table: OF_MATCH_CLK_MT8186_APMIXED.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8186_APMIXED_DRV);

module_description!("MediaTek MT8186 apmixedsys clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
