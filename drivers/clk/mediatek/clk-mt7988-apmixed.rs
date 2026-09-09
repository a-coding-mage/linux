// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023 MediaTek Inc.
 * Author: Sam Shih <sam.shih@mediatek.com>
 * Author: Xiufeng Li <Xiufeng.Li@mediatek.com>
 */

// Translated dependencies: linux/clk-provider.h, linux/of*.h,
// linux/platform_device.h, clk-mtk.h, clk-gate.h, clk-mux.h, clk-pll.h,
// and dt-bindings/clock/mediatek,mt7988-clk.h.

const MT7988_PLL_FMAX: u64 = 2500u64 * MHZ;
const MT7988_PCW_CHG_BIT: u32 = 2;

#[repr(C)]
struct mtk_pll_data {
    id: u32,
    name: *const core::ffi::c_char,
    reg: u32,
    pwr_reg: u32,
    en_mask: u32,
    flags: u32,
    rst_bar_mask: u32,
    fmax: u64,
    pcwbits: u32,
    pd_reg: u32,
    pd_shift: u32,
    tuner_reg: u32,
    tuner_en_reg: u32,
    tuner_en_bit: u32,
    pcw_reg: u32,
    pcw_shift: u32,
    pcw_chg_reg: u32,
    pcw_chg_bit: u32,
    parent_name: *const core::ffi::c_char,
}

macro_rules! pll {
    ($id:expr, $name:literal, $reg:expr, $pwr_reg:expr, $en_mask:expr,
     $flags:expr, $rst_bar_mask:expr, $pcwbits:expr, $pd_reg:expr,
     $pd_shift:expr, $tuner_reg:expr, $tuner_en_reg:expr, $tuner_en_bit:expr,
     $pcw_reg:expr, $pcw_shift:expr, $pcw_chg_reg:expr) => {
        mtk_pll_data {
            id: $id, name: concat!($name, "\0").as_ptr() as *const core::ffi::c_char,
            reg: $reg, pwr_reg: $pwr_reg, en_mask: $en_mask, flags: $flags,
            rst_bar_mask: 1u32 << $rst_bar_mask, fmax: MT7988_PLL_FMAX,
            pcwbits: $pcwbits, pd_reg: $pd_reg, pd_shift: $pd_shift,
            tuner_reg: $tuner_reg, tuner_en_reg: $tuner_en_reg,
            tuner_en_bit: $tuner_en_bit, pcw_reg: $pcw_reg,
            pcw_shift: $pcw_shift, pcw_chg_reg: $pcw_chg_reg,
            pcw_chg_bit: MT7988_PCW_CHG_BIT,
            parent_name: b"clkxtal\0".as_ptr() as *const core::ffi::c_char,
        }
    };
}

