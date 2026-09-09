/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 2000 MIPS Technologies, Inc.  All rights reserved.
 * Copyright (C) 2013 Imagination Technologies Ltd.
 *
 * Register definitions for Intel PIIX4 South Bridge Device.
 */

/* PIRQX Route Control */
pub const PIIX4_FUNC0_PIRQRC: u32 = 0x60;
pub const PIIX4_FUNC0_PIRQRC_IRQ_ROUTING_DISABLE: u32 = 1 << 7;
pub const PIIX4_FUNC0_PIRQRC_IRQ_ROUTING_MASK: u32 = 0xf;
pub const PIIX4_FUNC0_PIRQRC_IRQ_ROUTING_MAX: u32 = 16;
/* SERIRQ Control */
pub const PIIX4_FUNC0_SERIRQC: u32 = 0x64;
pub const PIIX4_FUNC0_SERIRQC_EN: u32 = 1 << 7;
pub const PIIX4_FUNC0_SERIRQC_CONT: u32 = 1 << 6;
/* Top Of Memory */
pub const PIIX4_FUNC0_TOM: u32 = 0x69;
pub const PIIX4_FUNC0_TOM_TOP_OF_MEMORY_MASK: u32 = 0xf0;
/* Deterministic Latency Control */
pub const PIIX4_FUNC0_DLC: u32 = 0x82;
pub const PIIX4_FUNC0_DLC_USBPR_EN: u32 = 1 << 2;
pub const PIIX4_FUNC0_DLC_PASSIVE_RELEASE_EN: u32 = 1 << 1;
pub const PIIX4_FUNC0_DLC_DELAYED_TRANSACTION_EN: u32 = 1 << 0;
/* General Configuration */
pub const PIIX4_FUNC0_GENCFG: u32 = 0xb0;
pub const PIIX4_FUNC0_GENCFG_SERIRQ: u32 = 1 << 16;

/* IDE Timing */
pub const PIIX4_FUNC1_IDETIM_PRIMARY_LO: u32 = 0x40;
pub const PIIX4_FUNC1_IDETIM_PRIMARY_HI: u32 = 0x41;
pub const PIIX4_FUNC1_IDETIM_PRIMARY_HI_IDE_DECODE_EN: u32 = 1 << 7;
pub const PIIX4_FUNC1_IDETIM_SECONDARY_LO: u32 = 0x42;
pub const PIIX4_FUNC1_IDETIM_SECONDARY_HI: u32 = 0x43;
pub const PIIX4_FUNC1_IDETIM_SECONDARY_HI_IDE_DECODE_EN: u32 = 1 << 7;

/* Power Management Configuration Space */
pub const PIIX4_FUNC3_PMBA: u32 = 0x40;
pub const PIIX4_FUNC3_PMREGMISC: u32 = 0x80;
pub const PIIX4_FUNC3_PMREGMISC_EN: u32 = 1 << 0;

/* Power Management IO Space */
pub const PIIX4_FUNC3IO_PMSTS: u32 = 0x00;
pub const PIIX4_FUNC3IO_PMSTS_PWRBTN_STS: u32 = 1 << 8;
pub const PIIX4_FUNC3IO_PMCNTRL: u32 = 0x04;
pub const PIIX4_FUNC3IO_PMCNTRL_SUS_EN: u32 = 1 << 13;
pub const PIIX4_FUNC3IO_PMCNTRL_SUS_TYP: u32 = 0x7 << 10;
pub const PIIX4_FUNC3IO_PMCNTRL_SUS_TYP_SOFF: u32 = 0x0 << 10;
pub const PIIX4_FUNC3IO_PMCNTRL_SUS_TYP_STR: u32 = 0x1 << 10;

/* Data for magic special PCI cycle */
pub const PIIX4_SUSPEND_MAGIC: u32 = 0x00120002;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
