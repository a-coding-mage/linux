// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// External Linux kernel, device-tree, clock, and MediaTek declarations are
// supplied by the surrounding translation unit.

const REGOFF_REF2USB: usize = 0x8;
const REGOFF_HDMI_REF: usize = 0x40;
const MT8173_PLL_FMAX: u64 = 3000u64 * MHZ;
const CON0_MT8173_RST_BAR: u32 = BIT(24);

macro_rules! pll_b {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr,
     $flags:expr, $pcwbits:expr, $pd_reg:expr, $pd_shift:expr,
     $tuner_reg:expr, $pcw_reg:expr, $pcw_shift:expr, $div_table:expr) => {
        MtkPllData {
            id: $id, name: $name, reg: $reg, pwr_reg: $pwr_reg,
            en_mask: $en_mask, flags: $flags,
            rst_bar_mask: CON0_MT8173_RST_BAR, fmax: MT8173_PLL_FMAX,
            pcwbits: $pcwbits, pd_reg: $pd_reg, pd_shift: $pd_shift,
            tuner_reg: $tuner_reg, pcw_reg: $pcw_reg,
            pcw_shift: $pcw_shift, div_table: $div_table,
        }
    };
}

macro_rules! pll {
    ($($args:expr),*) => { pll_b!($($args),*, core::ptr::null()) };
}

static MmpllDivTable: [MtkPllDivTable; 6] = [
    MtkPllDivTable { div: 0, freq: MT8173_PLL_FMAX },
    MtkPllDivTable { div: 1, freq: 1000000000 },
    MtkPllDivTable { div: 2, freq: 702000000 },
    MtkPllDivTable { div: 3, freq: 253500000 },
    MtkPllDivTable { div: 4, freq: 126750000 },
    MtkPllDivTable { div: 0, freq: 0 }, // sentinel
];

static Plls: [MtkPllData; 14] = [
    pll!(CLK_APMIXED_ARMCA15PLL, "armca15pll", 0x200, 0x20c, 0, PLL_AO, 21, 0x204, 24, 0x0, 0x204, 0),
    pll!(CLK_APMIXED_ARMCA7PLL, "armca7pll", 0x210, 0x21c, 0, PLL_AO, 21, 0x214, 24, 0x0, 0x214, 0),
    pll!(CLK_APMIXED_MAINPLL, "mainpll", 0x220, 0x22c, 0xf0000100, HAVE_RST_BAR, 21, 0x220, 4, 0x0, 0x224, 0),
    pll!(CLK_APMIXED_UNIVPLL, "univpll", 0x230, 0x23c, 0xfe000000, HAVE_RST_BAR, 7, 0x230, 4, 0x0, 0x234, 14),
    pll_b!(CLK_APMIXED_MMPLL, "mmpll", 0x240, 0x24c, 0, 0, 21, 0x244, 24, 0x0, 0x244, 0, MmpllDivTable.as_ptr()),
    pll!(CLK_APMIXED_MSDCPLL, "msdcpll", 0x250, 0x25c, 0, 0, 21, 0x250, 4, 0x0, 0x254, 0),
    pll!(CLK_APMIXED_VENCPLL, "vencpll", 0x260, 0x26c, 0, 0, 21, 0x260, 4, 0x0, 0x264, 0),
    pll!(CLK_APMIXED_TVDPLL, "tvdpll", 0x270, 0x27c, 0, 0, 21, 0x270, 4, 0x0, 0x274, 0),
    pll!(CLK_APMIXED_MPLL, "mpll", 0x280, 0x28c, 0, 0, 21, 0x280, 4, 0x0, 0x284, 0),
    pll!(CLK_APMIXED_VCODECPLL, "vcodecpll", 0x290, 0x29c, 0, 0, 21, 0x290, 4, 0x0, 0x294, 0),
    pll!(CLK_APMIXED_APLL1, "apll1", 0x2a0, 0x2b0, 0, 0, 31, 0x2a0, 4, 0x2a4, 0x2a4, 0),
    pll!(CLK_APMIXED_APLL2, "apll2", 0x2b4, 0x2c4, 0, 0, 31, 0x2b4, 4, 0x2b8, 0x2b8, 0),
    pll!(CLK_APMIXED_LVDSPLL, "lvdspll", 0x2d0, 0x2dc, 0, 0, 21, 0x2d0, 4, 0x0, 0x2d4, 0),
    pll!(CLK_APMIXED_MSDCPLL2, "msdcpll2", 0x2f0, 0x2fc, 0, 0, 21, 0x2f0, 4, 0x0, 0x2f4, 0),
];

#[repr(C)]
enum FhPllId {
    FhArmca7pll, FhArmca15pll, FhMainpll, FhMpll, FhMsdcpll,
    FhMmpll, FhVencpll, FhTvdpll, FhVcodecpll, FhLvdspll,
    FhMsdc2pll, FhNrFh,
}

macro_rules! fh {
    ($pll_id:expr, $fh_id:expr, $offset:expr) => {
        MtkPllFhData { data: MtkPllFhInner {
            pll_id: $pll_id, fh_id: $fh_id, fh_ver: FHCTL_PLLFH_V1,
            fhx_offset: $offset, dds_mask: GENMASK(21, 0),
            slope0_value: 0x6003c97, slope1_value: 0x6003c97,
            sfstrx_en: BIT(2), frddsx_en: BIT(1), fhctlx_en: BIT(0),
            tgl_org: BIT(31), dvfs_tri: BIT(31), pcwchg: BIT(31),
            dt_val: 0x0, df_val: 0x9, updnlmt_shft: 16,
            msk_frddsx_dys: GENMASK(23, 20), msk_frddsx_dts: GENMASK(19, 16),
        }}
    };
}

