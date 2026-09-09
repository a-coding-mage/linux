/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 * mcfdma.h -- Coldfire internal DMA support defines.
 *
 * (C) Copyright 1999, Rob Scott (rscott@mtrob.ml.org)
 */

/****************************************************************************/
/* The following declarations correspond to the !defined(CONFIG_M5272) branch. */

/*
 * Define the DMA register set addresses.
 * Note: these are longword registers, use unsigned long as data type.
 */
pub const MCFDMA_SAR: u32 = 0x00; /* DMA source address (r/w) */
pub const MCFDMA_DAR: u32 = 0x01; /* DMA destination adr (r/w) */
/* These are word registers, use unsigned short data type. */
pub const MCFDMA_DCR: u32 = 0x04; /* DMA control reg (r/w) */
pub const MCFDMA_BCR: u32 = 0x06; /* DMA byte count reg (r/w) */
/* These are byte registers, use unsigned char data type. */
pub const MCFDMA_DSR: u32 = 0x10; /* DMA status reg (r/w) */
pub const MCFDMA_DIVR: u32 = 0x14; /* DMA interrupt vec (r/w) */

/* Bit definitions for the DMA Control Register (DCR). */
pub const MCFDMA_DCR_INT: u32 = 0x8000; /* Enable completion irq */
pub const MCFDMA_DCR_EEXT: u32 = 0x4000; /* Enable external DMA req */
pub const MCFDMA_DCR_CS: u32 = 0x2000; /* Enable cycle steal */
pub const MCFDMA_DCR_AA: u32 = 0x1000; /* Enable auto alignment */
pub const MCFDMA_DCR_BWC_MASK: u32 = 0x0E00; /* Bandwidth ctl mask */
pub const MCFDMA_DCR_BWC_512: u32 = 0x0200; /* Bandwidth: 512 Bytes */
pub const MCFDMA_DCR_BWC_1024: u32 = 0x0400; /* Bandwidth: 1024 Bytes */
pub const MCFDMA_DCR_BWC_2048: u32 = 0x0600; /* Bandwidth: 2048 Bytes */
pub const MCFDMA_DCR_BWC_4096: u32 = 0x0800; /* Bandwidth: 4096 Bytes */
pub const MCFDMA_DCR_BWC_8192: u32 = 0x0a00; /* Bandwidth: 8192 Bytes */
pub const MCFDMA_DCR_BWC_16384: u32 = 0x0c00; /* Bandwidth: 16384 Bytes */
pub const MCFDMA_DCR_BWC_32768: u32 = 0x0e00; /* Bandwidth: 32768 Bytes */
pub const MCFDMA_DCR_SAA: u32 = 0x0100; /* Single Address Access */
pub const MCFDMA_DCR_S_RW: u32 = 0x0080; /* SAA read/write value */
pub const MCFDMA_DCR_SINC: u32 = 0x0040; /* Source addr inc enable */
pub const MCFDMA_DCR_SSIZE_MASK: u32 = 0x0030; /* Src xfer size */
pub const MCFDMA_DCR_SSIZE_LONG: u32 = 0x0000; /* Src xfer size, 00 = longw */
pub const MCFDMA_DCR_SSIZE_BYTE: u32 = 0x0010; /* Src xfer size, 01 = byte */
pub const MCFDMA_DCR_SSIZE_WORD: u32 = 0x0020; /* Src xfer size, 10 = word */
pub const MCFDMA_DCR_SSIZE_LINE: u32 = 0x0030; /* Src xfer size, 11 = line */
pub const MCFDMA_DCR_DINC: u32 = 0x0008; /* Dest addr inc enable */
pub const MCFDMA_DCR_DSIZE_MASK: u32 = 0x0006; /* Dest xfer size */
pub const MCFDMA_DCR_DSIZE_LONG: u32 = 0x0000; /* Dest xfer size, 00 = long */
pub const MCFDMA_DCR_DSIZE_BYTE: u32 = 0x0002; /* Dest xfer size, 01 = byte */
pub const MCFDMA_DCR_DSIZE_WORD: u32 = 0x0004; /* Dest xfer size, 10 = word */
pub const MCFDMA_DCR_DSIZE_LINE: u32 = 0x0006; /* Dest xfer size, 11 = line */
pub const MCFDMA_DCR_START: u32 = 0x0001; /* Start transfer */

/* Bit definitions for the DMA Status Register (DSR). */
pub const MCFDMA_DSR_CE: u32 = 0x40; /* Config error */
pub const MCFDMA_DSR_BES: u32 = 0x20; /* Bus Error on source */
pub const MCFDMA_DSR_BED: u32 = 0x10; /* Bus Error on dest */
pub const MCFDMA_DSR_REQ: u32 = 0x04; /* Requests remaining */
pub const MCFDMA_DSR_BSY: u32 = 0x02; /* Busy */
pub const MCFDMA_DSR_DONE: u32 = 0x01; /* DMA transfer complete */

