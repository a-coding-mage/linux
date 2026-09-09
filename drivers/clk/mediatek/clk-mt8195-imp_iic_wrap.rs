// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Translated dependencies:
// "clk-gate.h", "clk-mtk.h", dt-bindings/clock/mt8195-clk.h,
// <linux/clk-provider.h>, and <linux/platform_device.h>.

use core::ffi::c_void;

#[repr(C)]
pub struct MtkGateRegs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct MtkGate {
    pub id: i32,
    pub name: *const u8,
    pub parent_name: *const u8,
    pub regs: *const MtkGateRegs,
    pub shift: u8,
    pub ops: *const c_void,
    pub flags: u32,
}

#[repr(C)]
pub struct MtkClkDesc {
    pub clks: *const MtkGate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
    pub data: *const c_void,
}

#[repr(C)]
pub struct PlatformDriverInfo {
    pub name: *const u8,
    pub of_match_table: *const OfDeviceId,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub driver: PlatformDriverInfo,
}

extern "C" {
    pub static mtk_clk_gate_ops_setclr: c_void;
    pub fn mtk_clk_simple_probe(dev: *mut c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut c_void) -> i32;
}

// CLK_OPS_PARENT_ENABLE
const CLK_OPS_PARENT_ENABLE: u32 = 1;

static IMP_IIC_WRAP_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0xe08,
    clr_ofs: 0xe04,
    sta_ofs: 0xe00,
};

const fn gate_imp_iic_wrap(id: i32, name: &'static [u8], parent: &'static [u8], shift: u8) -> MtkGate {
    MtkGate {
        id,
        name: name.as_ptr(),
        parent_name: parent.as_ptr(),
        regs: &IMP_IIC_WRAP_CG_REGS,
        shift,
        ops: unsafe { &mtk_clk_gate_ops_setclr as *const c_void },
        flags: CLK_OPS_PARENT_ENABLE,
    }
}

extern "C" {
    static CLK_IMP_IIC_WRAP_S_I2C5: i32;
    static CLK_IMP_IIC_WRAP_S_I2C6: i32;
    static CLK_IMP_IIC_WRAP_S_I2C7: i32;
    static CLK_IMP_IIC_WRAP_W_I2C0: i32;
    static CLK_IMP_IIC_WRAP_W_I2C1: i32;
    static CLK_IMP_IIC_WRAP_W_I2C2: i32;
    static CLK_IMP_IIC_WRAP_W_I2C3: i32;
    static CLK_IMP_IIC_WRAP_W_I2C4: i32;
}

static IMP_IIC_WRAP_S_CLKS: [MtkGate; 3] = [
    gate_imp_iic_wrap(unsafe { CLK_IMP_IIC_WRAP_S_I2C5 }, b"imp_iic_wrap_s_i2c5\0", b"top_i2c\0", 0),
    gate_imp_iic_wrap(unsafe { CLK_IMP_IIC_WRAP_S_I2C6 }, b"imp_iic_wrap_s_i2c6\0", b"top_i2c\0", 1),
    gate_imp_iic_wrap(unsafe { CLK_IMP_IIC_WRAP_S_I2C7 }, b"imp_iic_wrap_s_i2c7\0", b"top_i2c\0", 2),
];

static IMP_IIC_WRAP_W_CLKS: [MtkGate; 5] = [
    gate_imp_iic_wrap(unsafe { CLK_IMP_IIC_WRAP_W_I2C0 }, b"imp_iic_wrap_w_i2c0\0", b"top_i2c\0", 0),
    gate_imp_iic_wrap(unsafe { CLK_IMP_IIC_WRAP_W_I2C1 }, b"imp_iic_wrap_w_i2c1\0", b"top_i2c\0", 1),
    gate_imp_iic_wrap(unsafe { CLK_IMP_IIC_WRAP_W_I2C2 }, b"imp_iic_wrap_w_i2c2\0", b"top_i2c\0", 2),
    gate_imp_iic_wrap(unsafe { CLK_IMP_IIC_WRAP_W_I2C3 }, b"imp_iic_wrap_w_i2c3\0", b"top_i2c\0", 3),
    gate_imp_iic_wrap(unsafe { CLK_IMP_IIC_WRAP_W_I2C4 }, b"imp_iic_wrap_w_i2c4\0", b"top_i2c\0", 4),
];

static IMP_IIC_WRAP_S_DESC: MtkClkDesc = MtkClkDesc { clks: IMP_IIC_WRAP_S_CLKS.as_ptr(), num_clks: IMP_IIC_WRAP_S_CLKS.len() };
static IMP_IIC_WRAP_W_DESC: MtkClkDesc = MtkClkDesc { clks: IMP_IIC_WRAP_W_CLKS.as_ptr(), num_clks: IMP_IIC_WRAP_W_CLKS.len() };

static OF_MATCH_CLK_MT8195_IMP_IIC_WRAP: [OfDeviceId; 3] = [
    OfDeviceId { compatible: b"mediatek,mt8195-imp_iic_wrap_s\0".as_ptr(), data: &IMP_IIC_WRAP_S_DESC as *const _ as *const c_void },
    OfDeviceId { compatible: b"mediatek,mt8195-imp_iic_wrap_w\0".as_ptr(), data: &IMP_IIC_WRAP_W_DESC as *const _ as *const c_void },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() }, // sentinel
];

static mut CLK_MT8195_IMP_IIC_WRAP_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: PlatformDriverInfo {
        name: b"clk-mt8195-imp_iic_wrap\0".as_ptr(),
        of_match_table: OF_MATCH_CLK_MT8195_IMP_IIC_WRAP.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_imp_iic_wrap);
// module_platform_driver(clk_mt8195_imp_iic_wrap_drv);
// MODULE_DESCRIPTION("MediaTek MT8195 I2C Wrapper clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
