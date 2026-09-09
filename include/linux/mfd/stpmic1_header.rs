/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) STMicroelectronics 2018 - All Rights Reserved
 * Author: Philippe Peurichard <philippe.peurichard@st.com>,
 * Pascal Paillet <p.paillet@st.com> for STMicroelectronics.
 */

// Translated from linux/mfd/stpmic1.h. BIT and GENMASK are represented by
// their direct local Rust equivalents below.

pub const TURN_ON_SR: u32 = 0x1;
pub const TURN_OFF_SR: u32 = 0x2;
pub const ICC_LDO_TURN_OFF_SR: u32 = 0x3;
pub const ICC_BUCK_TURN_OFF_SR: u32 = 0x4;
pub const RREQ_STATE_SR: u32 = 0x5;
pub const VERSION_SR: u32 = 0x6;

pub const MAIN_CR: u32 = 0x10;
pub const PADS_PULL_CR: u32 = 0x11;
pub const BUCKS_PD_CR: u32 = 0x12;
pub const LDO14_PD_CR: u32 = 0x13;
pub const LDO56_VREF_PD_CR: u32 = 0x14;
pub const VBUS_DET_VIN_CR: u32 = 0x15;
pub const PKEY_TURNOFF_CR: u32 = 0x16;
pub const BUCKS_MASK_RANK_CR: u32 = 0x17;
pub const BUCKS_MASK_RESET_CR: u32 = 0x18;
pub const LDOS_MASK_RANK_CR: u32 = 0x19;
pub const LDOS_MASK_RESET_CR: u32 = 0x1A;
pub const WCHDG_CR: u32 = 0x1B;
pub const WCHDG_TIMER_CR: u32 = 0x1C;
pub const BUCKS_ICCTO_CR: u32 = 0x1D;
pub const LDOS_ICCTO_CR: u32 = 0x1E;

pub const BUCK1_ACTIVE_CR: u32 = 0x20;
pub const BUCK2_ACTIVE_CR: u32 = 0x21;
pub const BUCK3_ACTIVE_CR: u32 = 0x22;
pub const BUCK4_ACTIVE_CR: u32 = 0x23;
pub const VREF_DDR_ACTIVE_CR: u32 = 0x24;
pub const LDO1_ACTIVE_CR: u32 = 0x25;
pub const LDO2_ACTIVE_CR: u32 = 0x26;
pub const LDO3_ACTIVE_CR: u32 = 0x27;
pub const LDO4_ACTIVE_CR: u32 = 0x28;
pub const LDO5_ACTIVE_CR: u32 = 0x29;
pub const LDO6_ACTIVE_CR: u32 = 0x2A;

pub const BUCK1_STDBY_CR: u32 = 0x30;
pub const BUCK2_STDBY_CR: u32 = 0x31;
pub const BUCK3_STDBY_CR: u32 = 0x32;
pub const BUCK4_STDBY_CR: u32 = 0x33;
pub const VREF_DDR_STDBY_CR: u32 = 0x34;
pub const LDO1_STDBY_CR: u32 = 0x35;
pub const LDO2_STDBY_CR: u32 = 0x36;
pub const LDO3_STDBY_CR: u32 = 0x37;
pub const LDO4_STDBY_CR: u32 = 0x38;
pub const LDO5_STDBY_CR: u32 = 0x39;
pub const LDO6_STDBY_CR: u32 = 0x3A;

pub const BST_SW_CR: u32 = 0x40;
pub const INT_PENDING_R1: u32 = 0x50;
pub const INT_PENDING_R2: u32 = 0x51;
pub const INT_PENDING_R3: u32 = 0x52;
pub const INT_PENDING_R4: u32 = 0x53;
pub const INT_DBG_LATCH_R1: u32 = 0x60;
pub const INT_DBG_LATCH_R2: u32 = 0x61;
pub const INT_DBG_LATCH_R3: u32 = 0x62;
pub const INT_DBG_LATCH_R4: u32 = 0x63;
pub const INT_CLEAR_R1: u32 = 0x70;
pub const INT_CLEAR_R2: u32 = 0x71;
pub const INT_CLEAR_R3: u32 = 0x72;
pub const INT_CLEAR_R4: u32 = 0x73;
pub const INT_MASK_R1: u32 = 0x80;
pub const INT_MASK_R2: u32 = 0x81;
pub const INT_MASK_R3: u32 = 0x82;
pub const INT_MASK_R4: u32 = 0x83;
pub const INT_SET_MASK_R1: u32 = 0x90;
pub const INT_SET_MASK_R2: u32 = 0x91;
pub const INT_SET_MASK_R3: u32 = 0x92;
pub const INT_SET_MASK_R4: u32 = 0x93;
pub const INT_CLEAR_MASK_R1: u32 = 0xA0;
pub const INT_CLEAR_MASK_R2: u32 = 0xA1;
pub const INT_CLEAR_MASK_R3: u32 = 0xA2;
pub const INT_CLEAR_MASK_R4: u32 = 0xA3;
pub const INT_SRC_R1: u32 = 0xB0;
pub const INT_SRC_R2: u32 = 0xB1;
pub const INT_SRC_R3: u32 = 0xB2;
pub const INT_SRC_R4: u32 = 0xB3;

pub const PMIC_MAX_REGISTER_ADDRESS: u32 = INT_SRC_R4;
pub const STPMIC1_PMIC_NUM_IRQ_REGS: u32 = 4;
pub const TURN_OFF_SR_ICC_EVENT: u32 = 0x08;

