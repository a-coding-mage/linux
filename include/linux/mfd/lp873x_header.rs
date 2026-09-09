/* SPDX-License-Identifier: GPL-2.0-only */
/* Functions to access LP873X power management chip. */

// Dependencies supplied by the surrounding kernel translation.

/* LP873x chip id list */
pub const LP873X: u8 = 0x00;

/* All register addresses */
pub const LP873X_REG_DEV_REV: u8 = 0x00;
pub const LP873X_REG_OTP_REV: u8 = 0x01;
pub const LP873X_REG_BUCK0_CTRL_1: u8 = 0x02;
pub const LP873X_REG_BUCK0_CTRL_2: u8 = 0x03;
pub const LP873X_REG_BUCK1_CTRL_1: u8 = 0x04;
pub const LP873X_REG_BUCK1_CTRL_2: u8 = 0x05;
pub const LP873X_REG_BUCK0_VOUT: u8 = 0x06;
pub const LP873X_REG_BUCK1_VOUT: u8 = 0x07;
pub const LP873X_REG_LDO0_CTRL: u8 = 0x08;
pub const LP873X_REG_LDO1_CTRL: u8 = 0x09;
pub const LP873X_REG_LDO0_VOUT: u8 = 0x0a;
pub const LP873X_REG_LDO1_VOUT: u8 = 0x0b;
pub const LP873X_REG_BUCK0_DELAY: u8 = 0x0c;
pub const LP873X_REG_BUCK1_DELAY: u8 = 0x0d;
pub const LP873X_REG_LDO0_DELAY: u8 = 0x0e;
pub const LP873X_REG_LDO1_DELAY: u8 = 0x0f;
pub const LP873X_REG_GPO_DELAY: u8 = 0x10;
pub const LP873X_REG_GPO2_DELAY: u8 = 0x11;
pub const LP873X_REG_GPO_CTRL: u8 = 0x12;
pub const LP873X_REG_CONFIG: u8 = 0x13;
pub const LP873X_REG_PLL_CTRL: u8 = 0x14;
pub const LP873X_REG_PGOOD_CTRL1: u8 = 0x15;
pub const LP873X_REG_PGOOD_CTRL2: u8 = 0x16;
pub const LP873X_REG_PG_FAULT: u8 = 0x17;
pub const LP873X_REG_RESET: u8 = 0x18;
pub const LP873X_REG_INT_TOP_1: u8 = 0x19;
pub const LP873X_REG_INT_TOP_2: u8 = 0x1a;
pub const LP873X_REG_INT_BUCK: u8 = 0x1b;
pub const LP873X_REG_INT_LDO: u8 = 0x1c;
pub const LP873X_REG_TOP_STAT: u8 = 0x1d;
pub const LP873X_REG_BUCK_STAT: u8 = 0x1e;
pub const LP873X_REG_LDO_STAT: u8 = 0x1f;
pub const LP873X_REG_TOP_MASK_1: u8 = 0x20;
pub const LP873X_REG_TOP_MASK_2: u8 = 0x21;
pub const LP873X_REG_BUCK_MASK: u8 = 0x22;
pub const LP873X_REG_LDO_MASK: u8 = 0x23;
pub const LP873X_REG_SEL_I_LOAD: u8 = 0x24;
pub const LP873X_REG_I_LOAD_2: u8 = 0x25;
pub const LP873X_REG_I_LOAD_1: u8 = 0x26;
pub const LP873X_REG_MAX: u8 = LP873X_REG_I_LOAD_1;

/* Register field definitions */
pub const LP873X_DEV_REV_DEV_ID: u8 = 0xc0;
pub const LP873X_DEV_REV_ALL_LAYER: u8 = 0x30;
pub const LP873X_DEV_REV_METAL_LAYER: u8 = 0x0f;
pub const LP873X_OTP_REV_OTP_ID: u8 = 0xff;

macro_rules! bit { ($n:expr) => { 1u8 << $n }; }

