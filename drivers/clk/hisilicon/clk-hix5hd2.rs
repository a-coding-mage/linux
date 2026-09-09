// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 Linaro Ltd.
 * Copyright (c) 2014 Hisilicon Limited.
 */

// Dependencies supplied by the surrounding kernel clock framework.

static mut HIX5HD2_FIXED_RATE_CLKS: [hisi_fixed_rate_clock; 30] = [
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_1200M, name: "1200m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 1200000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_400M, name: "400m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 400000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_48M, name: "48m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 48000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_24M, name: "24m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 24000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_600M, name: "600m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 600000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_300M, name: "300m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 300000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_75M, name: "75m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 75000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_200M, name: "200m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 200000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_100M, name: "100m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 100000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_40M, name: "40m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 40000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_150M, name: "150m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 150000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_1728M, name: "1728m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 1728000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_28P8M, name: "28p8m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 28000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_432M, name: "432m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 432000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_345P6M, name: "345p6m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 345000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_288M, name: "288m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 288000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_60M, name: "60m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 60000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_750M, name: "750m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 750000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_500M, name: "500m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 500000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_54M, name: "54m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 54000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_27M, name: "27m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 27000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_1500M, name: "1500m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 1500000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_375M, name: "375m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 375000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_187M, name: "187m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 187000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_250M, name: "250m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 250000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_125M, name: "125m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 125000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_2P02M, name: "2m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 2000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_50M, name: "50m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 50000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_25M, name: "25m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 25000000 },
    hisi_fixed_rate_clock { id: HIX5HD2_FIXED_83M, name: "83m", parent_name: core::ptr::null(), flags: 0, fixed_rate: 83333333 },
];

static SFC_MUX_P: [&str; 4] = ["24m", "150m", "200m", "100m"];
static mut SFC_MUX_TABLE: [u32; 5] = [0, 4, 5, 6, 7];
static SDIO_MUX_P: [&str; 4] = ["75m", "100m", "50m", "15m"];
static mut SDIO_MUX_TABLE: [u32; 4] = [0, 1, 2, 3];
static FEPHY_MUX_P: [&str; 2] = ["25m", "125m"];
static mut FEPHY_MUX_TABLE: [u32; 2] = [0, 1];

#[repr(C)]
enum hix5hd2_clk_type { TYPE_COMPLEX, TYPE_ETHER }

#[repr(C)]
struct hix5hd2_complex_clock {
    name: *const u8, parent_name: *const u8, id: u32, ctrl_reg: u32,
    ctrl_clk_mask: u32, ctrl_rst_mask: u32, phy_reg: u32,
    phy_clk_mask: u32, phy_rst_mask: u32, type_: hix5hd2_clk_type,
}

#[repr(C)]
struct hix5hd2_clk_complex {
    hw: clk_hw, id: u32, ctrl_reg: *mut core::ffi::c_void,
    ctrl_clk_mask: u32, ctrl_rst_mask: u32, phy_reg: *mut core::ffi::c_void,
    phy_clk_mask: u32, phy_rst_mask: u32,
}

unsafe fn to_complex_clk(hw: *mut clk_hw) -> *mut hix5hd2_clk_complex {
    (hw as *mut u8).sub(core::mem::offset_of!(hix5hd2_clk_complex, hw)) as *mut hix5hd2_clk_complex
}

unsafe fn clk_ether_prepare(hw: *mut clk_hw) -> i32 {
    let clk = &mut *to_complex_clk(hw); let mut val: u32;
    val = readl_relaxed(clk.ctrl_reg); val |= clk.ctrl_clk_mask | clk.ctrl_rst_mask; writel_relaxed(val, clk.ctrl_reg);
    val &= !clk.ctrl_rst_mask; writel_relaxed(val, clk.ctrl_reg);
    val = readl_relaxed(clk.phy_reg); val |= clk.phy_clk_mask; val &= !clk.phy_rst_mask; writel_relaxed(val, clk.phy_reg); mdelay(10);
    val &= !clk.phy_clk_mask; val |= clk.phy_rst_mask; writel_relaxed(val, clk.phy_reg); mdelay(10);
    val |= clk.phy_clk_mask; val &= !clk.phy_rst_mask; writel_relaxed(val, clk.phy_reg); mdelay(30); 0
}

