/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Following are the generic definitions for the ADDI-DATA timer/counter/
 * watchdog (TCW) registers and bits. Some of the registers are not used
 * depending on the use of the TCW.
 */

pub const ADDI_TCW_VAL_REG: u32 = 0x00;

pub const ADDI_TCW_SYNC_REG: u32 = 0x00;
pub const ADDI_TCW_SYNC_CTR_TRIG: u32 = 1u32 << 8;
pub const ADDI_TCW_SYNC_CTR_DIS: u32 = 1u32 << 7;
pub const ADDI_TCW_SYNC_CTR_ENA: u32 = 1u32 << 6;
pub const ADDI_TCW_SYNC_TIMER_TRIG: u32 = 1u32 << 5;
pub const ADDI_TCW_SYNC_TIMER_DIS: u32 = 1u32 << 4;
pub const ADDI_TCW_SYNC_TIMER_ENA: u32 = 1u32 << 3;
pub const ADDI_TCW_SYNC_WDOG_TRIG: u32 = 1u32 << 2;
pub const ADDI_TCW_SYNC_WDOG_DIS: u32 = 1u32 << 1;
pub const ADDI_TCW_SYNC_WDOG_ENA: u32 = 1u32 << 0;

pub const ADDI_TCW_RELOAD_REG: u32 = 0x04;

pub const ADDI_TCW_TIMEBASE_REG: u32 = 0x08;

pub const ADDI_TCW_CTRL_REG: u32 = 0x0c;
pub const ADDI_TCW_CTRL_EXT_CLK_STATUS: u32 = 1u32 << 21;
pub const ADDI_TCW_CTRL_CASCADE: u32 = 1u32 << 20;
pub const ADDI_TCW_CTRL_CNTR_ENA: u32 = 1u32 << 19;
pub const ADDI_TCW_CTRL_CNT_UP: u32 = 1u32 << 18;
pub const fn ADDI_TCW_CTRL_EXT_CLK(x: u32) -> u32 { (x & 3) << 16 }
pub const ADDI_TCW_CTRL_EXT_CLK_MASK: u32 = ADDI_TCW_CTRL_EXT_CLK(3);
pub const fn ADDI_TCW_CTRL_MODE(x: u32) -> u32 { (x & 7) << 13 }
pub const ADDI_TCW_CTRL_MODE_MASK: u32 = ADDI_TCW_CTRL_MODE(7);
pub const fn ADDI_TCW_CTRL_OUT(x: u32) -> u32 { (x & 3) << 11 }
pub const ADDI_TCW_CTRL_OUT_MASK: u32 = ADDI_TCW_CTRL_OUT(3);
pub const ADDI_TCW_CTRL_GATE: u32 = 1u32 << 10;
pub const ADDI_TCW_CTRL_TRIG: u32 = 1u32 << 9;
pub const fn ADDI_TCW_CTRL_EXT_GATE(x: u32) -> u32 { (x & 3) << 7 }
pub const ADDI_TCW_CTRL_EXT_GATE_MASK: u32 = ADDI_TCW_CTRL_EXT_GATE(3);
pub const fn ADDI_TCW_CTRL_EXT_TRIG(x: u32) -> u32 { (x & 3) << 5 }
pub const ADDI_TCW_CTRL_EXT_TRIG_MASK: u32 = ADDI_TCW_CTRL_EXT_TRIG(3);
pub const ADDI_TCW_CTRL_TIMER_ENA: u32 = 1u32 << 4;
pub const ADDI_TCW_CTRL_RESET_ENA: u32 = 1u32 << 3;
pub const ADDI_TCW_CTRL_WARN_ENA: u32 = 1u32 << 2;
pub const ADDI_TCW_CTRL_IRQ_ENA: u32 = 1u32 << 1;
pub const ADDI_TCW_CTRL_ENA: u32 = 1u32 << 0;

pub const ADDI_TCW_STATUS_REG: u32 = 0x10;
pub const ADDI_TCW_STATUS_SOFT_CLR: u32 = 1u32 << 3;
pub const ADDI_TCW_STATUS_HARDWARE_TRIG: u32 = 1u32 << 2;
pub const ADDI_TCW_STATUS_SOFT_TRIG: u32 = 1u32 << 1;
pub const ADDI_TCW_STATUS_OVERFLOW: u32 = 1u32 << 0;

pub const ADDI_TCW_IRQ_REG: u32 = 0x14;
pub const ADDI_TCW_IRQ: u32 = 1u32 << 0;

pub const ADDI_TCW_WARN_TIMEVAL_REG: u32 = 0x18;

pub const ADDI_TCW_WARN_TIMEBASE_REG: u32 = 0x1c;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
