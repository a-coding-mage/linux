/* SPDX-License-Identifier: GPL-2.0 */
/*
 * cbe_regs.h
 *
 * This file is intended to hold the various register definitions for CBE
 * on-chip system devices (memory controller, IO controller, etc...)
 *
 * (C) Copyright IBM Corporation 2001,2006
 *
 * Authors: Maximino Aguilar (maguilar@us.ibm.com)
 *          David J. Erb (djerb@us.ibm.com)
 *
 * (c) 2006 Benjamin Herrenschmidt <benh@kernel.crashing.org>, IBM Corp.
 */

// Dependency intent from the original header: <asm/cell-pmu.h>

/* Cell page table entries */
pub const CBE_IOPTE_PP_W: u64 = 0x8000_0000_0000_0000u64; /* protection: write */
pub const CBE_IOPTE_PP_R: u64 = 0x4000_0000_0000_0000u64; /* protection: read */
pub const CBE_IOPTE_M: u64 = 0x2000_0000_0000_0000u64; /* coherency required */
pub const CBE_IOPTE_SO_R: u64 = 0x1000_0000_0000_0000u64; /* ordering: writes */
pub const CBE_IOPTE_SO_RW: u64 = 0x1800_0000_0000_0000u64; /* ordering: r & w */
pub const CBE_IOPTE_RPN_Mask: u64 = 0x07ff_ffff_ffff_f000u64; /* RPN */
pub const CBE_IOPTE_H: u64 = 0x0000_0000_0000_0800u64; /* cache hint */
pub const CBE_IOPTE_IOID_Mask: u64 = 0x0000_0000_0000_07ffu64; /* ioid */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
