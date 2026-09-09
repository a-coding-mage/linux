/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    cs53l32a.h - definition for cs53l32a inputs and outputs

    Copyright (C) 2006 Hans Verkuil (hverkuil@kernel.org)

*/

/*
 * There are 2 physical inputs, but the second input can be
 * placed in two modes, the first mode bypasses the PGA (gain),
 * the second goes through the PGA. Hence there are three
 * possible inputs to choose from.
 */

/* CS53L32A HW inputs */
pub const CS53L32A_IN0: i32 = 0;
pub const CS53L32A_IN1: i32 = 1;
pub const CS53L32A_IN2: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
