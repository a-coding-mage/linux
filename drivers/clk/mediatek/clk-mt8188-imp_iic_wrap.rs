// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel clock-driver environment.

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
    pub flags: u32,
}

#[repr(C)]
pub struct MtkClkDesc {
    pub clks: *const MtkGate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub driver: Driver,
}

#[repr(C)]
pub struct Driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
}

unsafe extern "C" {
    pub static mtk_clk_gate_ops_setclr: core::ffi::c_void;
    pub fn mtk_clk_simple_probe(device: *mut core::ffi::c_void) -> i32;
    pub fn mtk_clk_simple_remove(device: *mut core::ffi::c_void) -> i32;
}

const CLK_OPS_PARENT_ENABLE: u32 = 1;

static IMP_IIC_WRAP_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0xe08,
    clr_ofs: 0xe04,
    sta_ofs: 0xe00,
};

macro_rules! gate_imp_iic_wrap {
    ($id:expr, $name:literal, $parent:literal, $shift:expr) => {
        MtkGate {
            id: $id,
            name: concat!($name, "\0").as_ptr() as *const core::ffi::c_char,
            parent_name: concat!($parent, "\0").as_ptr() as *const core::ffi::c_char,
            regs: &IMP_IIC_WRAP_CG_REGS,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_setclr as *const _ as *const core::ffi::c_void },
            flags: CLK_OPS_PARENT_ENABLE,
        }
    };
}

unsafe extern "C" {
    pub static CLK_IMP_IIC_WRAP_C_AP_CLOCK_I2C0: u32;
    pub static CLK_IMP_IIC_WRAP_C_AP_CLOCK_I2C2: u32;
    pub static CLK_IMP_IIC_WRAP_C_AP_CLOCK_I2C3: u32;
    pub static CLK_IMP_IIC_WRAP_W_AP_CLOCK_I2C1: u32;
    pub static CLK_IMP_IIC_WRAP_W_AP_CLOCK_I2C4: u32;
    pub static CLK_IMP_IIC_WRAP_EN_AP_CLOCK_I2C5: u32;
    pub static CLK_IMP_IIC_WRAP_EN_AP_CLOCK_I2C6: u32;
}

static IMP_IIC_WRAP_C_CLKS: [MtkGate; 3] = [
    gate_imp_iic_wrap!(unsafe { CLK_IMP_IIC_WRAP_C_AP_CLOCK_I2C0 }, "imp_iic_wrap_c_ap_clock_i2c0", "top_i2c", 0),
    gate_imp_iic_wrap!(unsafe { CLK_IMP_IIC_WRAP_C_AP_CLOCK_I2C2 }, "imp_iic_wrap_c_ap_clock_i2c2", "top_i2c", 1),
    gate_imp_iic_wrap!(unsafe { CLK_IMP_IIC_WRAP_C_AP_CLOCK_I2C3 }, "imp_iic_wrap_c_ap_clock_i2c3", "top_i2c", 2),
];

static IMP_IIC_WRAP_W_CLKS: [MtkGate; 2] = [
    gate_imp_iic_wrap!(unsafe { CLK_IMP_IIC_WRAP_W_AP_CLOCK_I2C1 }, "imp_iic_wrap_w_ap_clock_i2c1", "top_i2c", 0),
    gate_imp_iic_wrap!(unsafe { CLK_IMP_IIC_WRAP_W_AP_CLOCK_I2C4 }, "imp_iic_wrap_w_ap_clock_i2c4", "top_i2c", 1),
];

static IMP_IIC_WRAP_EN_CLKS: [MtkGate; 2] = [
    gate_imp_iic_wrap!(unsafe { CLK_IMP_IIC_WRAP_EN_AP_CLOCK_I2C5 }, "imp_iic_wrap_en_ap_clock_i2c5", "top_i2c", 0),
    gate_imp_iic_wrap!(unsafe { CLK_IMP_IIC_WRAP_EN_AP_CLOCK_I2C6 }, "imp_iic_wrap_en_ap_clock_i2c6", "top_i2c", 1),
];

static IMP_IIC_WRAP_C_DESC: MtkClkDesc = MtkClkDesc { clks: IMP_IIC_WRAP_C_CLKS.as_ptr(), num_clks: IMP_IIC_WRAP_C_CLKS.len() };
static IMP_IIC_WRAP_W_DESC: MtkClkDesc = MtkClkDesc { clks: IMP_IIC_WRAP_W_CLKS.as_ptr(), num_clks: IMP_IIC_WRAP_W_CLKS.len() };
static IMP_IIC_WRAP_EN_DESC: MtkClkDesc = MtkClkDesc { clks: IMP_IIC_WRAP_EN_CLKS.as_ptr(), num_clks: IMP_IIC_WRAP_EN_CLKS.len() };

static OF_MATCH_CLK_MT8188_IMP_IIC_WRAP: [OfDeviceId; 4] = [
    OfDeviceId { compatible: b"mediatek,mt8188-imp-iic-wrap-c\0".as_ptr() as *const _, data: &IMP_IIC_WRAP_C_DESC as *const _ as *const _ },
    OfDeviceId { compatible: b"mediatek,mt8188-imp-iic-wrap-w\0".as_ptr() as *const _, data: &IMP_IIC_WRAP_W_DESC as *const _ as *const _ },
    OfDeviceId { compatible: b"mediatek,mt8188-imp-iic-wrap-en\0".as_ptr() as *const _, data: &IMP_IIC_WRAP_EN_DESC as *const _ as *const _ },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

static CLK_MT8188_IMP_IIC_WRAP_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: b"clk-mt8188-imp_iic_wrap\0".as_ptr() as *const _,
        of_match_table: OF_MATCH_CLK_MT8188_IMP_IIC_WRAP.as_ptr(),
    },
};

// module_platform_driver(CLK_MT8188_IMP_IIC_WRAP_DRV);
// MODULE_DESCRIPTION("MediaTek MT8188 I2C Wrapper clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