unsafe fn clk_ether_unprepare(hw: *mut clk_hw) { let clk = &mut *to_complex_clk(hw); let mut val = readl_relaxed(clk.ctrl_reg); val &= !clk.ctrl_clk_mask; writel_relaxed(val, clk.ctrl_reg); }
unsafe fn clk_complex_enable(hw: *mut clk_hw) -> i32 { let clk = &mut *to_complex_clk(hw); let mut val = readl_relaxed(clk.ctrl_reg); val |= clk.ctrl_clk_mask; val &= !clk.ctrl_rst_mask; writel_relaxed(val, clk.ctrl_reg); val = readl_relaxed(clk.phy_reg); val |= clk.phy_clk_mask; val &= !clk.phy_rst_mask; writel_relaxed(val, clk.phy_reg); 0 }
unsafe fn clk_complex_disable(hw: *mut clk_hw) { let clk = &mut *to_complex_clk(hw); let mut val = readl_relaxed(clk.ctrl_reg); val |= clk.ctrl_rst_mask; val &= !clk.ctrl_clk_mask; writel_relaxed(val, clk.ctrl_reg); val = readl_relaxed(clk.phy_reg); val |= clk.phy_rst_mask; val &= !clk.phy_clk_mask; writel_relaxed(val, clk.phy_reg); }

static CLK_ETHER_OPS: clk_ops = clk_ops { prepare: Some(clk_ether_prepare), unprepare: Some(clk_ether_unprepare), ..clk_ops::EMPTY };
static CLK_COMPLEX_OPS: clk_ops = clk_ops { enable: Some(clk_complex_enable), disable: Some(clk_complex_disable), ..clk_ops::EMPTY };

static HIX5HD2_COMPLEX_CLKS: [hix5hd2_complex_clock; 4] = [
    hix5hd2_complex_clock { name: b"clk_mac0\0".as_ptr(), parent_name: b"clk_fephy\0".as_ptr(), id: HIX5HD2_MAC0_CLK, ctrl_reg: 0xcc, ctrl_clk_mask: 0xa, ctrl_rst_mask: 0x500, phy_reg: 0x120, phy_clk_mask: 0, phy_rst_mask: 0x10, type_: hix5hd2_clk_type::TYPE_ETHER },
    hix5hd2_complex_clock { name: b"clk_mac1\0".as_ptr(), parent_name: b"clk_fwd_sys\0".as_ptr(), id: HIX5HD2_MAC1_CLK, ctrl_reg: 0xcc, ctrl_clk_mask: 0x14, ctrl_rst_mask: 0xa00, phy_reg: 0x168, phy_clk_mask: 0x2, phy_rst_mask: 0, type_: hix5hd2_clk_type::TYPE_ETHER },
    hix5hd2_complex_clock { name: b"clk_sata\0".as_ptr(), parent_name: core::ptr::null(), id: HIX5HD2_SATA_CLK, ctrl_reg: 0xa8, ctrl_clk_mask: 0x1f, ctrl_rst_mask: 0x300, phy_reg: 0xac, phy_clk_mask: 0x1, phy_rst_mask: 0, type_: hix5hd2_clk_type::TYPE_COMPLEX },
    hix5hd2_complex_clock { name: b"clk_usb\0".as_ptr(), parent_name: core::ptr::null(), id: HIX5HD2_USB_CLK, ctrl_reg: 0xb8, ctrl_clk_mask: 0xff, ctrl_rst_mask: 0x3f000, phy_reg: 0xbc, phy_clk_mask: 0x7, phy_rst_mask: 0x3f00, type_: hix5hd2_clk_type::TYPE_COMPLEX },
];

// The fixed, mux, gate, and complex clock tables retain the source declarations;
// their framework-specific field types are supplied by the surrounding headers.
unsafe fn hix5hd2_clk_init(np: *mut device_node) {
    let clk_data = hisi_clk_init(np, HIX5HD2_NR_CLKS); if clk_data.is_null() { return; }
    hisi_clk_register_fixed_rate(HIX5HD2_FIXED_RATE_CLKS.as_mut_ptr(), HIX5HD2_FIXED_RATE_CLKS.len(), clk_data);
    hisi_clk_register_mux(core::ptr::null_mut(), 0, clk_data);
    hisi_clk_register_gate(core::ptr::null_mut(), 0, clk_data);
}

// CLK_OF_DECLARE(hix5hd2_clk, "hisilicon,hix5hd2-clock", hix5hd2_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
