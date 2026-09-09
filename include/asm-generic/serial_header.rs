/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This should not be an architecture specific #define, oh well.
 *
 * Traditionally, it just describes i8250 and related serial ports
 * that have this clock rate.
 */

pub const BASE_BAUD: i32 = 1843200 / 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
