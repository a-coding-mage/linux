/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 *	mcftimer.h -- ColdFire internal TIMER support defines.
 *
 *	(C) Copyright 1999-2006, Greg Ungerer <gerg@snapgear.com>
 * 	(C) Copyright 2000, Lineo Inc. (www.lineo.com) 
 */

/****************************************************************************/

/*
 *	Define the TIMER register set addresses.
 */
pub const MCFTIMER_TMR: u32 = 0x00; /* Timer Mode reg (r/w) */
pub const MCFTIMER_TRR: u32 = 0x04; /* Timer Reference (r/w) */
pub const MCFTIMER_TCR: u32 = 0x08; /* Timer Capture reg (r/w) */
pub const MCFTIMER_TCN: u32 = 0x0C; /* Timer Counter reg (r/w) */

/* CONFIG_M53xx or CONFIG_M5441x selects the alternate timer event address. */
#[cfg(any(feature = "CONFIG_M53xx", feature = "CONFIG_M5441x"))]
pub const MCFTIMER_TER: u32 = 0x03; /* Timer Event reg (r/w) */
#[cfg(not(any(feature = "CONFIG_M53xx", feature = "CONFIG_M5441x")))]
pub const MCFTIMER_TER: u32 = 0x11; /* Timer Event reg (r/w) */

/*
 *	Bit definitions for the Timer Mode Register (TMR).
 *	Register bit flags are common across ColdFires.
 */
pub const MCFTIMER_TMR_PREMASK: u32 = 0xff00; /* Prescalar mask */
pub const MCFTIMER_TMR_DISCE: u32 = 0x0000; /* Disable capture */
pub const MCFTIMER_TMR_ANYCE: u32 = 0x00c0; /* Capture any edge */
pub const MCFTIMER_TMR_FALLCE: u32 = 0x0080; /* Capture fallingedge */
pub const MCFTIMER_TMR_RISECE: u32 = 0x0040; /* Capture rising edge */
pub const MCFTIMER_TMR_ENOM: u32 = 0x0020; /* Enable output toggle */
pub const MCFTIMER_TMR_DISOM: u32 = 0x0000; /* Do single output pulse  */
pub const MCFTIMER_TMR_ENORI: u32 = 0x0010; /* Enable ref interrupt */
pub const MCFTIMER_TMR_DISORI: u32 = 0x0000; /* Disable ref interrupt */
pub const MCFTIMER_TMR_RESTART: u32 = 0x0008; /* Restart counter */
pub const MCFTIMER_TMR_FREERUN: u32 = 0x0000; /* Free running counter */
pub const MCFTIMER_TMR_CLKTIN: u32 = 0x0006; /* Input clock is TIN */
pub const MCFTIMER_TMR_CLK16: u32 = 0x0004; /* Input clock is /16 */
pub const MCFTIMER_TMR_CLK1: u32 = 0x0002; /* Input clock is /1 */
pub const MCFTIMER_TMR_CLKSTOP: u32 = 0x0000; /* Stop counter */
pub const MCFTIMER_TMR_ENABLE: u32 = 0x0001; /* Enable timer */
pub const MCFTIMER_TMR_DISABLE: u32 = 0x0000; /* Disable timer */

/*
 *	Bit definitions for the Timer Event Registers (TER).
 */
pub const MCFTIMER_TER_CAP: u32 = 0x01; /* Capture event */
pub const MCFTIMER_TER_REF: u32 = 0x02; /* Reference event */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
