/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/mfd/tps65218.h
 *
 * Functions to access TPS65218 power management chip.
 *
 * Copyright (C) 2014 Texas Instruments Incorporated - https://www.ti.com/
 */

// Dependencies supplied by the surrounding kernel translation.

/* TPS chip id list */
pub const TPS65218: u32 = 0xF0;

/* I2C ID for TPS65218 part */
pub const TPS65218_I2C_ID: u32 = 0x24;

/* All register addresses */
pub const TPS65218_REG_CHIPID: u32 = 0x00;
pub const TPS65218_REG_INT1: u32 = 0x01;
pub const TPS65218_REG_INT2: u32 = 0x02;
pub const TPS65218_REG_INT_MASK1: u32 = 0x03;
pub const TPS65218_REG_INT_MASK2: u32 = 0x04;
pub const TPS65218_REG_STATUS: u32 = 0x05;
pub const TPS65218_REG_CONTROL: u32 = 0x06;
pub const TPS65218_REG_FLAG: u32 = 0x07;
pub const TPS65218_REG_PASSWORD: u32 = 0x10;
pub const TPS65218_REG_ENABLE1: u32 = 0x11;
pub const TPS65218_REG_ENABLE2: u32 = 0x12;
pub const TPS65218_REG_CONFIG1: u32 = 0x13;
pub const TPS65218_REG_CONFIG2: u32 = 0x14;
pub const TPS65218_REG_CONFIG3: u32 = 0x15;
pub const TPS65218_REG_CONTROL_DCDC1: u32 = 0x16;
pub const TPS65218_REG_CONTROL_DCDC2: u32 = 0x17;
pub const TPS65218_REG_CONTROL_DCDC3: u32 = 0x18;
pub const TPS65218_REG_CONTROL_DCDC4: u32 = 0x19;
pub const TPS65218_REG_CONTRL_SLEW_RATE: u32 = 0x1A;
pub const TPS65218_REG_CONTROL_LDO1: u32 = 0x1B;
pub const TPS65218_REG_SEQ1: u32 = 0x20;
pub const TPS65218_REG_SEQ2: u32 = 0x21;
pub const TPS65218_REG_SEQ3: u32 = 0x22;
pub const TPS65218_REG_SEQ4: u32 = 0x23;
pub const TPS65218_REG_SEQ5: u32 = 0x24;
pub const TPS65218_REG_SEQ6: u32 = 0x25;
pub const TPS65218_REG_SEQ7: u32 = 0x26;

macro_rules! bit { ($n:expr) => { 1u32 << $n }; }

/* Register field definitions */
pub const TPS65218_CHIPID_CHIP_MASK: u32 = 0xF8;
pub const TPS65218_CHIPID_REV_MASK: u32 = 0x07;
pub const TPS65218_REV_1_0: u32 = 0x0;
pub const TPS65218_REV_1_1: u32 = 0x1;
pub const TPS65218_REV_2_0: u32 = 0x2;
pub const TPS65218_REV_2_1: u32 = 0x3;

