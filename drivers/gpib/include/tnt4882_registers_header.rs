/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *    copyright           : (C) 2002, 2004 by Frank Mori Hess
 ***************************************************************************/

// tnt4882 register offsets
pub const ACCWR: u8 = 0x5;
// offset of auxiliary command register in 9914 mode
pub const AUXCR: u8 = 0x6;
pub const INTRT: u8 = 0x7;
// register number for auxiliary command register when swap bit is set (9914 mode)
pub const SWAPPED_AUXCR: u8 = 0xa;
pub const HSSEL: u8 = 0xd; // handshake select register
pub const CNT2: u8 = 0x9;
pub const CNT3: u8 = 0xb;
pub const CFG: u8 = 0x10;
pub const SASR: u8 = 0x1b;
pub const IMR0: u8 = 0x1d;
pub const IMR3: u8 = 0x12;
pub const CNT0: u8 = 0x14;
pub const CNT1: u8 = 0x16;
pub const KEYREG: u8 = 0x17; // key control register (7210 mode only)
pub const CSR: u8 = KEYREG;
pub const FIFOB: u8 = 0x18;
pub const FIFOA: u8 = 0x19;
pub const CCR: u8 = 0x1a; // carry cycle register
pub const CMDR: u8 = 0x1c; // command register
pub const TIMER: u8 = 0x1e; // timer register

pub const STS1: u8 = 0x10; // T488 Status Register 1
pub const STS2: u8 = 0x1c; // T488 Status Register 2
pub const ISR0: u8 = IMR0;
pub const ISR3: u8 = 0x1a; // T488 Interrupt Status Register 3
pub const BCR: u8 = 0x1f; // bus control/status register
pub const BSR: u8 = BCR;

pub const tnt_pagein_offset: u8 = 0x11;

/*============================================================*/

/* TURBO-488 registers bit definitions */
pub const BCSR_REN_BIT: u8 = 0x1;
pub const BCSR_IFC_BIT: u8 = 0x2;
pub const BCSR_SRQ_BIT: u8 = 0x4;
pub const BCSR_EOI_BIT: u8 = 0x8;
pub const BCSR_NRFD_BIT: u8 = 0x10;
pub const BCSR_NDAC_BIT: u8 = 0x20;
pub const BCSR_DAV_BIT: u8 = 0x40;
pub const BCSR_ATN_BIT: u8 = 0x80;

/* CFG -- Configuration Register (write only) */
pub const TNT_COMMAND: u8 = 0x80; /* bytes are command bytes instead of data bytes
                                     * (tnt4882 one-chip and newer only?) */
pub const TNT_TLCHE: u8 = 1 << 6; /* halt transfer on imr0, imr1, or imr2 interrupt */
pub const TNT_IN: u8 = 1 << 5; /* transfer is GPIB read */
pub const TNT_A_B: u8 = 1 << 4; /* order to use fifos 1=fifo A first(big endian),
                                   * 0=fifo b first(little endian) */
pub const TNT_CCEN: u8 = 1 << 3; /* enable carry cycle */
pub const TNT_TMOE: u8 = 1 << 2; /* enable CPU bus time limit */
pub const TNT_TIM_BYTN: u8 = 1 << 1; /* tmot reg is: 1=125ns clocks, 0=num bytes */
pub const TNT_B_16BIT: u8 = 1 << 0; /* 1=FIFO is 16-bit register, 0=8-bit */

/* CMDR -- Command Register */
pub const CLRSC: u8 = 0x2; /* clear the system controller bit */
pub const SETSC: u8 = 0x3; /* set the system controller bit */
pub const GO: u8 = 0x4; /* start fifos */
pub const STOP: u8 = 0x8; /* stop fifos */
pub const RESET_FIFO: u8 = 0x10; /* reset the FIFOs */
pub const SOFT_RESET: u8 = 0x22; /* issue a software reset */
pub const HARD_RESET: u8 = 0x40; /* 500x only? */

/* HSSEL -- handshake select register (write only) */
pub const TNT_ONE_CHIP_BIT: u8 = 0x1;
pub const NODMA: u8 = 0x10;
pub const TNT_GO2SIDS_BIT: u8 = 0x20;

/* IMR0 -- Interrupt Mode Register 0 */
pub const TNT_SYNCIE_BIT: u8 = 0x1; /* handshake sync */
pub const TNT_TOIE_BIT: u8 = 0x2; /* timeout */
pub const TNT_ATNIE_BIT: u8 = 0x4; /* ATN interrupt */
pub const TNT_IFCIE_BIT: u8 = 0x8; /* interface clear interrupt */
pub const TNT_BTO_BIT: u8 = 0x10; /* byte timeout */
pub const TNT_NLEN_BIT: u8 = 0x20; /* treat new line as EOS char */
pub const TNT_STBOIE_BIT: u8 = 0x40; /* status byte out */
pub const TNT_IMR0_ALWAYS_BITS: u8 = 0x80; /* always set this bit on write */