static mut Pllfhs: [MtkPllFhData; 11] = [
    fh!(CLK_APMIXED_ARMCA7PLL, FhPllId::FhArmca7pll, 0x38),
    fh!(CLK_APMIXED_ARMCA15PLL, FhPllId::FhArmca15pll, 0x4c),
    fh!(CLK_APMIXED_MAINPLL, FhPllId::FhMainpll, 0x60),
    fh!(CLK_APMIXED_MPLL, FhPllId::FhMpll, 0x74),
    fh!(CLK_APMIXED_MSDCPLL, FhPllId::FhMsdcpll, 0x88),
    fh!(CLK_APMIXED_MMPLL, FhPllId::FhMmpll, 0x9c),
    fh!(CLK_APMIXED_VENCPLL, FhPllId::FhVencpll, 0xb0),
    fh!(CLK_APMIXED_TVDPLL, FhPllId::FhTvdpll, 0xc4),
    fh!(CLK_APMIXED_VCODECPLL, FhPllId::FhVcodecpll, 0xd8),
    fh!(CLK_APMIXED_LVDSPLL, FhPllId::FhLvdspll, 0xec),
    fh!(CLK_APMIXED_MSDCPLL2, FhPllId::FhMsdc2pll, 0x100),
];

static OF_MATCH_CLK_MT8173_APMIXED: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "mediatek,mt8173-apmixedsys" },
    OfDeviceId::sentinel(),
];

unsafe fn clk_mt8173_apmixed_probe(pdev: *mut PlatformDevice) -> i32 {
    let fhctl_node: *const u8 = b"mediatek,mt8173-fhctl\0".as_ptr();
    let mut clk_data: *mut ClkHwOnecellData;
    let dev: *mut Device = &mut (*pdev).dev;
    let mut base: *mut core::ffi::c_void;
    let mut hw: *mut ClkHw;
    let mut r: i32;

    base = of_iomap((*dev).of_node, 0);
    if base.is_null() { return -ENOMEM; }

    clk_data = mtk_alloc_clk_data(CLK_APMIXED_NR_CLK);
    if IS_ERR_OR_NULL(clk_data) {
        r = -ENOMEM;
        goto_unmap_io!();
    }

    fhctl_parse_dt(fhctl_node, Pllfhs.as_mut_ptr(), ARRAY_SIZE(Pllfhs));
    r = mtk_clk_register_pllfhs(dev, Plls.as_ptr(), ARRAY_SIZE(Plls), Pllfhs.as_mut_ptr(), ARRAY_SIZE(Pllfhs), clk_data);
    if r != 0 { goto_free_clk_data!(); }

    hw = mtk_clk_register_ref2usb_tx(b"ref2usb_tx\0".as_ptr(), b"clk26m\0".as_ptr(), base.add(REGOFF_REF2USB));
    if IS_ERR(hw) {
        r = PTR_ERR(hw);
        dev_err(dev, b"Failed to register ref2usb_tx: %d\n\0".as_ptr(), r);
        goto_unregister_plls!();
    }
    (*clk_data).hws[CLK_APMIXED_REF2USB_TX] = hw;

    hw = devm_clk_hw_register_divider(dev, b"hdmi_ref\0".as_ptr(), b"tvdpll_594m\0".as_ptr(), 0, base.add(REGOFF_HDMI_REF), 16, 3, CLK_DIVIDER_POWER_OF_TWO, core::ptr::null());
    (*clk_data).hws[CLK_APMIXED_HDMI_REF] = hw;
    r = of_clk_add_hw_provider((*dev).of_node, of_clk_hw_onecell_get, clk_data);
    if r != 0 { mtk_clk_unregister_ref2usb_tx((*clk_data).hws[CLK_APMIXED_REF2USB_TX]); mtk_clk_unregister_pllfhs(Plls.as_ptr(), ARRAY_SIZE(Plls), Pllfhs.as_mut_ptr(), ARRAY_SIZE(Pllfhs), clk_data); mtk_free_clk_data(clk_data); iounmap(base); return r; }
    0
}

unsafe fn clk_mt8173_apmixed_remove(pdev: *mut PlatformDevice) {
    let node = (*pdev).dev.of_node;
    let clk_data = platform_get_drvdata(pdev);
    of_clk_del_provider(node);
    mtk_clk_unregister_ref2usb_tx((*clk_data).hws[CLK_APMIXED_REF2USB_TX]);
    mtk_clk_unregister_pllfhs(Plls.as_ptr(), ARRAY_SIZE(Plls), Pllfhs.as_mut_ptr(), ARRAY_SIZE(Pllfhs), clk_data);
    mtk_free_clk_data(clk_data);
}

static mut CLK_MT8173_APMIXED_DRV: PlatformDriver = PlatformDriver {
    probe: clk_mt8173_apmixed_probe,
    remove: clk_mt8173_apmixed_remove,
    driver: Driver { name: "clk-mt8173-apmixed", of_match_table: &OF_MATCH_CLK_MT8173_APMIXED },
};

module_platform_driver!(CLK_MT8173_APMIXED_DRV);
// MODULE_DESCRIPTION("MediaTek MT8173 apmixed clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
