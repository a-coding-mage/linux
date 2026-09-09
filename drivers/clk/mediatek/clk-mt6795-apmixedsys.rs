// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the kernel clock framework and MT6795 bindings.

const REG_REF2USB: usize = 0x8;
const REG_AP_PLL_CON7: usize = 0x1c;
const MD1_MTCMOS_OFF: u32 = 1 << 0;
const MD1_MEM_OFF: u32 = 1 << 1;
const MD1_CLK_OFF: u32 = 1 << 4;
const MD1_ISO_OFF: u32 = 1 << 8;

const MT6795_PLL_FMAX: u64 = 3000u64 * MHZ;
const MT6795_CON0_EN: u32 = 1 << 0;
const MT6795_CON0_RST_BAR: u32 = 1 << 24;

const fn pll(
    id: i32,
    name: &'static str,
    reg: usize,
    pwr_reg: usize,
    en_mask: u32,
    flags: u32,
    pcwbits: u32,
    pd_reg: usize,
    pd_shift: u32,
    tuner_reg: usize,
    pcw_reg: usize,
    pcw_shift: u32,
) -> mtk_pll_data {
    mtk_pll_data {
        id,
        name,
        reg,
        pwr_reg,
        en_mask: MT6795_CON0_EN | en_mask,
        flags,
        rst_bar_mask: MT6795_CON0_RST_BAR,
        fmax: MT6795_PLL_FMAX,
        pcwbits,
        pd_reg,
        pd_shift,
        tuner_reg,
        pcw_reg,
        pcw_shift,
        div_table: core::ptr::null(),
        pll_en_bit: 0,
    }
}

static PLlS: [mtk_pll_data; 11] = [
    pll(CLK_APMIXED_ARMCA53PLL, "armca53pll", 0x200, 0x20c, 0, PLL_AO, 21, 0x204, 24, 0x0, 0x204, 0),
    pll(CLK_APMIXED_MAINPLL, "mainpll", 0x220, 0x22c, 0xf0000101, HAVE_RST_BAR, 21, 0x220, 4, 0x0, 0x224, 0),
    pll(CLK_APMIXED_UNIVPLL, "univpll", 0x230, 0x23c, 0xfe000101, HAVE_RST_BAR, 7, 0x230, 4, 0x0, 0x234, 14),
    pll(CLK_APMIXED_MMPLL, "mmpll", 0x240, 0x24c, 0, 0, 21, 0x244, 24, 0x0, 0x244, 0),
    pll(CLK_APMIXED_MSDCPLL, "msdcpll", 0x250, 0x25c, 0, 0, 21, 0x250, 4, 0x0, 0x254, 0),
    pll(CLK_APMIXED_VENCPLL, "vencpll", 0x260, 0x26c, 0, 0, 21, 0x260, 4, 0x0, 0x264, 0),
    pll(CLK_APMIXED_TVDPLL, "tvdpll", 0x270, 0x27c, 0, 0, 21, 0x270, 4, 0x0, 0x274, 0),
    pll(CLK_APMIXED_MPLL, "mpll", 0x280, 0x28c, 0, 0, 21, 0x280, 4, 0x0, 0x284, 0),
    pll(CLK_APMIXED_VCODECPLL, "vcodecpll", 0x290, 0x29c, 0, 0, 21, 0x290, 4, 0x0, 0x294, 0),
    pll(CLK_APMIXED_APLL1, "apll1", 0x2a0, 0x2b0, 0, 0, 31, 0x2a0, 4, 0x2a8, 0x2a4, 0),
    pll(CLK_APMIXED_APLL2, "apll2", 0x2b4, 0x2c4, 0, 0, 31, 0x2b4, 4, 0x2bc, 0x2b8, 0),
];

#[repr(i32)]
enum FhPllId {
    FhCa53pllLl,
    FhCa53pllBl,
    FhMainpll,
    FhMpll,
    FhMsdcpll,
    FhMmpll,
    FhVencpll,
    FhTvdpll,
    FhVcodecpll,
    FhNrFh,
}

const fn fh(pll_id: i32, fh_id: i32, slope: u32, offset: usize) -> mtk_pllfh_data {
    mtk_pllfh_data {
        data: mtk_pllfh_data_inner {
            pll_id, fh_id, fh_ver: FHCTL_PLLFH_V1, fhx_offset: offset,
            dds_mask: (1 << 22) - 1, slope0_value: slope, slope1_value: slope,
            sfstrx_en: 1 << 2, frddsx_en: 1 << 1, fhctlx_en: 1 << 0,
            tgl_org: 1 << 31, dvfs_tri: 1 << 31, pcwchg: 1 << 31,
            dt_val: 0, df_val: 0x9, updnlmt_shft: 16,
            msk_frddsx_dys: 0xf << 20, msk_frddsx_dts: 0xf << 16,
        },
    }
}

