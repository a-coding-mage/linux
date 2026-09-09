/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (C) 2025 Renesas Electronics Corporation.
 */

/*
 * Media Interface Connection Matrix
 * ===========================================================
 *
 * Selects the function of the Media interface of the MAC to be used
 *
 * SW_MODE[2:0] | Port 0      | Port 1      | Port 2      | Port 3
 * -------------|-------------|-------------|-------------|-------------
 * 000b         | ETHSW Port0 | ETHSW Port1 | ETHSW Port2 | GMAC1
 * 001b         | ESC Port0   | ESC Port1   | GMAC2       | GMAC1
 * 010b         | ESC Port0   | ESC Port1   | ETHSW Port2 | GMAC1
 * 011b         | ESC Port0   | ESC Port1   | ESC Port2   | GMAC1
 * 100b         | ETHSW Port0 | ESC Port1   | ESC Port2   | GMAC1
 * 101b         | ETHSW Port0 | ESC Port1   | ETHSW Port2 | GMAC1
 * 110b         | ETHSW Port0 | ETHSW Port1 | GMAC2       | GMAC1
 * 111b         | GMAC0       | GMAC1       | GMAC2       | -
 */
pub const ETHSS_GMAC0_PORT: i32 = 0;
pub const ETHSS_GMAC1_PORT: i32 = 1;
pub const ETHSS_GMAC2_PORT: i32 = 2;
pub const ETHSS_ESC_PORT0: i32 = 3;
pub const ETHSS_ESC_PORT1: i32 = 4;
pub const ETHSS_ESC_PORT2: i32 = 5;
pub const ETHSS_ETHSW_PORT0: i32 = 6;
pub const ETHSS_ETHSW_PORT1: i32 = 7;
pub const ETHSS_ETHSW_PORT2: i32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
