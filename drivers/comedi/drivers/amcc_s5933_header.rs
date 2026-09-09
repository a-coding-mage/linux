/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Stuff for AMCC S5933 PCI Controller
 *
 * Author: Michal Dobes <dobes@tesnet.cz>
 *
 * Inspirated from general-purpose AMCC S5933 PCI Matchmaker driver
 * made by Andrea Cisternino  <acister@pcape1.pi.infn.it>
 * and as result of espionage from MITE code made by David A. Schleef.
 * Thanks to AMCC for their on-line documentation and bus master DMA
 * example.
 */

/* AMCC Operation Register Offsets - PCI */
pub const AMCC_OP_REG_OMB1: u32 = 0x00;
pub const AMCC_OP_REG_OMB2: u32 = 0x04;
pub const AMCC_OP_REG_OMB3: u32 = 0x08;
pub const AMCC_OP_REG_OMB4: u32 = 0x0c;
pub const AMCC_OP_REG_IMB1: u32 = 0x10;
pub const AMCC_OP_REG_IMB2: u32 = 0x14;
pub const AMCC_OP_REG_IMB3: u32 = 0x18;
pub const AMCC_OP_REG_IMB4: u32 = 0x1c;
pub const AMCC_OP_REG_FIFO: u32 = 0x20;
pub const AMCC_OP_REG_MWAR: u32 = 0x24;
pub const AMCC_OP_REG_MWTC: u32 = 0x28;
pub const AMCC_OP_REG_MRAR: u32 = 0x2c;
pub const AMCC_OP_REG_MRTC: u32 = 0x30;
pub const AMCC_OP_REG_MBEF: u32 = 0x34;
pub const AMCC_OP_REG_INTCSR: u32 = 0x38;
pub const AMCC_OP_REG_INTCSR_SRC: u32 = AMCC_OP_REG_INTCSR + 2; /* INT source */
pub const AMCC_OP_REG_INTCSR_FEC: u32 = AMCC_OP_REG_INTCSR + 3; /* FIFO ctrl */
pub const AMCC_OP_REG_MCSR: u32 = 0x3c;
pub const AMCC_OP_REG_MCSR_NVDATA: u32 = AMCC_OP_REG_MCSR + 2; /* Data in byte 2 */
pub const AMCC_OP_REG_MCSR_NVCMD: u32 = AMCC_OP_REG_MCSR + 3; /* Command in byte 3 */

pub const AMCC_FIFO_DEPTH_DWORD: u32 = 8;
pub const AMCC_FIFO_DEPTH_BYTES: usize = 8 * core::mem::size_of::<u32>();

/* AMCC - PCI Interrupt Control/Status Register */
#[inline]
pub const fn INTCSR_OUTBOX_BYTE(x: u32) -> u32 { x & 0x3 }
#[inline]
pub const fn INTCSR_OUTBOX_SELECT(x: u32) -> u32 { (x & 0x3) << 2 }
pub const INTCSR_OUTBOX_EMPTY_INT: u32 = 0x10; /* enable outbox empty interrupt */
#[inline]
pub const fn INTCSR_INBOX_BYTE(x: u32) -> u32 { (x & 0x3) << 8 }
#[inline]
pub const fn INTCSR_INBOX_SELECT(x: u32) -> u32 { (x & 0x3) << 10 }
pub const INTCSR_INBOX_FULL_INT: u32 = 0x1000; /* enable inbox full interrupt */
/* read, or write clear inbox full interrupt */
pub const INTCSR_INBOX_INTR_STATUS: u32 = 0x20000;
/* read only, interrupt asserted */
pub const INTCSR_INTR_ASSERTED: u32 = 0x800000;

/* AMCC - PCI non-volatile ram command register (byte 3 of AMCC_OP_REG_MCSR) */
pub const MCSR_NV_LOAD_LOW_ADDR: u32 = 0x0;
pub const MCSR_NV_LOAD_HIGH_ADDR: u32 = 0x20;
pub const MCSR_NV_WRITE: u32 = 0x40;
pub const MCSR_NV_READ: u32 = 0x60;
pub const MCSR_NV_MASK: u32 = 0x60;
pub const MCSR_NV_ENABLE: u32 = 0x80;
pub const MCSR_NV_BUSY: u32 = MCSR_NV_ENABLE;

/* AMCC Operation Registers Size - PCI */
pub const AMCC_OP_REG_SIZE: u32 = 64; /* in bytes */

