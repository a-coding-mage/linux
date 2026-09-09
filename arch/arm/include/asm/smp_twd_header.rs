/* SPDX-License-Identifier: GPL-2.0 */
// Original header guard: __ASMARM_SMP_TWD_H

pub const TWD_TIMER_LOAD: u32 = 0x00;
pub const TWD_TIMER_COUNTER: u32 = 0x04;
pub const TWD_TIMER_CONTROL: u32 = 0x08;
pub const TWD_TIMER_INTSTAT: u32 = 0x0C;

pub const TWD_WDOG_LOAD: u32 = 0x20;
pub const TWD_WDOG_COUNTER: u32 = 0x24;
pub const TWD_WDOG_CONTROL: u32 = 0x28;
pub const TWD_WDOG_INTSTAT: u32 = 0x2C;
pub const TWD_WDOG_RESETSTAT: u32 = 0x30;
pub const TWD_WDOG_DISABLE: u32 = 0x34;

pub const TWD_TIMER_CONTROL_ENABLE: u32 = 1 << 0;
pub const TWD_TIMER_CONTROL_ONESHOT: u32 = 0 << 1;
pub const TWD_TIMER_CONTROL_PERIODIC: u32 = 1 << 1;
pub const TWD_TIMER_CONTROL_IT_ENABLE: u32 = 1 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
