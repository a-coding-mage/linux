/*
 * ds1307.h - platform_data for the ds1307 (and variants) rtc driver
 * (C) Copyright 2012 by Wolfram Sang, Pengutronix e.K.
 * same license as the driver
 */

// C dependency: u8 is supplied by <linux/types.h>.

pub const DS1307_TRICKLE_CHARGER_250_OHM: u8 = 0x01;
pub const DS1307_TRICKLE_CHARGER_2K_OHM: u8 = 0x02;
pub const DS1307_TRICKLE_CHARGER_4K_OHM: u8 = 0x03;
pub const DS1307_TRICKLE_CHARGER_NO_DIODE: u8 = 0x04;
pub const DS1307_TRICKLE_CHARGER_DIODE: u8 = 0x08;

#[repr(C)]
pub struct ds1307_platform_data {
    pub trickle_charger_setup: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
