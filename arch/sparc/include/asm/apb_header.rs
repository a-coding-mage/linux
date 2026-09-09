/* SPDX-License-Identifier: GPL-2.0 */
/*
 * apb.h: Advanced PCI Bridge Configuration Registers and Bits
 *
 * Copyright (C) 1998  Eddie C. Dost  (ecd@skynet.be)
 */

pub const APB_TICK_REGISTER: u32 = 0xb0;
pub const APB_INT_ACK: u32 = 0xb8;
pub const APB_PRIMARY_MASTER_RETRY_LIMIT: u32 = 0xc0;
pub const APB_DMA_ASFR: u32 = 0xc8;
pub const APB_DMA_AFAR: u32 = 0xd0;
pub const APB_PIO_TARGET_RETRY_LIMIT: u32 = 0xd8;
pub const APB_PIO_TARGET_LATENCY_TIMER: u32 = 0xd9;
pub const APB_DMA_TARGET_RETRY_LIMIT: u32 = 0xda;
pub const APB_DMA_TARGET_LATENCY_TIMER: u32 = 0xdb;
pub const APB_SECONDARY_MASTER_RETRY_LIMIT: u32 = 0xdc;
pub const APB_SECONDARY_CONTROL: u32 = 0xdd;
pub const APB_IO_ADDRESS_MAP: u32 = 0xde;
pub const APB_MEM_ADDRESS_MAP: u32 = 0xdf;

pub const APB_PCI_CONTROL_LOW: u32 = 0xe0;
pub const APB_PCI_CTL_LOW_ARB_PARK: u32 = 1u32 << 21;
pub const APB_PCI_CTL_LOW_ERRINT_EN: u32 = 1u32 << 8;

pub const APB_PCI_CONTROL_HIGH: u32 = 0xe4;
pub const APB_PCI_CTL_HIGH_SERR: u32 = 1u32 << 2;
pub const APB_PCI_CTL_HIGH_ARBITER_EN: u32 = 1u32 << 0;

pub const APB_PIO_ASFR: u32 = 0xe8;
pub const APB_PIO_AFAR: u32 = 0xf0;
pub const APB_DIAG_REGISTER: u32 = 0xf8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
