/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016-17 Synopsys, Inc. (www.synopsys.com)
 */

// Dependency supplied by the original include: <soc/arc/arc_aux.h>

/* Timer related Aux registers */
pub const ARC_REG_TIMER0_LIMIT: u32 = 0x23; // timer 0 limit
pub const ARC_REG_TIMER0_CTRL: u32 = 0x22; // timer 0 control
pub const ARC_REG_TIMER0_CNT: u32 = 0x21; // timer 0 count
pub const ARC_REG_TIMER1_LIMIT: u32 = 0x102; // timer 1 limit
pub const ARC_REG_TIMER1_CTRL: u32 = 0x101; // timer 1 control
pub const ARC_REG_TIMER1_CNT: u32 = 0x100; // timer 1 count

/* CTRL reg bits */
pub const ARC_TIMER_CTRL_IE: u32 = 1 << 0; // Interrupt when Count reaches limit
pub const ARC_TIMER_CTRL_NH: u32 = 1 << 1; // Count only when CPU NOT halted

pub const ARC_TIMERN_MAX: u32 = 0xFFFF_FFFF;

pub const ARC_REG_TIMERS_BCR: u32 = 0x75;

#[repr(C)]
pub struct bcr_timer {
    // The original declaration uses endian-dependent C bitfields:
    // CONFIG_CPU_BIG_ENDIAN: pad2:15, rtsc:1, pad1:5, rtc:1, t1:1, t0:1, ver:8
    // Otherwise: ver:8, t0:1, t1:1, rtc:1, pad1:5, rtsc:1, pad2:15
    pub bits: u32,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