pub const TPS65218_INT1_VPRG: u32 = bit!(5); pub const TPS65218_INT1_AC: u32 = bit!(4); pub const TPS65218_INT1_PB: u32 = bit!(3); pub const TPS65218_INT1_HOT: u32 = bit!(2); pub const TPS65218_INT1_CC_AQC: u32 = bit!(1); pub const TPS65218_INT1_PRGC: u32 = bit!(0);
pub const TPS65218_INT2_LS3_F: u32 = bit!(5); pub const TPS65218_INT2_LS2_F: u32 = bit!(4); pub const TPS65218_INT2_LS1_F: u32 = bit!(3); pub const TPS65218_INT2_LS3_I: u32 = bit!(2); pub const TPS65218_INT2_LS2_I: u32 = bit!(1); pub const TPS65218_INT2_LS1_I: u32 = bit!(0);
pub const TPS65218_INT_MASK1_VPRG: u32 = bit!(5); pub const TPS65218_INT_MASK1_AC: u32 = bit!(4); pub const TPS65218_INT_MASK1_PB: u32 = bit!(3); pub const TPS65218_INT_MASK1_HOT: u32 = bit!(2); pub const TPS65218_INT_MASK1_CC_AQC: u32 = bit!(1); pub const TPS65218_INT_MASK1_PRGC: u32 = bit!(0);
pub const TPS65218_INT_MASK2_LS3_F: u32 = bit!(5); pub const TPS65218_INT_MASK2_LS2_F: u32 = bit!(4); pub const TPS65218_INT_MASK2_LS1_F: u32 = bit!(3); pub const TPS65218_INT_MASK2_LS3_I: u32 = bit!(2); pub const TPS65218_INT_MASK2_LS2_I: u32 = bit!(1); pub const TPS65218_INT_MASK2_LS1_I: u32 = bit!(0);
pub const TPS65218_STATUS_FSEAL: u32 = bit!(7); pub const TPS65218_STATUS_EE: u32 = bit!(6); pub const TPS65218_STATUS_AC_STATE: u32 = bit!(5); pub const TPS65218_STATUS_PB_STATE: u32 = bit!(4); pub const TPS65218_STATUS_STATE_MASK: u32 = 0xC; pub const TPS65218_STATUS_CC_STAT: u32 = 0x3;
pub const TPS65218_CONTROL_OFFNPFO: u32 = bit!(1); pub const TPS65218_CONTROL_CC_AQ: u32 = bit!(0);
pub const TPS65218_FLAG_GPO3_FLG: u32 = bit!(7); pub const TPS65218_FLAG_GPO2_FLG: u32 = bit!(6); pub const TPS65218_FLAG_GPO1_FLG: u32 = bit!(5); pub const TPS65218_FLAG_LDO1_FLG: u32 = bit!(4); pub const TPS65218_FLAG_DC4_FLG: u32 = bit!(3); pub const TPS65218_FLAG_DC3_FLG: u32 = bit!(2); pub const TPS65218_FLAG_DC2_FLG: u32 = bit!(1); pub const TPS65218_FLAG_DC1_FLG: u32 = bit!(0);
pub const TPS65218_ENABLE1_DC6_EN: u32 = bit!(5); pub const TPS65218_ENABLE1_DC5_EN: u32 = bit!(4); pub const TPS65218_ENABLE1_DC4_EN: u32 = bit!(3); pub const TPS65218_ENABLE1_DC3_EN: u32 = bit!(2); pub const TPS65218_ENABLE1_DC2_EN: u32 = bit!(1); pub const TPS65218_ENABLE1_DC1_EN: u32 = bit!(0);
pub const TPS65218_ENABLE2_GPIO3: u32 = bit!(6); pub const TPS65218_ENABLE2_GPIO2: u32 = bit!(5); pub const TPS65218_ENABLE2_GPIO1: u32 = bit!(4); pub const TPS65218_ENABLE2_LS3_EN: u32 = bit!(3); pub const TPS65218_ENABLE2_LS2_EN: u32 = bit!(2); pub const TPS65218_ENABLE2_LS1_EN: u32 = bit!(1); pub const TPS65218_ENABLE2_LDO1_EN: u32 = bit!(0);

