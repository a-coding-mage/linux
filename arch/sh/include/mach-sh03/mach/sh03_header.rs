/* SPDX-License-Identifier: GPL-2.0 */

/*
 * linux/include/asm-sh/sh03/sh03.h
 *
 * Copyright (C) 2004  Interface Co., Ltd. Saito.K
 *
 * Interface CTP/PCI-SH03 support
 */

pub const PA_PCI_IO: u32 = 0xbe24_0000; // PCI I/O space
pub const PA_PCI_MEM: u32 = 0xbd00_0000; // PCI MEM space

pub const PCIPAR: u32 = 0xa400_0cf8; // PCI Config address
pub const PCIPDR: u32 = 0xa400_0cfc; // PCI Config data

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
