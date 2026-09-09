/* SPDX-License-Identifier: GPL-2.0 */

pub const CLK_I2S_CDCLK: i32 = 0; /* the CDCLK (CODECLKO) gate clock */

pub const CLK_I2S_RCLK_SRC: i32 = 1; /* the RCLKSRC mux clock (corresponding to
                                      * RCLKSRC bit in IISMOD register)
                                      */

pub const CLK_I2S_RCLK_PSR: i32 = 2; /* the RCLK prescaler divider clock
                                      * (corresponding to the IISPSR register)
                                      */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