pub const TPS65218_CONFIG1_TRST: u32 = bit!(7); pub const TPS65218_CONFIG1_GPO2_BUF: u32 = bit!(6); pub const TPS65218_CONFIG1_IO1_SEL: u32 = bit!(5); pub const TPS65218_CONFIG1_PGDLY_MASK: u32 = 0x18; pub const TPS65218_CONFIG1_STRICT: u32 = bit!(2); pub const TPS65218_CONFIG1_UVLO_MASK: u32 = 0x3; pub const TPS65218_CONFIG1_UVLO_2750000: u32 = 0x0; pub const TPS65218_CONFIG1_UVLO_2950000: u32 = 0x1; pub const TPS65218_CONFIG1_UVLO_3250000: u32 = 0x2; pub const TPS65218_CONFIG1_UVLO_3350000: u32 = 0x3;
pub const TPS65218_CONFIG2_DC12_RST: u32 = bit!(7); pub const TPS65218_CONFIG2_UVLOHYS: u32 = bit!(6); pub const TPS65218_CONFIG2_LS3ILIM_MASK: u32 = 0xC; pub const TPS65218_CONFIG2_LS2ILIM_MASK: u32 = 0x3;
pub const TPS65218_CONFIG3_LS3NPFO: u32 = bit!(5); pub const TPS65218_CONFIG3_LS2NPFO: u32 = bit!(4); pub const TPS65218_CONFIG3_LS1NPFO: u32 = bit!(3); pub const TPS65218_CONFIG3_LS3DCHRG: u32 = bit!(2); pub const TPS65218_CONFIG3_LS2DCHRG: u32 = bit!(1); pub const TPS65218_CONFIG3_LS1DCHRG: u32 = bit!(0);
pub const TPS65218_CONTROL_DCDC1_PFM: u32 = bit!(7); pub const TPS65218_CONTROL_DCDC1_MASK: u32 = 0x7F; pub const TPS65218_CONTROL_DCDC2_PFM: u32 = bit!(7); pub const TPS65218_CONTROL_DCDC2_MASK: u32 = 0x3F; pub const TPS65218_CONTROL_DCDC3_PFM: u32 = bit!(7); pub const TPS65218_CONTROL_DCDC3_MASK: u32 = 0x3F; pub const TPS65218_CONTROL_DCDC4_PFM: u32 = bit!(7); pub const TPS65218_CONTROL_DCDC4_MASK: u32 = 0x3F;
pub const TPS65218_SLEW_RATE_GO: u32 = bit!(7); pub const TPS65218_SLEW_RATE_GODSBL: u32 = bit!(6); pub const TPS65218_SLEW_RATE_SLEW_MASK: u32 = 0x7; pub const TPS65218_CONTROL_LDO1_MASK: u32 = 0x3F;
pub const TPS65218_SEQ1_DLY8: u32 = bit!(7); pub const TPS65218_SEQ1_DLY7: u32 = bit!(6); pub const TPS65218_SEQ1_DLY6: u32 = bit!(5); pub const TPS65218_SEQ1_DLY5: u32 = bit!(4); pub const TPS65218_SEQ1_DLY4: u32 = bit!(3); pub const TPS65218_SEQ1_DLY3: u32 = bit!(2); pub const TPS65218_SEQ1_DLY2: u32 = bit!(1); pub const TPS65218_SEQ1_DLY1: u32 = bit!(0);
pub const TPS65218_SEQ2_DLYFCTR: u32 = bit!(7); pub const TPS65218_SEQ2_DLY9: u32 = bit!(0);
pub const TPS65218_SEQ3_DC2_SEQ_MASK: u32 = 0xF0; pub const TPS65218_SEQ3_DC1_SEQ_MASK: u32 = 0xF; pub const TPS65218_SEQ4_DC4_SEQ_MASK: u32 = 0xF0; pub const TPS65218_SEQ4_DC3_SEQ_MASK: u32 = 0xF; pub const TPS65218_SEQ5_DC6_SEQ_MASK: u32 = 0xF0; pub const TPS65218_SEQ5_DC5_SEQ_MASK: u32 = 0xF; pub const TPS65218_SEQ6_LS1_SEQ_MASK: u32 = 0xF0; pub const TPS65218_SEQ6_LDO1_SEQ_MASK: u32 = 0xF; pub const TPS65218_SEQ7_GPO3_SEQ_MASK: u32 = 0xF0; pub const TPS65218_SEQ7_GPO1_SEQ_MASK: u32 = 0xF;
pub const TPS65218_PROTECT_NONE: u32 = 0; pub const TPS65218_PROTECT_L1: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tps65218_regulator_id {
    TPS65218_DCDC_1, TPS65218_DCDC_2, TPS65218_DCDC_3, TPS65218_DCDC_4, TPS65218_DCDC_5, TPS65218_DCDC_6,
    TPS65218_LDO_1, TPS65218_LS_2, TPS65218_LS_3,
}

pub const TPS65218_MAX_REG_ID: tps65218_regulator_id = tps65218_regulator_id::TPS65218_LDO_1;
pub const TPS65218_NUM_DCDC: usize = 6;
pub const TPS65218_NUM_LDO: usize = 1;
pub const TPS65218_NUM_LS: usize = 2;
pub const TPS65218_NUM_REGULATOR: usize = TPS65218_NUM_DCDC + TPS65218_NUM_LDO + TPS65218_NUM_LS;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tps65218_irqs {
    TPS65218_PRGC_IRQ, TPS65218_CC_AQC_IRQ, TPS65218_HOT_IRQ, TPS65218_PB_IRQ,
    TPS65218_AC_IRQ, TPS65218_VPRG_IRQ, TPS65218_INVALID1_IRQ, TPS65218_INVALID2_IRQ,
    TPS65218_LS1_I_IRQ, TPS65218_LS2_I_IRQ, TPS65218_LS3_I_IRQ, TPS65218_LS1_F_IRQ,
    TPS65218_LS2_F_IRQ, TPS65218_LS3_F_IRQ, TPS65218_INVALID3_IRQ, TPS65218_INVALID4_IRQ,
}

/**
 * struct tps65218 - tps65218 sub-driver chip access routines
 *
 * Device data may be used to access the TPS65218 chip
 */
#[repr(C)]
pub struct tps65218 {
    pub dev: *mut device,
    pub id: u32,
    pub rev: u8,
    pub tps_lock: mutex,
    pub irq: i32,
    pub irq_mask: u32,
    pub irq_data: *mut regmap_irq_chip_data,
    pub desc: [regulator_desc; TPS65218_NUM_REGULATOR],
    pub regmap: *mut regmap,
    pub strobes: *mut u8,
}

pub unsafe extern "C" fn tps65218_reg_write(tps: *mut tps65218, reg: u32, val: u32, level: u32) -> i32;
pub unsafe extern "C" fn tps65218_set_bits(tps: *mut tps65218, reg: u32, mask: u32, val: u32, level: u32) -> i32;
pub unsafe extern "C" fn tps65218_clear_bits(tps: *mut tps65218, reg: u32, mask: u32, level: u32) -> i32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
