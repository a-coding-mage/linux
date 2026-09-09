/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2014-2019 MediaTek Inc.
 *
 * Author: Tianping.Fang <tianping.fang@mediatek.com>
 *        Sean Wang <sean.wang@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const RTC_BBPU: u32 = 0x0000;
pub const RTC_BBPU_CBUSY: u32 = 1 << 6;
pub const RTC_BBPU_KEY: u32 = 0x43 << 8;

pub const RTC_WRTGR_MT6358: u32 = 0x003a;
pub const RTC_WRTGR_MT6397: u32 = 0x003c;
pub const RTC_WRTGR_MT6323: u32 = RTC_WRTGR_MT6397;

pub const RTC_IRQ_STA: u32 = 0x0002;
pub const RTC_IRQ_STA_AL: u32 = 1 << 0;
pub const RTC_IRQ_STA_LP: u32 = 1 << 3;

pub const RTC_IRQ_EN: u32 = 0x0004;
pub const RTC_IRQ_EN_AL: u32 = 1 << 0;
pub const RTC_IRQ_EN_ONESHOT: u32 = 1 << 2;
pub const RTC_IRQ_EN_LP: u32 = 1 << 3;
pub const RTC_IRQ_EN_ONESHOT_AL: u32 = RTC_IRQ_EN_ONESHOT | RTC_IRQ_EN_AL;

pub const RTC_AL_MASK: u32 = 0x0008;
pub const RTC_AL_MASK_DOW: u32 = 1 << 4;

pub const RTC_TC_SEC: u32 = 0x000a;
pub const RTC_TC_MTH_MASK: u32 = 0x000f;
/* Min, Hour, Dom... register offset to RTC_TC_SEC */
pub const RTC_OFFSET_SEC: u32 = 0;
pub const RTC_OFFSET_MIN: u32 = 1;
pub const RTC_OFFSET_HOUR: u32 = 2;
pub const RTC_OFFSET_DOM: u32 = 3;
pub const RTC_OFFSET_DOW: u32 = 4;
pub const RTC_OFFSET_MTH: u32 = 5;
pub const RTC_OFFSET_YEAR: u32 = 6;
pub const RTC_OFFSET_COUNT: u32 = 7;

pub const RTC_AL_SEC: u32 = 0x0018;

pub const RTC_AL_SEC_MASK: u32 = 0x003f;
pub const RTC_AL_MIN_MASK: u32 = 0x003f;
pub const RTC_AL_HOU_MASK: u32 = 0x001f;
pub const RTC_AL_DOM_MASK: u32 = 0x001f;
pub const RTC_AL_DOW_MASK: u32 = 0x0007;
pub const RTC_AL_MTH_MASK: u32 = 0x000f;
pub const RTC_AL_YEA_MASK: u32 = 0x007f;

pub const RTC_PDN2: u32 = 0x002e;
pub const RTC_PDN2_PWRON_ALARM: u32 = 1 << 4;

pub const MTK_RTC_POLL_DELAY_US: u32 = 10;
// Preserves the C expression: jiffies_to_usecs(HZ), supplied by the kernel.
pub const MTK_RTC_POLL_TIMEOUT: u64 = jiffies_to_usecs(HZ);

#[repr(C)]
pub struct mtk_rtc_data {
	pub wrtgr: u32,
}

#[repr(C)]
pub struct mt6397_rtc {
	pub rtc_dev: *mut rtc_device,

	/* Protect register access from multiple tasks */
	pub lock: mutex,
	pub regmap: *mut regmap,
	pub irq: i32,
	pub addr_base: u32,
	pub data: *const mtk_rtc_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
