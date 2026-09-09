/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Register definitions for TI BQ257XX
 * Copyright (C) 2020 Texas Instruments Incorporated - http://www.ti.com/
 */

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(high: u32, low: u32) -> u32 {
    ((1u32 << (high - low + 1)) - 1) << low
}
const fn field_prep(mask: u32, value: u32) -> u32 {
    (value << mask.trailing_zeros()) & mask
}

/* SPDX-License-Identifier: GPL-2.0 */
/*

pub const BQ25703_CHARGE_OPTION_0: u32 = 0x00u32;
pub const BQ25703_CHARGE_CURRENT: u32 = 0x02u32;
pub const BQ25703_MAX_CHARGE_VOLT: u32 = 0x04u32;
pub const BQ25703_OTG_VOLT: u32 = 0x06u32;
pub const BQ25703_OTG_CURRENT: u32 = 0x08u32;
pub const BQ25703_INPUT_VOLTAGE: u32 = 0x0au32;
pub const BQ25703_MIN_VSYS: u32 = 0x0cu32;
pub const BQ25703_IIN_HOST: u32 = 0x0eu32;
pub const BQ25703_CHARGER_STATUS: u32 = 0x20u32;
pub const BQ25703_PROCHOT_STATUS: u32 = 0x22u32;
pub const BQ25703_IIN_DPM: u32 = 0x24u32;
pub const BQ25703_ADCIBAT_CHG: u32 = 0x28u32;
pub const BQ25703_ADCIINCMPIN: u32 = 0x2au32;
pub const BQ25703_ADCVSYSVBAT: u32 = 0x2cu32;
pub const BQ25703_MANUFACT_DEV_ID: u32 = 0x2eu32;
pub const BQ25703_CHARGE_OPTION_1: u32 = 0x30u32;
pub const BQ25703_CHARGE_OPTION_2: u32 = 0x32u32;
pub const BQ25703_CHARGE_OPTION_3: u32 = 0x34u32;
pub const BQ25703_ADC_OPTION: u32 = 0x3au32;

pub const BQ25703_EN_LWPWR: u32 = bit(15u32);
pub const BQ25703_WDTMR_ADJ_MASK: u32 = genmask(14u32, 13u32);
pub const BQ25703_WDTMR_DISABLE: u32 = 0u32;
pub const BQ25703_WDTMR_5_SEC: u32 = 1u32;
pub const BQ25703_WDTMR_88_SEC: u32 = 2u32;
pub const BQ25703_WDTMR_175_SEC: u32 = 3u32;

pub const BQ25703_ICHG_MASK: u32 = genmask(12u32, 6u32);
pub const BQ25703_ICHG_STEP_UA: u32 = 64000u32;
pub const BQ25703_ICHG_MIN_UA: u32 = 64000u32;
pub const BQ25703_ICHG_MAX_UA: u32 = 8128000u32;

pub const BQ25703_MAX_CHARGE_VOLT_MASK: u32 = genmask(15u32, 4u32);
pub const BQ25703_VBATREG_STEP_UV: u32 = 16000u32;
pub const BQ25703_VBATREG_MIN_UV: u32 = 1024000u32;
pub const BQ25703_VBATREG_MAX_UV: u32 = 19200000u32;

pub const BQ25703_OTG_VOLT_MASK: u32 = genmask(13u32, 6u32);
pub const BQ25703_OTG_VOLT_STEP_UV: u32 = 64000u32;
pub const BQ25703_OTG_VOLT_MIN_UV: u32 = 4480000u32;
pub const BQ25703_OTG_VOLT_MAX_UV: u32 = 20800000u32;
pub const BQ25703_OTG_VOLT_NUM_VOLT: u32 = 256u32;

pub const BQ25703_OTG_CUR_MASK: u32 = genmask(14u32, 8u32);
pub const BQ25703_OTG_CUR_STEP_UA: u32 = 50000u32;
pub const BQ25703_OTG_CUR_MAX_UA: u32 = 6350000u32;

pub const BQ25703_MINVSYS_MASK: u32 = genmask(13u32, 8u32);
pub const BQ25703_MINVSYS_STEP_UV: u32 = 256000u32;
pub const BQ25703_MINVSYS_MIN_UV: u32 = 1024000u32;
pub const BQ25703_MINVSYS_MAX_UV: u32 = 16128000u32;

pub const BQ25703_STS_AC_STAT: u32 = bit(15u32);
pub const BQ25703_STS_IN_FCHRG: u32 = bit(10u32);
pub const BQ25703_STS_IN_PCHRG: u32 = bit(9u32);
pub const BQ25703_STS_FAULT_ACOV: u32 = bit(7u32);
pub const BQ25703_STS_FAULT_BATOC: u32 = bit(6u32);
pub const BQ25703_STS_FAULT_ACOC: u32 = bit(5u32);

pub const BQ25703_IINDPM_MASK: u32 = genmask(14u32, 8u32);
pub const BQ25703_IINDPM_STEP_UA: u32 = 50000u32;
pub const BQ25703_IINDPM_MIN_UA: u32 = 50000u32;
pub const BQ25703_IINDPM_MAX_UA: u32 = 6400000u32;
pub const BQ25703_IINDPM_DEFAULT_UA: u32 = 3300000u32;
pub const BQ25703_IINDPM_OFFSET_UA: u32 = 50000u32;

pub const BQ25703_ADCIBAT_DISCHG_MASK: u32 = genmask(6u32, 0u32);
pub const BQ25703_ADCIBAT_CHG_MASK: u32 = genmask(14u32, 8u32);
pub const BQ25703_ADCIBAT_CHG_STEP_UA: u32 = 64000u32;
pub const BQ25703_ADCIBAT_DIS_STEP_UA: u32 = 256000u32;

pub const BQ25703_ADCIIN: u32 = genmask(15u32, 8u32);
pub const BQ25703_ADCIINCMPIN_STEP: u32 = 50000u32;

pub const BQ25703_ADCVSYS_MASK: u32 = genmask(15u32, 8u32);
pub const BQ25703_ADCVBAT_MASK: u32 = genmask(7u32, 0u32);
pub const BQ25703_ADCVSYSVBAT_OFFSET_UV: u32 = 2880000u32;
pub const BQ25703_ADCVSYSVBAT_STEP: u32 = 64000u32;

pub const BQ25703_ADC_CH_MASK: u32 = genmask(7u32, 0u32);
pub const BQ25703_ADC_CONV_EN: u32 = bit(15u32);
pub const BQ25703_ADC_START: u32 = bit(14u32);
pub const BQ25703_ADC_FULL_SCALE: u32 = bit(13u32);
pub const BQ25703_ADC_CMPIN_EN: u32 = bit(7u32);
pub const BQ25703_ADC_VBUS_EN: u32 = bit(6u32);
pub const BQ25703_ADC_PSYS_EN: u32 = bit(5u32);
pub const BQ25703_ADC_IIN_EN: u32 = bit(4u32);
pub const BQ25703_ADC_IDCHG_EN: u32 = bit(3u32);
pub const BQ25703_ADC_ICHG_EN: u32 = bit(2u32);
pub const BQ25703_ADC_VSYS_EN: u32 = bit(1u32);
pub const BQ25703_ADC_VBAT_EN: u32 = bit(0u32);

pub const BQ25703_EN_OTG_MASK: u32 = bit(12u32);

pub const BQ25792_REG00_MIN_SYS_VOLTAGE: u32 = 0x00u32;
pub const BQ25792_REG01_CHARGE_VOLTAGE_LIMIT: u32 = 0x01u32;
pub const BQ25792_REG03_CHARGE_CURRENT_LIMIT: u32 = 0x03u32;
pub const BQ25792_REG05_INPUT_VOLTAGE_LIMIT: u32 = 0x05u32;
pub const BQ25792_REG06_INPUT_CURRENT_LIMIT: u32 = 0x06u32;
pub const BQ25792_REG08_PRECHARGE_CONTROL: u32 = 0x08u32;
pub const BQ25792_REG09_TERMINATION_CONTROL: u32 = 0x09u32;
pub const BQ25792_REG0A_RECHARGE_CONTROL: u32 = 0x0au32;
pub const BQ25792_REG0B_VOTG_REGULATION: u32 = 0x0bu32;
pub const BQ25792_REG0D_IOTG_REGULATION: u32 = 0x0du32;
pub const BQ25792_REG0E_TIMER_CONTROL: u32 = 0x0eu32;
pub const BQ25792_REG0F_CHARGER_CONTROL_0: u32 = 0x0fu32;
pub const BQ25792_REG10_CHARGER_CONTROL_1: u32 = 0x10u32;
pub const BQ25792_REG11_CHARGER_CONTROL_2: u32 = 0x11u32;
pub const BQ25792_REG12_CHARGER_CONTROL_3: u32 = 0x12u32;
pub const BQ25792_REG13_CHARGER_CONTROL_4: u32 = 0x13u32;
pub const BQ25792_REG14_CHARGER_CONTROL_5: u32 = 0x14u32;
/* REG15 reserved */
pub const BQ25792_REG16_TEMPERATURE_CONTROL: u32 = 0x16u32;
pub const BQ25792_REG17_NTC_CONTROL_0: u32 = 0x17u32;
pub const BQ25792_REG18_NTC_CONTROL_1: u32 = 0x18u32;
pub const BQ25792_REG19_ICO_CURRENT_LIMIT: u32 = 0x19u32;
pub const BQ25792_REG1B_CHARGER_STATUS_0: u32 = 0x1bu32;
pub const BQ25792_REG1C_CHARGER_STATUS_1: u32 = 0x1cu32;
pub const BQ25792_REG1D_CHARGER_STATUS_2: u32 = 0x1du32;
pub const BQ25792_REG1E_CHARGER_STATUS_3: u32 = 0x1eu32;
pub const BQ25792_REG1F_CHARGER_STATUS_4: u32 = 0x1fu32;
pub const BQ25792_REG20_FAULT_STATUS_0: u32 = 0x20u32;
pub const BQ25792_REG21_FAULT_STATUS_1: u32 = 0x21u32;
pub const BQ25792_REG22_CHARGER_FLAG_0: u32 = 0x22u32;
pub const BQ25792_REG23_CHARGER_FLAG_1: u32 = 0x23u32;
pub const BQ25792_REG24_CHARGER_FLAG_2: u32 = 0x24u32;
pub const BQ25792_REG25_CHARGER_FLAG_3: u32 = 0x25u32;
pub const BQ25792_REG26_FAULT_FLAG_0: u32 = 0x26u32;
pub const BQ25792_REG27_FAULT_FLAG_1: u32 = 0x27u32;
pub const BQ25792_REG28_CHARGER_MASK_0: u32 = 0x28u32;
pub const BQ25792_REG29_CHARGER_MASK_1: u32 = 0x29u32;
pub const BQ25792_REG2A_CHARGER_MASK_2: u32 = 0x2au32;
pub const BQ25792_REG2B_CHARGER_MASK_3: u32 = 0x2bu32;
pub const BQ25792_REG2C_FAULT_MASK_0: u32 = 0x2cu32;
pub const BQ25792_REG2D_FAULT_MASK_1: u32 = 0x2du32;
pub const BQ25792_REG2E_ADC_CONTROL: u32 = 0x2eu32;
pub const BQ25792_REG2F_ADC_FUNCTION_DISABLE_0: u32 = 0x2fu32;
pub const BQ25792_REG30_ADC_FUNCTION_DISABLE_1: u32 = 0x30u32;
pub const BQ25792_REG31_IBUS_ADC: u32 = 0x31u32;
pub const BQ25792_REG33_IBAT_ADC: u32 = 0x33u32;
pub const BQ25792_REG35_VBUS_ADC: u32 = 0x35u32;
pub const BQ25792_REG37_VAC1_ADC: u32 = 0x37u32;
pub const BQ25792_REG39_VAC2_ADC: u32 = 0x39u32;
pub const BQ25792_REG3B_VBAT_ADC: u32 = 0x3bu32;
pub const BQ25792_REG3D_VSYS_ADC: u32 = 0x3du32;
pub const BQ25792_REG3F_TS_ADC: u32 = 0x3fu32;
pub const BQ25792_REG41_TDIE_ADC: u32 = 0x41u32;
pub const BQ25792_REG43_DP_ADC: u32 = 0x43u32;
pub const BQ25792_REG45_DM_ADC: u32 = 0x45u32;
pub const BQ25792_REG47_DPDM_DRIVER: u32 = 0x47u32;
pub const BQ25792_REG48_PART_INFORMATION: u32 = 0x48u32;

