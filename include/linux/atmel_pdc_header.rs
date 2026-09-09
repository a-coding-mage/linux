/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/atmel_pdc.h
 *
 * Copyright (C) 2005 Ivan Kokshaysky
 * Copyright (C) SAN People
 *
 * Peripheral Data Controller (PDC) registers.
 * Based on AT91RM9200 datasheet revision E.
 */

// ATMEL_PDC_H include guard omitted; Rust item names provide the equivalent
// single-definition boundary.

pub const ATMEL_PDC_RPR: u32 = 0x100; // Receive Pointer Register
pub const ATMEL_PDC_RCR: u32 = 0x104; // Receive Counter Register
pub const ATMEL_PDC_TPR: u32 = 0x108; // Transmit Pointer Register
pub const ATMEL_PDC_TCR: u32 = 0x10c; // Transmit Counter Register
pub const ATMEL_PDC_RNPR: u32 = 0x110; // Receive Next Pointer Register
pub const ATMEL_PDC_RNCR: u32 = 0x114; // Receive Next Counter Register
pub const ATMEL_PDC_TNPR: u32 = 0x118; // Transmit Next Pointer Register
pub const ATMEL_PDC_TNCR: u32 = 0x11c; // Transmit Next Counter Register

pub const ATMEL_PDC_PTCR: u32 = 0x120; // Transfer Control Register
pub const ATMEL_PDC_RXTEN: u32 = 1 << 0; // Receiver Transfer Enable
pub const ATMEL_PDC_RXTDIS: u32 = 1 << 1; // Receiver Transfer Disable
pub const ATMEL_PDC_TXTEN: u32 = 1 << 8; // Transmitter Transfer Enable
pub const ATMEL_PDC_TXTDIS: u32 = 1 << 9; // Transmitter Transfer Disable

pub const ATMEL_PDC_PTSR: u32 = 0x124; // Transfer Status Register

pub const ATMEL_PDC_SCND_BUF_OFF: u32 = 0x10; // Offset between first and second buffer registers

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
