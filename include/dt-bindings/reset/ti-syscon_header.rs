/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * TI Syscon Reset definitions
 *
 * Copyright (C) 2015-2016 Texas Instruments Incorporated - https://www.ti.com/
 */

/*
 * The reset does not support the feature and corresponding
 * values are not valid
 */
pub const ASSERT_NONE: i32 = 1 << 0;
pub const DEASSERT_NONE: i32 = 1 << 1;
pub const STATUS_NONE: i32 = 1 << 2;

/* When set this function is activated by setting(vs clearing) this bit */
pub const ASSERT_SET: i32 = 1 << 3;
pub const DEASSERT_SET: i32 = 1 << 4;
pub const STATUS_SET: i32 = 1 << 5;

/* The following are the inverse of the above and are added for consistency */
pub const ASSERT_CLEAR: i32 = 0 << 3;
pub const DEASSERT_CLEAR: i32 = 0 << 4;
pub const STATUS_CLEAR: i32 = 0 << 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
