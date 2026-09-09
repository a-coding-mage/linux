/* SPDX-License-Identifier: GPL-2.0-only */
/* Mbus-L to Mbus Bridge Registers */

// Dependency provided by the translated dove header.

pub const CPU_CONFIG: usize = BRIDGE_VIRT_BASE + 0x0000;

pub const CPU_CONTROL: usize = BRIDGE_VIRT_BASE + 0x0104;
pub const CPU_CTRL_PCIE0_LINK: u32 = 0x00000001;
pub const CPU_RESET: u32 = 0x00000002;
pub const CPU_CTRL_PCIE1_LINK: u32 = 0x00000008;

pub const RSTOUTn_MASK: usize = BRIDGE_VIRT_BASE + 0x0108;
pub const RSTOUTn_MASK_PHYS: usize = BRIDGE_PHYS_BASE + 0x0108;
pub const SOFT_RESET_OUT_EN: u32 = 0x00000004;

pub const SYSTEM_SOFT_RESET: usize = BRIDGE_VIRT_BASE + 0x010c;
pub const SOFT_RESET: u32 = 0x00000001;

pub const BRIDGE_CAUSE: usize = BRIDGE_VIRT_BASE + 0x0110;
pub const BRIDGE_INT_TIMER1_CLR: i32 = !0x0004;

pub const IRQ_VIRT_BASE: usize = BRIDGE_VIRT_BASE + 0x0200;
pub const IRQ_CAUSE_LOW_OFF: usize = 0x0000;
pub const IRQ_MASK_LOW_OFF: usize = 0x0004;
pub const FIQ_MASK_LOW_OFF: usize = 0x0008;
pub const ENDPOINT_MASK_LOW_OFF: usize = 0x000c;
pub const IRQ_CAUSE_HIGH_OFF: usize = 0x0010;
pub const IRQ_MASK_HIGH_OFF: usize = 0x0014;
pub const FIQ_MASK_HIGH_OFF: usize = 0x0018;
pub const ENDPOINT_MASK_HIGH_OFF: usize = 0x001c;
pub const PCIE_INTERRUPT_MASK_OFF: usize = 0x0020;

pub const IRQ_MASK_LOW: usize = IRQ_VIRT_BASE + IRQ_MASK_LOW_OFF;
pub const FIQ_MASK_LOW: usize = IRQ_VIRT_BASE + FIQ_MASK_LOW_OFF;
pub const ENDPOINT_MASK_LOW: usize = IRQ_VIRT_BASE + ENDPOINT_MASK_LOW_OFF;
pub const IRQ_MASK_HIGH: usize = IRQ_VIRT_BASE + IRQ_MASK_HIGH_OFF;
pub const FIQ_MASK_HIGH: usize = IRQ_VIRT_BASE + FIQ_MASK_HIGH_OFF;
pub const ENDPOINT_MASK_HIGH: usize = IRQ_VIRT_BASE + ENDPOINT_MASK_HIGH_OFF;
pub const PCIE_INTERRUPT_MASK: usize = IRQ_VIRT_BASE + PCIE_INTERRUPT_MASK_OFF;

pub const POWER_MANAGEMENT: usize = BRIDGE_VIRT_BASE + 0x011c;

pub const TIMER_VIRT_BASE: usize = BRIDGE_VIRT_BASE + 0x0300;
pub const TIMER_PHYS_BASE: usize = BRIDGE_PHYS_BASE + 0x0300;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
