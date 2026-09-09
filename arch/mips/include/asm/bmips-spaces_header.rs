/* SPDX-License-Identifier: GPL-2.0 */

/* Avoid collisions with system base register (SBR) region on BMIPS3300 */
pub const FIXADDR_TOP: usize = 0xff000000usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
