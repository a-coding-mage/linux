/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for most PWM bindings.
 *
 * Most PWM bindings can include a flags cell as part of the PWM specifier.
 * In most cases, the format of the flags cell uses the standard values
 * defined in this header.
 */

pub const PWM_POLARITY_INVERTED: i32 = 1 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
