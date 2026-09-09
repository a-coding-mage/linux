/* SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause) */
/*
 * This header provides constants for most GPIO bindings.
 *
 * Most GPIO bindings include a flags cell as part of the GPIO specifier.
 * In most cases, the format of the flags cell uses the standard values
 * defined in this header.
 */

/* Bit 0 express polarity */
pub const GPIO_ACTIVE_HIGH: i32 = 0;
pub const GPIO_ACTIVE_LOW: i32 = 1;

/* Bit 1 express single-endedness */
pub const GPIO_PUSH_PULL: i32 = 0;
pub const GPIO_SINGLE_ENDED: i32 = 2;

/* Bit 2 express Open drain or open source */
pub const GPIO_LINE_OPEN_SOURCE: i32 = 0;
pub const GPIO_LINE_OPEN_DRAIN: i32 = 4;

/*
 * Open Drain/Collector is the combination of single-ended open drain interface.
 * Open Source/Emitter is the combination of single-ended open source interface.
 */
pub const GPIO_OPEN_DRAIN: i32 = GPIO_SINGLE_ENDED | GPIO_LINE_OPEN_DRAIN;
pub const GPIO_OPEN_SOURCE: i32 = GPIO_SINGLE_ENDED | GPIO_LINE_OPEN_SOURCE;

/* Bit 3 express GPIO suspend/resume and reset persistence */
pub const GPIO_PERSISTENT: i32 = 0;
pub const GPIO_TRANSITORY: i32 = 8;

/* Bit 4 express pull up */
pub const GPIO_PULL_UP: i32 = 16;

/* Bit 5 express pull down */
pub const GPIO_PULL_DOWN: i32 = 32;

/* Bit 6 express pull disable */
pub const GPIO_PULL_DISABLE: i32 = 64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
