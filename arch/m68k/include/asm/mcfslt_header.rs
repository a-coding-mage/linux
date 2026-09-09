/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 *	mcfslt.h -- ColdFire internal Slice (SLT) timer support defines.
 *
 *	(C) Copyright 2004, Greg Ungerer (gerg@snapgear.com)
 *	(C) Copyright 2009, Philippe De Muyter (phdm@macqel.be)
 */

/****************************************************************************/
// Original C header guard: mcfslt_h

/*
 *	Define the SLT timer register set addresses.
 */
pub const MCFSLT_STCNT: u32 = 0x00; /* Terminal count */
pub const MCFSLT_SCR: u32 = 0x04; /* Control */
pub const MCFSLT_SCNT: u32 = 0x08; /* Current count */
pub const MCFSLT_SSR: u32 = 0x0C; /* Status */

/*
 *	Bit definitions for the SCR control register.
 */
pub const MCFSLT_SCR_RUN: u32 = 0x04000000; /* Run mode (continuous) */
pub const MCFSLT_SCR_IEN: u32 = 0x02000000; /* Interrupt enable */
pub const MCFSLT_SCR_TEN: u32 = 0x01000000; /* Timer enable */

/*
 *	Bit definitions for the SSR status register.
 */
pub const MCFSLT_SSR_BE: u32 = 0x02000000; /* Bus error condition */
pub const MCFSLT_SSR_TE: u32 = 0x01000000; /* Timeout condition */

/****************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
