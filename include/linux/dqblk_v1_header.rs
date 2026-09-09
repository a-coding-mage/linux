/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	File with in-memory structures of old quota format
 */

/* Numbers of blocks needed for updates */
pub const V1_INIT_ALLOC: i32 = 1;
pub const V1_INIT_REWRITE: i32 = 1;
pub const V1_DEL_ALLOC: i32 = 0;
pub const V1_DEL_REWRITE: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
