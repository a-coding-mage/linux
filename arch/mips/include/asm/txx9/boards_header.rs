/* SPDX-License-Identifier: GPL-2.0 */

// Build-time condition preserved from CONFIG_TOSHIBA_RBTX4927.
#[cfg(CONFIG_TOSHIBA_RBTX4927)]
BOARD_VEC!(rbtx4927_vec);

#[cfg(CONFIG_TOSHIBA_RBTX4927)]
BOARD_VEC!(rbtx4937_vec);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