static plls: [mtk_pll_data; 12] = [
    pll!(CLK_APMIXED_NETSYSPLL, "netsyspll", 0x0104, 0x0110, 0x00000001, 0, 0, 32, 0x0104, 4, 0, 0, 0, 0x0108, 0, 0x0104),
    pll!(CLK_APMIXED_MPLL, "mpll", 0x0114, 0x0120, 0xff000001, HAVE_RST_BAR, 23, 32, 0x0114, 4, 0, 0, 0, 0x0118, 0, 0x0114),
    pll!(CLK_APMIXED_MMPLL, "mmpll", 0x0124, 0x0130, 0xff000001, HAVE_RST_BAR, 23, 32, 0x0124, 4, 0, 0, 0, 0x0128, 0, 0x0124),
    pll!(CLK_APMIXED_APLL2, "apll2", 0x0134, 0x0140, 0x00000001, 0, 0, 32, 0x0134, 4, 0x0704, 0x0700, 1, 0x0138, 0, 0x0134),
    pll!(CLK_APMIXED_NET1PLL, "net1pll", 0x0144, 0x0150, 0xff000001, HAVE_RST_BAR, 23, 32, 0x0144, 4, 0, 0, 0, 0x0148, 0, 0x0144),
    pll!(CLK_APMIXED_NET2PLL, "net2pll", 0x0154, 0x0160, 0xff000001, HAVE_RST_BAR | PLL_AO, 23, 32, 0x0154, 4, 0, 0, 0, 0x0158, 0, 0x0154),
    pll!(CLK_APMIXED_WEDMCUPLL, "wedmcupll", 0x0164, 0x0170, 0x00000001, 0, 0, 32, 0x0164, 4, 0, 0, 0, 0x0168, 0, 0x0164),
    pll!(CLK_APMIXED_SGMPLL, "sgmpll", 0x0174, 0x0180, 0x00000001, 0, 0, 32, 0x0174, 4, 0, 0, 0, 0x0178, 0, 0x0174),
    pll!(CLK_APMIXED_ARM_B, "arm_b", 0x0204, 0x0210, 0xff000001, HAVE_RST_BAR | PLL_AO, 23, 32, 0x0204, 4, 0, 0, 0, 0x0208, 0, 0x0204),
    pll!(CLK_APMIXED_CCIPLL2_B, "ccipll2_b", 0x0214, 0x0220, 0xff000001, HAVE_RST_BAR, 23, 32, 0x0214, 4, 0, 0, 0, 0x0218, 0, 0x0214),
    pll!(CLK_APMIXED_USXGMIIPLL, "usxgmiipll", 0x0304, 0x0310, 0xff000001, HAVE_RST_BAR, 23, 32, 0x0304, 4, 0, 0, 0, 0x0308, 0, 0x0304),
    pll!(CLK_APMIXED_MSDCPLL, "msdcpll", 0x0314, 0x0320, 0x00000001, 0, 0, 32, 0x0314, 4, 0, 0, 0, 0x0318, 0, 0x0314),
];

#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char }
static of_match_clk_mt7988_apmixed: [of_device_id; 2] = [
    of_device_id { compatible: b"mediatek,mt7988-apmixedsys\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

// The following probe and driver preserve the C control flow and external kernel calls.
unsafe fn clk_mt7988_apmixed_probe(pdev: *mut platform_device) -> i32 {
    let clk_data: *mut clk_hw_onecell_data;
    let node = (*pdev).dev.of_node;
    let mut r: i32;
    clk_data = mtk_alloc_clk_data(plls.len());
    if clk_data.is_null() { return -ENOMEM; }
    r = mtk_clk_register_plls(&mut (*pdev).dev, plls.as_ptr(), plls.len(), clk_data);
    if r != 0 { mtk_free_clk_data(clk_data); return r; }
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 {
        mtk_clk_unregister_plls(plls.as_ptr(), plls.len(), clk_data);
        mtk_free_clk_data(clk_data);
    }
    r
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe fn(*mut platform_device) -> i32>,
    driver: driver,
}
#[repr(C)]
struct driver {
    name: *const core::ffi::c_char,
    of_match_table: *const of_device_id,
}
static mut clk_mt7988_apmixed_drv: platform_driver = platform_driver {
    probe: Some(clk_mt7988_apmixed_probe),
    driver: driver {
        name: b"clk-mt7988-apmixed\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: of_match_clk_mt7988_apmixed.as_ptr(),
    },
};

extern "C" {
    static MHZ: u64;
    static HAVE_RST_BAR: u32;
    static PLL_AO: u32;
    static ENOMEM: i32;
    type platform_device;
    type clk_hw_onecell_data;
    fn mtk_alloc_clk_data(n: usize) -> *mut clk_hw_onecell_data;
    fn mtk_free_clk_data(data: *mut clk_hw_onecell_data);
    fn mtk_clk_register_plls(dev: *mut device, plls: *const mtk_pll_data, n: usize, data: *mut clk_hw_onecell_data) -> i32;
    fn mtk_clk_unregister_plls(plls: *const mtk_pll_data, n: usize, data: *mut clk_hw_onecell_data);
    fn of_clk_add_hw_provider(node: *mut device_node, get: unsafe extern "C" fn(), data: *mut clk_hw_onecell_data) -> i32;
    fn of_clk_hw_onecell_get();
    type device;
    type device_node;
}

// Equivalent to builtin_platform_driver(clk_mt7988_apmixed_drv).
// MODULE_DESCRIPTION("MediaTek MT7988 apmixedsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