/* Minimal System Voltage */
pub const BQ25792_REG00_VSYSMIN_MASK: u32 = genmask(5u32, 0u32);

pub const BQ25792_MINVSYS_MIN_UV: u32 = 2500000u32;
pub const BQ25792_MINVSYS_STEP_UV: u32 = 250000u32;
pub const BQ25792_MINVSYS_MAX_UV: u32 = 16000000u32;

/* Charge Voltage Limit */
pub const BQ25792_REG01_VREG_MASK: u32 = genmask(10u32, 0u32);

pub const BQ25792_VBATREG_MIN_UV: u32 = 3000000u32;
pub const BQ25792_VBATREG_STEP_UV: u32 = 10000u32;
pub const BQ25792_VBATREG_MAX_UV: u32 = 18800000u32;

/* Charge Current Limit */
pub const BQ25792_REG03_ICHG_MASK: u32 = genmask(8u32, 0u32);

pub const BQ25792_ICHG_MIN_UA: u32 = 50000u32;
pub const BQ25792_ICHG_STEP_UA: u32 = 10000u32;
pub const BQ25792_ICHG_MAX_UA: u32 = 5000000u32;

/* Input Voltage Limit */
pub const BQ25792_REG05_VINDPM_MASK: u32 = genmask(7u32, 0u32);

