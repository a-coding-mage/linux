// SPDX-License-Identifier: GPL-2.0
/*
 * Driver of Inno Codec for rk3036 by Rockchip Inc.
 *
 * Author: Zheng ShunQian<zhengsq@rock-chips.com>
 */

/* codec registers */
pub const INNO_R00: u32 = 0x00;
pub const INNO_R01: u32 = 0x0c;
pub const INNO_R02: u32 = 0x10;
pub const INNO_R03: u32 = 0x14;
pub const INNO_R04: u32 = 0x88;
pub const INNO_R05: u32 = 0x8c;
pub const INNO_R06: u32 = 0x90;
pub const INNO_R07: u32 = 0x94;
pub const INNO_R08: u32 = 0x98;
pub const INNO_R09: u32 = 0x9c;
pub const INNO_R10: u32 = 0xa0;

/* register bit filed */
pub const INNO_R00_CSR_RESET: u32 = 0x0 << 0; /*codec system reset*/
pub const INNO_R00_CSR_WORK: u32 = 0x1 << 0;
pub const INNO_R00_CDCR_RESET: u32 = 0x0 << 1; /*codec digital core reset*/
pub const INNO_R00_CDCR_WORK: u32 = 0x1 << 1;
pub const INNO_R00_PRB_DISABLE: u32 = 0x0 << 6; /*power reset bypass*/
pub const INNO_R00_PRB_ENABLE: u32 = 0x1 << 6;

pub const INNO_R01_I2SMODE_MSK: u32 = 0x1 << 4;
pub const INNO_R01_I2SMODE_SLAVE: u32 = 0x0 << 4;
pub const INNO_R01_I2SMODE_MASTER: u32 = 0x1 << 4;
pub const INNO_R01_PINDIR_MSK: u32 = 0x1 << 5;
pub const INNO_R01_PINDIR_IN_SLAVE: u32 = 0x0 << 5; /*direction of pin*/
pub const INNO_R01_PINDIR_OUT_MASTER: u32 = 0x1 << 5;

pub const INNO_R02_LRS_MSK: u32 = 0x1 << 2;
pub const INNO_R02_LRS_NORMAL: u32 = 0x0 << 2; /*DAC Left Right Swap*/
pub const INNO_R02_LRS_SWAP: u32 = 0x1 << 2;
pub const INNO_R02_DACM_MSK: u32 = 0x3 << 3;
pub const INNO_R02_DACM_PCM: u32 = 0x3 << 3; /*DAC Mode*/
pub const INNO_R02_DACM_I2S: u32 = 0x2 << 3;
pub const INNO_R02_DACM_LJM: u32 = 0x1 << 3;
pub const INNO_R02_DACM_RJM: u32 = 0x0 << 3;
pub const INNO_R02_VWL_MSK: u32 = 0x3 << 5;
pub const INNO_R02_VWL_32BIT: u32 = 0x3 << 5; /*1/2Frame Valid Word Len*/
pub const INNO_R02_VWL_24BIT: u32 = 0x2 << 5;
pub const INNO_R02_VWL_20BIT: u32 = 0x1 << 5;
pub const INNO_R02_VWL_16BIT: u32 = 0x0 << 5;
pub const INNO_R02_LRCP_MSK: u32 = 0x1 << 7;
pub const INNO_R02_LRCP_NORMAL: u32 = 0x0 << 7; /*Left Right Polarity*/
pub const INNO_R02_LRCP_REVERSAL: u32 = 0x1 << 7;

pub const INNO_R03_BCP_MSK: u32 = 0x1 << 0;
pub const INNO_R03_BCP_NORMAL: u32 = 0x0 << 0; /*DAC bit clock polarity*/
pub const INNO_R03_BCP_REVERSAL: u32 = 0x1 << 0;
pub const INNO_R03_DACR_MSK: u32 = 0x1 << 1;
pub const INNO_R03_DACR_RESET: u32 = 0x0 << 1; /*DAC Reset*/
pub const INNO_R03_DACR_WORK: u32 = 0x1 << 1;
pub const INNO_R03_FWL_MSK: u32 = 0x3 << 2;
pub const INNO_R03_FWL_32BIT: u32 = 0x3 << 2; /*1/2Frame Word Length*/
pub const INNO_R03_FWL_24BIT: u32 = 0x2 << 2;
pub const INNO_R03_FWL_20BIT: u32 = 0x1 << 2;
pub const INNO_R03_FWL_16BIT: u32 = 0x0 << 2;

