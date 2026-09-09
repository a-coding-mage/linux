/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for most Multiplexer bindings.
 *
 * Most Multiplexer bindings specify an idle state. In most cases, the
 * multiplexer can be left as is when idle, and in some cases it can
 * disconnect the input/output and leave the multiplexer in a high
 * impedance state.
 */

pub const MUX_IDLE_AS_IS: i32 = -1;
pub const MUX_IDLE_DISCONNECT: i32 = -2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
