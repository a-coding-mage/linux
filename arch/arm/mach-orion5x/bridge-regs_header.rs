/* SPDX-License-Identifier: GPL-2.0-only */
/* Orion CPU Bridge Registers */

// Dependency: symbols from "orion5x.h" are supplied externally.

pub const CPU_CONF: usize = ORION5X_BRIDGE_VIRT_BASE + 0x100;

pub const CPU_CTRL: usize = ORION5X_BRIDGE_VIRT_BASE + 0x104;

pub const RSTOUTn_MASK: usize = ORION5X_BRIDGE_VIRT_BASE + 0x108;
pub const RSTOUTn_MASK_PHYS: usize = ORION5X_BRIDGE_PHYS_BASE + 0x108;

pub const CPU_SOFT_RESET: usize = ORION5X_BRIDGE_VIRT_BASE + 0x10c;

pub const BRIDGE_CAUSE: usize = ORION5X_BRIDGE_VIRT_BASE + 0x110;

pub const POWER_MNG_CTRL_REG: usize = ORION5X_BRIDGE_VIRT_BASE + 0x11C;

pub const BRIDGE_INT_TIMER1_CLR: i32 = !0x0004;

pub const MAIN_IRQ_CAUSE: usize = ORION5X_BRIDGE_VIRT_BASE + 0x200;

pub const MAIN_IRQ_MASK: usize = ORION5X_BRIDGE_VIRT_BASE + 0x204;

pub const TIMER_VIRT_BASE: usize = ORION5X_BRIDGE_VIRT_BASE + 0x300;
pub const TIMER_PHYS_BASE: usize = ORION5X_BRIDGE_PHYS_BASE + 0x300;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
