/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C preprocessor condition:
 *   CONFIG_CPU_SUBTYPE_SH7722 || CONFIG_CPU_SUBTYPE_SH7723
 */
#[cfg(any(CONFIG_CPU_SUBTYPE_SH7722, CONFIG_CPU_SUBTYPE_SH7723))]
pub const rtc_reg_size: usize = core::mem::size_of::<u16>();

#[cfg(not(any(CONFIG_CPU_SUBTYPE_SH7722, CONFIG_CPU_SUBTYPE_SH7723)))]
pub const rtc_reg_size: usize = core::mem::size_of::<u32>();

pub const RTC_BIT_INVERTED: u32 = 0x40; /* bug on SH7750, SH7750S */
pub const RTC_DEF_CAPABILITIES: u32 = RTC_CAP_4_DIGIT_YEAR;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