pub const LP873X_BUCK0_CTRL_1_BUCK0_FPWM: u8 = bit!(3);
pub const LP873X_BUCK0_CTRL_1_BUCK0_RDIS_EN: u8 = bit!(2);
pub const LP873X_BUCK0_CTRL_1_BUCK0_EN_PIN_CTRL: u8 = bit!(1);
pub const LP873X_BUCK0_CTRL_1_BUCK0_EN: u8 = bit!(0);
pub const LP873X_BUCK0_CTRL_2_BUCK0_ILIM: u8 = 0x38;
pub const LP873X_BUCK0_CTRL_2_BUCK0_SLEW_RATE: u8 = 0x07;
pub const LP873X_BUCK1_CTRL_1_BUCK1_FPWM: u8 = bit!(3);
pub const LP873X_BUCK1_CTRL_1_BUCK1_RDIS_EN: u8 = bit!(2);
pub const LP873X_BUCK1_CTRL_1_BUCK1_EN_PIN_CTRL: u8 = bit!(1);
pub const LP873X_BUCK1_CTRL_1_BUCK1_EN: u8 = bit!(0);
pub const LP873X_BUCK1_CTRL_2_BUCK1_ILIM: u8 = 0x38;
pub const LP873X_BUCK1_CTRL_2_BUCK1_SLEW_RATE: u8 = 0x07;
pub const LP873X_BUCK0_VOUT_BUCK0_VSET: u8 = 0xff;
pub const LP873X_BUCK1_VOUT_BUCK1_VSET: u8 = 0xff;
pub const LP873X_LDO0_CTRL_LDO0_RDIS_EN: u8 = bit!(2);
pub const LP873X_LDO0_CTRL_LDO0_EN_PIN_CTRL: u8 = bit!(1);
pub const LP873X_LDO0_CTRL_LDO0_EN: u8 = bit!(0);
pub const LP873X_LDO1_CTRL_LDO1_RDIS_EN: u8 = bit!(2);
pub const LP873X_LDO1_CTRL_LDO1_EN_PIN_CTRL: u8 = bit!(1);
pub const LP873X_LDO1_CTRL_LDO1_EN: u8 = bit!(0);
pub const LP873X_LDO0_VOUT_LDO0_VSET: u8 = 0x1f;
pub const LP873X_LDO1_VOUT_LDO1_VSET: u8 = 0x1f;

/* Delay fields */
pub const LP873X_BUCK0_DELAY_BUCK0_SD_DELAY: u8 = 0xf0;
pub const LP873X_BUCK0_DELAY_BUCK0_SU_DELAY: u8 = 0x0f;
pub const LP873X_BUCK1_DELAY_BUCK1_SD_DELAY: u8 = 0xf0;
pub const LP873X_BUCK1_DELAY_BUCK1_SU_DELAY: u8 = 0x0f;
pub const LP873X_LDO0_DELAY_LDO0_SD_DELAY: u8 = 0xf0;
pub const LP873X_LDO0_DELAY_LDO0_SU_DELAY: u8 = 0x0f;
pub const LP873X_LDO1_DELAY_LDO1_SD_DELAY: u8 = 0xf0;
pub const LP873X_LDO1_DELAY_LDO1_SU_DELAY: u8 = 0x0f;
pub const LP873X_GPO_DELAY_GPO_SD_DELAY: u8 = 0xf0;
pub const LP873X_GPO_DELAY_GPO_SU_DELAY: u8 = 0x0f;
pub const LP873X_GPO2_DELAY_GPO2_SD_DELAY: u8 = 0xf0;
pub const LP873X_GPO2_DELAY_GPO2_SU_DELAY: u8 = 0x0f;

/* Remaining register fields, expressed as direct bit masks. */
pub const LP873X_GPO_CTRL_GPO2_OD: u8 = bit!(6);
pub const LP873X_GPO_CTRL_GPO2_EN_PIN_CTRL: u8 = bit!(5);
pub const LP873X_GPO_CTRL_GPO2_EN: u8 = bit!(4);
pub const LP873X_GPO_CTRL_GPO_OD: u8 = bit!(2);
pub const LP873X_GPO_CTRL_GPO_EN_PIN_CTRL: u8 = bit!(1);
pub const LP873X_GPO_CTRL_GPO_EN: u8 = bit!(0);
pub const LP873X_CONFIG_SU_DELAY_SEL: u8 = bit!(6);
pub const LP873X_CONFIG_SD_DELAY_SEL: u8 = bit!(5);
pub const LP873X_CONFIG_CLKIN_PIN_SEL: u8 = bit!(4);
pub const LP873X_CONFIG_CLKIN_PD: u8 = bit!(3);
pub const LP873X_CONFIG_EN_PD: u8 = bit!(2);
pub const LP873X_CONFIG_TDIE_WARN_LEVEL: u8 = bit!(1);
pub const LP873X_EN_SPREAD_SPEC: u8 = bit!(0);
pub const LP873X_PLL_CTRL_EN_PLL: u8 = bit!(6);
pub const LP873X_EXT_CLK_FREQ: u8 = 0x1f;

