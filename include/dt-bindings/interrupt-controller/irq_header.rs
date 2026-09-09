/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * This header provides constants for most IRQ bindings.
 *
 * Most IRQ bindings include a flags cell as part of the IRQ specifier.
 * In most cases, the format of the flags cell uses the standard values
 * defined in this header.
 */

pub const IRQ_TYPE_NONE: u32 = 0;
pub const IRQ_TYPE_EDGE_RISING: u32 = 1;
pub const IRQ_TYPE_EDGE_FALLING: u32 = 2;
pub const IRQ_TYPE_EDGE_BOTH: u32 = IRQ_TYPE_EDGE_FALLING | IRQ_TYPE_EDGE_RISING;
pub const IRQ_TYPE_LEVEL_HIGH: u32 = 4;
pub const IRQ_TYPE_LEVEL_LOW: u32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
