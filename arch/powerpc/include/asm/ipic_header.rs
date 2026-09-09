/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * IPIC external definitions and structure.
 *
 * Maintainer: Kumar Gala <galak@kernel.crashing.org>
 *
 * Copyright 2005 Freescale Semiconductor, Inc
 */

// The original declarations are available only when building the kernel.

/* Flags when we init the IPIC */
pub const IPIC_SPREADMODE_GRP_A: u32 = 0x00000001;
pub const IPIC_SPREADMODE_GRP_B: u32 = 0x00000002;
pub const IPIC_SPREADMODE_GRP_C: u32 = 0x00000004;
pub const IPIC_SPREADMODE_GRP_D: u32 = 0x00000008;
pub const IPIC_SPREADMODE_MIX_A: u32 = 0x00000010;
pub const IPIC_SPREADMODE_MIX_B: u32 = 0x00000020;
pub const IPIC_DISABLE_MCP_OUT: u32 = 0x00000040;
pub const IPIC_IRQ0_MCP: u32 = 0x00000080;

/* IPIC registers offsets */
pub const IPIC_SICFR: u32 = 0x00; /* System Global Interrupt Configuration Register */
pub const IPIC_SIVCR: u32 = 0x04; /* System Global Interrupt Vector Register */
pub const IPIC_SIPNR_H: u32 = 0x08; /* System Internal Interrupt Pending Register (HIGH) */
pub const IPIC_SIPNR_L: u32 = 0x0C; /* System Internal Interrupt Pending Register (LOW) */
pub const IPIC_SIPRR_A: u32 = 0x10; /* System Internal Interrupt group A Priority Register */
pub const IPIC_SIPRR_B: u32 = 0x14; /* System Internal Interrupt group B Priority Register */
pub const IPIC_SIPRR_C: u32 = 0x18; /* System Internal Interrupt group C Priority Register */
pub const IPIC_SIPRR_D: u32 = 0x1C; /* System Internal Interrupt group D Priority Register */
pub const IPIC_SIMSR_H: u32 = 0x20; /* System Internal Interrupt Mask Register (HIGH) */
pub const IPIC_SIMSR_L: u32 = 0x24; /* System Internal Interrupt Mask Register (LOW) */
pub const IPIC_SICNR: u32 = 0x28; /* System Internal Interrupt Control Register */
pub const IPIC_SEPNR: u32 = 0x2C; /* System External Interrupt Pending Register */
pub const IPIC_SMPRR_A: u32 = 0x30; /* System Mixed Interrupt group A Priority Register */
pub const IPIC_SMPRR_B: u32 = 0x34; /* System Mixed Interrupt group B Priority Register */
pub const IPIC_SEMSR: u32 = 0x38; /* System External Interrupt Mask Register */
pub const IPIC_SECNR: u32 = 0x3C; /* System External Interrupt Control Register */
pub const IPIC_SERSR: u32 = 0x40; /* System Error Status Register */
pub const IPIC_SERMR: u32 = 0x44; /* System Error Mask Register */
pub const IPIC_SERCR: u32 = 0x48; /* System Error Control Register */
pub const IPIC_SIFCR_H: u32 = 0x50; /* System Internal Interrupt Force Register (HIGH) */
pub const IPIC_SIFCR_L: u32 = 0x54; /* System Internal Interrupt Force Register (LOW) */
pub const IPIC_SEFCR: u32 = 0x58; /* System External Interrupt Force Register */
pub const IPIC_SERFR: u32 = 0x5C; /* System Error Force Register */
pub const IPIC_SCVCR: u32 = 0x60; /* System Critical Interrupt Vector Register */
pub const IPIC_SMVCR: u32 = 0x64; /* System Management Interrupt Vector Register */

#[repr(u32)]
pub enum ipic_prio_grp {
    IPIC_INT_GRP_A = IPIC_SIPRR_A,
    IPIC_INT_GRP_D = IPIC_SIPRR_D,
    IPIC_MIX_GRP_A = IPIC_SMPRR_A,
    IPIC_MIX_GRP_B = IPIC_SMPRR_B,
}

#[repr(u32)]
pub enum ipic_mcp_irq {
    IPIC_MCP_IRQ0 = 0,
    IPIC_MCP_WDT = 1,
    IPIC_MCP_SBA = 2,
    IPIC_MCP_PCI1 = 5,
    IPIC_MCP_PCI2 = 6,
    IPIC_MCP_MU = 7,
}

// Opaque types supplied by the kernel environment.
pub struct ipic;
pub struct device_node;

extern "C" {
    pub fn ipic_set_default_priority();
    pub fn ipic_get_mcp_status() -> u32;
    pub fn ipic_clear_mcp_status(mask: u32);
    pub fn ipic_init(node: *mut device_node, flags: core::ffi::c_uint) -> *mut ipic;
    pub fn ipic_get_irq() -> core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