const fn fh_default(pll_id: i32, fh_id: i32, offset: usize) -> mtk_pllfh_data { fh(pll_id, fh_id, 0x6003c97, offset) }
const fn fh_m(pll_id: i32, fh_id: i32, offset: usize) -> mtk_pllfh_data { fh(pll_id, fh_id, 0x6000140, offset) }

static mut PLLFHS: [mtk_pllfh_data; 8] = [
    fh_default(CLK_APMIXED_ARMCA53PLL, FhPllId::FhCa53pllBl as i32, 0x38),
    fh_default(CLK_APMIXED_MAINPLL, FhPllId::FhMainpll as i32, 0x60),
    fh_m(CLK_APMIXED_MPLL, FhPllId::FhMpll as i32, 0x74),
    fh_m(CLK_APMIXED_MSDCPLL, FhPllId::FhMsdcpll as i32, 0x88),
    fh_m(CLK_APMIXED_MMPLL, FhPllId::FhMmpll as i32, 0x9c),
    fh_m(CLK_APMIXED_VENCPLL, FhPllId::FhVencpll as i32, 0xb0),
    fh_m(CLK_APMIXED_TVDPLL, FhPllId::FhTvdpll as i32, 0xc4),
    fh_m(CLK_APMIXED_VCODECPLL, FhPllId::FhVcodecpll as i32, 0xd8),
];

unsafe fn clk_mt6795_apmixed_setup_md1(base: *mut u8) {
    let reg = base.add(REG_AP_PLL_CON7);
    writel(readl(reg) & !MD1_CLK_OFF, reg);
    writel(readl(reg) & !MD1_MTCMOS_OFF, reg);
    writel(readl(reg) & !MD1_ISO_OFF, reg);
    writel(readl(reg) & !MD1_MEM_OFF, reg);
}

static OF_MATCH_CLK_MT6795_APMIXED: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt6795-apmixedsys" },
    of_device_id { /* sentinel */ compatible: core::ptr::null() },
];

unsafe fn clk_mt6795_apmixed_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let node = dev.of_node;
    let fhctl_node: *const u8 = b"mediatek,mt6795-fhctl\0".as_ptr();
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { return PTR_ERR(base); }
    let clk_data = mtk_alloc_clk_data(CLK_APMIXED_NR_CLK);
    if clk_data.is_null() { return -ENOMEM; }
    fhctl_parse_dt(fhctl_node, PLLFHS.as_mut_ptr(), PLLFHS.len());
    let mut ret = mtk_clk_register_pllfhs(dev, PLlS.as_ptr(), PLlS.len(), PLLFHS.as_mut_ptr(), PLLFHS.len(), clk_data);
    if ret != 0 { mtk_free_clk_data(clk_data); return ret; }
    let hw = mtk_clk_register_ref2usb_tx(b"ref2usb_tx\0".as_ptr(), b"clk26m\0".as_ptr(), base.add(REG_REF2USB));
    if IS_ERR(hw) { ret = PTR_ERR(hw); dev_err(dev, b"Failed to register ref2usb_tx: %d\n".as_ptr(), ret); mtk_clk_unregister_pllfhs(PLlS.as_ptr(), PLlS.len(), PLLFHS.as_mut_ptr(), PLLFHS.len(), clk_data); mtk_free_clk_data(clk_data); return ret; }
    (*clk_data).hws[CLK_APMIXED_REF2USB_TX as usize] = hw;
    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if ret != 0 { dev_err(dev, b"Cannot register clock provider: %d\n".as_ptr(), ret); mtk_clk_unregister_ref2usb_tx((*clk_data).hws[CLK_APMIXED_REF2USB_TX as usize]); mtk_clk_unregister_pllfhs(PLlS.as_ptr(), PLlS.len(), PLLFHS.as_mut_ptr(), PLLFHS.len(), clk_data); mtk_free_clk_data(clk_data); return ret; }
    clk_mt6795_apmixed_setup_md1(base);
    0
}

unsafe fn clk_mt6795_apmixed_remove(pdev: *mut platform_device) {
    let node = (*pdev).dev.of_node;
    let clk_data = platform_get_drvdata(pdev);
    of_clk_del_provider(node);
    mtk_clk_unregister_ref2usb_tx((*clk_data).hws[CLK_APMIXED_REF2USB_TX as usize]);
    mtk_clk_unregister_pllfhs(PLlS.as_ptr(), PLlS.len(), PLLFHS.as_mut_ptr(), PLLFHS.len(), clk_data);
    mtk_free_clk_data(clk_data);
}

static mut CLK_MT6795_APMIXED_DRV: platform_driver = platform_driver {
    probe: Some(clk_mt6795_apmixed_probe),
    remove: Some(clk_mt6795_apmixed_remove),
    driver: device_driver { name: b"clk-mt6795-apmixed\0".as_ptr(), of_match_table: OF_MATCH_CLK_MT6795_APMIXED.as_ptr() },
};

// module_platform_driver(CLK_MT6795_APMIXED_DRV);
// MODULE_DESCRIPTION("MediaTek MT6795 apmixed clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
