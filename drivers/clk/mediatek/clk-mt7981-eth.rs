// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Sam Shih <sam.shih@mediatek.com>
 * Author: Wenzhen Yu <wenzhen.yu@mediatek.com>
 * Author: Jianhui Zhao <zhaojh329@gmail.com>
 * Author: Daniel Golle <daniel@makrotopia.org>
 */

// Dependencies supplied by the Linux clock, platform, device-tree, and
// MediaTek clock headers are intentionally left external.

#[repr(C)]
pub struct MtkGateRegs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct MtkGate {
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub regs: *const MtkGateRegs,
    pub shift: u32,
    pub ops: *const core::ffi::c_void,
}

#[repr(C)]
pub struct MtkClkDesc {
    pub clks: *const MtkGate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const core::ffi::c_char,
    pub data: *const MtkClkDesc,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
}

extern "C" {
    pub static mtk_clk_gate_ops_no_setclr_inv: core::ffi::c_void;
    pub fn mtk_clk_simple_probe(dev: *mut core::ffi::c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut core::ffi::c_void) -> i32;
}

// Clock identifiers are supplied by dt-bindings/clock/mediatek,mt7981-clk.h.
extern "C" {
    pub static CLK_SGM0_TX_EN: u32;
    pub static CLK_SGM0_RX_EN: u32;
    pub static CLK_SGM0_CK0_EN: u32;
    pub static CLK_SGM0_CDR_CK0_EN: u32;
    pub static CLK_SGM1_TX_EN: u32;
    pub static CLK_SGM1_RX_EN: u32;
    pub static CLK_SGM1_CK1_EN: u32;
    pub static CLK_SGM1_CDR_CK1_EN: u32;
    pub static CLK_ETH_FE_EN: u32;
    pub static CLK_ETH_GP2_EN: u32;
    pub static CLK_ETH_GP1_EN: u32;
    pub static CLK_ETH_WOCPU0_EN: u32;
}

static SGMII0_CG_REGS: MtkGateRegs = MtkGateRegs { set_ofs: 0xE4, clr_ofs: 0xE4, sta_ofs: 0xE4 };

static SGMII0_CLKS: [MtkGate; 4] = [
    MtkGate { id: unsafe { CLK_SGM0_TX_EN }, name: b"sgm0_tx_en\0".as_ptr() as _, parent_name: b"usb_tx250m\0".as_ptr() as _, regs: &SGMII0_CG_REGS, shift: 2, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
    MtkGate { id: unsafe { CLK_SGM0_RX_EN }, name: b"sgm0_rx_en\0".as_ptr() as _, parent_name: b"usb_eq_rx250m\0".as_ptr() as _, regs: &SGMII0_CG_REGS, shift: 3, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
    MtkGate { id: unsafe { CLK_SGM0_CK0_EN }, name: b"sgm0_ck0_en\0".as_ptr() as _, parent_name: b"usb_ln0\0".as_ptr() as _, regs: &SGMII0_CG_REGS, shift: 4, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
    MtkGate { id: unsafe { CLK_SGM0_CDR_CK0_EN }, name: b"sgm0_cdr_ck0_en\0".as_ptr() as _, parent_name: b"usb_cdr\0".as_ptr() as _, regs: &SGMII0_CG_REGS, shift: 5, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
];

static SGMII1_CG_REGS: MtkGateRegs = MtkGateRegs { set_ofs: 0xE4, clr_ofs: 0xE4, sta_ofs: 0xE4 };
static SGMII1_CLKS: [MtkGate; 4] = [
    MtkGate { id: unsafe { CLK_SGM1_TX_EN }, name: b"sgm1_tx_en\0".as_ptr() as _, parent_name: b"usb_tx250m\0".as_ptr() as _, regs: &SGMII1_CG_REGS, shift: 2, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
    MtkGate { id: unsafe { CLK_SGM1_RX_EN }, name: b"sgm1_rx_en\0".as_ptr() as _, parent_name: b"usb_eq_rx250m\0".as_ptr() as _, regs: &SGMII1_CG_REGS, shift: 3, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
    MtkGate { id: unsafe { CLK_SGM1_CK1_EN }, name: b"sgm1_ck1_en\0".as_ptr() as _, parent_name: b"usb_ln0\0".as_ptr() as _, regs: &SGMII1_CG_REGS, shift: 4, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
    MtkGate { id: unsafe { CLK_SGM1_CDR_CK1_EN }, name: b"sgm1_cdr_ck1_en\0".as_ptr() as _, parent_name: b"usb_cdr\0".as_ptr() as _, regs: &SGMII1_CG_REGS, shift: 5, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
];

static ETH_CG_REGS: MtkGateRegs = MtkGateRegs { set_ofs: 0x30, clr_ofs: 0x30, sta_ofs: 0x30 };
static ETH_CLKS: [MtkGate; 4] = [
    MtkGate { id: unsafe { CLK_ETH_FE_EN }, name: b"eth_fe_en\0".as_ptr() as _, parent_name: b"netsys_2x\0".as_ptr() as _, regs: &ETH_CG_REGS, shift: 6, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
    MtkGate { id: unsafe { CLK_ETH_GP2_EN }, name: b"eth_gp2_en\0".as_ptr() as _, parent_name: b"sgm_325m\0".as_ptr() as _, regs: &ETH_CG_REGS, shift: 7, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
    MtkGate { id: unsafe { CLK_ETH_GP1_EN }, name: b"eth_gp1_en\0".as_ptr() as _, parent_name: b"sgm_325m\0".as_ptr() as _, regs: &ETH_CG_REGS, shift: 8, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
    MtkGate { id: unsafe { CLK_ETH_WOCPU0_EN }, name: b"eth_wocpu0_en\0".as_ptr() as _, parent_name: b"netsys_wed_mcu\0".as_ptr() as _, regs: &ETH_CG_REGS, shift: 15, ops: unsafe { &mtk_clk_gate_ops_no_setclr_inv } },
];

static ETH_DESC: MtkClkDesc = MtkClkDesc { clks: ETH_CLKS.as_ptr(), num_clks: ETH_CLKS.len() };
static SGMII0_DESC: MtkClkDesc = MtkClkDesc { clks: SGMII0_CLKS.as_ptr(), num_clks: SGMII0_CLKS.len() };
static SGMII1_DESC: MtkClkDesc = MtkClkDesc { clks: SGMII1_CLKS.as_ptr(), num_clks: SGMII1_CLKS.len() };

static OF_MATCH_CLK_MT7981_ETH: [OfDeviceId; 4] = [
    OfDeviceId { compatible: b"mediatek,mt7981-ethsys\0".as_ptr() as _, data: &ETH_DESC },
    OfDeviceId { compatible: b"mediatek,mt7981-sgmiisys_0\0".as_ptr() as _, data: &SGMII0_DESC },
    OfDeviceId { compatible: b"mediatek,mt7981-sgmiisys_1\0".as_ptr() as _, data: &SGMII1_DESC },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

static CLK_MT7981_ETH_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    name: b"clk-mt7981-eth\0".as_ptr() as _,
    of_match_table: OF_MATCH_CLK_MT7981_ETH.as_ptr(),
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt7981_eth);
// module_platform_driver(clk_mt7981_eth_drv);
// MODULE_DESCRIPTION("MediaTek MT7981 Ethernet clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