/* Input Current Limit */
pub const BQ25792_REG06_IINDPM_MASK: u32 = genmask(8u32, 0u32);
pub const BQ25792_IINDPM_DEFAULT_UA: u32 = 3000000u32;
pub const BQ25792_IINDPM_STEP_UA: u32 = 10000u32;
pub const BQ25792_IINDPM_MIN_UA: u32 = 100000u32;
pub const BQ25792_IINDPM_MAX_UA: u32 = 3300000u32;

/* Precharge Control */
pub const BQ25792_REG08_VBAT_LOWV_MASK: u32 = genmask(7u32, 6u32);
pub const BQ25792_REG08_IPRECHG_MASK: u32 = genmask(5u32, 0u32);

/* Termination Control */
pub const BQ25792_REG09_REG_RST: u32 = bit(6u32);
pub const BQ25792_REG09_ITERM_MASK: u32 = genmask(4u32, 0u32);

/* Re-charge Control */
pub const BQ25792_REG0A_CELL_MASK: u32 = genmask(7u32, 6u32);
pub const BQ25792_REG0A_TRECHG_MASK: u32 = genmask(5u32, 4u32);
pub const BQ25792_REG0A_VRECHG_MASK: u32 = genmask(3u32, 0u32);

pub const BQ25792_CELL_1S: u32 = 0u32;
pub const BQ25792_CELL_2S: u32 = 1u32;
pub const BQ25792_CELL_3S: u32 = 2u32;
pub const BQ25792_CELL_4S: u32 = 3u32;

