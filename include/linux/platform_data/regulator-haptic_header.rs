/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Regulator Haptic Platform Data
 *
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 * Author: Jaewon Kim <jaewon02.kim@samsung.com>
 * Author: Hyunhee Kim <hyunhee.kim@samsung.com>
 */

/*
 * struct regulator_haptic_data - Platform device data
 *
 * @max_volt: maximum voltage value supplied to the haptic motor.
 *		<The unit of the voltage is a micro>
 * @min_volt: minimum voltage value supplied to the haptic motor.
 *		<The unit of the voltage is a micro>
 */
#[repr(C)]
pub struct regulator_haptic_data {
    pub max_volt: u32,
    pub min_volt: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
