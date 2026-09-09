/* SPDX-License-Identifier: GPL-2.0 */
/*
 * auxio.h: Definitions and code for the Auxiliary I/O registers.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 *
 * Refactoring for unified NCR/PCIO support 2002 Eric Brower (ebrower@usa.net)
 */

/*
 * AUXIO implementations:
 * sbus-based NCR89C105 "Slavio"
 *     LED/Floppy (AUX1) register
 *     Power (AUX2) register
 *
 * ebus-based auxio on PCIO
 *     LED Auxio Register
 *     Power Auxio Register
 *
 * Register definitions from NCR _NCR89C105 Chip Specification_
 *
 * SLAVIO AUX1 @ 0x1900000
 * -------------------------------------------------
 * | (R) | (R) |  D  | (R) |  E  |  M  |  T  |  L  |
 * -------------------------------------------------
 * (R) - bit 7:6,4 are reserved and should be masked in s/w
 *  D  - Floppy Density Sense (1=high density) R/O
 *  E  - Link Test Enable, directly reflected on AT&T 7213 LTE pin
 *  M  - Monitor/Mouse Mux, directly reflected on MON_MSE_MUX pin
 *  T  - Terminal Count: sends TC pulse to 82077 floppy controller
 *  L  - System LED on front panel (0=off, 1=on)
 */
pub const AUXIO_AUX1_MASK: i32 = 0xc0;
pub const AUXIO_AUX1_FDENS: i32 = 0x20;
pub const AUXIO_AUX1_LTE: i32 = 0x08;
pub const AUXIO_AUX1_MMUX: i32 = 0x04;
pub const AUXIO_AUX1_FTCNT: i32 = 0x02;
pub const AUXIO_AUX1_LED: i32 = 0x01;

/* SLAVIO AUX2 @ 0x1910000
 * -------------------------------------------------
 * | (R) | (R) |  D  | (R) | (R) | (R) |  C  |  F  |
 * -------------------------------------------------
 * (R) - bits 7:6,4:2 are reserved and should be masked in s/w
 *  D  - Power Failure Detect (1=power fail)
 *  C  - Clear Power Failure Detect Int (1=clear)
 *  F  - Power Off (1=power off)
 */
pub const AUXIO_AUX2_MASK: i32 = 0xdc;
pub const AUXIO_AUX2_PFAILDET: i32 = 0x20;
pub const AUXIO_AUX2_PFAILCLR: i32 = 0x02;
pub const AUXIO_AUX2_PWR_OFF: i32 = 0x01;

/* Register definitions from Sun Microsystems _PCIO_ p/n 802-7837
 *
 * PCIO LED Auxio @ 0x726000
 * -------------------------------------------------
 * |             31:1 Unused                 | LED |
 * -------------------------------------------------
 * Bits 31:1 unused
 * LED - System LED on front panel (0=off, 1=on)
 */
pub const AUXIO_PCIO_LED: i32 = 0x01;

/* PCIO Power Auxio @ 0x724000
 * -------------------------------------------------
 * |             31:2 Unused           | CPO | SPO |
 * -------------------------------------------------
 * Bits 31:2 unused
 * CPO - Courtesy Power Off (1=off)
 * SPO - System Power Off   (1=off)
 */
pub const AUXIO_PCIO_CPWR_OFF: i32 = 0x02;
pub const AUXIO_PCIO_SPWR_OFF: i32 = 0x01;

pub const AUXIO_LTE_ON: i32 = 1;
pub const AUXIO_LTE_OFF: i32 = 0;

/* auxio_set_lte - Set Link Test Enable (TPE Link Detect)
 *
 * on - AUXIO_LTE_ON or AUXIO_LTE_OFF
 */
extern "C" {
    pub fn auxio_set_lte(on: i32);
}

pub const AUXIO_LED_ON: i32 = 1;
pub const AUXIO_LED_OFF: i32 = 0;

/* auxio_set_led - Set system front panel LED
 *
 * on - AUXIO_LED_ON or AUXIO_LED_OFF
 */
extern "C" {
    pub fn auxio_set_led(on: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
