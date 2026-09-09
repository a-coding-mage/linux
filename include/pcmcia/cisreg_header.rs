/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * cisreg.h
 *
 * The initial developer of the original code is David A. Hinds
 * <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
 * are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
 *
 * (C) 1999             David A. Hinds
 */

/* Offsets from ConfigBase for CIS registers */
pub const CISREG_COR: u32 = 0x00;
pub const CISREG_CCSR: u32 = 0x02;
pub const CISREG_PRR: u32 = 0x04;
pub const CISREG_SCR: u32 = 0x06;
pub const CISREG_ESR: u32 = 0x08;
pub const CISREG_IOBASE_0: u32 = 0x0a;
pub const CISREG_IOBASE_1: u32 = 0x0c;
pub const CISREG_IOBASE_2: u32 = 0x0e;
pub const CISREG_IOBASE_3: u32 = 0x10;
pub const CISREG_IOSIZE: u32 = 0x12;

/* Configuration Option Register */
pub const COR_CONFIG_MASK: u32 = 0x3f;
pub const COR_MFC_CONFIG_MASK: u32 = 0x38;
pub const COR_FUNC_ENA: u32 = 0x01;
pub const COR_ADDR_DECODE: u32 = 0x02;
pub const COR_IREQ_ENA: u32 = 0x04;
pub const COR_LEVEL_REQ: u32 = 0x40;
pub const COR_SOFT_RESET: u32 = 0x80;

/* Card Configuration and Status Register */
pub const CCSR_INTR_ACK: u32 = 0x01;
pub const CCSR_INTR_PENDING: u32 = 0x02;
pub const CCSR_POWER_DOWN: u32 = 0x04;
pub const CCSR_AUDIO_ENA: u32 = 0x08;
pub const CCSR_IOIS8: u32 = 0x20;
pub const CCSR_SIGCHG_ENA: u32 = 0x40;
pub const CCSR_CHANGED: u32 = 0x80;

/* Pin Replacement Register */
pub const PRR_WP_STATUS: u32 = 0x01;
pub const PRR_READY_STATUS: u32 = 0x02;
pub const PRR_BVD2_STATUS: u32 = 0x04;
pub const PRR_BVD1_STATUS: u32 = 0x08;
pub const PRR_WP_EVENT: u32 = 0x10;
pub const PRR_READY_EVENT: u32 = 0x20;
pub const PRR_BVD2_EVENT: u32 = 0x40;
pub const PRR_BVD1_EVENT: u32 = 0x80;

/* Socket and Copy Register */
pub const SCR_SOCKET_NUM: u32 = 0x0f;
pub const SCR_COPY_NUM: u32 = 0x70;

/* Extended Status Register */
pub const ESR_REQ_ATTN_ENA: u32 = 0x01;
pub const ESR_REQ_ATTN: u32 = 0x10;

/* CardBus Function Status Registers */
pub const CBFN_EVENT: u32 = 0x00;
pub const CBFN_MASK: u32 = 0x04;
pub const CBFN_STATE: u32 = 0x08;
pub const CBFN_FORCE: u32 = 0x0c;

/* These apply to all the CardBus function registers */
pub const CBFN_WP: u32 = 0x0001;
pub const CBFN_READY: u32 = 0x0002;
pub const CBFN_BVD2: u32 = 0x0004;
pub const CBFN_BVD1: u32 = 0x0008;
pub const CBFN_GWAKE: u32 = 0x0010;
pub const CBFN_INTR: u32 = 0x8000;

/* Extra bits in the Function Event Mask Register */
pub const FEMR_BAM_ENA: u32 = 0x0020;
pub const FEMR_PWM_ENA: u32 = 0x0040;
pub const FEMR_WKUP_MASK: u32 = 0x4000;

/*
 * Indirect Addressing Registers for Zoomed Video: these are addresses
 * in common memory space
 */
pub const CISREG_ICTRL0: u32 = 0x02; /* control registers */
pub const CISREG_ICTRL1: u32 = 0x03;
pub const CISREG_IADDR0: u32 = 0x04; /* address registers */
pub const CISREG_IADDR1: u32 = 0x05;
pub const CISREG_IADDR2: u32 = 0x06;
pub const CISREG_IADDR3: u32 = 0x07;
pub const CISREG_IDATA0: u32 = 0x08; /* data registers */
pub const CISREG_IDATA1: u32 = 0x09;

pub const ICTRL0_COMMON: u32 = 0x01;
pub const ICTRL0_AUTOINC: u32 = 0x02;
pub const ICTRL0_BYTEGRAN: u32 = 0x04;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