pub const BQ25792_TRECHG_64MS: u32 = 0u32;
pub const BQ25792_TRECHG_256MS: u32 = 1u32;
pub const BQ25792_TRECHG_1024MS: u32 = 2u32;
pub const BQ25792_TRECHG_2048MS: u32 = 3u32;

pub const BQ25792_VRECHG_MIN_UV: u32 = 50000u32;
pub const BQ25792_VRECHG_STEP_UV: u32 = 50000u32;
pub const BQ25792_VRECHG_MAX_UV: u32 = 800000u32;

/* VOTG regulation */
pub const BQ25792_REG0B_VOTG_MASK: u32 = genmask(10u32, 0u32);

pub const BQ25792_OTG_VOLT_MIN_UV: u32 = 2800000u32;
pub const BQ25792_OTG_VOLT_STEP_UV: u32 = 10000u32;
pub const BQ25792_OTG_VOLT_MAX_UV: u32 = 22000000u32;
pub const BQ25792_OTG_VOLT_NUM_VOLT: u32 = ((BQ25792_OTG_VOLT_MAX_UV  						  - BQ25792_OTG_VOLT_MIN_UV) \;

/* IOTG regulation */
pub const BQ25792_REG0D_PRECHG_TMR: u32 = bit(7u32);
pub const BQ25792_REG0D_IOTG_MASK: u32 = genmask(6u32, 0u32);

pub const BQ25792_OTG_CUR_MIN_UA: u32 = 120000u32;
pub const BQ25792_OTG_CUR_STEP_UA: u32 = 40000u32;
pub const BQ25792_OTG_CUR_MAX_UA: u32 = 3320000u32;

/* Timer Control */
pub const BQ25792_REG0E_TOPOFF_TMR_MASK: u32 = genmask(7u32, 6u32);
pub const BQ25792_REG0E_EN_TRICHG_TMR: u32 = bit(5u32);
pub const BQ25792_REG0E_EN_PRECHG_TMR: u32 = bit(4u32);
pub const BQ25792_REG0E_EN_CHG_TMR: u32 = bit(3u32);
pub const BQ25792_REG0E_CHG_TMR_MASK: u32 = genmask(2u32, 1u32);
pub const BQ25792_REG0E_TMR2X_EN: u32 = bit(0u32);

/* Charger Control 0 */
pub const BQ25792_REG0F_EN_AUTO_IBATDIS: u32 = bit(7u32);
pub const BQ25792_REG0F_FORCE_IBATDIS: u32 = bit(6u32);
pub const BQ25792_REG0F_EN_CHG: u32 = bit(5u32);
pub const BQ25792_REG0F_EN_ICO: u32 = bit(4u32);
pub const BQ25792_REG0F_FORCE_ICO: u32 = bit(3u32);
pub const BQ25792_REG0F_EN_HIZ: u32 = bit(2u32);
pub const BQ25792_REG0F_EN_TERM: u32 = bit(1u32);
/* bit0 reserved */

/* Charger Control 1 */
pub const BQ25792_REG10_VAC_OVP_MASK: u32 = genmask(5u32, 4u32);
pub const BQ25792_REG10_WD_RST: u32 = bit(3u32);
pub const BQ25792_REG10_WATCHDOG_MASK: u32 = genmask(2u32, 0u32);

/* Charger Control 2 */
pub const BQ25792_REG11_FORCE_INDET: u32 = bit(7u32);
pub const BQ25792_REG11_AUTO_INDET_EN: u32 = bit(6u32);
pub const BQ25792_REG11_EN_12V: u32 = bit(5u32);
pub const BQ25792_REG11_EN_9V: u32 = bit(4u32);
pub const BQ25792_REG11_HVDCP_EN: u32 = bit(3u32);
pub const BQ25792_REG11_SDRV_CTRL_MASK: u32 = genmask(2u32, 1u32);
pub const BQ25792_REG11_SDRV_DLY: u32 = bit(0u32);

/* Charger Control 3 */
pub const BQ25792_REG12_DIS_ACDRV: u32 = bit(7u32);
pub const BQ25792_REG12_EN_OTG: u32 = bit(6u32);
pub const BQ25792_REG12_PFM_OTG_DIS: u32 = bit(5u32);
pub const BQ25792_REG12_PFM_FWD_DIS: u32 = bit(4u32);
pub const BQ25792_REG12_WKUP_DLY: u32 = bit(3u32);
pub const BQ25792_REG12_DIS_LDO: u32 = bit(2u32);
pub const BQ25792_REG12_DIS_OTG_OOA: u32 = bit(1u32);
pub const BQ25792_REG12_DIS_FWD_OOA: u32 = bit(0u32);

/* Charger Control 4 */
pub const BQ25792_REG13_EN_ACDRV2: u32 = bit(7u32);
pub const BQ25792_REG13_EN_ACDRV1: u32 = bit(6u32);
pub const BQ25792_REG13_PWM_FREQ: u32 = bit(5u32);
pub const BQ25792_REG13_DIS_STAT: u32 = bit(4u32);
pub const BQ25792_REG13_DIS_VSYS_SHORT: u32 = bit(3u32);
pub const BQ25792_REG13_DIS_VOTG_UVP: u32 = bit(2u32);
pub const BQ25792_REG13_FORCE_VINDPM_DET: u32 = bit(1u32);
pub const BQ25792_REG13_EN_IBUS_OCP: u32 = bit(0u32);

/* Charger Control 5 */
pub const BQ25792_REG14_SFET_PRESENT: u32 = bit(7u32);
/* bit6 reserved */
pub const BQ25792_REG14_EN_IBAT: u32 = bit(5u32);
pub const BQ25792_REG14_IBAT_REG_MASK: u32 = genmask(4u32, 3u32);
pub const BQ25792_REG14_EN_IINDPM: u32 = bit(2u32);
pub const BQ25792_REG14_EN_EXTILIM: u32 = bit(1u32);
pub const BQ25792_REG14_EN_BATOC: u32 = bit(0u32);

pub const BQ25792_IBAT_3A: u32 = field_prep(BQ25792_REG14_IBAT_REG_MASK, 0u32);
pub const BQ25792_IBAT_4A: u32 = field_prep(BQ25792_REG14_IBAT_REG_MASK, 1u32);
pub const BQ25792_IBAT_5A: u32 = field_prep(BQ25792_REG14_IBAT_REG_MASK, 2u32);
pub const BQ25792_IBAT_UNLIM: u32 = field_prep(BQ25792_REG14_IBAT_REG_MASK, 3u32);

/* Temperature Control */
pub const BQ25792_REG16_TREG_MASK: u32 = genmask(7u32, 6u32);
pub const BQ25792_REG16_TSHUT_MASK: u32 = genmask(5u32, 4u32);
pub const BQ25792_REG16_VBUS_PD_EN: u32 = bit(3u32);
pub const BQ25792_REG16_VAC1_PD_EN: u32 = bit(2u32);
pub const BQ25792_REG16_VAC2_PD_EN: u32 = bit(1u32);

/* NTC Control 0 */
pub const BQ25792_REG17_JEITA_VSET_MASK: u32 = genmask(7u32, 5u32);
pub const BQ25792_REG17_JEITA_ISETH_MASK: u32 = genmask(4u32, 3u32);
pub const BQ25792_REG17_JEITA_ISETC_MASK: u32 = genmask(2u32, 1u32);

/* NTC Control 1 */
pub const BQ25792_REG18_TS_COOL_MASK: u32 = genmask(7u32, 6u32);
pub const BQ25792_REG18_TS_WARM_MASK: u32 = genmask(5u32, 4u32);
pub const BQ25792_REG18_BHOT_MASK: u32 = genmask(3u32, 2u32);
pub const BQ25792_REG18_BCOLD: u32 = bit(1u32);
pub const BQ25792_REG18_TS_IGNORE: u32 = bit(0u32);

/* ICO Current Limit */
pub const BQ25792_REG19_ICO_ILIM_MASK: u32 = genmask(8u32, 0u32);

/* Charger Status 0 */
pub const BQ25792_REG1B_IINDPM_STAT: u32 = bit(7u32);
pub const BQ25792_REG1B_VINDPM_STAT: u32 = bit(6u32);
pub const BQ25792_REG1B_WD_STAT: u32 = bit(5u32);
pub const BQ25792_REG1B_POORSRC_STAT: u32 = bit(4u32);
pub const BQ25792_REG1B_PG_STAT: u32 = bit(3u32);
pub const BQ25792_REG1B_AC2_PRESENT_STAT: u32 = bit(2u32);
pub const BQ25792_REG1B_AC1_PRESENT_STAT: u32 = bit(1u32);
pub const BQ25792_REG1B_VBUS_PRESENT_STAT: u32 = bit(0u32);

/* Charger Status 1 */
pub const BQ25792_REG1C_CHG_STAT_MASK: u32 = genmask(7u32, 5u32);
pub const BQ25792_REG1C_VBUS_STAT_MASK: u32 = genmask(4u32, 1u32);
pub const BQ25792_REG1C_BC12_DONE_STAT: u32 = bit(0u32);

/* Charger Status 2 */
pub const BQ25792_REG1D_ICO_STAT_MASK: u32 = genmask(7u32, 6u32);
pub const BQ25792_REG1D_TREG_STAT: u32 = bit(2u32);
pub const BQ25792_REG1D_DPDM_STAT: u32 = bit(1u32);
pub const BQ25792_REG1D_VBAT_PRESENT_STAT: u32 = bit(0u32);

/* Charger Status 3 */
pub const BQ25792_REG1E_ACRB2_STAT: u32 = bit(7u32);
pub const BQ25792_REG1E_ACRB1_STAT: u32 = bit(6u32);
pub const BQ25792_REG1E_ADC_DONE_STAT: u32 = bit(5u32);
pub const BQ25792_REG1E_VSYS_STAT: u32 = bit(4u32);
pub const BQ25792_REG1E_CHG_TMR_STAT: u32 = bit(3u32);
pub const BQ25792_REG1E_TRICHG_TMR_STAT: u32 = bit(2u32);
pub const BQ25792_REG1E_PRECHG_TMR_STAT: u32 = bit(1u32);

/* Charger Status 4 */
pub const BQ25792_REG1F_VBATOTG_LOW_STAT: u32 = bit(4u32);
pub const BQ25792_REG1F_TS_COLD_STAT: u32 = bit(3u32);
pub const BQ25792_REG1F_TS_COOL_STAT: u32 = bit(2u32);
pub const BQ25792_REG1F_TS_WARM_STAT: u32 = bit(1u32);
pub const BQ25792_REG1F_TS_HOT_STAT: u32 = bit(0u32);

/* FAULT Status 0 */
pub const BQ25792_REG20_IBAT_REG_STAT: u32 = bit(7u32);
pub const BQ25792_REG20_VBUS_OVP_STAT: u32 = bit(6u32);
pub const BQ25792_REG20_VBAT_OVP_STAT: u32 = bit(5u32);
pub const BQ25792_REG20_IBUS_OCP_STAT: u32 = bit(4u32);
pub const BQ25792_REG20_IBAT_OCP_STAT: u32 = bit(3u32);
pub const BQ25792_REG20_CONV_OCP_STAT: u32 = bit(2u32);
pub const BQ25792_REG20_VAC2_OVP_STAT: u32 = bit(1u32);
pub const BQ25792_REG20_VAC1_OVP_STAT: u32 = bit(0u32);

pub const BQ25792_REG20_OVERVOLTAGE_MASK: u32 = (BQ25792_REG20_VBAT_OVP_STAT |  						 BQ25792_REG20_VAC2_OVP_STAT | \;
pub const BQ25792_REG20_OVERCURRENT_MASK: u32 = (BQ25792_REG20_IBAT_OCP_STAT |  						 BQ25792_REG20_CONV_OCP_STAT);

/* FAULT Status 1 */
pub const BQ25792_REG21_VSYS_SHORT_STAT: u32 = bit(7u32);
pub const BQ25792_REG21_VSYS_OVP_STAT: u32 = bit(6u32);
pub const BQ25792_REG21_OTG_OVP_STAT: u32 = bit(5u32);
pub const BQ25792_REG21_OTG_UVP_STAT: u32 = bit(4u32);
pub const BQ25792_REG21_TSHUT_STAT: u32 = bit(2u32);


/* Charger Flag 0 */
pub const BQ25792_REG22_IINDPM_FLAG: u32 = bit(7u32);
pub const BQ25792_REG22_VINDPM_FLAG: u32 = bit(6u32);
pub const BQ25792_REG22_WD_FLAG: u32 = bit(5u32);
pub const BQ25792_REG22_POORSRC_FLAG: u32 = bit(4u32);
pub const BQ25792_REG22_PG_FLAG: u32 = bit(3u32);
pub const BQ25792_REG22_AC2_PRESENT_FLAG: u32 = bit(2u32);
pub const BQ25792_REG22_AC1_PRESENT_FLAG: u32 = bit(1u32);
pub const BQ25792_REG22_VBUS_PRESENT_FLAG: u32 = bit(0u32);

/* Charger Flag 1 */
pub const BQ25792_REG23_CHG_FLAG: u32 = bit(7u32);
pub const BQ25792_REG23_ICO_FLAG: u32 = bit(6u32);
pub const BQ25792_REG23_VBUS_FLAG: u32 = bit(4u32);
pub const BQ25792_REG23_TREG_FLAG: u32 = bit(2u32);
pub const BQ25792_REG23_VBAT_PRESENT_FLAG: u32 = bit(1u32);
pub const BQ25792_REG23_BC12_DONE_FLAG: u32 = bit(0u32);

/* Charger Flag 2 */
pub const BQ25792_REG24_DPDM_DONE_FLAG: u32 = bit(6u32);
pub const BQ25792_REG24_ADC_DONE_FLAG: u32 = bit(5u32);
pub const BQ25792_REG24_VSYS_FLAG: u32 = bit(4u32);
pub const BQ25792_REG24_CHG_TMR_FLAG: u32 = bit(3u32);
pub const BQ25792_REG24_TRICHG_TMR_FLAG: u32 = bit(2u32);
pub const BQ25792_REG24_PRECHG_TMR_FLAG: u32 = bit(1u32);
pub const BQ25792_REG24_TOPOFF_TMR_FLAG: u32 = bit(0u32);

/* Charger Flag 3 */
pub const BQ25792_REG25_VBATOTG_LOW_FLAG: u32 = bit(4u32);
pub const BQ25792_REG25_TS_COLD_FLAG: u32 = bit(3u32);
pub const BQ25792_REG25_TS_COOL_FLAG: u32 = bit(2u32);
pub const BQ25792_REG25_TS_WARM_FLAG: u32 = bit(1u32);
pub const BQ25792_REG25_TS_HOT_FLAG: u32 = bit(0u32);

/* FAULT Flag 0 */
pub const BQ25792_REG26_IBAT_REG_FLAG: u32 = bit(7u32);
pub const BQ25792_REG26_VBUS_OVP_FLAG: u32 = bit(6u32);
pub const BQ25792_REG26_VBAT_OVP_FLAG: u32 = bit(5u32);
pub const BQ25792_REG26_IBUS_OCP_FLAG: u32 = bit(4u32);
pub const BQ25792_REG26_IBAT_OCP_FLAG: u32 = bit(3u32);
pub const BQ25792_REG26_CONV_OCP_FLAG: u32 = bit(2u32);
pub const BQ25792_REG26_VAC2_OVP_FLAG: u32 = bit(1u32);
pub const BQ25792_REG26_VAC1_OVP_FLAG: u32 = bit(0u32);

/* FAULT Flag 1 */
pub const BQ25792_REG27_VSYS_SHORT_FLAG: u32 = bit(7u32);
pub const BQ25792_REG27_VSYS_OVP_FLAG: u32 = bit(6u32);
pub const BQ25792_REG27_OTG_OVP_FLAG: u32 = bit(5u32);
pub const BQ25792_REG27_OTG_UVP_FLAG: u32 = bit(4u32);
pub const BQ25792_REG27_TSHUT_FLAG: u32 = bit(2u32);

/* Charger Mask 0 */
pub const BQ25792_REG28_IINDPM_MASK: u32 = bit(7u32);
pub const BQ25792_REG28_VINDPM_MASK: u32 = bit(6u32);
pub const BQ25792_REG28_WD_MASK: u32 = bit(5u32);
pub const BQ25792_REG28_POORSRC_MASK: u32 = bit(4u32);
pub const BQ25792_REG28_PG_MASK: u32 = bit(3u32);
pub const BQ25792_REG28_AC2_PRESENT_MASK: u32 = bit(2u32);
pub const BQ25792_REG28_AC1_PRESENT_MASK: u32 = bit(1u32);
pub const BQ25792_REG28_VBUS_PRESENT_MASK: u32 = bit(0u32);

/* Charger Mask 1 */
pub const BQ25792_REG29_CHG_MASK: u32 = bit(7u32);
pub const BQ25792_REG29_ICO_MASK: u32 = bit(6u32);
pub const BQ25792_REG29_VBUS_MASK: u32 = bit(4u32);
pub const BQ25792_REG29_TREG_MASK: u32 = bit(2u32);
pub const BQ25792_REG29_VBAT_PRESENT_MASK: u32 = bit(1u32);
pub const BQ25792_REG29_BC12_DONE_MASK: u32 = bit(0u32);

/* Charger Mask 2 */
pub const BQ25792_REG2A_DPDM_DONE_MASK: u32 = bit(6u32);
pub const BQ25792_REG2A_ADC_DONE_MASK: u32 = bit(5u32);
pub const BQ25792_REG2A_VSYS_MASK: u32 = bit(4u32);
pub const BQ25792_REG2A_CHG_TMR_MASK: u32 = bit(3u32);
pub const BQ25792_REG2A_TRICHG_TMR_MASK: u32 = bit(2u32);
pub const BQ25792_REG2A_PRECHG_TMR_MASK: u32 = bit(1u32);
pub const BQ25792_REG2A_TOPOFF_TMR_MASK: u32 = bit(0u32);

/* Charger Mask 3 */
pub const BQ25792_REG2B_VBATOTG_LOW_MASK: u32 = bit(4u32);
pub const BQ25792_REG2B_TS_COLD_MASK: u32 = bit(3u32);
pub const BQ25792_REG2B_TS_COOL_MASK: u32 = bit(2u32);
pub const BQ25792_REG2B_TS_WARM_MASK: u32 = bit(1u32);
pub const BQ25792_REG2B_TS_HOT_MASK: u32 = bit(0u32);

/* FAULT Mask 0 */
pub const BQ25792_REG2C_IBAT_REG_MASK: u32 = bit(7u32);
pub const BQ25792_REG2C_VBUS_OVP_MASK: u32 = bit(6u32);
pub const BQ25792_REG2C_VBAT_OVP_MASK: u32 = bit(5u32);
pub const BQ25792_REG2C_IBUS_OCP_MASK: u32 = bit(4u32);
pub const BQ25792_REG2C_IBAT_OCP_MASK: u32 = bit(3u32);
pub const BQ25792_REG2C_CONV_OCP_MASK: u32 = bit(2u32);
pub const BQ25792_REG2C_VAC2_OVP_MASK: u32 = bit(1u32);
pub const BQ25792_REG2C_VAC1_OVP_MASK: u32 = bit(0u32);

/* FAULT Mask 1 */
pub const BQ25792_REG2D_VSYS_SHORT_MASK: u32 = bit(7u32);
pub const BQ25792_REG2D_VSYS_OVP_MASK: u32 = bit(6u32);
pub const BQ25792_REG2D_OTG_OVP_MASK: u32 = bit(5u32);
pub const BQ25792_REG2D_OTG_UVP_MASK: u32 = bit(4u32);
pub const BQ25792_REG2D_TSHUT_MASK: u32 = bit(2u32);

/* ADC Control */
pub const BQ25792_REG2E_ADC_EN: u32 = bit(7u32);
pub const BQ25792_REG2E_ADC_RATE: u32 = bit(6u32);
pub const BQ25792_REG2E_ADC_SAMPLE_MASK: u32 = genmask(5u32, 4u32);
pub const BQ25792_REG2E_ADC_AVG: u32 = bit(3u32);
pub const BQ25792_REG2E_ADC_AVG_INIT: u32 = bit(2u32);

/* ADC Function Disable 0 */
pub const BQ25792_REG2F_IBUS_ADC_DIS: u32 = bit(7u32);
pub const BQ25792_REG2F_IBAT_ADC_DIS: u32 = bit(6u32);
pub const BQ25792_REG2F_VBUS_ADC_DIS: u32 = bit(5u32);
pub const BQ25792_REG2F_VBAT_ADC_DIS: u32 = bit(4u32);
pub const BQ25792_REG2F_VSYS_ADC_DIS: u32 = bit(3u32);
pub const BQ25792_REG2F_TS_ADC_DIS: u32 = bit(2u32);
pub const BQ25792_REG2F_TDIE_ADC_DIS: u32 = bit(1u32);

/* ADC Function Disable 1 */
pub const BQ25792_REG30_DP_ADC_DIS: u32 = bit(7u32);
pub const BQ25792_REG30_DM_ADC_DIS: u32 = bit(6u32);
pub const BQ25792_REG30_VAC2_ADC_DIS: u32 = bit(5u32);
pub const BQ25792_REG30_VAC1_ADC_DIS: u32 = bit(4u32);

/* 0x31-0x45: ADC result registers (16-bit, RO): single full-width field */

pub const BQ25792_ADCVSYSVBAT_STEP_UV: u32 = 1000u32;
pub const BQ25792_ADCIBAT_STEP_UA: u32 = 1000u32;

/* DPDM Driver */
pub const BQ25792_REG47_DPLUS_DAC_MASK: u32 = genmask(7u32, 5u32);
pub const BQ25792_REG47_DMINUS_DAC_MASK: u32 = genmask(4u32, 2u32);

/* Part Information */
pub const BQ25792_REG48_PN_MASK: u32 = genmask(5u32, 3u32);
pub const BQ25792_REG48_DEV_REV_MASK: u32 = genmask(2u32, 0u32);




// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
