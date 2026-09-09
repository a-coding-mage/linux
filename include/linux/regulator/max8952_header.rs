/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * max8952.h - Voltage regulation for the Maxim 8952
 *
 *  Copyright (C) 2010 Samsung Electronics
 *  MyungJoo Ham <myungjoo.ham@samsung.com>
 */

// Dependency supplied by the Linux regulator machine interface.
pub use crate::regulator_init_data;

pub const MAX8952_DVS_MODE0: u32 = 0;
pub const MAX8952_DVS_MODE1: u32 = 1;
pub const MAX8952_DVS_MODE2: u32 = 2;
pub const MAX8952_DVS_MODE3: u32 = 3;

pub const MAX8952_DVS_770mV: u32 = 0;
pub const MAX8952_DVS_780mV: u32 = 1;
pub const MAX8952_DVS_790mV: u32 = 2;
pub const MAX8952_DVS_800mV: u32 = 3;
pub const MAX8952_DVS_810mV: u32 = 4;
pub const MAX8952_DVS_820mV: u32 = 5;
pub const MAX8952_DVS_830mV: u32 = 6;
pub const MAX8952_DVS_840mV: u32 = 7;
pub const MAX8952_DVS_850mV: u32 = 8;
pub const MAX8952_DVS_860mV: u32 = 9;
pub const MAX8952_DVS_870mV: u32 = 10;
pub const MAX8952_DVS_880mV: u32 = 11;
pub const MAX8952_DVS_890mV: u32 = 12;
pub const MAX8952_DVS_900mV: u32 = 13;
pub const MAX8952_DVS_910mV: u32 = 14;
pub const MAX8952_DVS_920mV: u32 = 15;
pub const MAX8952_DVS_930mV: u32 = 16;
pub const MAX8952_DVS_940mV: u32 = 17;
pub const MAX8952_DVS_950mV: u32 = 18;
pub const MAX8952_DVS_960mV: u32 = 19;
pub const MAX8952_DVS_970mV: u32 = 20;
pub const MAX8952_DVS_980mV: u32 = 21;
pub const MAX8952_DVS_990mV: u32 = 22;
pub const MAX8952_DVS_1000mV: u32 = 23;
pub const MAX8952_DVS_1010mV: u32 = 24;
pub const MAX8952_DVS_1020mV: u32 = 25;
pub const MAX8952_DVS_1030mV: u32 = 26;
pub const MAX8952_DVS_1040mV: u32 = 27;
pub const MAX8952_DVS_1050mV: u32 = 28;
pub const MAX8952_DVS_1060mV: u32 = 29;
pub const MAX8952_DVS_1070mV: u32 = 30;
pub const MAX8952_DVS_1080mV: u32 = 31;
pub const MAX8952_DVS_1090mV: u32 = 32;
pub const MAX8952_DVS_1100mV: u32 = 33;
pub const MAX8952_DVS_1110mV: u32 = 34;
pub const MAX8952_DVS_1120mV: u32 = 35;
pub const MAX8952_DVS_1130mV: u32 = 36;
pub const MAX8952_DVS_1140mV: u32 = 37;
pub const MAX8952_DVS_1150mV: u32 = 38;
pub const MAX8952_DVS_1160mV: u32 = 39;
pub const MAX8952_DVS_1170mV: u32 = 40;
pub const MAX8952_DVS_1180mV: u32 = 41;
pub const MAX8952_DVS_1190mV: u32 = 42;
pub const MAX8952_DVS_1200mV: u32 = 43;
pub const MAX8952_DVS_1210mV: u32 = 44;
pub const MAX8952_DVS_1220mV: u32 = 45;
pub const MAX8952_DVS_1230mV: u32 = 46;
pub const MAX8952_DVS_1240mV: u32 = 47;
pub const MAX8952_DVS_1250mV: u32 = 48;
pub const MAX8952_DVS_1260mV: u32 = 49;
pub const MAX8952_DVS_1270mV: u32 = 50;
pub const MAX8952_DVS_1280mV: u32 = 51;
pub const MAX8952_DVS_1290mV: u32 = 52;
pub const MAX8952_DVS_1300mV: u32 = 53;
pub const MAX8952_DVS_1310mV: u32 = 54;
pub const MAX8952_DVS_1320mV: u32 = 55;
pub const MAX8952_DVS_1330mV: u32 = 56;
pub const MAX8952_DVS_1340mV: u32 = 57;
pub const MAX8952_DVS_1350mV: u32 = 58;
pub const MAX8952_DVS_1360mV: u32 = 59;
pub const MAX8952_DVS_1370mV: u32 = 60;
pub const MAX8952_DVS_1380mV: u32 = 61;
pub const MAX8952_DVS_1390mV: u32 = 62;
pub const MAX8952_DVS_1400mV: u32 = 63;

pub const MAX8952_SYNC_FREQ_26MHZ: u32 = 0; // Default
pub const MAX8952_SYNC_FREQ_13MHZ: u32 = 1;
pub const MAX8952_SYNC_FREQ_19_2MHZ: u32 = 2;

pub const MAX8952_RAMP_32mV_us: u32 = 0; // Default
pub const MAX8952_RAMP_16mV_us: u32 = 1;
pub const MAX8952_RAMP_8mV_us: u32 = 2;
pub const MAX8952_RAMP_4mV_us: u32 = 3;
pub const MAX8952_RAMP_2mV_us: u32 = 4;
pub const MAX8952_RAMP_1mV_us: u32 = 5;
pub const MAX8952_RAMP_0_5mV_us: u32 = 6;
pub const MAX8952_RAMP_0_25mV_us: u32 = 7;

pub const MAX8952_NUM_DVS_MODE: usize = 4;

#[repr(C)]
pub struct max8952_platform_data {
    pub default_mode: u32,
    pub dvs_mode: [u32; MAX8952_NUM_DVS_MODE], // MAX8952_DVS_MODEx_XXXXmV
    pub sync_freq: u32,
    pub ramp_speed: u32,
    pub reg_data: *mut regulator_init_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
