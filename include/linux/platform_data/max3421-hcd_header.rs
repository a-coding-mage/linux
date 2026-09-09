/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2014 eGauge Systems LLC
 *	Contributed by David Mosberger-Tang <davidm@egauge.net>
 *
 * Platform-data structure for MAX3421 USB HCD driver.
 *
 */

/*
 * This structure defines the mapping of certain auxiliary functions to the
 * MAX3421E GPIO pins.  The chip has eight GP inputs and eight GP outputs.
 * A value of 0 indicates that the pin is not used/wired to anything.
 *
 * At this point, the only control the max3421-hcd driver cares about is
 * to control Vbus (5V to the peripheral).
 */
#[repr(C)]
pub struct max3421_hcd_platform_data {
    pub vbus_gpout: u8,          /* pin controlling Vbus */
    pub vbus_active_level: u8,   /* level that turns on power */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
