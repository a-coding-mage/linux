/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/*
 * Copyright (C) STMicroelectronics 2018 - All Rights Reserved
 * Author: Philippe Peurichard <philippe.peurichard@st.com>,
 * Pascal Paillet <p.paillet@st.com> for STMicroelectronics.
 */

/* IRQ definitions */
pub const IT_PONKEY_F: i32 = 0;
pub const IT_PONKEY_R: i32 = 1;
pub const IT_WAKEUP_F: i32 = 2;
pub const IT_WAKEUP_R: i32 = 3;
pub const IT_VBUS_OTG_F: i32 = 4;
pub const IT_VBUS_OTG_R: i32 = 5;
pub const IT_SWOUT_F: i32 = 6;
pub const IT_SWOUT_R: i32 = 7;

pub const IT_CURLIM_BUCK1: i32 = 8;
pub const IT_CURLIM_BUCK2: i32 = 9;
pub const IT_CURLIM_BUCK3: i32 = 10;
pub const IT_CURLIM_BUCK4: i32 = 11;
pub const IT_OCP_OTG: i32 = 12;
pub const IT_OCP_SWOUT: i32 = 13;
pub const IT_OCP_BOOST: i32 = 14;
pub const IT_OVP_BOOST: i32 = 15;

pub const IT_CURLIM_LDO1: i32 = 16;
pub const IT_CURLIM_LDO2: i32 = 17;
pub const IT_CURLIM_LDO3: i32 = 18;
pub const IT_CURLIM_LDO4: i32 = 19;
pub const IT_CURLIM_LDO5: i32 = 20;
pub const IT_CURLIM_LDO6: i32 = 21;
pub const IT_SHORT_SWOTG: i32 = 22;
pub const IT_SHORT_SWOUT: i32 = 23;

pub const IT_TWARN_F: i32 = 24;
pub const IT_TWARN_R: i32 = 25;
pub const IT_VINLOW_F: i32 = 26;
pub const IT_VINLOW_R: i32 = 27;
pub const IT_SWIN_F: i32 = 30;
pub const IT_SWIN_R: i32 = 31;

/* BUCK MODES definitions */
pub const STPMIC1_BUCK_MODE_NORMAL: i32 = 0;
pub const STPMIC1_BUCK_MODE_LP: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
