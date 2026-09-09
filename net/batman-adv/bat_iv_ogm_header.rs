/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

// Dependency intent from the original header: #include "main.h"

unsafe extern "C" {
    pub fn batadv_iv_init() -> i32;
    pub fn batadv_iv_deinit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