/* AMCC Operation Register Offsets - Add-on */
pub const AMCC_OP_REG_AIMB1: u32 = 0x00;
pub const AMCC_OP_REG_AIMB2: u32 = 0x04;
pub const AMCC_OP_REG_AIMB3: u32 = 0x08;
pub const AMCC_OP_REG_AIMB4: u32 = 0x0c;
pub const AMCC_OP_REG_AOMB1: u32 = 0x10;
pub const AMCC_OP_REG_AOMB2: u32 = 0x14;
pub const AMCC_OP_REG_AOMB3: u32 = 0x18;
pub const AMCC_OP_REG_AOMB4: u32 = 0x1c;
pub const AMCC_OP_REG_AFIFO: u32 = 0x20;
pub const AMCC_OP_REG_AMWAR: u32 = 0x24;
pub const AMCC_OP_REG_APTA: u32 = 0x28;
pub const AMCC_OP_REG_APTD: u32 = 0x2c;
pub const AMCC_OP_REG_AMRAR: u32 = 0x30;
pub const AMCC_OP_REG_AMBEF: u32 = 0x34;
pub const AMCC_OP_REG_AINT: u32 = 0x38;
pub const AMCC_OP_REG_AGCSTS: u32 = 0x3c;
pub const AMCC_OP_REG_AMWTC: u32 = 0x58;
pub const AMCC_OP_REG_AMRTC: u32 = 0x5c;

/* AMCC - Add-on General Control/Status Register */
pub const AGCSTS_CONTROL_MASK: u32 = 0xfffff000;
pub const AGCSTS_NV_ACC_MASK: u32 = 0xe0000000;
pub const AGCSTS_RESET_MASK: u32 = 0x0e000000;
pub const AGCSTS_NV_DA_MASK: u32 = 0x00ff0000;
pub const AGCSTS_BIST_MASK: u32 = 0x0000f000;
pub const AGCSTS_STATUS_MASK: u32 = 0x000000ff;
pub const AGCSTS_TCZERO_MASK: u32 = 0x000000c0;
pub const AGCSTS_FIFO_ST_MASK: u32 = 0x0000003f;
pub const AGCSTS_TC_ENABLE: u32 = 0x10000000;
pub const AGCSTS_RESET_MBFLAGS: u32 = 0x08000000;
pub const AGCSTS_RESET_P2A_FIFO: u32 = 0x04000000;
pub const AGCSTS_RESET_A2P_FIFO: u32 = 0x02000000;
pub const AGCSTS_RESET_FIFOS: u32 = AGCSTS_RESET_A2P_FIFO | AGCSTS_RESET_P2A_FIFO;
pub const AGCSTS_A2P_TCOUNT: u32 = 0x00000080;
pub const AGCSTS_P2A_TCOUNT: u32 = 0x00000040;
pub const AGCSTS_FS_P2A_EMPTY: u32 = 0x00000020;
pub const AGCSTS_FS_P2A_HALF: u32 = 0x00000010;
pub const AGCSTS_FS_P2A_FULL: u32 = 0x00000008;
pub const AGCSTS_FS_A2P_EMPTY: u32 = 0x00000004;
pub const AGCSTS_FS_A2P_HALF: u32 = 0x00000002;
pub const AGCSTS_FS_A2P_FULL: u32 = 0x00000001;

/* AMCC - Add-on Interrupt Control/Status Register */
pub const AINT_INT_MASK: u32 = 0x00ff0000;
pub const AINT_SEL_MASK: u32 = 0x0000ffff;
pub const AINT_IS_ENSEL_MASK: u32 = 0x00001f1f;
pub const AINT_INT_ASSERTED: u32 = 0x00800000;
pub const AINT_BM_ERROR: u32 = 0x00200000;
pub const AINT_BIST_INT: u32 = 0x00100000;
pub const AINT_RT_COMPLETE: u32 = 0x00080000;
pub const AINT_WT_COMPLETE: u32 = 0x00040000;
pub const AINT_OUT_MB_INT: u32 = 0x00020000;
pub const AINT_IN_MB_INT: u32 = 0x00010000;
pub const AINT_READ_COMPL: u32 = 0x00008000;
pub const AINT_WRITE_COMPL: u32 = 0x00004000;
pub const AINT_OMB_ENABLE: u32 = 0x00001000;
pub const AINT_OMB_SELECT: u32 = 0x00000c00;
pub const AINT_OMB_BYTE: u32 = 0x00000300;
pub const AINT_IMB_ENABLE: u32 = 0x00000010;
pub const AINT_IMB_SELECT: u32 = 0x0000000c;
pub const AINT_IMB_BYTE: u32 = 0x00000003;

/* these are bits from various different registers, needs cleanup XXX */
/* Enable Bus Mastering */
pub const EN_A2P_TRANSFERS: u32 = 0x00000400;
/* FIFO Flag Reset */
pub const RESET_A2P_FLAGS: u32 = 0x04000000;
/* FIFO Relative Priority */
pub const A2P_HI_PRIORITY: u32 = 0x00000100;
/* Identify Interrupt Sources */
pub const ANY_S593X_INT: u32 = 0x00800000;
pub const READ_TC_INT: u32 = 0x00080000;
pub const WRITE_TC_INT: u32 = 0x00040000;
pub const IN_MB_INT: u32 = 0x00020000;
pub const MASTER_ABORT_INT: u32 = 0x00100000;
pub const TARGET_ABORT_INT: u32 = 0x00200000;
pub const BUS_MASTER_INT: u32 = 0x00200000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
