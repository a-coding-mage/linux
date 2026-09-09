/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Microsemi Ocelot Switch driver
 *
 * License: Dual MIT/GPL
 * Copyright (c) 2017 Microsemi Corporation
 * Copyright 2020 NXP
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/ptp_clock_kernel.h, soc/mscc/ocelot.h

pub const OCELOT_MAX_PTP_ID: u32 = 63;
pub const OCELOT_PTP_FIFO_SIZE: u32 = 128;

pub const PTP_PIN_CFG_RSZ: u32 = 0x20;
pub const PTP_PIN_TOD_SEC_MSB_RSZ: u32 = PTP_PIN_CFG_RSZ;
pub const PTP_PIN_TOD_SEC_LSB_RSZ: u32 = PTP_PIN_CFG_RSZ;
pub const PTP_PIN_TOD_NSEC_RSZ: u32 = PTP_PIN_CFG_RSZ;
pub const PTP_PIN_WF_HIGH_PERIOD_RSZ: u32 = PTP_PIN_CFG_RSZ;
pub const PTP_PIN_WF_LOW_PERIOD_RSZ: u32 = PTP_PIN_CFG_RSZ;

pub const PTP_PIN_CFG_DOM: u32 = 1 << 0;
pub const PTP_PIN_CFG_SYNC: u32 = 1 << 2;

#[inline]
pub const fn PTP_PIN_CFG_ACTION(x: u32) -> u32 {
    x << 3
}

pub const PTP_PIN_CFG_ACTION_MASK: u32 = PTP_PIN_CFG_ACTION(0x7);

pub const PTP_PIN_ACTION_IDLE: u32 = 0;
pub const PTP_PIN_ACTION_LOAD: u32 = 1;
pub const PTP_PIN_ACTION_SAVE: u32 = 2;
pub const PTP_PIN_ACTION_CLOCK: u32 = 3;
pub const PTP_PIN_ACTION_DELTA: u32 = 4;
pub const PTP_PIN_ACTION_NOSYNC: u32 = 5;
pub const PTP_PIN_ACTION_SYNC: u32 = 6;

pub const PTP_CFG_MISC_PTP_EN: u32 = 1 << 2;

pub const PTP_CFG_CLK_ADJ_CFG_ENA: u32 = 1 << 0;
pub const PTP_CFG_CLK_ADJ_CFG_DIR: u32 = 1 << 1;

pub const PTP_CFG_CLK_ADJ_FREQ_NS: u32 = 1 << 30;

extern "C" {
    pub fn ocelot_ptp_gettime64(
        ptp: *mut ptp_clock_info,
        ts: *mut timespec64,
    ) -> i32;
    pub fn ocelot_ptp_settime64(
        ptp: *mut ptp_clock_info,
        ts: *const timespec64,
    ) -> i32;
    pub fn ocelot_ptp_adjtime(ptp: *mut ptp_clock_info, delta: i64) -> i32;
    pub fn ocelot_ptp_adjfine(ptp: *mut ptp_clock_info, scaled_ppm: isize) -> i32;
    pub fn ocelot_ptp_verify(
        ptp: *mut ptp_clock_info,
        pin: u32,
        func: ptp_pin_function,
        chan: u32,
    ) -> i32;
    pub fn ocelot_ptp_enable(
        ptp: *mut ptp_clock_info,
        rq: *mut ptp_clock_request,
        on: i32,
    ) -> i32;
    pub fn ocelot_init_timestamp(
        ocelot: *mut ocelot,
        info: *const ptp_clock_info,
    ) -> i32;
    pub fn ocelot_deinit_timestamp(ocelot: *mut ocelot) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
