/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for the Qualcomm PMIC GPIO binding.
 */

pub const PMIC_GPIO_PULL_UP_30: i32 = 0;
pub const PMIC_GPIO_PULL_UP_1P5: i32 = 1;
pub const PMIC_GPIO_PULL_UP_31P5: i32 = 2;
pub const PMIC_GPIO_PULL_UP_1P5_30: i32 = 3;

pub const PMIC_GPIO_STRENGTH_NO: i32 = 0;
pub const PMIC_GPIO_STRENGTH_HIGH: i32 = 1;
pub const PMIC_GPIO_STRENGTH_MED: i32 = 2;
pub const PMIC_GPIO_STRENGTH_LOW: i32 = 3;

/*
 * Note: PM8018 GPIO3 and GPIO4 are supporting
 * only S3 and L2 options (1.8V)
 */
pub const PM8018_GPIO_L6: i32 = 0;
pub const PM8018_GPIO_L5: i32 = 1;
pub const PM8018_GPIO_S3: i32 = 2;
pub const PM8018_GPIO_L14: i32 = 3;
pub const PM8018_GPIO_L2: i32 = 4;
pub const PM8018_GPIO_L4: i32 = 5;
pub const PM8018_GPIO_VDD: i32 = 6;

/*
 * Note: PM8038 GPIO7 and GPIO8 are supporting
 * only L11 and L4 options (1.8V)
 */
pub const PM8038_GPIO_VPH: i32 = 0;
pub const PM8038_GPIO_BB: i32 = 1;
pub const PM8038_GPIO_L11: i32 = 2;
pub const PM8038_GPIO_L15: i32 = 3;
pub const PM8038_GPIO_L4: i32 = 4;
pub const PM8038_GPIO_L3: i32 = 5;
pub const PM8038_GPIO_L17: i32 = 6;

pub const PM8058_GPIO_VPH: i32 = 0;
pub const PM8058_GPIO_BB: i32 = 1;
pub const PM8058_GPIO_S3: i32 = 2;
pub const PM8058_GPIO_L3: i32 = 3;
pub const PM8058_GPIO_L7: i32 = 4;
pub const PM8058_GPIO_L6: i32 = 5;
pub const PM8058_GPIO_L5: i32 = 6;
pub const PM8058_GPIO_L2: i32 = 7;

/*
 * Note: PM8916 GPIO1 and GPIO2 are supporting
 * only L2(1.15V) and L5(1.8V) options
 */
pub const PM8916_GPIO_VPH: i32 = 0;
pub const PM8916_GPIO_L2: i32 = 2;
pub const PM8916_GPIO_L5: i32 = 3;

pub const PM8917_GPIO_VPH: i32 = 0;
pub const PM8917_GPIO_S4: i32 = 2;
pub const PM8917_GPIO_L15: i32 = 3;
pub const PM8917_GPIO_L4: i32 = 4;
pub const PM8917_GPIO_L3: i32 = 5;
pub const PM8917_GPIO_L17: i32 = 6;

pub const PM8921_GPIO_VPH: i32 = 0;
pub const PM8921_GPIO_BB: i32 = 1;
pub const PM8921_GPIO_S4: i32 = 2;
pub const PM8921_GPIO_L15: i32 = 3;
pub const PM8921_GPIO_L4: i32 = 4;
pub const PM8921_GPIO_L3: i32 = 5;
pub const PM8921_GPIO_L17: i32 = 6;

/*
 * Note: PM8941 gpios from 15 to 18 are supporting
 * only S3 and L6 options (1.8V)
 */
pub const PM8941_GPIO_VPH: i32 = 0;
pub const PM8941_GPIO_L1: i32 = 1;
pub const PM8941_GPIO_S3: i32 = 2;
pub const PM8941_GPIO_L6: i32 = 3;

/*
 * Note: PMA8084 gpios from 15 to 18 are supporting
 * only S4 and L6 options (1.8V)
 */
pub const PMA8084_GPIO_VPH: i32 = 0;
pub const PMA8084_GPIO_L1: i32 = 1;
pub const PMA8084_GPIO_S4: i32 = 2;
pub const PMA8084_GPIO_L6: i32 = 3;