pub const LP873X_PGOOD_CTRL1_PGOOD_POL: u8 = bit!(7);
pub const LP873X_PGOOD_CTRL1_PGOOD_OD: u8 = bit!(6);
pub const LP873X_PGOOD_CTRL1_PGOOD_WINDOW_LDO: u8 = bit!(5);
pub const LP873X_PGOOD_CTRL1_PGOOD_WINDOWN_BUCK: u8 = bit!(4);
pub const LP873X_PGOOD_CTRL1_PGOOD_EN_PGOOD_LDO1: u8 = bit!(3);
pub const LP873X_PGOOD_CTRL1_PGOOD_EN_PGOOD_LDO0: u8 = bit!(2);
pub const LP873X_PGOOD_CTRL1_PGOOD_EN_PGOOD_BUCK1: u8 = bit!(1);
pub const LP873X_PGOOD_CTRL1_PGOOD_EN_PGOOD_BUCK0: u8 = bit!(0);
pub const LP873X_PGOOD_CTRL2_EN_PGOOD_TWARN: u8 = bit!(2);
pub const LP873X_PGOOD_CTRL2_EN_PG_FAULT_GATE: u8 = bit!(1);
pub const LP873X_PGOOD_CTRL2_PGOOD_MODE: u8 = bit!(0);
pub const LP873X_PG_FAULT_PG_FAULT_LDO1: u8 = bit!(3);
pub const LP873X_PG_FAULT_PG_FAULT_LDO0: u8 = bit!(2);
pub const LP873X_PG_FAULT_PG_FAULT_BUCK1: u8 = bit!(1);
pub const LP873X_PG_FAULT_PG_FAULT_BUCK0: u8 = bit!(0);
pub const LP873X_RESET_SW_RESET: u8 = bit!(0);

pub const LP873X_INT_TOP_1_PGOOD_INT: u8 = bit!(7);
pub const LP873X_INT_TOP_1_LDO_INT: u8 = bit!(6);
pub const LP873X_INT_TOP_1_BUCK_INT: u8 = bit!(5);
pub const LP873X_INT_TOP_1_SYNC_CLK_INT: u8 = bit!(4);
pub const LP873X_INT_TOP_1_TDIE_SD_INT: u8 = bit!(3);
pub const LP873X_INT_TOP_1_TDIE_WARN_INT: u8 = bit!(2);
pub const LP873X_INT_TOP_1_OVP_INT: u8 = bit!(1);
pub const LP873X_INT_TOP_1_I_MEAS_INT: u8 = bit!(0);
pub const LP873X_INT_TOP_2_RESET_REG_INT: u8 = bit!(0);
pub const LP873X_INT_BUCK_BUCK1_PG_INT: u8 = bit!(6);
pub const LP873X_INT_BUCK_BUCK1_SC_INT: u8 = bit!(5);
pub const LP873X_INT_BUCK_BUCK1_ILIM_INT: u8 = bit!(4);
pub const LP873X_INT_BUCK_BUCK0_PG_INT: u8 = bit!(2);
pub const LP873X_INT_BUCK_BUCK0_SC_INT: u8 = bit!(1);
pub const LP873X_INT_BUCK_BUCK0_ILIM_INT: u8 = bit!(0);
pub const LP873X_INT_LDO_LDO1_PG_INT: u8 = bit!(6);
pub const LP873X_INT_LDO_LDO1_SC_INT: u8 = bit!(5);
pub const LP873X_INT_LDO_LDO1_ILIM_INT: u8 = bit!(4);
pub const LP873X_INT_LDO_LDO0_PG_INT: u8 = bit!(2);
pub const LP873X_INT_LDO_LDO0_SC_INT: u8 = bit!(1);
pub const LP873X_INT_LDO_LDO0_ILIM_INT: u8 = bit!(0);

