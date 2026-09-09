/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * IPIC private definitions and structure.
 *
 * Maintainer: Kumar Gala <galak@kernel.crashing.org>
 *
 * Copyright 2005 Freescale Semiconductor, Inc
 */

/* Dependency supplied by <asm/ipic.h>. */

pub const NR_IPIC_INTS: u32 = 128;

/* External IRQS */
pub const IPIC_IRQ_EXT0: u32 = 48;
pub const IPIC_IRQ_EXT1: u32 = 17;
pub const IPIC_IRQ_EXT7: u32 = 23;

/* Default Priority Registers */
pub const IPIC_PRIORITY_DEFAULT: u32 = 0x0530_9770;

/* System Global Interrupt Configuration Register */
pub const SICFR_IPSA: u32 = 0x0001_0000;
pub const SICFR_IPSB: u32 = 0x0002_0000;
pub const SICFR_IPSC: u32 = 0x0004_0000;
pub const SICFR_IPSD: u32 = 0x0008_0000;
pub const SICFR_MPSA: u32 = 0x0020_0000;
pub const SICFR_MPSB: u32 = 0x0040_0000;

/* System External Interrupt Mask Register */
pub const SEMSR_SIRQ0: u32 = 0x0000_8000;

/* System Error Control Register */
pub const SERCR_MCPR: u32 = 0x0000_0001;

/* External type supplied by the IRQ subsystem. */
#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ipic {
    /* Volatile MMIO register pointer. */
    pub regs: *mut u32,

    /* The remapper for this IPIC */
    pub irqhost: *mut irq_domain,
}

#[repr(C)]
pub struct ipic_info {
    pub ack: u8,       /* pending register offset from base if the irq
                          supports ack operation */
    pub mask: u8,      /* mask register offset from base */
    pub prio: u8,      /* priority register offset from base */
    pub force: u8,     /* force register offset from base */
    pub bit: u8,       /* register bit position (as per doc)
                          bit mask = 1 << (31 - bit) */
    pub prio_mask: u8, /* priority mask value */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
