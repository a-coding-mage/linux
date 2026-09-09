/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2025 ROHM Semiconductors
 *
 * The digital interface of trhe BD96802 PMIC is a reduced version of the
 * BD96801. Hence the BD96801 definitions are used for registers and masks
 * while this header only holds the IRQ definitions - mainly to avoid gaps in
 * IRQ numbers caused by the lack of some BUCKs / LDOs and their respective
 * IRQs.
 */

/* ERRB IRQs */
// Reg 0x52, 0x53, 0x54 - ERRB system IRQs
pub const BD96802_OTP_ERR_STAT: i32 = 0;
pub const BD96802_DBIST_ERR_STAT: i32 = 1;
pub const BD96802_EEP_ERR_STAT: i32 = 2;
pub const BD96802_ABIST_ERR_STAT: i32 = 3;
pub const BD96802_PRSTB_ERR_STAT: i32 = 4;
pub const BD96802_DRMOS1_ERR_STAT: i32 = 5;
pub const BD96802_DRMOS2_ERR_STAT: i32 = 6;
pub const BD96802_SLAVE_ERR_STAT: i32 = 7;
pub const BD96802_VREF_ERR_STAT: i32 = 8;
pub const BD96802_TSD_ERR_STAT: i32 = 9;
pub const BD96802_UVLO_ERR_STAT: i32 = 10;
pub const BD96802_OVLO_ERR_STAT: i32 = 11;
pub const BD96802_OSC_ERR_STAT: i32 = 12;
pub const BD96802_PON_ERR_STAT: i32 = 13;
pub const BD96802_POFF_ERR_STAT: i32 = 14;
pub const BD96802_CMD_SHDN_ERR_STAT: i32 = 15;
pub const BD96802_INT_SHDN_ERR_STAT: i32 = 16;

// Reg 0x55 BUCK1 ERR IRQs
pub const BD96802_BUCK1_PVIN_ERR_STAT: i32 = 17;
pub const BD96802_BUCK1_OVP_ERR_STAT: i32 = 18;
pub const BD96802_BUCK1_UVP_ERR_STAT: i32 = 19;
pub const BD96802_BUCK1_SHDN_ERR_STAT: i32 = 20;

// Reg 0x56 BUCK2 ERR IRQs
pub const BD96802_BUCK2_PVIN_ERR_STAT: i32 = 21;
pub const BD96802_BUCK2_OVP_ERR_STAT: i32 = 22;
pub const BD96802_BUCK2_UVP_ERR_STAT: i32 = 23;
pub const BD96802_BUCK2_SHDN_ERR_STAT: i32 = 24;

/* INTB IRQs */
// Reg 0x5c (System INTB)
pub const BD96802_TW_STAT: i32 = 0;
pub const BD96802_WDT_ERR_STAT: i32 = 1;
pub const BD96802_I2C_ERR_STAT: i32 = 2;
pub const BD96802_CHIP_IF_ERR_STAT: i32 = 3;

// Reg 0x5d (BUCK1 INTB)
pub const BD96802_BUCK1_OCPH_STAT: i32 = 4;
pub const BD96802_BUCK1_OCPL_STAT: i32 = 5;
pub const BD96802_BUCK1_OCPN_STAT: i32 = 6;
pub const BD96802_BUCK1_OVD_STAT: i32 = 7;
pub const BD96802_BUCK1_UVD_STAT: i32 = 8;
pub const BD96802_BUCK1_TW_CH_STAT: i32 = 9;

// Reg 0x5e (BUCK2 INTB)
pub const BD96802_BUCK2_OCPH_STAT: i32 = 10;
pub const BD96802_BUCK2_OCPL_STAT: i32 = 11;
pub const BD96802_BUCK2_OCPN_STAT: i32 = 12;
pub const BD96802_BUCK2_OVD_STAT: i32 = 13;
pub const BD96802_BUCK2_UVD_STAT: i32 = 14;
pub const BD96802_BUCK2_TW_CH_STAT: i32 = 15;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
