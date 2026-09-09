/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/mach-sa1100/include/mach/neponset.h
 *
 * Created 2000/06/05 by Nicolas Pitre <nico@fluxnic.net>
 *
 * This file contains the hardware specific definitions for Assabet
 * Only include this file from SA1100-specific files.
 *
 * 2000/05/23 John Dorsey <john+@cs.cmu.edu>
 *      Definitions for Neponset added.
 */

/*
 * Neponset definitions:
 */
pub const NCR_GP01_OFF: u32 = 1 << 0;
pub const NCR_TP_PWR_EN: u32 = 1 << 1;
pub const NCR_MS_PWR_EN: u32 = 1 << 2;
pub const NCR_ENET_OSC_EN: u32 = 1 << 3;
pub const NCR_SPI_KB_WK_UP: u32 = 1 << 4;
pub const NCR_A0VPP: u32 = 1 << 5;
pub const NCR_A1VPP: u32 = 1 << 6;

unsafe extern "C" {
    pub fn neponset_ncr_frob(clear: u32, set: u32);
}

#[inline]
pub unsafe fn neponset_ncr_set(v: u32) {
    neponset_ncr_frob(0, v);
}

#[inline]
pub unsafe fn neponset_ncr_clear(v: u32) {
    neponset_ncr_frob(v, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
