/* linux/i2c/tps6507x-ts.h
 *
 * Functions to access TPS65070 touch screen chip.
 *
 * Copyright (c) 2009 RidgeRun (todd.fischer@ridgerun.com)
 *
 *
 *  For licencing details see kernel-base/COPYING
 */

/* Board specific touch screen initial values */
#[repr(C)]
pub struct touchscreen_init_data {
    pub poll_period: i32, /* ms */
    pub min_pressure: u16, /* min reading to be treated as a touch */
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