/* The following declarations correspond to the CONFIG_M5272 branch. */
pub const MCFDMA_DMR: u32 = 0x00; /* Mode Register (r/w) */
pub const MCFDMA_DIR: u32 = 0x03; /* Interrupt trigger register (r/w) */
pub const MCFDMA_DSAR: u32 = 0x03; /* Source Address register (r/w) */
pub const MCFDMA_DDAR: u32 = 0x04; /* Destination Address register (r/w) */
pub const MCFDMA_DBCR: u32 = 0x02; /* Byte Count Register (r/w) */

/* Bit definitions for the DMA Mode Register (DMR). */
pub const MCFDMA_DMR_RESET: u32 = 0x80000000; /* Reset bit */
pub const MCFDMA_DMR_EN: u32 = 0x40000000; /* DMA enable */
pub const MCFDMA_DMR_RQM: u32 = 0x000C0000; /* Request Mode Mask */
pub const MCFDMA_DMR_RQM_DUAL: u32 = 0x000C0000; /* Dual address mode, the only valid mode */
pub const MCFDMA_DMR_DSTM: u32 = 0x00002000; /* Destination addressing mask */
pub const MCFDMA_DMR_DSTM_SA: u32 = 0x00000000; /* Destination uses static addressing */
pub const MCFDMA_DMR_DSTM_IA: u32 = 0x00002000; /* Destination uses incremental addressing */
pub const MCFDMA_DMR_DSTT_UD: u32 = 0x00000400; /* Destination is user data */
pub const MCFDMA_DMR_DSTT_UC: u32 = 0x00000800; /* Destination is user code */
pub const MCFDMA_DMR_DSTT_SD: u32 = 0x00001400; /* Destination is supervisor data */
pub const MCFDMA_DMR_DSTT_SC: u32 = 0x00001800; /* Destination is supervisor code */
pub const MCFDMA_DMR_DSTS_OFF: u32 = 0x8; /* offset to the destination size bits */
pub const MCFDMA_DMR_DSTS_LONG: u32 = 0x00000000; /* Long destination size */
pub const MCFDMA_DMR_DSTS_BYTE: u32 = 0x00000100; /* Byte destination size */
pub const MCFDMA_DMR_DSTS_WORD: u32 = 0x00000200; /* Word destination size */
pub const MCFDMA_DMR_DSTS_LINE: u32 = 0x00000300; /* Line destination size */
pub const MCFDMA_DMR_SRCM: u32 = 0x00000020; /* Source addressing mask */
pub const MCFDMA_DMR_SRCM_SA: u32 = 0x00000000; /* Source uses static addressing */
pub const MCFDMA_DMR_SRCM_IA: u32 = 0x00000020; /* Source uses incremental addressing */
pub const MCFDMA_DMR_SRCT_UD: u32 = 0x00000004; /* Source is user data */
pub const MCFDMA_DMR_SRCT_UC: u32 = 0x00000008; /* Source is user code */
pub const MCFDMA_DMR_SRCT_SD: u32 = 0x00000014; /* Source is supervisor data */
pub const MCFDMA_DMR_SRCT_SC: u32 = 0x00000018; /* Source is supervisor code */
pub const MCFDMA_DMR_SRCS_OFF: u32 = 0x0; /* Offset to the source size bits */
pub const MCFDMA_DMR_SRCS_LONG: u32 = 0x00000000; /* Long source size */
pub const MCFDMA_DMR_SRCS_BYTE: u32 = 0x00000001; /* Byte source size */
pub const MCFDMA_DMR_SRCS_WORD: u32 = 0x00000002; /* Word source size */
pub const MCFDMA_DMR_SRCS_LINE: u32 = 0x00000003; /* Line source size */

/* Bit definitions for the DMA interrupt register (DIR). */
pub const MCFDMA_DIR_INVEN: u32 = 0x1000; /* Invalid Combination interrupt enable */
pub const MCFDMA_DIR_ASCEN: u32 = 0x0800; /* Address Sequence Complete (Completion) interrupt enable */
pub const MCFDMA_DIR_TEEN: u32 = 0x0200; /* Transfer Error interrupt enable */
pub const MCFDMA_DIR_TCEN: u32 = 0x0100; /* Transfer Complete (a bus transfer, that is) interrupt enable */
pub const MCFDMA_DIR_INV: u32 = 0x0010; /* Invalid Combination */
pub const MCFDMA_DIR_ASC: u32 = 0x0008; /* Address Sequence Complete (DMA Completion) */
pub const MCFDMA_DIR_TE: u32 = 0x0002; /* Transfer Error */
pub const MCFDMA_DIR_TC: u32 = 0x0001; /* Transfer Complete */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