pub const PM8994_GPIO_VPH: i32 = 0;
pub const PM8994_GPIO_S4: i32 = 2;
pub const PM8994_GPIO_L12: i32 = 3;

/* To be used with "function" */
pub const PMIC_GPIO_FUNC_NORMAL: &str = "normal";
pub const PMIC_GPIO_FUNC_PAIRED: &str = "paired";
pub const PMIC_GPIO_FUNC_FUNC1: &str = "func1";
pub const PMIC_GPIO_FUNC_FUNC2: &str = "func2";
pub const PMIC_GPIO_FUNC_FUNC3: &str = "func3";
pub const PMIC_GPIO_FUNC_FUNC4: &str = "func4";
pub const PMIC_GPIO_FUNC_DTEST1: &str = "dtest1";
pub const PMIC_GPIO_FUNC_DTEST2: &str = "dtest2";
pub const PMIC_GPIO_FUNC_DTEST3: &str = "dtest3";
pub const PMIC_GPIO_FUNC_DTEST4: &str = "dtest4";

pub const PM8038_GPIO1_2_LPG_DRV: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8038_GPIO3_5V_BOOST_EN: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8038_GPIO4_SSBI_ALT_CLK: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8038_GPIO5_6_EXT_REG_EN: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8038_GPIO10_11_EXT_REG_EN: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8038_GPIO6_7_CLK: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8038_GPIO9_BAT_ALRM_OUT: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8038_GPIO6_12_KYPD_DRV: &str = PMIC_GPIO_FUNC_FUNC2;

pub const PM8058_GPIO7_8_MP3_CLK: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8058_GPIO7_8_BCLK_19P2MHZ: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PM8058_GPIO9_26_KYPD_DRV: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8058_GPIO21_23_UART_TX: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PM8058_GPIO24_26_LPG_DRV: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PM8058_GPIO33_BCLK_19P2MHZ: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8058_GPIO34_35_MP3_CLK: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8058_GPIO36_BCLK_19P2MHZ: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8058_GPIO37_UPL_OUT: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8058_GPIO37_UART_M_RX: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PM8058_GPIO38_XO_SLEEP_CLK: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8058_GPIO38_39_CLK_32KHZ: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PM8058_GPIO39_MP3_CLK: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8058_GPIO40_EXT_BB_EN: &str = PMIC_GPIO_FUNC_FUNC1;

pub const PM8916_GPIO1_BAT_ALRM_OUT: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8916_GPIO1_KEYP_DRV: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PM8916_GPIO2_DIV_CLK: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8916_GPIO2_SLEEP_CLK: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PM8916_GPIO3_KEYP_DRV: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8916_GPIO4_KEYP_DRV: &str = PMIC_GPIO_FUNC_FUNC2;

pub const PM8917_GPIO9_18_KEYP_DRV: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8917_GPIO20_BAT_ALRM_OUT: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8917_GPIO21_23_UART_TX: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PM8917_GPIO25_26_EXT_REG_EN: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8917_GPIO37_38_XO_SLEEP_CLK: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8917_GPIO37_38_MP3_CLK: &str = PMIC_GPIO_FUNC_FUNC2;

pub const PM8941_GPIO9_14_KYPD_DRV: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8941_GPIO15_18_DIV_CLK: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8941_GPIO15_18_SLEEP_CLK: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PM8941_GPIO23_26_KYPD_DRV: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8941_GPIO23_26_LPG_DRV_HI: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PM8941_GPIO31_BAT_ALRM_OUT: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8941_GPIO33_36_LPG_DRV_3D: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PM8941_GPIO33_36_LPG_DRV_HI: &str = PMIC_GPIO_FUNC_FUNC2;

pub const PMA8084_GPIO4_5_LPG_DRV: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PMA8084_GPIO7_10_LPG_DRV: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PMA8084_GPIO5_14_KEYP_DRV: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PMA8084_GPIO19_21_KEYP_DRV: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PMA8084_GPIO15_18_DIV_CLK: &str = PMIC_GPIO_FUNC_FUNC1;
pub const PMA8084_GPIO15_18_SLEEP_CLK: &str = PMIC_GPIO_FUNC_FUNC2;
pub const PMA8084_GPIO22_BAT_ALRM_OUT: &str = PMIC_GPIO_FUNC_FUNC1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
