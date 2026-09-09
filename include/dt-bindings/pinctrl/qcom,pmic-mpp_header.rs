/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for the Qualcomm PMIC's
 * Multi-Purpose Pin binding.
 */

/* power-source */

/* Digital Input/Output: level [PM8058] */
pub const PM8058_MPP_VPH: i32 = 0;
pub const PM8058_MPP_S3: i32 = 1;
pub const PM8058_MPP_L2: i32 = 2;
pub const PM8058_MPP_L3: i32 = 3;

/* Digital Input/Output: level [PM8901] */
pub const PM8901_MPP_MSMIO: i32 = 0;
pub const PM8901_MPP_DIG: i32 = 1;
pub const PM8901_MPP_L5: i32 = 2;
pub const PM8901_MPP_S4: i32 = 3;
pub const PM8901_MPP_VPH: i32 = 4;

/* Digital Input/Output: level [PM8921] */
pub const PM8921_MPP_S4: i32 = 1;
pub const PM8921_MPP_L15: i32 = 3;
pub const PM8921_MPP_L17: i32 = 4;
pub const PM8921_MPP_VPH: i32 = 7;

/* Digital Input/Output: level [PM8821] */
pub const PM8821_MPP_1P8: i32 = 0;
pub const PM8821_MPP_VPH: i32 = 7;

/* Digital Input/Output: level [PM8018] */
pub const PM8018_MPP_L4: i32 = 0;
pub const PM8018_MPP_L14: i32 = 1;
pub const PM8018_MPP_S3: i32 = 2;
pub const PM8018_MPP_L6: i32 = 3;
pub const PM8018_MPP_L2: i32 = 4;
pub const PM8018_MPP_L5: i32 = 5;
pub const PM8018_MPP_VPH: i32 = 7;

/* Digital Input/Output: level [PM8038] */
pub const PM8038_MPP_L20: i32 = 0;
pub const PM8038_MPP_L11: i32 = 1;
pub const PM8038_MPP_L5: i32 = 2;
pub const PM8038_MPP_L15: i32 = 3;
pub const PM8038_MPP_L17: i32 = 4;
pub const PM8038_MPP_VPH: i32 = 7;

pub const PM8841_MPP_VPH: i32 = 0;
pub const PM8841_MPP_S3: i32 = 2;

pub const PM8916_MPP_VPH: i32 = 0;
pub const PM8916_MPP_L2: i32 = 2;
pub const PM8916_MPP_L5: i32 = 3;

pub const PM8941_MPP_VPH: i32 = 0;
pub const PM8941_MPP_L1: i32 = 1;
pub const PM8941_MPP_S3: i32 = 2;
pub const PM8941_MPP_L6: i32 = 3;

pub const PMA8084_MPP_VPH: i32 = 0;
pub const PMA8084_MPP_L1: i32 = 1;
pub const PMA8084_MPP_S4: i32 = 2;
pub const PMA8084_MPP_L6: i32 = 3;

pub const PM8994_MPP_VPH: i32 = 0;
/* Only supported for MPP_05-MPP_08 */
pub const PM8994_MPP_L19: i32 = 1;
pub const PM8994_MPP_S4: i32 = 2;
pub const PM8994_MPP_L12: i32 = 3;

/*
 * Analog Input - Set the source for analog input.
 * To be used with "qcom,amux-route" property
 */
pub const PMIC_MPP_AMUX_ROUTE_CH5: i32 = 0;
pub const PMIC_MPP_AMUX_ROUTE_CH6: i32 = 1;
pub const PMIC_MPP_AMUX_ROUTE_CH7: i32 = 2;
pub const PMIC_MPP_AMUX_ROUTE_CH8: i32 = 3;
pub const PMIC_MPP_AMUX_ROUTE_ABUS1: i32 = 4;
pub const PMIC_MPP_AMUX_ROUTE_ABUS2: i32 = 5;
pub const PMIC_MPP_AMUX_ROUTE_ABUS3: i32 = 6;
pub const PMIC_MPP_AMUX_ROUTE_ABUS4: i32 = 7;

/* Analog Output: level */
pub const PMIC_MPP_AOUT_LVL_1V25: i32 = 0;
pub const PMIC_MPP_AOUT_LVL_1V25_2: i32 = 1;
pub const PMIC_MPP_AOUT_LVL_0V625: i32 = 2;
pub const PMIC_MPP_AOUT_LVL_0V3125: i32 = 3;
pub const PMIC_MPP_AOUT_LVL_MPP: i32 = 4;
pub const PMIC_MPP_AOUT_LVL_ABUS1: i32 = 5;
pub const PMIC_MPP_AOUT_LVL_ABUS2: i32 = 6;
pub const PMIC_MPP_AOUT_LVL_ABUS3: i32 = 7;

/* To be used with "function" */
pub const PMIC_MPP_FUNC_NORMAL: &str = "normal";
pub const PMIC_MPP_FUNC_PAIRED: &str = "paired";
pub const PMIC_MPP_FUNC_DTEST1: &str = "dtest1";
pub const PMIC_MPP_FUNC_DTEST2: &str = "dtest2";
pub const PMIC_MPP_FUNC_DTEST3: &str = "dtest3";
pub const PMIC_MPP_FUNC_DTEST4: &str = "dtest4";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
