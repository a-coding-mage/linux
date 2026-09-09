// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Chen Zhong <chen.zhong@mediatek.com>
 *        Sean Wang <sean.wang@mediatek.com>
 */

// Dependencies supplied by the Linux clock-provider, platform-device,
// MediaTek clock, gate, and MT7622 clock-binding headers.

#[repr(C)]
pub struct mtk_gate_regs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct mtk_gate {
    pub id: u32,
    pub name: *const u8,
    pub parent_name: *const u8,
    pub regs: *const mtk_gate_regs,
    pub shift: u8,
    pub ops: *const core::ffi::c_void,
}

#[repr(C)]
pub struct mtk_clk_rst_desc {
    pub version: u32,
    pub rst_bank_ofs: *const u16,
    pub rst_bank_nr: usize,
}

#[repr(C)]
pub struct mtk_clk_desc {
    pub clks: *const mtk_gate,
    pub num_clks: usize,
    pub rst_desc: *const mtk_clk_rst_desc,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
}

extern "C" {
    pub static mtk_clk_gate_ops_no_setclr_inv: core::ffi::c_void;
    pub fn mtk_clk_simple_probe(dev: *mut core::ffi::c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut core::ffi::c_void) -> i32;
}

// GATE_MTK(_id, _name, _parent, _regs, _shift, _ops)
macro_rules! gate_mtk {
    ($id:expr, $name:expr, $parent:expr, $regs:expr, $shift:expr, $ops:expr) => {
        mtk_gate { id: $id as u32, name: concat!($name, "\0").as_ptr(),
            parent_name: concat!($parent, "\0").as_ptr(), regs: $regs,
            shift: $shift, ops: $ops as *const _ as *const core::ffi::c_void }
    };
}

const MTK_RST_SIMPLE: u32 = 0;

static eth_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x30, clr_ofs: 0x30, sta_ofs: 0x30 };

static eth_clks: [mtk_gate; 5] = [
    gate_mtk!(CLK_ETH_HSDMA_EN, "eth_hsdma_en", "eth_sel", &eth_cg_regs, 5, &mtk_clk_gate_ops_no_setclr_inv),
    gate_mtk!(CLK_ETH_ESW_EN, "eth_esw_en", "eth_500m", &eth_cg_regs, 6, &mtk_clk_gate_ops_no_setclr_inv),
    gate_mtk!(CLK_ETH_GP2_EN, "eth_gp2_en", "txclk_src_pre", &eth_cg_regs, 7, &mtk_clk_gate_ops_no_setclr_inv),
    gate_mtk!(CLK_ETH_GP1_EN, "eth_gp1_en", "txclk_src_pre", &eth_cg_regs, 8, &mtk_clk_gate_ops_no_setclr_inv),
    gate_mtk!(CLK_ETH_GP0_EN, "eth_gp0_en", "txclk_src_pre", &eth_cg_regs, 9, &mtk_clk_gate_ops_no_setclr_inv),
];

static sgmii_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0xE4, clr_ofs: 0xE4, sta_ofs: 0xE4 };

static sgmii_clks: [mtk_gate; 4] = [
    gate_mtk!(CLK_SGMII_TX250M_EN, "sgmii_tx250m_en", "ssusb_tx250m", &sgmii_cg_regs, 2, &mtk_clk_gate_ops_no_setclr_inv),
    gate_mtk!(CLK_SGMII_RX250M_EN, "sgmii_rx250m_en", "ssusb_eq_rx250m", &sgmii_cg_regs, 3, &mtk_clk_gate_ops_no_setclr_inv),
    gate_mtk!(CLK_SGMII_CDR_REF, "sgmii_cdr_ref", "ssusb_cdr_ref", &sgmii_cg_regs, 4, &mtk_clk_gate_ops_no_setclr_inv),
    gate_mtk!(CLK_SGMII_CDR_FB, "sgmii_cdr_fb", "ssusb_cdr_fb", &sgmii_cg_regs, 5, &mtk_clk_gate_ops_no_setclr_inv),
];

static rst_ofs: [u16; 1] = [0x34];
static clk_rst_desc: mtk_clk_rst_desc = mtk_clk_rst_desc { version: MTK_RST_SIMPLE, rst_bank_ofs: rst_ofs.as_ptr(), rst_bank_nr: rst_ofs.len() };
static eth_desc: mtk_clk_desc = mtk_clk_desc { clks: eth_clks.as_ptr(), num_clks: eth_clks.len(), rst_desc: &clk_rst_desc };
static sgmii_desc: mtk_clk_desc = mtk_clk_desc { clks: sgmii_clks.as_ptr(), num_clks: sgmii_clks.len(), rst_desc: core::ptr::null() };

static of_match_clk_mt7622_eth: [of_device_id; 3] = [
    of_device_id { compatible: b"mediatek,mt7622-ethsys\0".as_ptr(), data: &eth_desc as *const _ as *const _ },
    of_device_id { compatible: b"mediatek,mt7622-sgmiisys\0".as_ptr(), data: &sgmii_desc as *const _ as *const _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut clk_mt7622_eth_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe), remove: Some(mtk_clk_simple_remove),
    driver: driver { name: b"clk-mt7622-eth\0".as_ptr(), of_match_table: of_match_clk_mt7622_eth.as_ptr() },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt7622_eth);
// module_platform_driver(clk_mt7622_eth_drv);
// MODULE_DESCRIPTION("MediaTek MT7622 Ethernet clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