pub const LDO_VOLTAGE_MASK: u32 = 0x7c;
pub const BUCK_VOLTAGE_MASK: u32 = 0xfc;
pub const LDO_BUCK_VOLTAGE_SHIFT: u32 = 2;
pub const LDO_ENABLE_MASK: u32 = 1;
pub const BUCK_ENABLE_MASK: u32 = 1;
pub const BUCK_HPLP_ENABLE_MASK: u32 = 2;
pub const BUCK_HPLP_SHIFT: u32 = 1;
pub const STDBY_ENABLE_MASK: u32 = 1;
pub const BUCKS_PD_CR_REG_MASK: u32 = 0xff;
pub const BUCK_MASK_RANK_REGISTER_MASK: u32 = 0x0f;
pub const BUCK_MASK_RESET_REGISTER_MASK: u32 = 0x0f;
pub const LDO1234_PULL_DOWN_REGISTER_MASK: u32 = 0xff;
pub const LDO56_VREF_PD_CR_REG_MASK: u32 = 0x3f;
pub const LDO_MASK_RANK_REGISTER_MASK: u32 = 0x3f;
pub const LDO_MASK_RESET_REGISTER_MASK: u32 = 0x3f;

pub const BUCK1_PULL_DOWN_REG: u32 = BUCKS_PD_CR;
pub const BUCK1_PULL_DOWN_MASK: u32 = 1;
pub const BUCK2_PULL_DOWN_REG: u32 = BUCKS_PD_CR;
pub const BUCK2_PULL_DOWN_MASK: u32 = 4;
pub const BUCK3_PULL_DOWN_REG: u32 = BUCKS_PD_CR;
pub const BUCK3_PULL_DOWN_MASK: u32 = 0x10;
pub const BUCK4_PULL_DOWN_REG: u32 = BUCKS_PD_CR;
pub const BUCK4_PULL_DOWN_MASK: u32 = 0x40;
pub const LDO1_PULL_DOWN_REG: u32 = LDO14_PD_CR;
pub const LDO1_PULL_DOWN_MASK: u32 = 1;
pub const LDO2_PULL_DOWN_REG: u32 = LDO14_PD_CR;
pub const LDO2_PULL_DOWN_MASK: u32 = 4;
pub const LDO3_PULL_DOWN_REG: u32 = LDO14_PD_CR;
pub const LDO3_PULL_DOWN_MASK: u32 = 0x10;
pub const LDO4_PULL_DOWN_REG: u32 = LDO14_PD_CR;
pub const LDO4_PULL_DOWN_MASK: u32 = 0x40;
pub const LDO5_PULL_DOWN_REG: u32 = LDO56_VREF_PD_CR;
pub const LDO5_PULL_DOWN_MASK: u32 = 1;
pub const LDO6_PULL_DOWN_REG: u32 = LDO56_VREF_PD_CR;
pub const LDO6_PULL_DOWN_MASK: u32 = 4;
pub const VREF_DDR_PULL_DOWN_REG: u32 = LDO56_VREF_PD_CR;
pub const VREF_DDR_PULL_DOWN_MASK: u32 = 0x10;
pub const BUCKS_ICCTO_CR_REG_MASK: u32 = 0x7f;
pub const LDOS_ICCTO_CR_REG_MASK: u32 = 0x3f;
pub const LDO_BYPASS_MASK: u32 = 0x80;

pub const OCP_OFF_DBG: u32 = 0x10;
pub const PWRCTRL_POLARITY_HIGH: u32 = 8;
pub const PWRCTRL_ENABLE: u32 = 4;
pub const RESTART_REQUEST_ENABLE: u32 = 2;
pub const SOFTWARE_SWITCH_OFF: u32 = 1;
pub const WAKEUP_DETECTOR_DISABLED: u32 = 0x10;
pub const PWRCTRL_PD_ACTIVE: u32 = 8;
pub const PWRCTRL_PU_ACTIVE: u32 = 4;
pub const WAKEUP_PD_ACTIVE: u32 = 2;
pub const PONKEY_PU_INACTIVE: u32 = 1;
pub const SWIN_DETECTOR_ENABLED: u32 = 0x80;
pub const SWOUT_DETECTOR_ENABLED: u32 = 0x40;
pub const VINLOW_ENABLED: u32 = 1;
pub const VINLOW_CTRL_REG_MASK: u32 = 0xff;
pub const BOOST_OVP_DISABLED: u32 = 0x80;
pub const VBUS_OTG_DETECTION_DISABLED: u32 = 0x40;
pub const SW_OUT_DISCHARGE: u32 = 0x20;
pub const VBUS_OTG_DISCHARGE: u32 = 0x10;
pub const OCP_LIMIT_HIGH: u32 = 8;
pub const SWIN_SWOUT_ENABLED: u32 = 4;
pub const USBSW_OTG_SWITCH_ENABLED: u32 = 2;
pub const BOOST_ENABLED: u32 = 1;
pub const PONKEY_PWR_OFF: u32 = 0x80;
pub const PONKEY_CC_FLAG_CLEAR: u32 = 0x40;
pub const PONKEY_TURNOFF_TIMER_MASK: u32 = 0x0f;
pub const PONKEY_TURNOFF_MASK: u32 = 0xff;

/* stpmic1 master device for sub-drivers. */
#[repr(C)]
pub struct stpmic1 {
    pub dev: *mut core::ffi::c_void,
    pub regmap: *mut core::ffi::c_void,
    pub irq: core::ffi::c_int,
    pub irq_data: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
