/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by mv78xx0.h: BRIDGE_VIRT_BASE and BRIDGE_PHYS_BASE.

pub const CPU_CONTROL: usize = BRIDGE_VIRT_BASE + 0x0104;
pub const L2_WRITETHROUGH: u32 = 0x0002_0000;

pub const RSTOUTn_MASK: usize = BRIDGE_VIRT_BASE + 0x0108;
pub const RSTOUTn_MASK_PHYS: usize = BRIDGE_PHYS_BASE + 0x0108;
pub const SOFT_RESET_OUT_EN: u32 = 0x0000_0004;

pub const SYSTEM_SOFT_RESET: usize = BRIDGE_VIRT_BASE + 0x010c;
pub const SOFT_RESET: u32 = 0x0000_0001;

pub const BRIDGE_INT_TIMER1_CLR: i32 = !0x0004i32;

pub const IRQ_VIRT_BASE: usize = BRIDGE_VIRT_BASE + 0x0200;
pub const IRQ_CAUSE_ERR_OFF: u32 = 0x0000;
pub const IRQ_CAUSE_LOW_OFF: u32 = 0x0004;
pub const IRQ_CAUSE_HIGH_OFF: u32 = 0x0008;
pub const IRQ_MASK_ERR_OFF: u32 = 0x000c;
pub const IRQ_MASK_LOW_OFF: u32 = 0x0010;
pub const IRQ_MASK_HIGH_OFF: u32 = 0x0014;

pub const TIMER_VIRT_BASE: usize = BRIDGE_VIRT_BASE + 0x0300;
pub const TIMER_PHYS_BASE: usize = BRIDGE_PHYS_BASE + 0x0300;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
