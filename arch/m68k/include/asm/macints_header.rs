/*
** macints.h -- Macintosh Linux interrupt handling structs and prototypes
**
** Copyright 1997 by Michael Schmitz
**
** This file is subject to the terms and conditions of the GNU General Public
** License.  See the file COPYING in the main directory of this archive
** for more details.
**
*/

// #include <asm/irq.h>

/*
 * Base IRQ number for all Mac68K interrupt sources. Each source
 * has eight indexes (base -> base+7).
 */

pub const VIA1_SOURCE_BASE: i32 = 8;
pub const VIA2_SOURCE_BASE: i32 = 16;
pub const PSC3_SOURCE_BASE: i32 = 24;
pub const PSC4_SOURCE_BASE: i32 = 32;
pub const PSC5_SOURCE_BASE: i32 = 40;
pub const PSC6_SOURCE_BASE: i32 = 48;
pub const NUBUS_SOURCE_BASE: i32 = 56;
pub const BABOON_SOURCE_BASE: i32 = 64;

/*
 * Maximum IRQ number is BABOON_SOURCE_BASE + 7,
 * giving us IRQs up through 71
 */

pub const NUM_MAC_SOURCES: i32 = 72;

/*
 * clean way to separate IRQ into its source and index
 */

#[inline]
pub const fn irq_src(irq: i32) -> i32 { irq >> 3 }

#[inline]
pub const fn irq_idx(irq: i32) -> i32 { irq & 7 }

/* VIA1 interrupts */
pub const IRQ_VIA1_0: i32 = 8; /* one second int. */
pub const IRQ_VIA1_1: i32 = 9; /* VBlank int. */
pub const IRQ_MAC_VBL: i32 = IRQ_VIA1_1;
pub const IRQ_VIA1_2: i32 = 10; /* ADB SR shifts complete */
pub const IRQ_MAC_ADB: i32 = IRQ_VIA1_2;
pub const IRQ_MAC_ADB_SR: i32 = IRQ_VIA1_2;
pub const IRQ_VIA1_3: i32 = 11; /* ADB SR CB2 ?? */
pub const IRQ_MAC_ADB_SD: i32 = IRQ_VIA1_3;
pub const IRQ_VIA1_4: i32 = 12; /* ADB SR ext. clock pulse */
pub const IRQ_MAC_ADB_CL: i32 = IRQ_VIA1_4;
pub const IRQ_VIA1_5: i32 = 13;
pub const IRQ_MAC_TIMER_2: i32 = IRQ_VIA1_5;
pub const IRQ_VIA1_6: i32 = 14;
pub const IRQ_MAC_TIMER_1: i32 = IRQ_VIA1_6;
pub const IRQ_VIA1_7: i32 = 15;

/* VIA2/RBV interrupts */
pub const IRQ_VIA2_0: i32 = 16;
pub const IRQ_MAC_SCSIDRQ: i32 = IRQ_VIA2_0;
pub const IRQ_VIA2_1: i32 = 17;
pub const IRQ_MAC_NUBUS: i32 = IRQ_VIA2_1;
pub const IRQ_VIA2_2: i32 = 18;
pub const IRQ_VIA2_3: i32 = 19;
pub const IRQ_MAC_SCSI: i32 = IRQ_VIA2_3;
pub const IRQ_VIA2_4: i32 = 20;
pub const IRQ_VIA2_5: i32 = 21;
pub const IRQ_VIA2_6: i32 = 22;
pub const IRQ_VIA2_7: i32 = 23;

/* Level 3 (PSC, AV Macs only) interrupts */
pub const IRQ_PSC3_0: i32 = 24;
pub const IRQ_MAC_MACE: i32 = IRQ_PSC3_0;
pub const IRQ_PSC3_1: i32 = 25;
pub const IRQ_PSC3_2: i32 = 26;
pub const IRQ_PSC3_3: i32 = 27;

/* Level 4 (PSC, AV Macs only) interrupts */
pub const IRQ_PSC4_0: i32 = 32;
pub const IRQ_PSC4_1: i32 = 33;
pub const IRQ_MAC_SCC_A: i32 = IRQ_PSC4_1;
pub const IRQ_PSC4_2: i32 = 34;
pub const IRQ_MAC_SCC_B: i32 = IRQ_PSC4_2;
pub const IRQ_PSC4_3: i32 = 35;
pub const IRQ_MAC_MACE_DMA: i32 = IRQ_PSC4_3;

/* OSS Level 4 interrupts */
pub const IRQ_MAC_SCC: i32 = 33;

/* Level 5 (PSC, AV Macs only) interrupts */
pub const IRQ_PSC5_0: i32 = 40;
pub const IRQ_PSC5_1: i32 = 41;
pub const IRQ_PSC5_2: i32 = 42;
pub const IRQ_PSC5_3: i32 = 43;

/* Level 6 (PSC, AV Macs only) interrupts */
pub const IRQ_PSC6_0: i32 = 48;
pub const IRQ_PSC6_1: i32 = 49;
pub const IRQ_PSC6_2: i32 = 50;
pub const IRQ_PSC6_3: i32 = 51;

/* Nubus interrupts (cascaded to VIA2) */
pub const IRQ_NUBUS_9: i32 = 56;
pub const IRQ_NUBUS_A: i32 = 57;
pub const IRQ_NUBUS_B: i32 = 58;
pub const IRQ_NUBUS_C: i32 = 59;
pub const IRQ_NUBUS_D: i32 = 60;
pub const IRQ_NUBUS_E: i32 = 61;
pub const IRQ_NUBUS_F: i32 = 62;

/* Baboon interrupts (cascaded to nubus slot $C) */
pub const IRQ_BABOON_0: i32 = 64;
pub const IRQ_BABOON_1: i32 = 65;
pub const IRQ_BABOON_2: i32 = 66;
pub const IRQ_BABOON_3: i32 = 67;

#[inline]
pub const fn slot2irq(x: i32) -> i32 { x + 47 }

#[inline]
pub const fn irq2slot(x: i32) -> i32 { x - 47 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
