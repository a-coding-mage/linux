/* SPDX-License-Identifier: GPL-2.0+ */
// s2dos05.h
//
// Copyright (c) 2016 Samsung Electronics Co., Ltd
//              http://www.samsung.com
// Copyright (C) 2024 Dzmitry Sankouski <dsankouski@gmail.com>

// S2DOS05 registers
// Slave Addr : 0xC0
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum S2DOS05_reg {
    S2DOS05_REG_DEV_ID,
    S2DOS05_REG_TOPSYS_STAT,
    S2DOS05_REG_STAT,
    S2DOS05_REG_EN,
    S2DOS05_REG_LDO1_CFG,
    S2DOS05_REG_LDO2_CFG,
    S2DOS05_REG_LDO3_CFG,
    S2DOS05_REG_LDO4_CFG,
    S2DOS05_REG_BUCK_CFG,
    S2DOS05_REG_BUCK_VOUT,
    S2DOS05_REG_IRQ_MASK = 0x0D,
    S2DOS05_REG_SSD_TSD = 0x0E,
    S2DOS05_REG_OCL = 0x10,
    S2DOS05_REG_IRQ = 0x11,
}

// S2DOS05 regulator ids
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum S2DOS05_regulators {
    S2DOS05_LDO1,
    S2DOS05_LDO2,
    S2DOS05_LDO3,
    S2DOS05_LDO4,
    S2DOS05_BUCK1,
    S2DOS05_REG_MAX,
}

pub const S2DOS05_IRQ_PWRMT_MASK: i32 = 1 << 5;
pub const S2DOS05_IRQ_TSD_MASK: i32 = 1 << 4;
pub const S2DOS05_IRQ_SSD_MASK: i32 = 1 << 3;
pub const S2DOS05_IRQ_SCP_MASK: i32 = 1 << 2;
pub const S2DOS05_IRQ_UVLO_MASK: i32 = 1 << 1;
pub const S2DOS05_IRQ_OCD_MASK: i32 = 1 << 0;

pub const S2DOS05_BUCK_MIN1: i32 = 506250;
pub const S2DOS05_LDO_MIN1: i32 = 1500000;
pub const S2DOS05_LDO_MIN2: i32 = 2700000;
pub const S2DOS05_BUCK_STEP1: i32 = 6250;
pub const S2DOS05_LDO_STEP1: i32 = 25000;
pub const S2DOS05_LDO_VSEL_MASK: i32 = 0x7F;
pub const S2DOS05_LDO_FD_MASK: i32 = 0x80;
pub const S2DOS05_BUCK_VSEL_MASK: i32 = 0xFF;
pub const S2DOS05_BUCK_FD_MASK: i32 = 0x08;

pub const S2DOS05_ENABLE_MASK_L1: i32 = 1 << 0;
pub const S2DOS05_ENABLE_MASK_L2: i32 = 1 << 1;
pub const S2DOS05_ENABLE_MASK_L3: i32 = 1 << 2;
pub const S2DOS05_ENABLE_MASK_L4: i32 = 1 << 3;
pub const S2DOS05_ENABLE_MASK_B1: i32 = 1 << 4;

pub const S2DOS05_RAMP_DELAY: i32 = 12000;

pub const S2DOS05_ENABLE_TIME_LDO: i32 = 50;
pub const S2DOS05_ENABLE_TIME_BUCK: i32 = 350;

pub const S2DOS05_LDO_N_VOLTAGES: i32 = S2DOS05_LDO_VSEL_MASK + 1;
pub const S2DOS05_BUCK_N_VOLTAGES: i32 = S2DOS05_BUCK_VSEL_MASK + 1;

pub const S2DOS05_REGULATOR_MAX: S2DOS05_regulators = S2DOS05_regulators::S2DOS05_REG_MAX;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