pub const INNO_R04_DACR_SW_SHIFT: u32 = 0;
pub const INNO_R04_DACL_SW_SHIFT: u32 = 1;
pub const INNO_R04_DACR_CLK_SHIFT: u32 = 2;
pub const INNO_R04_DACL_CLK_SHIFT: u32 = 3;
pub const INNO_R04_DACR_VREF_SHIFT: u32 = 4;
pub const INNO_R04_DACL_VREF_SHIFT: u32 = 5;

pub const INNO_R05_HPR_EN_SHIFT: u32 = 0;
pub const INNO_R05_HPL_EN_SHIFT: u32 = 1;
pub const INNO_R05_HPR_WORK_SHIFT: u32 = 2;
pub const INNO_R05_HPL_WORK_SHIFT: u32 = 3;

pub const INNO_R06_VOUTR_CZ_SHIFT: u32 = 0;
pub const INNO_R06_VOUTL_CZ_SHIFT: u32 = 1;
pub const INNO_R06_DACR_HILO_VREF_SHIFT: u32 = 2;
pub const INNO_R06_DACL_HILO_VREF_SHIFT: u32 = 3;
pub const INNO_R06_DAC_EN_SHIFT: u32 = 5;

pub const INNO_R06_DAC_PRECHARGE: u32 = 0x0 << 4; /*PreCharge control for DAC*/
pub const INNO_R06_DAC_DISCHARGE: u32 = 0x1 << 4;

pub const INNO_HP_GAIN_SHIFT: u32 = 0;
/* Gain of output, 1.5db step: -39db(0x0) ~ 0db(0x1a) ~ 6db(0x1f) */
pub const INNO_HP_GAIN_0DB: u32 = 0x1a;
pub const INNO_HP_GAIN_N39DB: u32 = 0x0;

pub const INNO_R09_HP_ANTIPOP_MSK: u32 = 0x3;
pub const INNO_R09_HP_ANTIPOP_OFF: u32 = 0x1;
pub const INNO_R09_HP_ANTIPOP_ON: u32 = 0x2;
pub const INNO_R09_HPR_ANITPOP_SHIFT: u32 = 0;
pub const INNO_R09_HPL_ANITPOP_SHIFT: u32 = 2;
pub const INNO_R09_HPR_MUTE_SHIFT: u32 = 4;
pub const INNO_R09_HPL_MUTE_SHIFT: u32 = 5;
pub const INNO_R09_DACR_SWITCH_SHIFT: u32 = 6;
pub const INNO_R09_DACL_SWITCH_SHIFT: u32 = 7;

pub const INNO_R10_CHARGE_SEL_CUR_400I_YES: u32 = 0x0 << 0;
pub const INNO_R10_CHARGE_SEL_CUR_400I_NO: u32 = 0x1 << 0;
pub const INNO_R10_CHARGE_SEL_CUR_260I_YES: u32 = 0x0 << 1;
pub const INNO_R10_CHARGE_SEL_CUR_260I_NO: u32 = 0x1 << 1;
pub const INNO_R10_CHARGE_SEL_CUR_130I_YES: u32 = 0x0 << 2;
pub const INNO_R10_CHARGE_SEL_CUR_130I_NO: u32 = 0x1 << 2;
pub const INNO_R10_CHARGE_SEL_CUR_100I_YES: u32 = 0x0 << 3;
pub const INNO_R10_CHARGE_SEL_CUR_100I_NO: u32 = 0x1 << 3;
pub const INNO_R10_CHARGE_SEL_CUR_050I_YES: u32 = 0x0 << 4;
pub const INNO_R10_CHARGE_SEL_CUR_050I_NO: u32 = 0x1 << 4;
pub const INNO_R10_CHARGE_SEL_CUR_027I_YES: u32 = 0x0 << 5;
pub const INNO_R10_CHARGE_SEL_CUR_027I_NO: u32 = 0x1 << 5;

pub const INNO_R10_MAX_CUR: u32 = INNO_R10_CHARGE_SEL_CUR_400I_YES
    | INNO_R10_CHARGE_SEL_CUR_260I_YES
    | INNO_R10_CHARGE_SEL_CUR_130I_YES
    | INNO_R10_CHARGE_SEL_CUR_100I_YES
    | INNO_R10_CHARGE_SEL_CUR_050I_YES
    | INNO_R10_CHARGE_SEL_CUR_027I_YES;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
