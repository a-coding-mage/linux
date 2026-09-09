/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Definitions for the interrupt related bits in the I/O ASIC
 * interrupt status register (and the interrupt mask register, of course)
 *
 * Created with Information from:
 *
 * "DEC 3000 300/400/500/600/700/800/900 AXP Models System Programmer's Manual"
 *
 * and the Mach Sources
 *
 * Copyright (C) 199x  the Anonymous
 * Copyright (C) 2002  Maciej W. Rozycki
 */

/*
 * The upper 16 bits are a part of the I/O ASIC's internal DMA engine
 * and thus are common to all I/O ASIC machines.  The exception is
 * the Maxine, which makes use of the FLOPPY and ISDN bits (otherwise
 * unused) and has a different SCC wiring.
 */
/* all systems */
pub const IO_INR_SCC0A_TXDMA: u32 = 31; /* SCC0A transmit page end */
pub const IO_INR_SCC0A_TXERR: u32 = 30; /* SCC0A transmit memory read error */
pub const IO_INR_SCC0A_RXDMA: u32 = 29; /* SCC0A receive half page */
pub const IO_INR_SCC0A_RXERR: u32 = 28; /* SCC0A receive overrun */
pub const IO_INR_ASC_DMA: u32 = 19; /* ASC buffer pointer loaded */
pub const IO_INR_ASC_ERR: u32 = 18; /* ASC page overrun */
pub const IO_INR_ASC_MERR: u32 = 17; /* ASC memory read error */
pub const IO_INR_LANCE_MERR: u32 = 16; /* LANCE memory read error */

/* except Maxine */
pub const IO_INR_SCC1A_TXDMA: u32 = 27; /* SCC1A transmit page end */
pub const IO_INR_SCC1A_TXERR: u32 = 26; /* SCC1A transmit memory read error */
pub const IO_INR_SCC1A_RXDMA: u32 = 25; /* SCC1A receive half page */
pub const IO_INR_SCC1A_RXERR: u32 = 24; /* SCC1A receive overrun */
pub const IO_INR_RES_23: u32 = 23; /* unused */
pub const IO_INR_RES_22: u32 = 22; /* unused */
pub const IO_INR_RES_21: u32 = 21; /* unused */
pub const IO_INR_RES_20: u32 = 20; /* unused */

/* Maxine */
pub const IO_INR_AB_TXDMA: u32 = 27; /* ACCESS.bus transmit page end */
pub const IO_INR_AB_TXERR: u32 = 26; /* ACCESS.bus xmit memory read error */
pub const IO_INR_AB_RXDMA: u32 = 25; /* ACCESS.bus receive half page */
pub const IO_INR_AB_RXERR: u32 = 24; /* ACCESS.bus receive overrun */
pub const IO_INR_FLOPPY_ERR: u32 = 23; /* FDC error */
pub const IO_INR_ISDN_TXDMA: u32 = 22; /* ISDN xmit buffer pointer loaded */
pub const IO_INR_ISDN_RXDMA: u32 = 21; /* ISDN recv buffer pointer loaded */
pub const IO_INR_ISDN_ERR: u32 = 20; /* ISDN memory read/overrun error */

pub const IO_INR_DMA: u32 = 16; /* first DMA IRQ */

/*
 * The lower 16 bits are system-specific and thus defined in
 * system-specific headers.
 */

pub const IO_IRQ_BASE: u32 = 8; /* first IRQ assigned to I/O ASIC */
pub const IO_IRQ_LINES: u32 = 32; /* number of I/O ASIC interrupts */

#[inline]
pub const fn IO_IRQ_NR(n: u32) -> u32 {
    n.wrapping_add(IO_IRQ_BASE)
}

#[inline]
pub const fn IO_IRQ_MASK(n: u32) -> u32 {
    1u32.wrapping_shl(n)
}

pub const IO_IRQ_ALL: u32 = 0x0000ffff;
pub const IO_IRQ_DMA: u32 = 0xffff0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
