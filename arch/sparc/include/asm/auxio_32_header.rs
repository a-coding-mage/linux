/* SPDX-License-Identifier: GPL-2.0 */
/*
 * auxio.h:  Definitions and code for the Auxiliary I/O register.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependency: <asm/vaddrs.h> supplies related address definitions.

/* This register is an unsigned char in IO space.  It does two things.
 * First, it is used to control the front panel LED light on machines
 * that have it (good for testing entry points to trap handlers and irq's)
 * Secondly, it controls various floppy drive parameters.
 */
pub const AUXIO_ORMEIN: u8 = 0xf0;    /* All writes must set these bits. */
pub const AUXIO_ORMEIN4M: u8 = 0xc0;  /* sun4m - All writes must set these bits. */
pub const AUXIO_FLPY_DENS: u8 = 0x20; /* Floppy density, high if set. Read only. */
pub const AUXIO_FLPY_DCHG: u8 = 0x10; /* A disk change occurred.  Read only. */
pub const AUXIO_EDGE_ON: u8 = 0x10;   /* sun4m - On means Jumper block is in. */
pub const AUXIO_FLPY_DSEL: u8 = 0x08; /* Drive select/start-motor. Write only. */
pub const AUXIO_LINK_TEST: u8 = 0x08; /* sun4m - On means TPE Carrier detect. */

/* Set the following to one, then zero, after doing a pseudo DMA transfer. */
pub const AUXIO_FLPY_TCNT: u8 = 0x04; /* Floppy terminal count. Write only. */

/* Set the following to zero to eject the floppy. */
pub const AUXIO_FLPY_EJCT: u8 = 0x02; /* Eject floppy disk.  Write only. */
pub const AUXIO_LED: u8 = 0x01;       /* On if set, off if unset. Read/Write */

/*
 * NOTE: these routines are implementation dependent--
 * understand the hardware you are querying!
 */
extern "C" {
    pub fn set_auxio(bits_on: u8, bits_off: u8);
    pub fn get_auxio() -> u8; /* .../asm/floppy.h */
}

/*
 * The following routines are provided for driver-compatibility
 * with sparc64 (primarily sunlance.c)
 */

pub const AUXIO_LTE_ON: i32 = 1;
pub const AUXIO_LTE_OFF: i32 = 0;

/* auxio_set_lte - Set Link Test Enable (TPE Link Detect)
 *
 * on - AUXIO_LTE_ON or AUXIO_LTE_OFF
 */
#[inline]
pub unsafe fn auxio_set_lte(on: i32) {
    if on != 0 {
        set_auxio(AUXIO_LINK_TEST, 0);
    } else {
        set_auxio(0, AUXIO_LINK_TEST);
    }
}

pub const AUXIO_LED_ON: i32 = 1;
pub const AUXIO_LED_OFF: i32 = 0;

/* auxio_set_led - Set system front panel LED
 *
 * on - AUXIO_LED_ON or AUXIO_LED_OFF
 */
#[inline]
pub unsafe fn auxio_set_led(on: i32) {
    if on != 0 {
        set_auxio(AUXIO_LED, 0);
    } else {
        set_auxio(0, AUXIO_LED);
    }
}

/* AUXIO2 (Power Off Control) */
extern "C" {
    /* C declaration: extern volatile u8 __iomem *auxio_power_register; */
    pub static mut auxio_power_register: *mut u8;
}

pub const AUXIO_POWER_DETECT_FAILURE: u8 = 32;
pub const AUXIO_POWER_CLEAR_FAILURE: u8 = 2;
pub const AUXIO_POWER_OFF: u8 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
