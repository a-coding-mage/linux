/*
 * Device Tree constants for Microsemi VSC8531 PHY
 *
 * Author: Nagaraju Lakkaraju
 *
 * License: Dual MIT/GPL
 * Copyright (c) 2017 Microsemi Corporation
 */

// Device Tree constants for Microsemi VSC8531 PHY

/* PHY LED Modes */
pub const VSC8531_LINK_ACTIVITY: i32 = 0;
pub const VSC8531_LINK_1000_ACTIVITY: i32 = 1;
pub const VSC8531_LINK_100_ACTIVITY: i32 = 2;
pub const VSC8531_LINK_10_ACTIVITY: i32 = 3;
pub const VSC8531_LINK_100_1000_ACTIVITY: i32 = 4;
pub const VSC8531_LINK_10_1000_ACTIVITY: i32 = 5;
pub const VSC8531_LINK_10_100_ACTIVITY: i32 = 6;
pub const VSC8584_LINK_100FX_1000X_ACTIVITY: i32 = 7;
pub const VSC8531_DUPLEX_COLLISION: i32 = 8;
pub const VSC8531_COLLISION: i32 = 9;
pub const VSC8531_ACTIVITY: i32 = 10;
pub const VSC8584_100FX_1000X_ACTIVITY: i32 = 11;
pub const VSC8531_AUTONEG_FAULT: i32 = 12;
pub const VSC8531_SERIAL_MODE: i32 = 13;
pub const VSC8531_FORCE_LED_OFF: i32 = 14;
pub const VSC8531_FORCE_LED_ON: i32 = 15;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
