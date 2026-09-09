/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * adv7183.h - definition for adv7183 inputs and outputs
 *
 * Copyright (c) 2011 Analog Devices Inc.
 */

/* ADV7183 HW inputs */
pub const ADV7183_COMPOSITE0: i32 = 0; /* CVBS in on AIN1 */
pub const ADV7183_COMPOSITE1: i32 = 1; /* CVBS in on AIN2 */
pub const ADV7183_COMPOSITE2: i32 = 2; /* CVBS in on AIN3 */
pub const ADV7183_COMPOSITE3: i32 = 3; /* CVBS in on AIN4 */
pub const ADV7183_COMPOSITE4: i32 = 4; /* CVBS in on AIN5 */
pub const ADV7183_COMPOSITE5: i32 = 5; /* CVBS in on AIN6 */
pub const ADV7183_COMPOSITE6: i32 = 6; /* CVBS in on AIN7 */
pub const ADV7183_COMPOSITE7: i32 = 7; /* CVBS in on AIN8 */
pub const ADV7183_COMPOSITE8: i32 = 8; /* CVBS in on AIN9 */
pub const ADV7183_COMPOSITE9: i32 = 9; /* CVBS in on AIN10 */
pub const ADV7183_COMPOSITE10: i32 = 10; /* CVBS in on AIN11 */

pub const ADV7183_SVIDEO0: i32 = 11; /* Y on AIN1, C on AIN4 */
pub const ADV7183_SVIDEO1: i32 = 12; /* Y on AIN2, C on AIN5 */
pub const ADV7183_SVIDEO2: i32 = 13; /* Y on AIN3, C on AIN6 */

pub const ADV7183_COMPONENT0: i32 = 14; /* Y on AIN1, Pr on AIN4, Pb on AIN5 */
pub const ADV7183_COMPONENT1: i32 = 15; /* Y on AIN2, Pr on AIN3, Pb on AIN6 */

/* ADV7183 HW outputs */
pub const ADV7183_8BIT_OUT: i32 = 0;
pub const ADV7183_16BIT_OUT: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
