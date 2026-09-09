// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-mvebu/kirkwood.h
 *
 * Generic definitions for Marvell Kirkwood SoC flavors:
 * 88F6180, 88F6192 and 88F6281.
 */

pub const KIRKWOOD_REGS_PHYS_BASE: u32 = 0xf1000000;
pub const DDR_PHYS_BASE: u32 = KIRKWOOD_REGS_PHYS_BASE + 0x00000;
pub const BRIDGE_PHYS_BASE: u32 = KIRKWOOD_REGS_PHYS_BASE + 0x20000;

pub const DDR_OPERATION_BASE: u32 = DDR_PHYS_BASE + 0x1418;

pub const CPU_CONFIG_PHYS: u32 = BRIDGE_PHYS_BASE + 0x0100;
pub const CPU_CONFIG_ERROR_PROP: u32 = 0x00000004;

pub const CPU_CONTROL_PHYS: u32 = BRIDGE_PHYS_BASE + 0x0104;
pub const MEMORY_PM_CTRL_PHYS: u32 = BRIDGE_PHYS_BASE + 0x0118;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
