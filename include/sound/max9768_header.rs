/*
 * Platform data for MAX9768
 * Copyright (C) 2011, 2012 by Wolfram Sang, Pengutronix e.K.
 * same licence as the driver
 */

/**
 * struct max9768_pdata - optional platform specific MAX9768 configuration
 * @flags: configuration flags, e.g. set classic PWM mode (check datasheet
 *         regarding "filterless modulation" which is default).
 */
#[repr(C)]
pub struct max9768_pdata {
    pub flags: u32,
}

pub const MAX9768_FLAG_CLASSIC_PWM: u32 = 1 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