pub const LP873X_TOP_STAT_PGOOD_STAT: u8 = bit!(7);
pub const LP873X_TOP_STAT_SYNC_CLK_STAT: u8 = bit!(4);
pub const LP873X_TOP_STAT_TDIE_SD_STAT: u8 = bit!(3);
pub const LP873X_TOP_STAT_TDIE_WARN_STAT: u8 = bit!(2);
pub const LP873X_TOP_STAT_OVP_STAT: u8 = bit!(1);
pub const LP873X_BUCK_STAT_BUCK1_STAT: u8 = bit!(7);
pub const LP873X_BUCK_STAT_BUCK1_PG_STAT: u8 = bit!(6);
pub const LP873X_BUCK_STAT_BUCK1_ILIM_STAT: u8 = bit!(4);
pub const LP873X_BUCK_STAT_BUCK0_STAT: u8 = bit!(3);
pub const LP873X_BUCK_STAT_BUCK0_PG_STAT: u8 = bit!(2);
pub const LP873X_BUCK_STAT_BUCK0_ILIM_STAT: u8 = bit!(0);
pub const LP873X_LDO_STAT_LDO1_STAT: u8 = bit!(7);
pub const LP873X_LDO_STAT_LDO1_PG_STAT: u8 = bit!(6);
pub const LP873X_LDO_STAT_LDO1_ILIM_STAT: u8 = bit!(4);
pub const LP873X_LDO_STAT_LDO0_STAT: u8 = bit!(3);
pub const LP873X_LDO_STAT_LDO0_PG_STAT: u8 = bit!(2);
pub const LP873X_LDO_STAT_LDO0_ILIM_STAT: u8 = bit!(0);

pub const LP873X_TOP_MASK_1_PGOOD_INT_MASK: u8 = bit!(7);
pub const LP873X_TOP_MASK_1_SYNC_CLK_MASK: u8 = bit!(4);
pub const LP873X_TOP_MASK_1_TDIE_WARN_MASK: u8 = bit!(2);
pub const LP873X_TOP_MASK_1_I_MEAS_MASK: u8 = bit!(0);
pub const LP873X_TOP_MASK_2_RESET_REG_MASK: u8 = bit!(0);
pub const LP873X_BUCK_MASK_BUCK1_PGF_MASK: u8 = bit!(7);
pub const LP873X_BUCK_MASK_BUCK1_PGR_MASK: u8 = bit!(6);
pub const LP873X_BUCK_MASK_BUCK1_ILIM_MASK: u8 = bit!(4);
pub const LP873X_BUCK_MASK_BUCK0_PGF_MASK: u8 = bit!(3);
pub const LP873X_BUCK_MASK_BUCK0_PGR_MASK: u8 = bit!(2);
pub const LP873X_BUCK_MASK_BUCK0_ILIM_MASK: u8 = bit!(0);
pub const LP873X_LDO_MASK_LDO1_PGF_MASK: u8 = bit!(7);
pub const LP873X_LDO_MASK_LDO1_PGR_MASK: u8 = bit!(6);
pub const LP873X_LDO_MASK_LDO1_ILIM_MASK: u8 = bit!(4);
pub const LP873X_LDO_MASK_LDO0_PGF_MASK: u8 = bit!(3);
pub const LP873X_LDO_MASK_LDO0_PGR_MASK: u8 = bit!(2);
pub const LP873X_LDO_MASK_LDO0_ILIM_MASK: u8 = bit!(0);
pub const LP873X_SEL_I_LOAD_CURRENT_BUCK_SELECT: u8 = bit!(0);
pub const LP873X_I_LOAD_2_BUCK_LOAD_CURRENT: u8 = bit!(0);
pub const LP873X_I_LOAD_1_BUCK_LOAD_CURRENT: u8 = 0xff;

/* Number of step-down converters available */
pub const LP873X_NUM_BUCK: usize = 2;
/* Number of LDO voltage regulators available */
pub const LP873X_NUM_LDO: usize = 2;
/* Number of total regulators available */
pub const LP873X_NUM_REGULATOR: usize = LP873X_NUM_BUCK + LP873X_NUM_LDO;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum lp873x_regulator_id {
    /* BUCK's */
    LP873X_BUCK_0,
    LP873X_BUCK_1,
    /* LDOs */
    LP873X_LDO_0,
    LP873X_LDO_1,
}

pub const LP873X_MAX_REG_ID: lp873x_regulator_id = lp873x_regulator_id::LP873X_LDO_1;

/**
 * struct lp873x - state holder for the lp873x driver
 * @dev: struct device pointer for MFD device
 * @rev: revision of the lp873x
 * @lock: lock guarding the data structure
 * @regmap: register map of the lp873x PMIC
 *
 * Device data may be used to access the LP873X chip
 */
#[repr(C)]
pub struct lp873x {
    pub dev: *mut device,
    pub rev: u8,
    pub regmap: *mut regmap,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
