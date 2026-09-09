/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 *	mcfpit.h -- ColdFire internal PIT timer support defines.
 *
 *	(C) Copyright 2003, Greg Ungerer (gerg@snapgear.com)
 */

/****************************************************************************/

/*
 *	Define the PIT timer register address offsets.
 */
pub const MCFPIT_PCSR: u16 = 0x0; /* PIT control register */
pub const MCFPIT_PMR: u16 = 0x2; /* PIT modulus register */
pub const MCFPIT_PCNTR: u16 = 0x4; /* PIT count register */

/*
 *	Bit definitions for the PIT Control and Status register.
 */
pub const MCFPIT_PCSR_CLK1: u16 = 0x0000; /* System clock divisor */
pub const MCFPIT_PCSR_CLK2: u16 = 0x0100; /* System clock divisor */
pub const MCFPIT_PCSR_CLK4: u16 = 0x0200; /* System clock divisor */
pub const MCFPIT_PCSR_CLK8: u16 = 0x0300; /* System clock divisor */
pub const MCFPIT_PCSR_CLK16: u16 = 0x0400; /* System clock divisor */
pub const MCFPIT_PCSR_CLK32: u16 = 0x0500; /* System clock divisor */
pub const MCFPIT_PCSR_CLK64: u16 = 0x0600; /* System clock divisor */
pub const MCFPIT_PCSR_CLK128: u16 = 0x0700; /* System clock divisor */
pub const MCFPIT_PCSR_CLK256: u16 = 0x0800; /* System clock divisor */
pub const MCFPIT_PCSR_CLK512: u16 = 0x0900; /* System clock divisor */
pub const MCFPIT_PCSR_CLK1024: u16 = 0x0a00; /* System clock divisor */
pub const MCFPIT_PCSR_CLK2048: u16 = 0x0b00; /* System clock divisor */
pub const MCFPIT_PCSR_CLK4096: u16 = 0x0c00; /* System clock divisor */
pub const MCFPIT_PCSR_CLK8192: u16 = 0x0d00; /* System clock divisor */
pub const MCFPIT_PCSR_CLK16384: u16 = 0x0e00; /* System clock divisor */
pub const MCFPIT_PCSR_CLK32768: u16 = 0x0f00; /* System clock divisor */
pub const MCFPIT_PCSR_DOZE: u16 = 0x0040; /* Clock run in doze mode */
pub const MCFPIT_PCSR_HALTED: u16 = 0x0020; /* Clock run in halt mode */
pub const MCFPIT_PCSR_OVW: u16 = 0x0010; /* Overwrite PIT counter now */
pub const MCFPIT_PCSR_PIE: u16 = 0x0008; /* Enable PIT interrupt */
pub const MCFPIT_PCSR_PIF: u16 = 0x0004; /* PIT interrupt flag */
pub const MCFPIT_PCSR_RLD: u16 = 0x0002; /* Reload counter */
pub const MCFPIT_PCSR_EN: u16 = 0x0001; /* Enable PIT */
pub const MCFPIT_PCSR_DISABLE: u16 = 0x0000; /* Disable PIT */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
