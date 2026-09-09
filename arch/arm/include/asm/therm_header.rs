/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/include/asm/therm.h: Definitions for Dallas Semiconductor
 *  DS1620 thermometer driver (as used in the Rebel.com NetWinder)
 */

/* ioctl numbers for /dev/therm */
pub const CMD_SET_THERMOSTATE: i32 = 0x53;
pub const CMD_GET_THERMOSTATE: i32 = 0x54;
pub const CMD_GET_STATUS: i32 = 0x56;
pub const CMD_GET_TEMPERATURE: i32 = 0x57;
pub const CMD_SET_THERMOSTATE2: i32 = 0x58;
pub const CMD_GET_THERMOSTATE2: i32 = 0x59;
pub const CMD_GET_TEMPERATURE2: i32 = 0x5a;
pub const CMD_GET_FAN: i32 = 0x5b;
pub const CMD_SET_FAN: i32 = 0x5c;

pub const FAN_OFF: i32 = 0;
pub const FAN_ON: i32 = 1;
pub const FAN_ALWAYS_ON: i32 = 2;

#[repr(C)]
pub struct therm {
    pub hi: i32,
    pub lo: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