/* ISR0 -- Interrupt Status Register 0 */
pub const TNT_SYNC_BIT: u8 = 0x1; /* handshake sync */
pub const TNT_TO_BIT: u8 = 0x2; /* timeout */
pub const TNT_ATNI_BIT: u8 = 0x4; /* ATN interrupt */
pub const TNT_IFCI_BIT: u8 = 0x8; /* interface clear interrupt */
pub const TNT_EOS_BIT: u8 = 0x10; /* end of string */
pub const TNT_NL_BIT: u8 = 0x20; /* new line receive */
pub const TNT_STBO_BIT: u8 = 0x40; /* status byte out */
pub const TNT_NBA_BIT: u8 = 0x80; /* new byte available */

/* ISR3 -- Interrupt Status Register 3 (read only) */
pub const HR_DONE: u8 = 1 << 0; /* transfer done */
pub const HR_TLCI: u8 = 1 << 1; /* isr0, isr1, or isr2 interrupt asserted */
pub const HR_NEF: u8 = 1 << 2; /* NOT empty fifo */
pub const HR_NFF: u8 = 1 << 3; /* NOT full fifo */
pub const HR_STOP: u8 = 1 << 4; /* fifo empty or STOP command issued */
pub const HR_SRQI_CIC: u8 = 1 << 5; /* SRQ asserted and we are CIC (500x only?) */
pub const HR_INTR: u8 = 1 << 7; /* isr3 interrupt active */

pub const MSTD: u8 = 0x20; /* enable 350ns T1 delay */

/* STS1 -- Status Register 1 (read only) */
pub const S_DONE: u8 = 0x80; /* DMA done */
pub const S_SC: u8 = 0x40; /* is system controller */
pub const S_IN: u8 = 0x20; /* DMA in (to memory) */
pub const S_DRQ: u8 = 0x10; /* DRQ line (for diagnostics) */
pub const S_STOP: u8 = 0x08; /* DMA stopped */
pub const S_NDAV: u8 = 0x04; /* inverse of DAV */
pub const S_HALT: u8 = 0x02; /* status of transfer machine */
pub const S_GSYNC: u8 = 0x01; /* indicates if GPIB is in sync w I/O */

/* STS2 -- Status Register 2 */
pub const AFFN: u8 = 1 << 3; /* "A full FIFO NOT" (0=FIFO full) */
pub const AEFN: u8 = 1 << 2; /* "A empty FIFO NOT" (0=FIFO empty) */
pub const BFFN: u8 = 1 << 1; /* "B full FIFO NOT" (0=FIFO full) */
pub const BEFN: u8 = 1 << 0; /* "B empty FIFO NOT" (0=FIFO empty) */

// Auxiliary commands
pub const AUX_9914: u8 = 0x15; // switch to 9914 mode
pub const AUX_REQT: u8 = 0x18;
pub const AUX_REQF: u8 = 0x19;
pub const AUX_PAGEIN: u8 = 0x50; // page in alternate registers
pub const AUX_HLDI: u8 = 0x51; // rfd holdoff immediately
pub const AUX_CLEAR_END: u8 = 0x55;
pub const AUX_7210: u8 = 0x99; // switch to 7210 mode

pub const AUXRG: u8 = 0x40;
pub const AUXRI: u8 = 0xe0;

pub const NTNL_BIT: u8 = 0x8; /* no talking when no listeners bit (prevents bus errors when data written at wrong time) */
pub const RPP2_BIT: u8 = 0x4; /* set/clear local rpp message */
pub const CHES_BIT: u8 = 0x1; /* clear holdoff on end select bit */

pub const SISB: u8 = 0x1; // static interrupt bits (don't clear isr1, isr2 on read)
pub const PP2: u8 = 0x4; // ignore remote parallel poll configuration
pub const USTD: u8 = 0x8; // ultra short (1100 nanosec) T1 delay

pub const ACRDY_BIT: u8 = 0x4; /* acceptor ready state */
pub const ADHS_BIT: u8 = 0x8; /* acceptor data holdoff state */
pub const ANHS2_BIT: u8 = 0x10; /* acceptor not ready holdoff immediately state */
pub const ANHS1_BIT: u8 = 0x20; /* acceptor not ready holdoff state */
pub const AEHS_BIT: u8 = 0x40; /* acceptor end holdoff state */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
