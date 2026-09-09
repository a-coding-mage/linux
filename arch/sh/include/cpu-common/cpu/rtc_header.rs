/* SPDX-License-Identifier: GPL-2.0 */

// `u16` corresponds to the C type used by the source header.
pub const rtc_reg_size: usize = core::mem::size_of::<u16>();
pub const RTC_BIT_INVERTED: i32 = 0;
pub const RTC_DEF_CAPABILITIES: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
